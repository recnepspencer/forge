#[path = "phase_chain/admission.rs"]
mod admission;
#[path = "result_surface/artifact.rs"]
mod artifact;
mod authoring;
#[path = "certification/mod.rs"]
mod certification;
#[path = "runtime_proof/diagnostics.rs"]
mod diagnostics;
mod digest;
#[path = "phase_chain/execution.rs"]
mod execution;
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
#[path = "phase_chain/phase_report.rs"]
mod phase_report;
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
#[path = "phase_chain/scaffold.rs"]
mod scaffold;
#[path = "phase_chain/scaffold_geometry.rs"]
mod scaffold_geometry;
#[path = "phase_chain/admitted_scaffold.rs"]
mod scaffold_realization;
#[path = "phase_chain/specs.rs"]
mod specs;
#[path = "phase_chain/topology_counts.rs"]
mod topology_counts;

pub use admission::AdmittedPrimitiveConstructionIntent;
pub use artifact::{
    build_canonical_primitive_construction_artifact, CanonicalPrimitiveConstructionArtifact,
    PrimitiveConstructionArtifactError,
};
pub use authoring::{
    primitive_construction_authoring, PrimitiveConstructionAuthoringSession,
    PrimitiveConstructionAuthorityChainReport, PrimitiveConstructionQueryGapRow,
    WorthKernelAuthorityError,
};
pub use certification::{
    prepare_primitive_chosen_intent_resolution_report,
    prepare_primitive_construction_compound_adversarial_siege_report,
    prepare_primitive_construction_compound_grazing_boundary_report,
    prepare_primitive_construction_compound_motion_parity_report,
    prepare_primitive_construction_conditioning_witness_report,
    prepare_primitive_construction_corpus_replay_siege,
    prepare_primitive_construction_family_boundary_report,
    prepare_primitive_construction_intent_arbitration_hostility_suite_report,
    prepare_primitive_construction_intent_arbitration_report_bundle,
    prepare_primitive_construction_motion_dx_surface_report,
    prepare_primitive_construction_motion_resolution_policy_report,
    prepare_primitive_construction_move_motion_report_bundle,
    prepare_primitive_construction_move_motion_report_bundle_with_catalog,
    prepare_primitive_construction_move_witness_resolution_report,
    prepare_primitive_construction_move_witness_resolution_report_with_catalog,
    prepare_primitive_construction_points_toward_motion_report_bundle,
    prepare_primitive_construction_points_toward_motion_report_bundle_with_catalog,
    prepare_primitive_construction_points_toward_witness_resolution_report,
    prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog,
    prepare_primitive_construction_preserved_intent_resolution_report,
    prepare_primitive_construction_realization_exhaustion_report,
    prepare_primitive_construction_realization_exhaustion_witness_report,
    prepare_primitive_construction_realization_report_bundle,
    prepare_primitive_construction_realization_strategy_report,
    prepare_primitive_construction_reorient_motion_report_bundle,
    prepare_primitive_construction_reorient_motion_report_bundle_with_catalog,
    prepare_primitive_construction_reorient_witness_resolution_report,
    prepare_primitive_construction_reorient_witness_resolution_report_with_catalog,
    prepare_primitive_construction_rotate_motion_report_bundle,
    prepare_primitive_construction_rotate_motion_report_bundle_with_catalog,
    prepare_primitive_construction_rotate_witness_resolution_report,
    prepare_primitive_construction_rotate_witness_resolution_report_with_catalog,
    prepare_primitive_construction_stability_class_report,
    prepare_primitive_intent_arbitration_policy_report,
    prepare_primitive_intent_conflict_dx_surface_report,
    PrimitiveConstructionChosenIntentResolutionAuthority,
    PrimitiveConstructionChosenIntentResolutionCase,
    PrimitiveConstructionChosenIntentResolutionReport,
    PrimitiveConstructionChosenIntentResolutionReportError,
    PrimitiveConstructionChosenIntentResolutionRow,
    PrimitiveConstructionCompoundAdversarialSiegeError,
    PrimitiveConstructionCompoundAdversarialSiegeReport,
    PrimitiveConstructionCompoundAuthoringOrderRow,
    PrimitiveConstructionCompoundGrazingBoundaryReport,
    PrimitiveConstructionCompoundGrazingBoundaryRow, PrimitiveConstructionCompoundGrazingKind,
    PrimitiveConstructionCompoundMotionKind, PrimitiveConstructionCompoundMotionParityReport,
    PrimitiveConstructionCompoundMotionParityRow, PrimitiveConstructionCompoundRow,
    PrimitiveConstructionCompoundRowClass, PrimitiveConstructionCompoundTopologyClass,
    PrimitiveConstructionCompoundWorkloadFamily, PrimitiveConstructionConditioningWitnessReport,
    PrimitiveConstructionCorpusAuthoringOrderRow, PrimitiveConstructionCorpusOutcomeDisposition,
    PrimitiveConstructionCorpusParameterRole, PrimitiveConstructionCorpusRejectionWitnessRow,
    PrimitiveConstructionCorpusReplaySiegeError, PrimitiveConstructionCorpusReplaySiegeReport,
    PrimitiveConstructionCorpusReplaySiegeRow,
    PrimitiveConstructionFamilyBoundaryLowerLayerWitnessSummary,
    PrimitiveConstructionFamilyBoundaryReport, PrimitiveConstructionFamilyBoundaryReportError,
    PrimitiveConstructionFamilyBoundaryRow, PrimitiveConstructionFamilyBoundaryTransitionClass,
    PrimitiveConstructionIntentArbitrationBundleCase,
    PrimitiveConstructionIntentArbitrationConflictClass,
    PrimitiveConstructionIntentArbitrationDxSurface,
    PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    PrimitiveConstructionIntentArbitrationDxSurfaceRow,
    PrimitiveConstructionIntentArbitrationHostilitySuiteReport,
    PrimitiveConstructionIntentArbitrationPolicyCase,
    PrimitiveConstructionIntentArbitrationPolicyReport,
    PrimitiveConstructionIntentArbitrationPolicyReportError,
    PrimitiveConstructionIntentArbitrationPolicyRow,
    PrimitiveConstructionIntentArbitrationReportBundle,
    PrimitiveConstructionIntentArbitrationReportBundleError, PrimitiveConstructionMotionDxSurface,
    PrimitiveConstructionMotionDxSurfaceReport, PrimitiveConstructionMotionDxSurfaceReportError,
    PrimitiveConstructionMotionDxSurfaceRow, PrimitiveConstructionMotionReportBundle,
    PrimitiveConstructionMotionReportBundleError, PrimitiveConstructionMotionResolutionPolicyCase,
    PrimitiveConstructionMotionResolutionPolicyReport,
    PrimitiveConstructionMotionResolutionPolicyReportError,
    PrimitiveConstructionMotionResolutionPolicyRow,
    PrimitiveConstructionMotionWitnessResolutionFailureKind,
    PrimitiveConstructionMotionWitnessResolutionKind,
    PrimitiveConstructionMotionWitnessResolutionReport,
    PrimitiveConstructionMotionWitnessResolutionStatus,
    PrimitiveConstructionObservedIntentRelation,
    PrimitiveConstructionPreservedIntentResolutionCase,
    PrimitiveConstructionPreservedIntentResolutionReport,
    PrimitiveConstructionPreservedIntentResolutionReportError,
    PrimitiveConstructionPreservedIntentResolutionRow, PrimitiveConstructionPreservedIntentTruth,
    PrimitiveConstructionRealizationExhaustionReport,
    PrimitiveConstructionRealizationExhaustionStatus,
    PrimitiveConstructionRealizationExhaustionWitnessReport,
    PrimitiveConstructionRealizationExhaustionWitnessRow,
    PrimitiveConstructionRealizationReportBundle, PrimitiveConstructionRealizationStrategyReport,
    PrimitiveConstructionRequestedMotionWitness, PrimitiveConstructionResolvedMotionWitness,
    PrimitiveConstructionStabilityClassReport,
};
pub use diagnostics::{
    prepare_primitive_construction_rejection_locality_report,
    PrimitiveConstructionBlockingBoundary, PrimitiveConstructionRejectionLocalityReport,
    PrimitiveConstructionRejectionLocalityRow,
};
pub use execution::{PreparedPrimitiveConstructionExecution, PrimitiveConstructionExecutionError};
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
    prepare_primitive_construction_outcome, PrimitiveConstructionAcceptedOutcome,
    PrimitiveConstructionPreparedOutcome, PrimitiveConstructionRejectedOutcome,
    PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};
