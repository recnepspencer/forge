use crate::runtime::{ForgeQueryIntentSourceLane, ForgeQueryRuntimeFacadeFamily};

mod certification;
mod decisions;
pub(crate) mod dx;
mod eligibility;
mod families;
mod handoffs;
mod inventory;
mod plans;
mod stops;
mod support;
mod surface_catalog;
mod trace;

pub(crate) use certification::{certification_bridge, certification_runtime};
pub use certification::{
    certify_intent_admission, forge_query_intent_admission_certification_output_manifest,
    forge_query_intent_admission_closeout_extension_outputs,
    forge_query_intent_admission_compile_fail_targets,
    forge_query_intent_admission_crate_doc_example_targets,
    forge_query_intent_admission_doc_example_report,
    forge_query_intent_admission_golden_transcripts,
    forge_query_intent_admission_legacy_parity_report, forge_query_intent_admission_oracle_report,
    forge_query_intent_admission_representative_family_report,
    forge_query_intent_admission_representative_output_report,
    forge_query_intent_admission_required_certification_outputs,
    forge_query_intent_admission_seeded_certification_report,
    forge_query_intent_admission_slope_report,
    forge_query_intent_admission_support_traceability_report,
    ForgeQueryIntentAdmissionCertificationBundle,
    ForgeQueryIntentAdmissionCertificationCounterSnapshot,
    ForgeQueryIntentAdmissionCertificationOutput, ForgeQueryIntentAdmissionCompileFailTarget,
    ForgeQueryIntentAdmissionCrateDocExampleTarget, ForgeQueryIntentAdmissionDocExampleReport,
    ForgeQueryIntentAdmissionDocExampleRow, ForgeQueryIntentAdmissionGoldenTranscript,
    ForgeQueryIntentAdmissionLegacyParityCheck, ForgeQueryIntentAdmissionLegacyParityLane,
    ForgeQueryIntentAdmissionLegacyParityReport, ForgeQueryIntentAdmissionLegacyParityRow,
    ForgeQueryIntentAdmissionOracleComparisonRow, ForgeQueryIntentAdmissionOracleLane,
    ForgeQueryIntentAdmissionOracleManifestRow, ForgeQueryIntentAdmissionOracleReport,
    ForgeQueryIntentAdmissionProofShapeAudit, ForgeQueryIntentAdmissionPublicBoundaryAudit,
    ForgeQueryIntentAdmissionRepresentativeFamilyLane,
    ForgeQueryIntentAdmissionRepresentativeFamilyReport,
    ForgeQueryIntentAdmissionRepresentativeFamilyRow,
    ForgeQueryIntentAdmissionRepresentativeOutputReport,
    ForgeQueryIntentAdmissionSeedGeneratorClass, ForgeQueryIntentAdmissionSeedReplayRow,
    ForgeQueryIntentAdmissionSeededCertificationReport, ForgeQueryIntentAdmissionSlopeLane,
    ForgeQueryIntentAdmissionSlopeReport, ForgeQueryIntentAdmissionSupportTraceabilityReport,
    ForgeQueryIntentAdmissionSupportTraceabilityRow, ForgeQueryIntentAdmissionTopologyAudit,
    ForgeQueryIntentAdmissionTopologyAuditRow, ForgeQueryIntentAdmissionTopologyDomain,
    ForgeQueryIntentAdmissionWidthRunRow, ForgeQueryIntentAdmissionWidthRunScale,
};
pub(crate) use certification::{
    INTENT_ADMISSION_CERTIFICATION_CHILD_MODULES, INTENT_ADMISSION_CERTIFICATION_EXPORTED_SURFACE,
    INTENT_ADMISSION_CERTIFICATION_MODULE_ROOT,
};
pub use decisions::{
    admit_runtime_intent_request, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdvisoryDecision, ForgeQueryIntentViolationDecision,
};
pub(crate) use decisions::{
    INTENT_ADMISSION_DECISIONS_CHILD_MODULES, INTENT_ADMISSION_DECISIONS_EXPORTED_SURFACE,
    INTENT_ADMISSION_DECISIONS_MODULE_ROOT,
};
#[allow(unused_imports)]
pub use dx::{
    forge_query_basis_observation_intent, forge_query_projection_consumption_intent,
    ForgeQueryAdmittedRuntimeEffectWriteIntent, ForgeQueryAdmittedRuntimeExistingTruthProbeIntent,
    ForgeQueryAdmittedRuntimeInspectionIntent, ForgeQueryAdmittedRuntimeIntent,
    ForgeQueryAdmittedRuntimeWriteBatchIntent, ForgeQueryAdmittedRuntimeWriteIntent,
    ForgeQueryAdmittedWorkspaceDerivedInspectionIntent,
    ForgeQueryAdmittedWorkspaceDerivedMaterializationIntent,
    ForgeQueryAdmittedWorkspaceLiveReadIntent, ForgeQueryAdmittedWorkspaceReadIntent,
    ForgeQueryBasisObservationAdmittedIntent, ForgeQueryBasisObservationIntentAuthoring,
    ForgeQueryBasisObservationIntentReview, ForgeQueryProjectionConsumptionAdmittedIntent,
    ForgeQueryProjectionConsumptionIntentAuthoring, ForgeQueryProjectionConsumptionIntentReview,
    ForgeQueryRuntimeEffectWriteIntentAdmissionReview, ForgeQueryRuntimeEffectWriteIntentAuthoring,
    ForgeQueryRuntimeExistingTruthProbeIntentAdmissionReview,
    ForgeQueryRuntimeExistingTruthProbeIntentAuthoring,
    ForgeQueryRuntimeInspectionIntentAdmissionReview, ForgeQueryRuntimeInspectionIntentAuthoring,
    ForgeQueryRuntimeIntentAdmissionReview, ForgeQueryRuntimeIntentAuthoring,
    ForgeQueryRuntimeWriteBatchIntentAdmissionReview, ForgeQueryRuntimeWriteBatchIntentAuthoring,
    ForgeQueryRuntimeWriteIntentAdmissionReview, ForgeQueryRuntimeWriteIntentAuthoring,
    ForgeQueryWorkspaceDerivedInspectionIntentAdmissionReview,
    ForgeQueryWorkspaceDerivedInspectionIntentAuthoring,
    ForgeQueryWorkspaceDerivedMaterializationIntentAdmissionReview,
    ForgeQueryWorkspaceDerivedMaterializationIntentAuthoring,
    ForgeQueryWorkspaceLiveReadIntentAdmissionReview, ForgeQueryWorkspaceLiveReadIntentAuthoring,
    ForgeQueryWorkspaceReadIntentAdmissionReview, ForgeQueryWorkspaceReadIntentAuthoring,
};
pub(crate) use dx::{
    INTENT_ADMISSION_DX_CHILD_MODULES, INTENT_ADMISSION_DX_EXPORTED_SURFACE,
    INTENT_ADMISSION_DX_MODULE_ROOT,
};
pub use eligibility::{
    ForgeQueryAuthoritativeMutationBatchIntentSeed, ForgeQueryAuthoritativeMutationIntentSeed,
    ForgeQueryAuthoritativeMutationPreflight, ForgeQueryDerivedViewIntentSeed,
    ForgeQueryExistingTruthProbeIntentSeed, ForgeQueryExistingTruthProbeRoutingPreflight,
    ForgeQueryGenericInspectionIntentSeed, ForgeQueryGenericInspectionIntentTarget,
    ForgeQueryGenericInspectionIntentTargetSeed, ForgeQueryIntentAdmissionAuthorityLaneEligibility,
    ForgeQueryIntentAdmissionBasisEligibility, ForgeQueryIntentAdmissionCapabilityEligibility,
    ForgeQueryIntentAdmissionEligibility, ForgeQueryIntentAdmissionInvariantEligibility,
    ForgeQueryIntentAdmissionPolicyEligibility, ForgeQueryIntentAdmissionPreDecisionPosture,
    ForgeQueryIntentAdmissionProjectionSourceEligibility,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSourceLaneEligibility, ForgeQueryIntentAdmissionSupportEligibility,
    ForgeQueryLiveReadIntentSeed, ForgeQueryRawIntentAdmissionRequest,
    ForgeQueryReadExecutionIntentSeed,
};
pub(crate) use eligibility::{
    INTENT_ADMISSION_ELIGIBILITY_CHILD_MODULES, INTENT_ADMISSION_ELIGIBILITY_EXPORTED_SURFACE,
    INTENT_ADMISSION_ELIGIBILITY_MODULE_ROOT,
};
pub use families::{
    forge_query_intent_admission_family_inventory, ForgeQueryIntentAdmissionFamily,
    ForgeQueryIntentAdmissionFamilyInventory, ForgeQueryIntentAdmissionFamilyInventoryRow,
};
pub(crate) use families::{
    INTENT_ADMISSION_FAMILIES_CHILD_MODULES, INTENT_ADMISSION_FAMILIES_EXPORTED_SURFACE,
    INTENT_ADMISSION_FAMILIES_MODULE_ROOT,
};
pub(crate) use handoffs::{admit_authoritative_execution, admit_effect_execution};
pub use handoffs::{
    ForgeQueryAdmittedIntentExecutionHandoff, ForgeQueryAuthoritativeIntentExecutionBinding,
    ForgeQueryAuthoritativeIntentExecutionHandoff,
    ForgeQueryAuthoritativeMutationBatchExecutionBinding,
    ForgeQueryAuthoritativeMutationBatchExecutionHandoff,
    ForgeQueryAuthoritativeMutationExecutionBinding,
    ForgeQueryAuthoritativeMutationExecutionHandoff, ForgeQueryDerivedInspectionExecutionBinding,
    ForgeQueryDerivedInspectionExecutionHandoff, ForgeQueryDerivedMaterializationExecutionBinding,
    ForgeQueryDerivedMaterializationExecutionHandoff,
    ForgeQueryEffectTriggeredIntentExecutionBinding,
    ForgeQueryEffectTriggeredIntentExecutionHandoff, ForgeQueryExistingTruthProbeExecutionBinding,
    ForgeQueryExistingTruthProbeExecutionHandoff, ForgeQueryLiveReadExecutionBinding,
    ForgeQueryLiveReadExecutionHandoff, ForgeQueryReadExecutionBinding,
    ForgeQueryReadExecutionHandoff, ForgeQueryUnifiedInspectionExecutionBinding,
    ForgeQueryUnifiedInspectionExecutionHandoff,
};
pub(crate) use handoffs::{
    INTENT_ADMISSION_HANDOFFS_CHILD_MODULES, INTENT_ADMISSION_HANDOFFS_EXPORTED_SURFACE,
    INTENT_ADMISSION_HANDOFFS_MODULE_ROOT,
};
pub use inventory::{
    forge_query_intent_admission_coverage_inventory, forge_query_intent_admission_mutation_audit,
    ForgeQueryIntentAdmissionCoverageInventory, ForgeQueryIntentAdmissionCoverageRow,
    ForgeQueryIntentAdmissionCoverageStatus, ForgeQueryIntentAdmissionCoveredEntrypoint,
    ForgeQueryIntentAdmissionDecisionClass, ForgeQueryIntentAdmissionEligibilityAuthority,
    ForgeQueryIntentAdmissionExecutionBoundary, ForgeQueryIntentAdmissionExecutionHandoffInventory,
    ForgeQueryIntentAdmissionExecutionSeam, ForgeQueryIntentAdmissionMutationAudit,
    ForgeQueryIntentAdmissionMutationAuditRow, ForgeQueryIntentAdmissionPlanKind,
    ForgeQueryIntentAdmissionResultArtifact, ForgeQueryIntentAdmissionSurfaceDescriptor,
};
pub use plans::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryAuthoritativeIntentExecutionPlan,
    ForgeQueryAuthoritativeMutationBatchExecutionPlan,
    ForgeQueryAuthoritativeMutationExecutionPlan, ForgeQueryBasisObservationPlan,
    ForgeQueryDerivedInspectionExecutionPlan, ForgeQueryDerivedMaterializationExecutionPlan,
    ForgeQueryEffectTriggeredIntentExecutionPlan, ForgeQueryExistingTruthProbeExecutionPlan,
    ForgeQueryLiveReadExecutionPlan, ForgeQueryProjectionConsumptionPlan,
    ForgeQueryReadExecutionPlan, ForgeQueryUnifiedInspectionExecutionPlan,
};
pub use stops::{
    ForgeQueryIntentAdvisoryStop, ForgeQueryIntentNonAdmittedStop, ForgeQueryIntentViolationStop,
};
pub use support::{
    forge_query_intent_admission_support_matrix, ForgeQueryIntentAdmissionSupportDetail,
    ForgeQueryIntentAdmissionSupportMatrix, ForgeQueryIntentAdmissionSupportPosture,
    ForgeQueryIntentAdmissionSupportRow,
};
pub(crate) use support::{
    INTENT_ADMISSION_SUPPORT_CHILD_MODULES, INTENT_ADMISSION_SUPPORT_EXPORTED_SURFACE,
    INTENT_ADMISSION_SUPPORT_MODULE_ROOT,
};
pub use trace::{
    ForgeQueryIntentDecisionTraceEnvelope, ForgeQueryIntentDecisionTraceEnvelopeKind,
    ForgeQueryIntentDecisionTraceEvidence, ForgeQueryIntentDecisionTraceEvidenceOwner,
    ForgeQueryIntentDecisionTraceRow, ForgeQueryIntentDecisionTraceStage,
    ForgeQueryIntentEligibilityTraceEvidence,
};
pub(crate) use trace::{
    INTENT_ADMISSION_TRACE_CHILD_MODULES, INTENT_ADMISSION_TRACE_EXPORTED_SURFACE,
    INTENT_ADMISSION_TRACE_MODULE_ROOT,
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
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite => {
            ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite => {
            ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::BasisObservation => {
            ForgeQueryIntentAdmissionFamily::BasisUseIntent
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ProjectionConsumption => {
            ForgeQueryIntentAdmissionFamily::ProjectionConsumptionIntent
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily
        | ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext
        | ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead => {
            ForgeQueryIntentAdmissionFamily::ReadExecutionIntent
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization
        | ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection
        | ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection
        | ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred => {
            ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteExistingTruthProbeRouting => {
            ForgeQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent
        }
    }
}
