use super::runtime::EligibilityFacts;
use crate::intent_admission::{
    WorthQueryExistingTruthProbeRoutingPreflight, WorthQueryIntentAdmissionExecutionSeam,
};

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

pub(super) fn resolve_existing_truth_probe_routing_eligibility(
    request: &WorthQueryRawIntentAdmissionRequest,
) -> EligibilityFacts {
    let seed = request
        .existing_truth_probe_seed()
        .expect("existing truth probe request must preserve routing seed");
    let (capability_posture, pre_decision_posture) = match seed.preflight() {
        WorthQueryExistingTruthProbeRoutingPreflight::Admitted => (
            WorthQueryIntentAdmissionCapabilityEligibility::Admitted,
            WorthQueryIntentAdmissionPreDecisionPosture::Admitted,
        ),
        WorthQueryExistingTruthProbeRoutingPreflight::BindingDenied(_) => (
            WorthQueryIntentAdmissionCapabilityEligibility::Violation {
                stage: "existing-truth-binding-admission",
                detail: "existing-truth-probe-binding-denied",
            },
            WorthQueryIntentAdmissionPreDecisionPosture::Violation {
                stage: "existing-truth-binding-admission",
                message: "existing-truth-probe-binding-denied",
            },
        ),
        WorthQueryExistingTruthProbeRoutingPreflight::ProbeDenied(_) => (
            WorthQueryIntentAdmissionCapabilityEligibility::Violation {
                stage: "lower-runtime-capability-routing",
                detail: "existing-truth-probe-routing-denied",
            },
            WorthQueryIntentAdmissionPreDecisionPosture::Violation {
                stage: "lower-runtime-capability-routing",
                message: "existing-truth-probe-routing-denied",
            },
        ),
    };

    (
        WorthQueryIntentAdmissionSupportEligibility::Admitted,
        capability_posture,
        WorthQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
        WorthQueryIntentAdmissionBasisEligibility::NotApplicableForRuntimeIntentFloor,
        WorthQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
        WorthQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor,
        WorthQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
            WorthQueryIntentAdmissionExecutionSeam::BackendExistingTruthProbeRoute,
        ),
        WorthQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
        WorthQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
        pre_decision_posture,
    )
}
