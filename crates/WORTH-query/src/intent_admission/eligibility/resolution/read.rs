use super::super::artifact::WorthQueryIntentAdmissionPreDecisionPosture;
use super::super::facts::{
    WorthQueryIntentAdmissionAuthorityLaneEligibility, WorthQueryIntentAdmissionBasisEligibility,
    WorthQueryIntentAdmissionCapabilityEligibility, WorthQueryIntentAdmissionInvariantEligibility,
    WorthQueryIntentAdmissionPolicyEligibility,
    WorthQueryIntentAdmissionProjectionSourceEligibility,
    WorthQueryIntentAdmissionRoutingSupportEligibility,
    WorthQueryIntentAdmissionSourceLaneEligibility, WorthQueryIntentAdmissionSupportEligibility,
};
use super::super::request::WorthQueryRawIntentAdmissionRequest;
use crate::intent_admission::WorthQueryIntentAdmissionExecutionSeam;

pub(super) fn resolve_read_execution_eligibility(
    request: &WorthQueryRawIntentAdmissionRequest,
    requires_basis_context: bool,
    requires_live_view_seed: bool,
) -> (
    WorthQueryIntentAdmissionSupportEligibility,
    WorthQueryIntentAdmissionCapabilityEligibility,
    WorthQueryIntentAdmissionPolicyEligibility,
    WorthQueryIntentAdmissionBasisEligibility,
    WorthQueryIntentAdmissionInvariantEligibility,
    WorthQueryIntentAdmissionProjectionSourceEligibility,
    WorthQueryIntentAdmissionRoutingSupportEligibility,
    WorthQueryIntentAdmissionSourceLaneEligibility,
    WorthQueryIntentAdmissionAuthorityLaneEligibility,
    WorthQueryIntentAdmissionPreDecisionPosture,
) {
    if requires_live_view_seed {
        let Some(_seed) = request.live_read_execution_seed() else {
            panic!("live read execution request must preserve live read execution seed");
        };
        return (
            WorthQueryIntentAdmissionSupportEligibility::Admitted,
            WorthQueryIntentAdmissionCapabilityEligibility::Admitted,
            WorthQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
            WorthQueryIntentAdmissionBasisEligibility::ReadExecutionCurrentRuntimeAdmitted,
            WorthQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
            WorthQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor,
            WorthQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
                WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute,
            ),
            WorthQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
            WorthQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
            WorthQueryIntentAdmissionPreDecisionPosture::Admitted,
        );
    }
    let seed = request
        .read_execution_seed()
        .expect("read execution request must preserve read execution seed");
    let (capability, basis, pre_decision) = match seed.basis_context() {
        Some(context) => {
            if context.query_digest() != seed.read_family().read_graph().query_digest() {
                (
                    WorthQueryIntentAdmissionCapabilityEligibility::Violation {
                        stage: "read-basis-context-admission",
                        detail:
                            "read-basis-context query digest does not match requested read family",
                    },
                    WorthQueryIntentAdmissionBasisEligibility::ReadExecutionBasisContextViolation(
                        "read-basis-context query digest does not match requested read family",
                    ),
                    WorthQueryIntentAdmissionPreDecisionPosture::Violation {
                        stage: "read-basis-context-admission",
                        message:
                            "read-basis-context query digest does not match requested read family",
                    },
                )
            } else {
                (
                    WorthQueryIntentAdmissionCapabilityEligibility::Admitted,
                    WorthQueryIntentAdmissionBasisEligibility::ReadExecutionBasisContextAdmitted,
                    WorthQueryIntentAdmissionPreDecisionPosture::Admitted,
                )
            }
        }
        None if requires_basis_context => (
            WorthQueryIntentAdmissionCapabilityEligibility::Violation {
                stage: "read-basis-context-admission",
                detail: "read-basis-context-required-for-covered-entrypoint",
            },
            WorthQueryIntentAdmissionBasisEligibility::ReadExecutionBasisContextViolation(
                "read-basis-context-required-for-covered-entrypoint",
            ),
            WorthQueryIntentAdmissionPreDecisionPosture::Violation {
                stage: "read-basis-context-admission",
                message: "read-basis-context-required-for-covered-entrypoint",
            },
        ),
        None => (
            WorthQueryIntentAdmissionCapabilityEligibility::Admitted,
            WorthQueryIntentAdmissionBasisEligibility::ReadExecutionCurrentRuntimeAdmitted,
            WorthQueryIntentAdmissionPreDecisionPosture::Admitted,
        ),
    };

    (
        WorthQueryIntentAdmissionSupportEligibility::Admitted,
        capability,
        WorthQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
        basis,
        WorthQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
        WorthQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor,
        WorthQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
            WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute,
        ),
        WorthQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
        WorthQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
        pre_decision,
    )
}
