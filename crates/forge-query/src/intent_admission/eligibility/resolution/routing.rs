use super::runtime::EligibilityFacts;
use crate::intent_admission::{
    ForgeQueryExistingTruthProbeRoutingPreflight, ForgeQueryIntentAdmissionExecutionSeam,
};

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

pub(super) fn resolve_existing_truth_probe_routing_eligibility(
    request: &ForgeQueryRawIntentAdmissionRequest,
) -> EligibilityFacts {
    let seed = request
        .existing_truth_probe_seed()
        .expect("existing truth probe request must preserve routing seed");
    let (capability_posture, pre_decision_posture) = match seed.preflight() {
        ForgeQueryExistingTruthProbeRoutingPreflight::Admitted => (
            ForgeQueryIntentAdmissionCapabilityEligibility::Admitted,
            ForgeQueryIntentAdmissionPreDecisionPosture::Admitted,
        ),
        ForgeQueryExistingTruthProbeRoutingPreflight::BindingDenied(_) => (
            ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
                stage: "existing-truth-binding-admission",
                detail: "existing-truth-probe-binding-denied",
            },
            ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
                stage: "existing-truth-binding-admission",
                message: "existing-truth-probe-binding-denied",
            },
        ),
        ForgeQueryExistingTruthProbeRoutingPreflight::ProbeDenied(_) => (
            ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
                stage: "lower-runtime-capability-routing",
                detail: "existing-truth-probe-routing-denied",
            },
            ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
                stage: "lower-runtime-capability-routing",
                message: "existing-truth-probe-routing-denied",
            },
        ),
    };

    (
        ForgeQueryIntentAdmissionSupportEligibility::Admitted,
        capability_posture,
        ForgeQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionBasisEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
        ForgeQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
            ForgeQueryIntentAdmissionExecutionSeam::BackendExistingTruthProbeRoute,
        ),
        ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
        ForgeQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
        pre_decision_posture,
    )
}
