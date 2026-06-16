use std::time::Duration;
use anyhow::{Result, anyhow};
use fred::interfaces::{ClientLike, SortedSetsInterface, LuaInterface};
use fred::clients::RedisClient;
use uuid::Uuid;
use chrono::Utc;
use std::future::Future;
use std::pin::Pin;

const ACQUIRE_SCRIPT: &str = r#"
redis.call('ZREMRANGEBYSCORE', KEYS[1], '-inf', ARGV[1])
local count = redis.call('ZCARD', KEYS[1])
if count < tonumber(ARGV[3]) then
    redis.call('ZADD', KEYS[1], ARGV[2], ARGV[4])
    return count + 1
end
return 0
"#;

const GATE_KEY: &str = "cmdp:conngate:global";

#[async_trait::async_trait]
pub trait ConnectionGate: Send + Sync {
    async fn acquire(&self) -> Result<Box<dyn Releaser>>;
}

pub trait Releaser: Send + Sync {
    fn release<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}

pub struct RedisCmdpGlobalConnectionGate {
    client: RedisClient,
    enabled: bool,
    limit: usize,
    slot_ttl_seconds: i64,
    max_retry_attempts: usize,
    retry_delay_ms: u64,
    enable_counter_logging: bool,
}

impl RedisCmdpGlobalConnectionGate {
    pub fn new(
        client: RedisClient,
        enabled: bool,
        limit: usize,
        slot_ttl_seconds: i64,
        max_retry_attempts: usize,
        retry_delay_ms: u64,
        enable_counter_logging: bool,
    ) -> Self {
        Self {
            client,
            enabled,
            limit,
            slot_ttl_seconds,
            max_retry_attempts,
            retry_delay_ms,
            enable_counter_logging,
        }
    }

    async fn try_acquire_slot(&self) -> Result<Option<RedisReleaser>> {
        if !self.enabled {
            return Ok(Some(RedisReleaser::no_op()));
        }

        let now_ms = Utc::now().timestamp_millis();
        let expiry_ms = now_ms + (self.slot_ttl_seconds * 1000);
        let slot_id = Uuid::new_v4().to_string();

        let keys = vec![GATE_KEY];
        let args = vec![
            now_ms.to_string(),
            expiry_ms.to_string(),
            self.limit.to_string(),
            slot_id.clone(),
        ];

        let result: Result<i64, _> = self.client.eval(ACQUIRE_SCRIPT, keys, args).await;

        match result {
            Ok(count) if count > 0 => {
                if self.enable_counter_logging {
                    tracing::warn!(
                        "CMDP global connection gate: slot acquired — activeSlots={}, limit={}, slotId={}.",
                        count, self.limit, slot_id
                    );
                }
                Ok(Some(RedisReleaser::new(self.client.clone(), slot_id, self.slot_ttl_seconds)))
            }
            Ok(_) => Ok(None), // Limit reached
            Err(e) => {
                // Fail open
                tracing::warn!("Redis Lua script failed during global CMDP connection gating; allowing connection through. Error: {}", e);
                Ok(Some(RedisReleaser::no_op()))
            }
        }
    }
}

#[async_trait::async_trait]
impl ConnectionGate for RedisCmdpGlobalConnectionGate {
    async fn acquire(&self) -> Result<Box<dyn Releaser>> {
        if !self.enabled {
            return Ok(Box::new(RedisReleaser::no_op()));
        }

        // If Redis is not connected, fail open immediately
        if !self.client.is_connected() {
            tracing::warn!("Redis is not connected; allowing connection through (fail-open).");
            return Ok(Box::new(RedisReleaser::no_op()));
        }

        // Try to load the script first if not loaded. Fred handles EVALSHA fallback, but we'll let Fred do it.
        // Actually, Fred's evalsha auto-loads if it gets a NOSCRIPT error.

        for attempt in 0..self.max_retry_attempts {
            if let Some(releaser) = self.try_acquire_slot().await? {
                return Ok(Box::new(releaser));
            }

            tracing::warn!(
                "Global CMDP connection limit reached (attempt {}/{}), retrying after {} ms.",
                attempt + 1, self.max_retry_attempts, self.retry_delay_ms
            );

            tokio::time::sleep(Duration::from_millis(self.retry_delay_ms)).await;
        }

        Err(anyhow!(
            "Global CMDP connection limit of {} concurrent connections was exceeded after {} retry attempts.",
            self.limit, self.max_retry_attempts
        ))
    }
}

pub struct RedisReleaser {
    client: Option<RedisClient>,
    slot_id: String,
    slot_ttl_seconds: i64,
    released: bool,
}

impl RedisReleaser {
    fn new(client: RedisClient, slot_id: String, slot_ttl_seconds: i64) -> Self {
        Self {
            client: Some(client),
            slot_id,
            slot_ttl_seconds,
            released: false,
        }
    }

    fn no_op() -> Self {
        Self {
            client: None,
            slot_id: String::new(),
            slot_ttl_seconds: 0,
            released: true,
        }
    }
}

impl Releaser for RedisReleaser {
    fn release<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            if self.released {
                return;
            }
            self.released = true;

            if let Some(client) = &self.client {
                if let Err(e) = client.zrem::<(), _, _>(GATE_KEY, self.slot_id.as_str()).await {
                    tracing::warn!(
                        "Failed to release global CMDP connection slot {} from Redis; slot will expire automatically after {}s. Error: {}",
                        self.slot_id, self.slot_ttl_seconds, e
                    );
                }
            }
        })
    }
}

impl Drop for RedisReleaser {
    fn drop(&mut self) {
        if !self.released {
            // Ideally, the caller should have awaited release(). 
            // In Rust async Drop is not trivial, so we rely on explicit release or letting it expire naturally.
            tracing::warn!("RedisReleaser dropped without explicit release. Slot {} will expire in Redis naturally.", self.slot_id);
        }
    }
}

pub struct NullConnectionGate;

#[async_trait::async_trait]
impl ConnectionGate for NullConnectionGate {
    async fn acquire(&self) -> Result<Box<dyn Releaser>> {
        Ok(Box::new(RedisReleaser::no_op()))
    }
}
