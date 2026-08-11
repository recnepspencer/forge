use crate::runtime::adapter::{EventHarnessAdapter, HarnessAdapter};
use crate::scenario::{ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioFixture};

use super::bundles::HarnessTimelineBundle;
use super::core::{HarnessRunner, LoadedHarnessRun};
use super::error::HarnessError;

impl<A> HarnessRunner<A>
where
    A: HarnessAdapter + EventHarnessAdapter,
    A::TargetId: PartialEq,
{
    pub fn execute_with_events(
        &self,
        fixture: &ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&MutationBatch<A::Mutation>>,
        request: &ExecutionRequest<A::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<HarnessTimelineBundle<A::TargetId>, HarnessError<A::Error>> {
        let LoadedHarnessRun { runtime, core } =
            self.execute_loaded(fixture, mutation_batch, request, profile)?;
        let events = if request.capture.mask.events {
            self.adapter
                .capture_events(&runtime, fixture, request, profile)
                .map_err(HarnessError::Adapter)?
        } else {
            Vec::new()
        };
        Ok(HarnessTimelineBundle {
            core,
            events,
            event_streams: Vec::new(),
            performance: None,
        })
    }
}
