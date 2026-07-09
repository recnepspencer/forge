use super::{
    S8FutureLayoutCustomizationAdmission, S8FutureLayoutCustomizationDeferred,
    S8FutureLayoutCustomizationDenial, S8FutureLayoutCustomizationRequest,
};
use crate::strategy_registry::{layout_admission_registry, S8LayoutAdmissionRequest};
use worth_proof::TransitionOutcome;

pub type S8FutureLayoutCustomizationOutcome = TransitionOutcome<
    S8FutureLayoutCustomizationAdmission,
    S8FutureLayoutCustomizationDenial,
    S8FutureLayoutCustomizationDeferred,
>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutCustomizationBoundaryFacade;

impl LayoutCustomizationBoundaryFacade {
    pub fn admit(
        &self,
        request: S8FutureLayoutCustomizationRequest,
    ) -> S8FutureLayoutCustomizationOutcome {
        if request.authority_source().family_id()
            != request.capability_request().key_domain().family_id()
        {
            return TransitionOutcome::denied(
                S8FutureLayoutCustomizationDenial::AuthoritySourceDoesNotMatchKeyDomain,
            );
        }

        let phase_eight_capability = match request.capability_request().phase_eight_capability() {
            Some(capability) => capability,
            None => {
                return TransitionOutcome::denied(
                    S8FutureLayoutCustomizationDenial::RebuildableProjectionNotYetSupported {
                        key_domain: request.capability_request().key_domain(),
                    },
                );
            }
        };

        let admitted_family = match request.capability_request().admitted_strategy_family() {
            Some(family) => family,
            None => {
                return TransitionOutcome::denied(
                    S8FutureLayoutCustomizationDenial::NoStrategySupportsRequestedCapability {
                        capability: request.capability_request(),
                        key_domain: request.capability_request().key_domain(),
                    },
                );
            }
        };

        if !request
            .workload_envelope()
            .supports_capability(request.capability_request())
        {
            return TransitionOutcome::denied(
                S8FutureLayoutCustomizationDenial::WorkloadEnvelopeDoesNotSupportCapability {
                    capability: request.capability_request(),
                    envelope: request.workload_envelope(),
                },
            );
        }

        let layout_request = S8LayoutAdmissionRequest::new(
            request.authority_source(),
            request.capability_request().key_domain(),
            admitted_family,
            phase_eight_capability,
            request.workload_envelope().admitted_lane(),
        );

        match layout_admission_registry().admit_with(layout_request) {
            TransitionOutcome::Success(snapshot) => TransitionOutcome::success(
                S8FutureLayoutCustomizationAdmission::new(request, snapshot),
            ),
            TransitionOutcome::Denied(denial) => TransitionOutcome::denied(
                S8FutureLayoutCustomizationDenial::StoreAdmissionDenied(denial),
            ),
            TransitionOutcome::Deferred(deferred) => TransitionOutcome::deferred(
                S8FutureLayoutCustomizationDeferred::StoreAdmissionDeferred(deferred),
            ),
            TransitionOutcome::Stale(stale) => match stale {},
            TransitionOutcome::RebindRequired(rebind) => match rebind {},
            TransitionOutcome::Failed(failed) => match failed {},
        }
    }
}

pub const fn layout_customization_boundary() -> LayoutCustomizationBoundaryFacade {
    LayoutCustomizationBoundaryFacade
}
