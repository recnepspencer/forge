use crate::runtime::adapter::{DiagnosticsHarnessAdapter, HarnessAdapter};
use crate::scenario::{ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioFixture};

use super::bundles::HarnessDiagnosedBundle;
use super::core::{HarnessRunner, LoadedHarnessRun};
use super::error::HarnessError;

impl<A> HarnessRunner<A>
where
    A: HarnessAdapter + DiagnosticsHarnessAdapter,
    A::TargetId: PartialEq,
{
    pub fn execute_diagnosed(
        &self,
        fixture: &ScenarioFixture<A::Fixture>,
        mutation_batch: Option<&MutationBatch<A::Mutation>>,
        request: &ExecutionRequest<A::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<HarnessDiagnosedBundle<A::TargetId>, HarnessError<A::Error>> {
        let LoadedHarnessRun { runtime, core } =
            self.execute_loaded(fixture, mutation_batch, request, profile)?;
        let diagnostics = if request.capture.mask.diagnostics {
            Some(
                self.adapter
                    .capture_diagnostics(&runtime, fixture, profile)
                    .map_err(HarnessError::Adapter)?,
            )
        } else {
            None
        };
        Ok(HarnessDiagnosedBundle { core, diagnostics })
    }
}
