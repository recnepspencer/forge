use crate::runtime::adapter::{
    EventStreamHarnessAdapter, HarnessAdapter, PerformanceHarnessAdapter,
};
use crate::scenario::{ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioFixture};

use super::bundles::HarnessTimelineBundle;
use super::core::{HarnessRunner, LoadedHarnessRun};
use super::error::HarnessError;

impl<A> HarnessRunner<A>
where
    A: HarnessAdapter + EventStreamHarnessAdapter + PerformanceHarnessAdapter,
    A::TargetId: PartialEq,
{
    pub fn execute_streamed(
        &self,
        fixture: &ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&MutationBatch<A::Mutation>>,
        request: &ExecutionRequest<A::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<HarnessTimelineBundle<A::TargetId>, HarnessError<A::Error>> {
        let LoadedHarnessRun { runtime, core } =
            self.execute_loaded(fixture, mutation_batch, request, profile)?;
        let event_streams = if request.capture.mask.event_streams {
            self.adapter
                .capture_event_streams(&runtime, fixture, request, profile)
                .map_err(HarnessError::Adapter)?
        } else {
            Vec::new()
        };
        let performance = if request.capture.mask.performance {
            Some(
                self.adapter
                    .capture_performance(&runtime, fixture, profile)
                    .map_err(HarnessError::Adapter)?,
            )
        } else {
            None
        };
        Ok(HarnessTimelineBundle {
            core,
            events: Vec::new(),
            event_streams,
            performance,
        })
    }
}
