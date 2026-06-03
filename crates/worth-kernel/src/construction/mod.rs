#[path = "phase_chain/admitted_scaffold/mod.rs"]
mod admitted_scaffold;
#[path = "result_surface/artifact.rs"]
mod artifact;
mod authoring;
mod authoring_authority;
mod authoring_entry;
mod authoring_input;
#[path = "certification/mod.rs"]
mod certification;
#[path = "runtime_proof/continuity_branch_runtime.rs"]
mod continuity_branch_runtime;
#[path = "runtime_proof/continuity_replay.rs"]
mod continuity_replay;
#[path = "runtime_proof/diagnostics.rs"]
mod diagnostics;
mod digest;
#[path = "result_surface/evidence.rs"]
mod evidence;
#[path = "runtime_proof/family_coverage.rs"]
mod family_coverage;
#[path = "phase_chain/intent.rs"]
mod intent;
#[path = "runtime_proof/arbitration/replay.rs"]
mod intent_arbitration_replay;
#[path = "runtime_proof/motion/branch_runtime.rs"]
mod motion_branch_runtime;
#[path = "runtime_proof/motion/replay.rs"]
mod motion_replay;
#[path = "result_surface/outcome.rs"]
mod outcome;
#[path = "runtime_proof/parity.rs"]
mod parity;
#[path = "runtime_proof/preview_branch_runtime.rs"]
mod preview_branch_runtime;
#[path = "runtime_proof/preview_replay.rs"]
mod preview_replay;
#[path = "runtime_proof/profile_branch_runtime.rs"]
mod profile_branch_runtime;
#[path = "runtime_proof/profile_replay.rs"]
mod profile_replay;
#[path = "proof/mod.rs"]
mod proof;
#[path = "runtime_proof/query/mod.rs"]
mod query;
#[path = "runtime_proof/realization_truth.rs"]
mod realization_truth;
#[path = "phase_chain/request.rs"]
mod request;
#[path = "result_surface/result.rs"]
mod result;
#[path = "runtime_proof/runtime_basis.rs"]
mod runtime_basis;
#[path = "phase_chain/specs.rs"]
mod specs;

