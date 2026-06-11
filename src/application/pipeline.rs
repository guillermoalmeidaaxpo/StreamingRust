use std::sync::Arc;
use std::pin::Pin;
use futures::Stream;
use anyhow::{Result, anyhow};
use crate::domain::{Request, DataItem, SourceKind};
use super::ports::{Repository, Validator, RequestContext, Planner};
use super::transform::TransformationProcessor;
use super::validator::{RequestValidationStrategyResolver, DataRowsNumberValidator};

pub struct Pipeline {
    validator: Arc<dyn Validator>,
    validation_resolver: Arc<RequestValidationStrategyResolver>,
    row_validator: DataRowsNumberValidator,
    planner: Arc<dyn Planner>,
    repositories: std::collections::HashMap<SourceKind, Arc<dyn Repository>>,
    transformer: TransformationProcessor,
}

impl Pipeline {
    pub fn new(
        validator: Arc<dyn Validator>,
        validation_resolver: Arc<RequestValidationStrategyResolver>,
        row_validator: DataRowsNumberValidator,
        planner: Arc<dyn Planner>,
        repositories: std::collections::HashMap<SourceKind, Arc<dyn Repository>>,
    ) -> Self {
        Self {
            validator,
            validation_resolver,
            row_validator,
            planner,
            repositories,
            transformer: TransformationProcessor::new(),
        }
    }

    pub fn row_validator(&self) -> &DataRowsNumberValidator {
        &self.row_validator
    }

    pub async fn execute(&self, ctx: RequestContext, requests: Vec<Request>) -> Result<Vec<DataItem>> {
        // 1. Basic Contract Validation
        self.validator.validate(&requests)?;

        // 2. Resolve Strategy-based Validation
        let strategy = self.validation_resolver.resolve(ctx.data_category)?;
        strategy.validate(&requests)?;

        // 3. Build Plan
        let plan = self.planner.build_plan(ctx, requests).await?;

        // 4. Execute Plan in Parallel
        use futures::stream::{StreamExt, FuturesUnordered};

        let mut futures = FuturesUnordered::new();

        for step in plan.steps {
            let repo = self.repositories.get(&step.command.source)
                .ok_or_else(|| anyhow!("No repository for source {:?}", step.command.source))?
                .clone();
            
            let transformer = self.transformer.clone();
            
            futures.push(tokio::spawn(async move {
                let mut items = repo.execute(step.query).await?;
                transformer.process(&mut items, &step.command);
                Ok::<Vec<DataItem>, anyhow::Error>(items)
            }));
        }

        let mut all_items = Vec::new();
        while let Some(result) = futures.next().await {
            match result {
                Ok(Ok(items)) => all_items.extend(items),
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(anyhow!("Task join error: {}", e)),
            }
        }

        Ok(all_items)
    }

    pub async fn stream(&self, ctx: RequestContext, requests: Vec<Request>) -> Result<Pin<Box<dyn Stream<Item = Result<DataItem>> + Send>>> {
        // 1. Basic Contract Validation
        self.validator.validate(&requests)?;

        // 2. Resolve Strategy-based Validation
        let strategy = self.validation_resolver.resolve(ctx.data_category)?;
        strategy.validate(&requests)?;

        // 3. Build Plan
        let plan = self.planner.build_plan(ctx, requests).await?;

        // 4. Execute Plan as a Stream
        use async_stream::try_stream;
        use futures::stream::StreamExt;

        let repositories = self.repositories.clone();
        let transformer = self.transformer.clone();

        let stream = try_stream! {
            for step in plan.steps {
                let repo = repositories.get(&step.command.source)
                    .ok_or_else(|| anyhow!("No repository for source {:?}", step.command.source))?;
                
                let mut db_stream = repo.stream(step.query).await?;
                while let Some(item_result) = db_stream.next().await {
                    let mut item = item_result?;
                    transformer.process(std::slice::from_mut(&mut item), &step.command);
                    yield item;
                }
            }
        };

        Ok(Box::pin(stream))
    }
}
