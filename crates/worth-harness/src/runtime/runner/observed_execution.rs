use crate::runtime::adapter::{
    DiagnosticsHarnessAdapter, ExplanationHarnessAdapter, HarnessAdapter, ProvenanceHarnessAdapter,
};
use crate::scenario::{ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioFixture};

use super::bundles::HarnessObservedBundle;
use super::core::{HarnessRunner, LoadedHarnessRun};
use super::error::HarnessError;

impl<A> HarnessRunner<A>
where
    A: HarnessAdapter
        + DiagnosticsHarnessAdapter
        + ExplanationHarnessAdapter
        + ProvenanceHarnessAdapter,
    A::TargetId: PartialEq,
{
    pub fn execute_observed(
        &self,
        fixture: &ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&MutationBatch<A::Mutation>>,
        request: &ExecutionRequest<A::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<HarnessObservedBundle<A::TargetId>, HarnessError<A::Error>> {
        let LoadedHarnessRun { runtime, core } =
            self.execute_loaded(fixture, mutation_batch, request, profile)?;

        let capture_request = self.capture_request(request);
        let diagnostics = if request.capture.mask.diagnostics {
            Some(
                self.adapter
                    .capture_diagnostics(&runtime, fixture, profile)
                    .map_err(HarnessError::Adapter)?,
            )
        } else {
            None
        };
        let explanations = if request.capture.mask.explanations {
            self.adapter
                .capture_explanations(&runtime, fixture, &capture_request, profile)
                .map_err(HarnessError::Adapter)?
        } else {
            Vec::new()
        };
        let provenance = if request.capture.mask.provenance {
            self.adapter
                .capture_provenance(&runtime, fixture, &capture_request, profile)
                .map_err(HarnessError::Adapter)?
        } else {
            Vec::new()
        };

        Ok(HarnessObservedBundle {
            core,
            diagnostics,
            explanations,
            provenance,
            events: Vec::new(),
            performance: None,
        })
    }
}
