use forge_proof::TransitionOutcome;
use forge_query::facade::ForgeQueryRuntimeError;

use super::facade::{ForgeServerDirectProjectionOutcome, ForgeServerForgeNativeDirectFacade};

impl ForgeServerForgeNativeDirectFacade {
    pub(super) fn runtime_error_outcome<T>(
        &self,
        error: ForgeQueryRuntimeError,
    ) -> TransitionOutcome<
        T,
        crate::ForgeServerQueryHandoffDenial,
        crate::ForgeServerQueryHandoffDeferred,
        crate::ForgeServerQueryHandoffStale,
        crate::ForgeServerQueryHandoffRebindRequired,
        crate::ForgeServerQueryHandoffFailure,
    > {
        match error {
            ForgeQueryRuntimeError::MissingLiveView(_)
            | ForgeQueryRuntimeError::MissingLiveSubscription(_) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::RetainedQueryArtifactUnavailable,
                    self.admission.request_context().diagnostics_profile(),
                    error.to_string(),
                ))
            }
            ForgeQueryRuntimeError::MutationBindingDenied(_) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectMutationBindingDenied,
                    self.admission.request_context().diagnostics_profile(),
                    error.to_string(),
                ))
            }
            ForgeQueryRuntimeError::ExistingTruthAssertionDenied(_) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectMutationAssertionDenied,
                    self.admission.request_context().diagnostics_profile(),
                    error.to_string(),
                ))
            }
            ForgeQueryRuntimeError::MutationContinuityDenied(_) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectMutationContinuityDenied,
                    self.admission.request_context().diagnostics_profile(),
                    error.to_string(),
                ))
            }
            ForgeQueryRuntimeError::MutationNamingDenied(_) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectMutationNamingDenied,
                    self.admission.request_context().diagnostics_profile(),
                    error.to_string(),
                ))
            }
            ForgeQueryRuntimeError::MutationTargetReferenceDenied(_) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectMutationTargetReferenceDenied,
                    self.admission.request_context().diagnostics_profile(),
                    error.to_string(),
                ))
            }
            _ => TransitionOutcome::failed(crate::ForgeServerQueryHandoffFailure::new(
                "direct_query_execution_failed",
            )),
        }
    }

    pub(super) fn projection_error_outcome(
        &self,
        error: forge_query::facade::ProjectionFactConsumptionPathError,
    ) -> ForgeServerDirectProjectionOutcome {
        match error {
            forge_query::facade::ProjectionFactConsumptionPathError::Declaration(detail) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectProjectionBindingInvalid,
                    self.admission.request_context().diagnostics_profile(),
                    format!("{detail:?}"),
                ))
            }
            forge_query::facade::ProjectionFactConsumptionPathError::Extraction(_) => {
                TransitionOutcome::failed(crate::ForgeServerQueryHandoffFailure::new(
                    "direct_projection_extraction_failed",
                ))
            }
        }
    }
}