pub use parity::{
    prepare_primitive_construction_branch_local_parity_report,
    prepare_primitive_construction_replay_parity_report,
    PrimitiveConstructionBranchLocalParityReport, PrimitiveConstructionReplayParityReport,
};
pub use phase_report::PrimitiveConstructionPhaseChainReport;
pub use query::{
    prepare_primitive_construction_query_basis_preview_parity_report,
    prepare_primitive_construction_query_boundary_gap_register,
    prepare_primitive_construction_query_existing_truth_binding_report,
    prepare_primitive_construction_query_graph_composition_parity_report,
    prepare_primitive_construction_query_inspection_parity_report,
    prepare_primitive_construction_query_intent_arbitration_inspection_parity_report,
    prepare_primitive_construction_query_intent_arbitration_projection_consumption_receipt_report,
    prepare_primitive_construction_query_motion_inspection_parity_report,
    prepare_primitive_construction_query_motion_projection_consumption_receipt_report,
    prepare_primitive_construction_query_no_local_runtime_workaround_audit,
    prepare_primitive_construction_query_projection_consumption_receipt_report,
    PrimitiveConstructionExistingTruthBindingPosture,
    PrimitiveConstructionIntentArbitrationQueryFactProvenance,
    PrimitiveConstructionIntentArbitrationQueryInspectionSurface,
    PrimitiveConstructionIntentArbitrationQueryReadSurface, PrimitiveConstructionIntentChosenTruth,
    PrimitiveConstructionMotionQueryFactProvenance,
    PrimitiveConstructionMotionQueryInspectionSurface, PrimitiveConstructionMotionQueryReadSurface,
    PrimitiveConstructionQueryBasisPreviewParityReport,
    PrimitiveConstructionQueryBoundaryGapRegister, PrimitiveConstructionQueryBoundaryGapRowReport,
    PrimitiveConstructionQueryBoundaryGapStatus, PrimitiveConstructionQueryBoundaryUsagePosture,
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
    PrimitiveConstructionQueryProjectionConsumptionReceiptError,
    PrimitiveConstructionQueryProjectionConsumptionReceiptReport,
};
pub use request::{
    PrimitiveConstructionFamily, PrimitiveConstructionPhaseError, PrimitiveConstructionRequest,
};
pub use result::{
    prepare_primitive_construction_result, PreparedPrimitiveConstructionResult,
    PrimitiveConstructionResultError, PrimitiveConstructionResultEvidence,
};
pub use runtime_basis::{
    prepare_primitive_construction_branch_preview_runtime_report,
    PrimitiveConstructionBranchPreviewRuntimeReport, PrimitiveConstructionRuntimeBasisError,
    PrimitiveConstructionRuntimeBasisLaneReport,
};
pub use scaffold::{lower_scaffold_to_topology, PrimitiveConstructionScaffold};
pub use specs::{
    OrthotopeSpec, RegularPrismSpec, RegularPyramidSpec, ShellWithHoleSpec, SimplexSolidSpec,
    WireBodySpec,
};

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
