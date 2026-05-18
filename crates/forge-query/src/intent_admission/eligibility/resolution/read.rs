use super::super::artifact::ForgeQueryIntentAdmissionPreDecisionPosture;
use super::super::facts::{
    ForgeQueryIntentAdmissionAuthorityLaneEligibility, ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility, ForgeQueryIntentAdmissionInvariantEligibility,
    ForgeQueryIntentAdmissionPolicyEligibility,
    ForgeQueryIntentAdmissionProjectionSourceEligibility,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSourceLaneEligibility, ForgeQueryIntentAdmissionSupportEligibility,
};
use super::super::request::ForgeQueryRawIntentAdmissionRequest;
use crate::intent_admission::ForgeQueryIntentAdmissionExecutionSeam;

pub(super) fn resolve_read_execution_eligibility(
    request: &ForgeQueryRawIntentAdmissionRequest,
    requires_basis_context: bool,
    requires_live_view_seed: bool,
) -> (
    ForgeQueryIntentAdmissionSupportEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility,
    ForgeQueryIntentAdmissionPolicyEligibility,
    ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionInvariantEligibility,
    ForgeQueryIntentAdmissionProjectionSourceEligibility,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSourceLaneEligibility,
    ForgeQueryIntentAdmissionAuthorityLaneEligibility,
    ForgeQueryIntentAdmissionPreDecisionPosture,
) {
    if requires_live_view_seed {
        let Some(_seed) = request.live_read_execution_seed() else {
            panic!("live read execution request must preserve live read execution seed");
        };
        return (
            ForgeQueryIntentAdmissionSupportEligibility::Admitted,
            ForgeQueryIntentAdmissionCapabilityEligibility::Admitted,
            ForgeQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
            ForgeQueryIntentAdmissionBasisEligibility::ReadExecutionCurrentRuntimeAdmitted,
            ForgeQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
            ForgeQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor,
            ForgeQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
                ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute,
            ),
            ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
            ForgeQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
            ForgeQueryIntentAdmissionPreDecisionPosture::Admitted,
        );
    }
    let seed = request
        .read_execution_seed()
        .expect("read execution request must preserve read execution seed");
    let (capability, basis, pre_decision) = match seed.basis_context() {
        Some(context) => {
            if context.query_digest() != seed.read_family().read_graph().query_digest() {
                (
                    ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
                        stage: "read-basis-context-admission",
                        detail:
                            "read-basis-context query digest does not match requested read family",
                    },
                    ForgeQueryIntentAdmissionBasisEligibility::ReadExecutionBasisContextViolation(
                        "read-basis-context query digest does not match requested read family",
                    ),
                    ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
                        stage: "read-basis-context-admission",
                        message:
                            "read-basis-context query digest does not match requested read family",
                    },
                )
            } else {
                (
                    ForgeQueryIntentAdmissionCapabilityEligibility::Admitted,
                    ForgeQueryIntentAdmissionBasisEligibility::ReadExecutionBasisContextAdmitted,
                    ForgeQueryIntentAdmissionPreDecisionPosture::Admitted,
                )
            }
        }
        None if requires_basis_context => (
            ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
                stage: "read-basis-context-admission",
                detail: "read-basis-context-required-for-covered-entrypoint",
            },
            ForgeQueryIntentAdmissionBasisEligibility::ReadExecutionBasisContextViolation(
                "read-basis-context-required-for-covered-entrypoint",
            ),
            ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
                stage: "read-basis-context-admission",
                message: "read-basis-context-required-for-covered-entrypoint",
            },
        ),
        None => (
            ForgeQueryIntentAdmissionCapabilityEligibility::Admitted,
            ForgeQueryIntentAdmissionBasisEligibility::ReadExecutionCurrentRuntimeAdmitted,
            ForgeQueryIntentAdmissionPreDecisionPosture::Admitted,
        ),
    };

    (
        ForgeQueryIntentAdmissionSupportEligibility::Admitted,
        capability,
        ForgeQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
        basis,
        ForgeQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
        ForgeQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
            ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute,
        ),
        ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
        ForgeQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
        pre_decision,
    )
}
