use std::future::Future;
use std::pin::Pin;

use crate::capture::{RunRecord, SnapshotRecord};
use crate::runtime::adapter::HarnessAdapterAsync;
use crate::scenario::{ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioFixture};

use super::bundles::HarnessCoreBundle;
use super::error::HarnessError;

pub struct AsyncHarnessRunner<A> {
    adapter: A,
}

impl<A> AsyncHarnessRunner<A> {
    pub fn new(adapter: A) -> Self {
        Self { adapter }
    }
}

impl<A> AsyncHarnessRunner<A>
where
    A: HarnessAdapterAsync,
    A::TargetId: PartialEq,
{
    pub fn execute_core_async<'a>(
        &'a self,
        fixture: &'a ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&'a MutationBatch<A::Mutation>>,
        request: &'a ExecutionRequest<A::TargetId>,
        profile: &'a ExecutionProfile,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<HarnessCoreBundle<A::TargetId>, HarnessError<A::Error>>>
                + 'a,
        >,
    > {
        Box::pin(self.execute_async(fixture, mutation_batch, request, profile))
    }

    async fn execute_async(
        &self,
        fixture: &ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&MutationBatch<A::Mutation>>,
        request: &ExecutionRequest<A::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<HarnessCoreBundle<A::TargetId>, HarnessError<A::Error>> {
        let capabilities = self.adapter.capabilities();
        if !capabilities.supports_execution_mode(profile.execution_mode) {
            return Err(HarnessError::UnsupportedExecutionMode(
                profile.execution_mode,
            ));
        }
        let mut runtime = self
            .adapter
            .create_runtime_async()
            .await
            .map_err(HarnessError::Adapter)?;
        self.adapter
            .load_fixture_async(&mut runtime, fixture)
            .await
            .map_err(HarnessError::Adapter)?;
        let scenario = self.adapter.scenario_record(fixture);
        let capture_request = request.clone();
        let pre_snapshot = self
            .capture_optional_snapshot_async(
                &runtime,
                fixture,
                &capture_request,
                profile,
                request.capture.mask.pre_snapshot,
            )
            .await?;
        let run = self
            .apply_mutation_and_execute_async(
                &mut runtime,
                fixture,
                mutation_batch,
                request,
                profile,
            )
            .await?;
        let post_snapshot = self
            .capture_optional_snapshot_async(
                &runtime,
                fixture,
                &capture_request,
                profile,
                request.capture.mask.post_snapshot,
            )
            .await?;
        Ok(HarnessCoreBundle {
            scenario,
            pre_snapshot,
            run,
            post_snapshot,
        })
    }

    async fn capture_optional_snapshot_async(
        &self,
        runtime: &A::Runtime,
        fixture: &ScenarioFixture<A::Fixture>,
        request: &ExecutionRequest<A::TargetId>,
        profile: &ExecutionProfile,
        enabled: bool,
    ) -> Result<Option<SnapshotRecord<A::TargetId>>, HarnessError<A::Error>> {
        if enabled {
            Ok(Some(
                self.adapter
                    .capture_snapshot_async(runtime, fixture, request, profile)
                    .await
                    .map_err(HarnessError::Adapter)?,
            ))
        } else {
            Ok(None)
        }
    }

    async fn apply_mutation_and_execute_async(
        &self,
        runtime: &mut A::Runtime,
        fixture: &ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&MutationBatch<A::Mutation>>,
        request: &ExecutionRequest<A::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<RunRecord<A::TargetId>, HarnessError<A::Error>> {
        if let Some(batch) = mutation_batch {
            self.adapter
                .apply_mutation_batch_async(runtime, batch)
                .await
                .map_err(HarnessError::Adapter)?;
        }
        self.adapter
            .execute_async(runtime, fixture, request, profile)
            .await
            .map_err(HarnessError::Adapter)
    }
}
