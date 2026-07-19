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

pub(super) fn resolve_inspection_materialization_eligibility(
    request: &WorthQueryRawIntentAdmissionRequest,
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
    let authority_lane = if let Some(seed) = request.derived_view_seed() {
        seed.authority_lane()
    } else if request.generic_inspection_seed().is_some() {
        crate::runtime::WorthQueryAuthorityLane::DerivedRuntimeState
    } else {
        panic!("inspection-materialization request must preserve inspection seed")
    };
    (
        WorthQueryIntentAdmissionSupportEligibility::Admitted,
        WorthQueryIntentAdmissionCapabilityEligibility::Admitted,
        WorthQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
        WorthQueryIntentAdmissionBasisEligibility::NotApplicableForRuntimeIntentFloor,
        WorthQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
        WorthQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor,
        WorthQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
            WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute,
        ),
        WorthQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
        WorthQueryIntentAdmissionAuthorityLaneEligibility::MatchesExpected(authority_lane),
        WorthQueryIntentAdmissionPreDecisionPosture::Admitted,
    )
}
