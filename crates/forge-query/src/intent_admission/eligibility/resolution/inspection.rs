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

pub(super) fn resolve_inspection_materialization_eligibility(
    request: &ForgeQueryRawIntentAdmissionRequest,
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
    let authority_lane = if let Some(seed) = request.derived_view_seed() {
        seed.authority_lane()
    } else if request.generic_inspection_seed().is_some() {
        crate::runtime::ForgeQueryAuthorityLane::DerivedRuntimeState
    } else {
        panic!("inspection-materialization request must preserve inspection seed")
    };
    (
        ForgeQueryIntentAdmissionSupportEligibility::Admitted,
        ForgeQueryIntentAdmissionCapabilityEligibility::Admitted,
        ForgeQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionBasisEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
        ForgeQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
            ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute,
        ),
        ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
        ForgeQueryIntentAdmissionAuthorityLaneEligibility::MatchesExpected(authority_lane),
        ForgeQueryIntentAdmissionPreDecisionPosture::Admitted,
    )
}
