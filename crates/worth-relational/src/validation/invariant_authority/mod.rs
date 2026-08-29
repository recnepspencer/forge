mod custom_invariant_provenance_diagnostic_projection;
mod diagnostic_projection;
pub(crate) mod failure_diagnostics;
mod invariant_violation_diagnostic_projection;
pub(crate) mod preparation_diagnostics;

use crate::publication::bundle::PublicationStage;
use crate::publication::data::PublicationError;
use crate::runtime::RelationalRuntime;
use crate::validation::engine::InvariantExecutionResult;

use failure_diagnostics::{emit_collect_all_failure_diagnostics, emit_publication_failure};
use preparation_diagnostics::emit_preparation_diagnostics;

impl RelationalRuntime {
    pub(crate) fn certification_invariant_authority(&self) -> CertificationInvariantAuthority<'_> {
        CertificationInvariantAuthority::new(self)
    }

    pub fn certify_current_state(&self) -> Result<InvariantExecutionResult, PublicationError> {
        self.certification_invariant_authority()
            .enforce_certification_boundary()
    }

    pub(crate) fn publish_invariant_preparation_diagnostics(
        &self,
        results: &[InvariantExecutionResult],
    ) {
        for result in results {
            emit_preparation_diagnostics(self, result);
        }
    }
}

/// Runtime authority for the explicit certification boundary.
///
/// Ordinary commit preparation uses `PreparationInvariantAuthority` instead.
pub(crate) struct CertificationInvariantAuthority<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> CertificationInvariantAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub(crate) fn enforce_certification_boundary(
        &self,
    ) -> Result<InvariantExecutionResult, PublicationError> {
        let result = self.runtime.validation().certification_state();
        emit_preparation_diagnostics(self.runtime, &result);
        let collect_all = emit_collect_all_failure_diagnostics(self.runtime, &result);
        if let Some(failure) = result.summary().publication_failure() {
            if !collect_all {
                emit_publication_failure(self.runtime, &result, failure);
            }
            return Err(failure
                .clone()
                .into_publication_error(PublicationStage::InvariantCheck));
        }
        Ok(result)
    }
}
