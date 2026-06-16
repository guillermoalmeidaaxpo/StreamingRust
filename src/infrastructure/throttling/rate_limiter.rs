use std::time::Duration;
use anyhow::{Result, anyhow};
use fred::clients::RedisClient;
use fred::interfaces::{ClientLike, LuaInterface};
use chrono::Utc;

const INCR_WITH_TTL_SCRIPT: &str = r#"
local current = redis.call('INCR', KEYS[1])
if current == 1 then
    redis.call('EXPIRE', KEYS[1], ARGV[1])
end
return current
"#;

const REDIS_KEY_PREFIX: &str = "cmdp:rl:";

#[async_trait::async_trait]
pub trait QueryRateLimiter: Send + Sync {
    async fn throttle(&self) -> Result<()>;
}

pub struct RedisCmdpQueryRateLimiter {
    client: RedisClient,
    enabled: bool,
    permit_limit: usize,
    window_seconds: i64,
    max_retry_attempts: usize,
    retry_base_delay_ms: u64,
    enable_counter_logging: bool,
}

impl RedisCmdpQueryRateLimiter {
    pub fn new(
        client: RedisClient,
        enabled: bool,
        permit_limit: usize,
        window_seconds: i64,
        max_retry_attempts: usize,
        retry_base_delay_ms: u64,
        enable_counter_logging: bool,
    ) -> Self {
        Self {
            client,
            enabled,
            permit_limit,
            window_seconds,
            max_retry_attempts,
            retry_base_delay_ms,
            enable_counter_logging,
        }
    }

    async fn try_acquire_slot(&self) -> Result<(bool, Duration)> {
        if !self.enabled {
            return Ok((true, Duration::ZERO));
        }

        if !self.client.is_connected() {
            tracing::warn!("Redis is not connected for CMDP rate limiting; allowing query through (fail-open).");
            return Ok((true, Duration::ZERO));
        }

        let window_index = Utc::now().timestamp() / self.window_seconds;
        let key = format!("{}{}", REDIS_KEY_PREFIX, window_index);
        let window_secs = self.window_seconds.to_string();

        let result: Result<i64, _> = self.client.eval(
            INCR_WITH_TTL_SCRIPT,
            vec![key.as_str()],
            vec![window_secs.as_str()],
        ).await;

        let count = match result {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("Redis Lua script failed during CMDP rate limiting; allowing query through (fail-open). Error: {}", e);
                return Ok((true, Duration::ZERO));
            }
        };

        if count as usize <= self.permit_limit {
            if self.enable_counter_logging {
                tracing::warn!(
                    "CMDP rate limiter: slot acquired — counter={}, limit={}, window={}s, key={}.",
                    count, self.permit_limit, self.window_seconds, key
                );
            }
            return Ok((true, Duration::ZERO));
        }

        // Slot not available — wait until the current window expires, plus a random
        // jitter to spread retries uniformly across pods.
        let ttl_ms = match self.client.ttl::<Option<i64>, _>(key.as_str()).await {
            Ok(Some(ttl)) if ttl >= 0 => ttl * 1000,
            _ => self.window_seconds * 1000,
        };

        // Generate a pseudo-random jitter using a UUID to avoid introducing the rand crate dependency
        let jitter = (uuid::Uuid::new_v4().as_u128() as u64) % self.retry_base_delay_ms;
        let retry_after = Duration::from_millis((ttl_ms as u64) + jitter);

        Ok((false, retry_after))
    }
}

#[async_trait::async_trait]
impl QueryRateLimiter for RedisCmdpQueryRateLimiter {
    async fn throttle(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        for attempt in 0..self.max_retry_attempts {
            let (acquired, retry_after) = self.try_acquire_slot().await?;
            if acquired {
                return Ok(());
            }

            tracing::warn!(
                "CMDP rate limit reached (attempt {}/{}), retrying after {} ms.",
                attempt + 1, self.max_retry_attempts, retry_after.as_millis()
            );

            tokio::time::sleep(retry_after).await;
        }

        Err(anyhow!(
            "CMDP query rate limit of {} per {}s was exceeded after {} retry attempts.",
            self.permit_limit, self.window_seconds, self.max_retry_attempts
        ))
    }
}

pub struct NullQueryRateLimiter;

#[async_trait::async_trait]
impl QueryRateLimiter for NullQueryRateLimiter {
    async fn throttle(&self) -> Result<()> {
        Ok(())
    }
}
