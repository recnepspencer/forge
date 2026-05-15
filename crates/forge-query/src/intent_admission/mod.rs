use crate::runtime::{ForgeQueryIntentSourceLane, ForgeQueryRuntimeFacadeFamily};

mod certification;
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

pub use certification::{
    certify_intent_admission_runtime_floor, forge_query_intent_admission_compile_fail_targets,
    forge_query_intent_admission_doc_example_report,
    forge_query_intent_admission_golden_transcripts,
    forge_query_intent_admission_legacy_parity_report, forge_query_intent_admission_oracle_report,
    forge_query_intent_admission_representative_family_report,
    forge_query_intent_admission_representative_output_report,
    forge_query_intent_admission_seeded_certification_report,
    forge_query_intent_admission_slope_report,
    forge_query_intent_admission_support_traceability_report,
    ForgeQueryIntentAdmissionCertificationBundle,
    ForgeQueryIntentAdmissionCertificationCounterSnapshot,
    ForgeQueryIntentAdmissionCertificationOutput, ForgeQueryIntentAdmissionCompileFailTarget,
    ForgeQueryIntentAdmissionDocExampleReport, ForgeQueryIntentAdmissionDocExampleRow,
    ForgeQueryIntentAdmissionGoldenTranscript, ForgeQueryIntentAdmissionLegacyParityLane,
    ForgeQueryIntentAdmissionLegacyParityReport, ForgeQueryIntentAdmissionLegacyParityRow,
    ForgeQueryIntentAdmissionOracleComparisonRow, ForgeQueryIntentAdmissionOracleLane,
    ForgeQueryIntentAdmissionOracleManifestRow, ForgeQueryIntentAdmissionOracleReport,
    ForgeQueryIntentAdmissionProofShapeAudit, ForgeQueryIntentAdmissionPublicBoundaryAudit,
    ForgeQueryIntentAdmissionRepresentativeFamilyLane,
    ForgeQueryIntentAdmissionRepresentativeFamilyReport,
    ForgeQueryIntentAdmissionRepresentativeFamilyRow,
    ForgeQueryIntentAdmissionRepresentativeOutputReport,
    ForgeQueryIntentAdmissionSeedGeneratorClass, ForgeQueryIntentAdmissionSeedReplayRow,
    ForgeQueryIntentAdmissionSeededCertificationReport, ForgeQueryIntentAdmissionSlopeReport,
    ForgeQueryIntentAdmissionSupportTraceabilityReport,
    ForgeQueryIntentAdmissionSupportTraceabilityRow, ForgeQueryIntentAdmissionTopologyAudit,
    ForgeQueryIntentAdmissionTopologyAuditRow, ForgeQueryIntentAdmissionTopologyDomain,
};
pub use decisions::{
    admit_runtime_intent_request, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdvisoryDecision, ForgeQueryIntentViolationDecision,
};
pub use dx::{
    forge_query_basis_observation_intent, forge_query_projection_consumption_intent,
    ForgeQueryAdmittedRuntimeEffectWriteIntent, ForgeQueryAdmittedRuntimeIntent,
    ForgeQueryBasisObservationAdmittedIntent, ForgeQueryBasisObservationIntentAuthoring,
    ForgeQueryBasisObservationIntentReview, ForgeQueryProjectionConsumptionAdmittedIntent,
    ForgeQueryProjectionConsumptionIntentAuthoring, ForgeQueryProjectionConsumptionIntentReview,
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
    ForgeQueryBasisObservationPlan, ForgeQueryEffectTriggeredIntentExecutionPlan,
    ForgeQueryProjectionConsumptionPlan,
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
    ForgeQueryIntentDecisionTraceEvidence, ForgeQueryIntentDecisionTraceEvidenceOwner,
    ForgeQueryIntentDecisionTraceRow, ForgeQueryIntentDecisionTraceStage,
    ForgeQueryIntentEligibilityTraceEvidence,
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
        ForgeQueryIntentAdmissionCoveredEntrypoint::BasisObservation => {
            ForgeQueryIntentAdmissionFamily::BasisUseIntent
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ProjectionConsumption => {
            ForgeQueryIntentAdmissionFamily::ProjectionConsumptionIntent
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadNeighborDeferred => {
            ForgeQueryIntentAdmissionFamily::ReadExecutionIntent
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred => {
            ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent
        }
    }
}
