use super::{
    S8FutureLayoutCustomizationAdmission, S8FutureLayoutCustomizationDeferred,
    S8FutureLayoutCustomizationDenial, S8FutureLayoutCustomizationRequest,
};
use crate::strategy_registry::{
    layout_admission_registry, S8LayoutAdmissionOutcome, S8LayoutAdmissionRequest,
};
use forge_proof::TransitionOutcome;

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

        let admission = layout_admission_registry().admit(layout_request);
        let admission_transition = admission.production_transition();
        match admission.into_result() {
            Ok(snapshot) => TransitionOutcome::success(S8FutureLayoutCustomizationAdmission::new(
                request,
                snapshot,
                admission_transition,
            )),
            Err(denial) => {
                TransitionOutcome::denied(S8FutureLayoutCustomizationDenial::StoreAdmissionDenied(
                    super::S8LayoutAdmissionDenialProjection::new(denial, admission_transition),
                ))
            }
        }
    }
}

pub const fn layout_customization_boundary() -> LayoutCustomizationBoundaryFacade {
    LayoutCustomizationBoundaryFacade
}
