use super::support::*;
use crate::facade::runtime::{
    admit_runtime_intent_request, forge_query_intent_admission_coverage_inventory,
    forge_query_intent_admission_family_inventory, forge_query_intent_admission_support_matrix,
    ForgeQueryAdmittedIntentExecutionHandoff, ForgeQueryAuthoritativeIntentExecutionHandoff,
    ForgeQueryAuthorityLane, ForgeQueryEffectTriggeredIntentExecutionHandoff,
    ForgeQueryIntentAdmissionAuthorityLaneEligibility, ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility, ForgeQueryIntentAdmissionCoverageStatus,
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdmissionDecisionClass, ForgeQueryIntentAdmissionEligibilityAuthority,
    ForgeQueryIntentAdmissionExecutionBoundary, ForgeQueryIntentAdmissionExecutionHandoffInventory,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentAdmissionInvariantEligibility,
    ForgeQueryIntentAdmissionPlanKind, ForgeQueryIntentAdmissionPolicyEligibility,
    ForgeQueryIntentAdmissionPreDecisionPosture,
    ForgeQueryIntentAdmissionProjectionSourceEligibility, ForgeQueryIntentAdmissionResultArtifact,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSourceLaneEligibility, ForgeQueryIntentAdmissionSupportDetail,
    ForgeQueryIntentAdmissionSupportEligibility, ForgeQueryIntentAdmissionSupportPosture,
    ForgeQueryIntentAdmissionSurfaceDescriptor, ForgeQueryIntentDecisionTraceEnvelope,
    ForgeQueryIntentDecisionTraceEnvelopeKind, ForgeQueryIntentDecisionTraceStage,
    ForgeQueryIntentDeclaration, ForgeQueryIntentNonAdmittedStop, ForgeQueryIntentSourceLane,
};
use std::cell::Cell;
use std::rc::Rc;

mod dx;
mod execution;
mod inventory;
mod phase_four;
mod phase_three;
mod phase_two_eligibility;

fn intent_runtime_with_authority<T: ForgeQueryIntentAuthorityAdapter + 'static>(
    authority: T,
) -> ForgeQueryRuntime {
    bridge_runtime_with_support_and_intent_authority(intent_support_profile(), authority)
}

fn trace_stages(
    envelope: &ForgeQueryIntentDecisionTraceEnvelope,
) -> Vec<ForgeQueryIntentDecisionTraceStage> {
    envelope.rows().iter().map(|row| row.stage()).collect()
}
