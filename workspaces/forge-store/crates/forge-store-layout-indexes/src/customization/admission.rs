use super::{
    FutureLayoutCustomizationAdmission, FutureLayoutCustomizationDeferred,
    FutureLayoutCustomizationDenial, FutureLayoutCustomizationRequest,
};
use crate::strategy::registry::{
    layout_admission_registry, LayoutAdmissionRequest, LayoutAdmissionView,
};
use forge_proof::TransitionOutcome;

pub type FutureLayoutCustomizationOutcome = TransitionOutcome<
    FutureLayoutCustomizationAdmission,
    FutureLayoutCustomizationDenial,
    FutureLayoutCustomizationDeferred,
>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutCustomizationBoundaryFacade;

impl LayoutCustomizationBoundaryFacade {
    pub fn admit(
        &self,
        request: FutureLayoutCustomizationRequest,
    ) -> FutureLayoutCustomizationOutcome {
        if request.authority_source() != request.capability_request().admitted_key_domain().family()
        {
            return TransitionOutcome::denied(
                FutureLayoutCustomizationDenial::AuthoritySourceDoesNotMatchKeyDomain,
            );
        }

        let requested_capability = match request.capability_request().requested_capability() {
            Some(capability) => capability,
            None => {
                return TransitionOutcome::denied(
                    FutureLayoutCustomizationDenial::RebuildableProjectionNotYetSupported {
                        key_domain: request.capability_request().key_domain(),
                    },
                );
            }
        };

        let admitted_family = match request.capability_request().admitted_strategy_family() {
            Some(family) => family,
            None => {
                return TransitionOutcome::denied(
                    FutureLayoutCustomizationDenial::NoStrategySupportsRequestedCapability {
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
                FutureLayoutCustomizationDenial::WorkloadEnvelopeDoesNotSupportCapability {
                    capability: request.capability_request(),
                    envelope: request.workload_envelope(),
                },
            );
        }

        let layout_request = LayoutAdmissionRequest::from_admitted(
            request.authority_source(),
            request.capability_request().admitted_key_domain(),
            admitted_family,
            requested_capability,
            request.workload_envelope().admitted_lane(),
        );

        let admission = layout_admission_registry().admit(layout_request);
        match admission.view() {
            LayoutAdmissionView::Admitted(snapshot) => TransitionOutcome::success(
                FutureLayoutCustomizationAdmission::new(request, snapshot.clone()),
            ),
            LayoutAdmissionView::Denied(denial) => {
                TransitionOutcome::denied(FutureLayoutCustomizationDenial::StoreAdmissionDenied(
                    super::LayoutAdmissionDenialProjection::new(denial.clone()),
                ))
            }
        }
    }
}

pub const fn layout_customization_boundary() -> LayoutCustomizationBoundaryFacade {
    LayoutCustomizationBoundaryFacade
}