pub use crate::spatial_intent::PrimitiveConstructionSpatialIntentError;
pub use authoring::{
    primitive_construction_authoring, PrimitiveConstructionAuthoringSession,
    PrimitiveConstructionAuthorityChainReport, PrimitiveConstructionQueryEntryError,
    WorthKernelAuthorityError,
};
pub use authoring_entry::PrimitiveConstructionAuthoringEntry;
pub use authoring_input::{
    PrimitiveConstructionAuthoringInput, PrimitiveConstructionCatalogAuthoringInput,
};
pub use certification::{
    prepare_primitive_chosen_intent_resolution_report,
    prepare_primitive_construction_conditioning_witness_report,
    prepare_primitive_construction_continuity_hostility_suite_report,
    prepare_primitive_construction_continuity_surface_report,
    prepare_primitive_construction_intent_arbitration_hostility_suite_report,
    prepare_primitive_construction_motion_dx_surface_report,
    prepare_primitive_construction_motion_resolution_policy_report,
    prepare_primitive_construction_move_witness_resolution_report,
    prepare_primitive_construction_move_witness_resolution_report_with_catalog,
    prepare_primitive_construction_points_toward_witness_resolution_report,
    prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog,
    prepare_primitive_construction_policy_pressure_delta_report,
    prepare_primitive_construction_policy_pressure_report,
    prepare_primitive_construction_policy_profile_report,
    prepare_primitive_construction_preserved_intent_resolution_report,
    prepare_primitive_construction_preview_continuity_hostility_suite_report,
    prepare_primitive_construction_preview_hostility_suite_report,
    prepare_primitive_construction_preview_surface_report,
    prepare_primitive_construction_realization_exhaustion_report,
    prepare_primitive_construction_realization_exhaustion_witness_report,
    prepare_primitive_construction_realization_report_bundle,
    prepare_primitive_construction_realization_strategy_report,
    prepare_primitive_construction_reorient_witness_resolution_report,
    prepare_primitive_construction_reorient_witness_resolution_report_with_catalog,
    prepare_primitive_construction_rotate_witness_resolution_report,
    prepare_primitive_construction_rotate_witness_resolution_report_with_catalog,
    prepare_primitive_construction_stability_class_report,
    prepare_primitive_intent_arbitration_policy_report,
    prepare_primitive_intent_conflict_dx_surface_report,
    PrimitiveConstructionChosenIntentResolutionAuthority,
    PrimitiveConstructionChosenIntentResolutionCase,
    PrimitiveConstructionChosenIntentResolutionReport,
    PrimitiveConstructionChosenIntentResolutionReportError,
    PrimitiveConstructionChosenIntentResolutionRow, PrimitiveConstructionConditioningWitnessReport,
    PrimitiveConstructionContinuityCase, PrimitiveConstructionContinuityHostilitySuiteReport,
    PrimitiveConstructionContinuityResolutionSource, PrimitiveConstructionContinuityRow,
    PrimitiveConstructionContinuitySurfaceReport,
    PrimitiveConstructionContinuitySurfaceReportError,
    PrimitiveConstructionFamilyBoundaryLowerLayerWitnessSummary,
    PrimitiveConstructionIntentArbitrationBundleCase,
    PrimitiveConstructionIntentArbitrationConflictClass,
    PrimitiveConstructionIntentArbitrationDxSurface,
    PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    PrimitiveConstructionIntentArbitrationDxSurfaceRow,
    PrimitiveConstructionIntentArbitrationHostilitySuiteReport,
    PrimitiveConstructionIntentArbitrationPolicyCase,
    PrimitiveConstructionIntentArbitrationPolicyReport,
    PrimitiveConstructionIntentArbitrationPolicyReportError,
    PrimitiveConstructionIntentArbitrationPolicyRow, PrimitiveConstructionMotionDxSurface,
    PrimitiveConstructionMotionDxSurfaceReport, PrimitiveConstructionMotionDxSurfaceReportError,
    PrimitiveConstructionMotionDxSurfaceRow, PrimitiveConstructionMotionResolutionPolicyCase,
    PrimitiveConstructionMotionResolutionPolicyReport,
    PrimitiveConstructionMotionResolutionPolicyReportError,
    PrimitiveConstructionMotionResolutionPolicyRow,
    PrimitiveConstructionMotionWitnessResolutionFailureKind,
    PrimitiveConstructionMotionWitnessResolutionKind,
    PrimitiveConstructionMotionWitnessResolutionReport,
    PrimitiveConstructionMotionWitnessResolutionStatus,
    PrimitiveConstructionObservedIntentRelation, PrimitiveConstructionPolicyPressureCase,
    PrimitiveConstructionPolicyPressureDeltaCase, PrimitiveConstructionPolicyPressureDeltaReport,
    PrimitiveConstructionPolicyPressureDeltaReportError,
    PrimitiveConstructionPolicyPressureDeltaRow, PrimitiveConstructionPolicyPressureRow,
    PrimitiveConstructionPolicyPressureSetup, PrimitiveConstructionPolicyPressureSurfaceReport,
    PrimitiveConstructionPolicyPressureSurfaceReportError, PrimitiveConstructionPolicyProfileCase,
    PrimitiveConstructionPolicyProfileRow, PrimitiveConstructionPolicyProfileSurfaceReport,
    PrimitiveConstructionPreservedIntentResolutionCase,
    PrimitiveConstructionPreservedIntentResolutionReport,
    PrimitiveConstructionPreservedIntentResolutionReportError,
    PrimitiveConstructionPreservedIntentResolutionRow, PrimitiveConstructionPreservedIntentTruth,
    PrimitiveConstructionPreviewCase, PrimitiveConstructionPreviewContinuityHostilityCase,
    PrimitiveConstructionPreviewContinuityHostilityRow,
    PrimitiveConstructionPreviewContinuityHostilitySuiteError,
    PrimitiveConstructionPreviewContinuityHostilitySuiteReport,
    PrimitiveConstructionPreviewHostilitySuiteReport, PrimitiveConstructionPreviewRow,
    PrimitiveConstructionPreviewSurfaceReport, PrimitiveConstructionPreviewSurfaceReportError,
    PrimitiveConstructionRealizationExhaustionReport,
    PrimitiveConstructionRealizationExhaustionStatus,
    PrimitiveConstructionRealizationExhaustionWitnessReport,
    PrimitiveConstructionRealizationExhaustionWitnessRow,
    PrimitiveConstructionRealizationReportBundle, PrimitiveConstructionRealizationStrategyReport,
    PrimitiveConstructionRequestedMotionWitness, PrimitiveConstructionStabilityClassReport,
};
pub use continuity_branch_runtime::{
    prepare_primitive_construction_continuity_branch_preview_runtime_report,
    PrimitiveConstructionContinuityBranchPreviewRuntimeError,
    PrimitiveConstructionContinuityBranchPreviewRuntimeReport,
};
pub use continuity_replay::{
    prepare_primitive_construction_continuity_replay_parity_report,
    PrimitiveConstructionContinuityReplayParityError,
    PrimitiveConstructionContinuityReplayParityReport,
};
pub use diagnostics::{
    prepare_primitive_construction_rejection_locality_report,
    PrimitiveConstructionBlockingBoundary, PrimitiveConstructionRejectionLocalityReport,
    PrimitiveConstructionRejectionLocalityRow,
};
pub use family_coverage::{
    primitive_construction_family_coverage_report, PrimitiveConstructionFamilyCoverageReport,
    PrimitiveConstructionFamilyCoverageRow, PrimitiveConstructionFamilyCoverageStatus,
};
pub use intent::PrimitiveConstructionIntent;
pub use intent_arbitration_replay::{
    prepare_primitive_construction_intent_arbitration_replay_parity_report,
    PrimitiveConstructionIntentArbitrationReplayParityError,
    PrimitiveConstructionIntentArbitrationReplayParityReport,
};
pub use motion_branch_runtime::{
    prepare_primitive_construction_move_branch_preview_runtime_report,
    prepare_primitive_construction_move_branch_preview_runtime_report_with_catalog,
    prepare_primitive_construction_points_toward_branch_preview_runtime_report,
    prepare_primitive_construction_points_toward_branch_preview_runtime_report_with_catalog,
    prepare_primitive_construction_reorient_branch_preview_runtime_report,
    prepare_primitive_construction_reorient_branch_preview_runtime_report_with_catalog,
    prepare_primitive_construction_rotate_branch_preview_runtime_report,
    prepare_primitive_construction_rotate_branch_preview_runtime_report_with_catalog,
    PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionMotionRuntimeSurfaceStatus,
};
pub use motion_replay::{
    prepare_primitive_construction_move_replay_parity_report,
    prepare_primitive_construction_move_replay_parity_report_with_catalog,
    prepare_primitive_construction_points_toward_replay_parity_report,
    prepare_primitive_construction_points_toward_replay_parity_report_with_catalog,
    prepare_primitive_construction_reorient_replay_parity_report,
    prepare_primitive_construction_reorient_replay_parity_report_with_catalog,
    prepare_primitive_construction_rotate_replay_parity_report,
    prepare_primitive_construction_rotate_replay_parity_report_with_catalog,
    PrimitiveConstructionMotionReplayParityReport,
};
pub use outcome::{
    PrimitiveConstructionAcceptedOutcome, PrimitiveConstructionPreparedOutcome,
    PrimitiveConstructionRejectedOutcome, PrimitiveConstructionRejectionClass,
    PrimitiveConstructionRejectionLocality,
};
pub use parity::{
    prepare_primitive_construction_branch_local_parity_report,
    prepare_primitive_construction_replay_parity_report,
    PrimitiveConstructionBranchLocalParityReport, PrimitiveConstructionReplayParityReport,
};
pub use preview_branch_runtime::{
    prepare_primitive_construction_preview_branch_preview_runtime_report,
    PrimitiveConstructionPreviewBranchPreviewRuntimeError,
    PrimitiveConstructionPreviewBranchPreviewRuntimeReport,
};
pub use preview_replay::{
    prepare_primitive_construction_preview_replay_parity_report,
    PrimitiveConstructionPreviewReplayParityError, PrimitiveConstructionPreviewReplayParityReport,
};
pub use profile_branch_runtime::{
    prepare_primitive_construction_policy_profile_branch_preview_runtime_report,
    PrimitiveConstructionPolicyProfileBranchPreviewRuntimeError,
    PrimitiveConstructionPolicyProfileBranchPreviewRuntimeReport,
};
pub use profile_replay::{
    prepare_primitive_construction_policy_profile_replay_parity_report,
    PrimitiveConstructionPolicyProfileReplayParityError,
    PrimitiveConstructionPolicyProfileReplayParityReport,
};
pub(crate) use proof::{
    prepare_primitive_construction_proof_substrate_closeout_report,
    PrimitiveConstructionProofSubstrateCloseoutReport,
    PrimitiveConstructionProofSubstrateCloseoutReportError,
};
pub(crate) use query::{
    prepare_primitive_construction_query_basis_preview_parity_report,
    prepare_primitive_construction_query_boundary_gap_register,
    prepare_primitive_construction_query_continuity_inspection_parity_report,
    prepare_primitive_construction_query_continuity_projection_consumption_receipt_report,
    prepare_primitive_construction_query_existing_truth_binding_report,
    prepare_primitive_construction_query_graph_composition_parity_report,
    prepare_primitive_construction_query_inspection_parity_report,
    prepare_primitive_construction_query_intent_arbitration_inspection_parity_report,
    prepare_primitive_construction_query_intent_arbitration_projection_consumption_receipt_report,
    prepare_primitive_construction_query_motion_inspection_parity_report,
    prepare_primitive_construction_query_motion_projection_consumption_receipt_report,
    prepare_primitive_construction_query_no_local_runtime_workaround_audit,
    prepare_primitive_construction_query_policy_profile_inspection_parity_report,
    prepare_primitive_construction_query_policy_profile_projection_consumption_receipt_report,
    prepare_primitive_construction_query_preview_inspection_parity_report,
    prepare_primitive_construction_query_preview_projection_consumption_receipt_report,
    prepare_primitive_construction_query_projection_consumption_receipt_report,
    PrimitiveConstructionContinuityQueryFactProvenance,
    PrimitiveConstructionContinuityQueryInspectionSurface,
    PrimitiveConstructionContinuityQueryReadSurface,
    PrimitiveConstructionExistingTruthBindingPosture,
    PrimitiveConstructionIntentArbitrationQueryFactProvenance,
    PrimitiveConstructionIntentArbitrationQueryInspectionSurface,
    PrimitiveConstructionIntentArbitrationQueryReadSurface, PrimitiveConstructionIntentChosenTruth,
    PrimitiveConstructionMotionQueryFactProvenance,
    PrimitiveConstructionMotionQueryInspectionSurface, PrimitiveConstructionMotionQueryReadSurface,
    PrimitiveConstructionPolicyProfileQueryFactProvenance,
    PrimitiveConstructionPolicyProfileQueryInspectionSurface,
    PrimitiveConstructionPolicyProfileQueryReadSurface,
    PrimitiveConstructionPreviewQueryFactProvenance,
    PrimitiveConstructionPreviewQueryInspectionSurface,
    PrimitiveConstructionPreviewQueryReadSurface,
    PrimitiveConstructionQueryBasisPreviewParityReport,
    PrimitiveConstructionQueryBoundaryGapRegister, PrimitiveConstructionQueryBoundaryGapRowReport,
    PrimitiveConstructionQueryBoundaryGapStatus, PrimitiveConstructionQueryBoundaryUsagePosture,
    PrimitiveConstructionQueryContinuityParityError,
    PrimitiveConstructionQueryContinuityParityReport,
    PrimitiveConstructionQueryExistingTruthBindingReport,
    PrimitiveConstructionQueryGraphCompositionParityError,
    PrimitiveConstructionQueryGraphCompositionParityReport,
    PrimitiveConstructionQueryInspectionParityError,
    PrimitiveConstructionQueryInspectionParityReport,
    PrimitiveConstructionQueryIntentArbitrationParityError,
    PrimitiveConstructionQueryIntentArbitrationParityReport,
    PrimitiveConstructionQueryMotionWitnessParityError,
    PrimitiveConstructionQueryMotionWitnessParityReport,
    PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit,
    PrimitiveConstructionQueryPolicyProfileParityError,
    PrimitiveConstructionQueryPolicyProfileParityReport,
    PrimitiveConstructionQueryPreviewParityError, PrimitiveConstructionQueryPreviewParityReport,
    PrimitiveConstructionQueryProjectionConsumptionReceiptError,
    PrimitiveConstructionQueryProjectionConsumptionReceiptReport,
};
pub use request::{
    PrimitiveConstructionFamily, PrimitiveConstructionPhaseError, PrimitiveConstructionRequest,
};
pub use result::{PreparedPrimitiveConstructionResult, PrimitiveConstructionResultError};
pub use runtime_basis::{
    prepare_primitive_construction_branch_preview_runtime_report,
    PrimitiveConstructionBranchPreviewRuntimeReport, PrimitiveConstructionRuntimeBasisError,
    PrimitiveConstructionRuntimeBasisLaneReport,
};
pub use specs::{
    OrthotopeSpec, RegularPrismSpec, RegularPyramidSpec, ShellWithHoleSpec, SimplexSolidSpec,
    WireBodySpec,
};

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
