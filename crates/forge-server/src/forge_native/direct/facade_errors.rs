use forge_proof::TransitionOutcome;
use forge_query::facade::ForgeQueryRuntimeError;

use crate::ForgeServerOperationDenial;

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
            ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily,
                    self.admission.request_context().diagnostics_profile(),
                    format!(
                        "query workspace does not admit `{}` facade family: {}",
                        denial.family().as_str(),
                        denial.reason()
                    ),
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

    pub(super) fn operation_denial_outcome<T>(
        &self,
        denial: ForgeServerOperationDenial,
    ) -> TransitionOutcome<
        T,
        crate::ForgeServerQueryHandoffDenial,
        crate::ForgeServerQueryHandoffDeferred,
        crate::ForgeServerQueryHandoffStale,
        crate::ForgeServerQueryHandoffRebindRequired,
        crate::ForgeServerQueryHandoffFailure,
    > {
        let (code, facts) = match denial {
            ForgeServerOperationDenial::UnregisteredFamily { .. } => (
                crate::ForgeServerQueryHandoffDenialCode::OperationFamilyNotRegistered,
                None,
            ),
            ForgeServerOperationDenial::DisabledFamily { .. } => (
                crate::ForgeServerQueryHandoffDenialCode::OperationFamilyDisabled,
                None,
            ),
            ForgeServerOperationDenial::SurfaceFamilyNotExposed { .. } => (
                crate::ForgeServerQueryHandoffDenialCode::OperationFamilyNotExposedOnSurface,
                None,
            ),
            ForgeServerOperationDenial::UnknownOperationName {
                ref operation_name, ..
            } => (
                crate::ForgeServerQueryHandoffDenialCode::UnknownOperationName,
                Some(
                    crate::ForgeServerQueryHandoffDenialFacts::default()
                        .with_rejected_operation_name(operation_name.clone()),
                ),
            ),
        };
        let denial = crate::ForgeServerQueryHandoffDenial::new(
            code,
            self.admission.request_context().diagnostics_profile(),
            denial.detail(),
        );
        TransitionOutcome::Denied(match facts {
            Some(facts) => denial.with_facts(facts),
            None => denial,
        })
    }

    pub(super) fn direct_mutation_scheduler_runtime_outcome<T>(
        &self,
        runtime_failure: &crate::ForgeServerSchedulerRuntimeFailure,
    ) -> TransitionOutcome<
        T,
        crate::ForgeServerQueryHandoffDenial,
        crate::ForgeServerQueryHandoffDeferred,
        crate::ForgeServerQueryHandoffStale,
        crate::ForgeServerQueryHandoffRebindRequired,
        crate::ForgeServerQueryHandoffFailure,
    > {
        match runtime_failure {
            crate::ForgeServerSchedulerRuntimeFailure::DirectMutationAssertionDenied { detail } => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectMutationAssertionDenied,
                    self.admission.request_context().diagnostics_profile(),
                    detail.clone(),
                ))
            }
            crate::ForgeServerSchedulerRuntimeFailure::DirectMutationBindingDenied { detail } => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectMutationBindingDenied,
                    self.admission.request_context().diagnostics_profile(),
                    detail.clone(),
                ))
            }
            crate::ForgeServerSchedulerRuntimeFailure::DirectMutationContinuityDenied {
                detail,
            } => TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                crate::ForgeServerQueryHandoffDenialCode::DirectMutationContinuityDenied,
                self.admission.request_context().diagnostics_profile(),
                detail.clone(),
            )),
            crate::ForgeServerSchedulerRuntimeFailure::DirectMutationNamingDenied { detail } => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectMutationNamingDenied,
                    self.admission.request_context().diagnostics_profile(),
                    detail.clone(),
                ))
            }
            crate::ForgeServerSchedulerRuntimeFailure::DirectMutationTargetReferenceDenied {
                detail,
            } => TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                crate::ForgeServerQueryHandoffDenialCode::DirectMutationTargetReferenceDenied,
                self.admission.request_context().diagnostics_profile(),
                detail.clone(),
            )),
            crate::ForgeServerSchedulerRuntimeFailure::Opaque { .. } => {
                TransitionOutcome::failed(crate::ForgeServerQueryHandoffFailure::new(
                    "direct_mutation_scheduler_runtime_failed",
                ))
            }
        }
    }
}
