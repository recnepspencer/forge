use crate::runtime::{ForgeQueryIntentSourceLane, ForgeQueryRuntimeFacadeFamily};

mod decisions;
pub(crate) mod dx;
mod eligibility;
mod execution_bindings;
mod families;
mod handoffs;
mod inventory;
mod plans;
mod stops;
mod support;
mod trace;

pub use decisions::{
    admit_runtime_intent_request, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdvisoryDecision, ForgeQueryIntentViolationDecision,
};
pub use dx::{
    ForgeQueryAdmittedRuntimeEffectWriteIntent, ForgeQueryAdmittedRuntimeIntent,
    ForgeQueryRuntimeEffectWriteIntentAdmissionReview, ForgeQueryRuntimeEffectWriteIntentAuthoring,
    ForgeQueryRuntimeIntentAdmissionReview, ForgeQueryRuntimeIntentAuthoring,
};
pub use eligibility::{
    ForgeQueryIntentAdmissionAuthorityLaneEligibility, ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility, ForgeQueryIntentAdmissionEligibility,
    ForgeQueryIntentAdmissionInvariantEligibility, ForgeQueryIntentAdmissionPolicyEligibility,
    ForgeQueryIntentAdmissionPreDecisionPosture,
    ForgeQueryIntentAdmissionProjectionSourceEligibility,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSourceLaneEligibility, ForgeQueryIntentAdmissionSupportEligibility,
    ForgeQueryRawIntentAdmissionRequest,
};
pub use execution_bindings::{
    ForgeQueryAuthoritativeIntentExecutionBinding, ForgeQueryEffectTriggeredIntentExecutionBinding,
};
pub use families::{
    forge_query_intent_admission_family_inventory, ForgeQueryIntentAdmissionFamily,
    ForgeQueryIntentAdmissionFamilyInventory, ForgeQueryIntentAdmissionFamilyInventoryRow,
};
pub(crate) use handoffs::{admit_authoritative_execution, admit_effect_execution};
pub use handoffs::{
    ForgeQueryAdmittedIntentExecutionHandoff, ForgeQueryAuthoritativeIntentExecutionHandoff,
    ForgeQueryEffectTriggeredIntentExecutionHandoff,
};
pub use inventory::{
    forge_query_intent_admission_coverage_inventory, ForgeQueryIntentAdmissionCoverageInventory,
    ForgeQueryIntentAdmissionCoverageRow, ForgeQueryIntentAdmissionCoverageStatus,
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionDecisionClass,
    ForgeQueryIntentAdmissionEligibilityAuthority, ForgeQueryIntentAdmissionExecutionBoundary,
    ForgeQueryIntentAdmissionExecutionHandoffInventory, ForgeQueryIntentAdmissionExecutionSeam,
    ForgeQueryIntentAdmissionPlanKind, ForgeQueryIntentAdmissionResultArtifact,
    ForgeQueryIntentAdmissionSurfaceDescriptor,
};
pub use plans::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryAuthoritativeIntentExecutionPlan,
    ForgeQueryEffectTriggeredIntentExecutionPlan,
};
pub use stops::{
    ForgeQueryIntentAdvisoryStop, ForgeQueryIntentNonAdmittedStop, ForgeQueryIntentViolationStop,
};
pub use support::{
    forge_query_intent_admission_support_matrix, ForgeQueryIntentAdmissionSupportDetail,
    ForgeQueryIntentAdmissionSupportMatrix, ForgeQueryIntentAdmissionSupportPosture,
    ForgeQueryIntentAdmissionSupportRow,
};
pub use trace::{
    ForgeQueryIntentDecisionTraceEnvelope, ForgeQueryIntentDecisionTraceEnvelopeKind,
    ForgeQueryIntentDecisionTraceRow, ForgeQueryIntentDecisionTraceStage,
};

pub(crate) fn intent_runtime_facade_family(
    source_lane: ForgeQueryIntentSourceLane,
) -> ForgeQueryRuntimeFacadeFamily {
    match source_lane {
        ForgeQueryIntentSourceLane::EffectTriggered => ForgeQueryRuntimeFacadeFamily::Effect,
        ForgeQueryIntentSourceLane::UserAuthored
        | ForgeQueryIntentSourceLane::PreviewLocal
        | ForgeQueryIntentSourceLane::BranchLocal
        | ForgeQueryIntentSourceLane::DerivedRuntime => ForgeQueryRuntimeFacadeFamily::Intent,
    }
}

pub(crate) fn intent_family_for_entrypoint(
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
) -> ForgeQueryIntentAdmissionFamily {
    match entrypoint {
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent => {
            ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent => {
            ForgeQueryIntentAdmissionFamily::EffectTriggeredWriteIntent
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadNeighborDeferred => {
            ForgeQueryIntentAdmissionFamily::ReadExecutionIntent
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred => {
            ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent
        }
    }
}
