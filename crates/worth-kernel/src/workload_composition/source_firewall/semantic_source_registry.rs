use crate::workload_composition::source_firewall::forbidden_surface::WorthTouchedGraphConflictForbiddenSurface;

#[derive(Clone, Copy)]
pub(crate) enum SemanticSourceCoverageKind {
    ExactFile,
    Subtree,
}

#[derive(Clone, Copy)]
pub(crate) struct SemanticSourceCoverage {
    forbidden_surface: WorthTouchedGraphConflictForbiddenSurface,
    source_path: &'static str,
    kind: SemanticSourceCoverageKind,
    explicit_owned_paths: &'static [&'static str],
    explicit_allowed_surfaces: &'static [&'static str],
}

impl SemanticSourceCoverage {
    pub(crate) const fn exact_file(
        forbidden_surface: WorthTouchedGraphConflictForbiddenSurface,
        source_path: &'static str,
        explicit_owned_paths: &'static [&'static str],
        explicit_allowed_surfaces: &'static [&'static str],
    ) -> Self {
        Self {
            forbidden_surface,
            source_path,
            kind: SemanticSourceCoverageKind::ExactFile,
            explicit_owned_paths,
            explicit_allowed_surfaces,
        }
    }

    pub(crate) const fn subtree_with_owned_files_and_allowed(
        forbidden_surface: WorthTouchedGraphConflictForbiddenSurface,
        source_path: &'static str,
        explicit_owned_paths: &'static [&'static str],
        explicit_allowed_surfaces: &'static [&'static str],
    ) -> Self {
        Self {
            forbidden_surface,
            source_path,
            kind: SemanticSourceCoverageKind::Subtree,
            explicit_owned_paths,
            explicit_allowed_surfaces,
        }
    }

    pub(crate) const fn forbidden_surface(self) -> WorthTouchedGraphConflictForbiddenSurface {
        self.forbidden_surface
    }

    pub(crate) const fn explicit_allowed_surfaces(self) -> &'static [&'static str] {
        self.explicit_allowed_surfaces
    }

    pub(crate) const fn explicit_owned_paths(self) -> &'static [&'static str] {
        self.explicit_owned_paths
    }

    pub(crate) fn matches_path(self, source_path: &str) -> bool {
        match self.kind {
            SemanticSourceCoverageKind::ExactFile => {
                source_path == self.source_path || source_path.ends_with(self.source_path)
            }
            SemanticSourceCoverageKind::Subtree => {
                source_path.starts_with(self.source_path)
                    || source_path.contains(&format!("/{}", self.source_path))
                    || source_path.ends_with(self.source_path)
            }
        }
    }
}

use WorthTouchedGraphConflictForbiddenSurface as Forbidden;

const PROJECTED_OVERLAP_FACES_OWNED_FILES: &[&str] = &[
    "crates/worth-spatial/src/workload_platform/projected_overlap_faces/authority.rs",
    "crates/worth-spatial/src/workload_platform/projected_overlap_faces/bundle.rs",
    "crates/worth-spatial/src/workload_platform/projected_overlap_faces/certified_face.rs",
    "crates/worth-spatial/src/workload_platform/projected_overlap_faces/certified_pair.rs",
    "crates/worth-spatial/src/workload_platform/projected_overlap_faces/certified_set.rs",
    "crates/worth-spatial/src/workload_platform/projected_overlap_faces/denial.rs",
    "crates/worth-spatial/src/workload_platform/projected_overlap_faces/face_set.rs",
    "crates/worth-spatial/src/workload_platform/projected_overlap_faces/mod.rs",
];

const OVERLAP_EDGE_CHAINS_OWNED_FILES: &[&str] = &[
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/overlap_edge_chains/boundary_role.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/overlap_edge_chains/chain_member.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/overlap_edge_chains/chain_row.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/overlap_edge_chains/chain_set.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/overlap_edge_chains/construction.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/overlap_edge_chains/counters.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/overlap_edge_chains/denial.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/overlap_edge_chains/identity.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/overlap_edge_chains/indexed_inputs.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/overlap_edge_chains/mod.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/overlap_edge_chains/validation.rs",
];

const TRAVERSAL_VIEWS_OWNED_FILES: &[&str] = &[
    "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/closeout.rs",
    "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/counters.rs",
    "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/diagnostic_projection.rs",
    "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/executor.rs",
    "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/input.rs",
    "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/mod.rs",
    "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/old_authority_residue.rs",
    "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/output.rs",
    "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/phase_seed.rs",
    "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/read_stage/executor.rs",
    "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/read_stage/mod.rs",
    "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/read_stage/receipt.rs",
    "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/read_stage/source.rs",
    "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/read_stage/traversal_rows.rs",
];

const EVIDENCE_LOOKUP_SOURCE_FIREWALL_OWNED_FILES: &[&str] = &[
    "crates/worth-spatial/src/workload_platform/evidence_lookup_source_firewall/counters.rs",
    "crates/worth-spatial/src/workload_platform/evidence_lookup_source_firewall/covered_root.rs",
    "crates/worth-spatial/src/workload_platform/evidence_lookup_source_firewall/error.rs",
    "crates/worth-spatial/src/workload_platform/evidence_lookup_source_firewall/exception.rs",
    "crates/worth-spatial/src/workload_platform/evidence_lookup_source_firewall/exception_summary.rs",
    "crates/worth-spatial/src/workload_platform/evidence_lookup_source_firewall/mod.rs",
    "crates/worth-spatial/src/workload_platform/evidence_lookup_source_firewall/report.rs",
    "crates/worth-spatial/src/workload_platform/evidence_lookup_source_firewall/row.rs",
    "crates/worth-spatial/src/workload_platform/evidence_lookup_source_firewall/scan_roots.rs",
    "crates/worth-spatial/src/workload_platform/evidence_lookup_source_firewall/semantic_shape.rs",
];

const SPATIAL_UNDO_FAMILY_EXECUTION_OWNED_FILES: &[&str] = &[
    "crates/worth-spatial/src/replay_undo_semantic_graph/undo_family_execution/boolean_event_ledger_rollback_request.rs",
    "crates/worth-spatial/src/replay_undo_semantic_graph/undo_family_execution/mod.rs",
    "crates/worth-spatial/src/replay_undo_semantic_graph/undo_family_execution/projection_receipt_rollback_request.rs",
    "crates/worth-spatial/src/replay_undo_semantic_graph/undo_family_execution/rollback_admission.rs",
];

const TOPOLOGY_UNDO_FAMILY_EXECUTION_OWNED_FILES: &[&str] = &[
    "crates/worth-topo/src/replay_undo_semantic_graph/undo_family_execution/materialized_graph_rollback_request.rs",
    "crates/worth-topo/src/replay_undo_semantic_graph/undo_family_execution/mod.rs",
    "crates/worth-topo/src/replay_undo_semantic_graph/undo_family_execution/rollback_admission.rs",
    "crates/worth-topo/src/replay_undo_semantic_graph/undo_family_execution/traversal_views_rollback_request.rs",
];

const CONFLICT_INPUT_OWNED_FILES: &[&str] = &[
    "crates/worth-kernel/src/workload_composition/conflict_input/handoff_guards.rs",
    "crates/worth-kernel/src/workload_composition/conflict_input/mod.rs",
    "crates/worth-kernel/src/workload_composition/conflict_input/spatial.rs",
    "crates/worth-kernel/src/workload_composition/conflict_input/topology.rs",
];

const LOOKUP_CONSUMED_WORKLOAD_OWNED_FILES: &[&str] = &[
    "crates/worth-kernel/src/workload_composition/worth_workload/lookup_consumed_workload/mod.rs",
];

const HIGH_VALENCE_SINGULARITY_OWNED_FILES: &[&str] = &[
    "crates/worth-spatial/src/workload_platform/high_valence_singularity/mod.rs",
    "crates/worth-spatial/src/workload_platform/high_valence_singularity/singularity_counters.rs",
    "crates/worth-spatial/src/workload_platform/high_valence_singularity/singularity_receipt.rs",
    "crates/worth-spatial/src/workload_platform/high_valence_singularity/singularity_workload.rs",
];

const DUPLICATE_SPLIT_NORMALIZATION_OWNED_FILES: &[&str] = &[
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization/contradiction_basis.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization/counters.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization/denial.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization/duplicate_key.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization/grouping.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization/identity.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization/mod.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization/normalization.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization/normalized_cut.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization/normalized_cut_builder.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization/retained_interval_entry.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization/tests_support.rs",
];

const PROJECTED_OVERLAP_FACES_ALLOWED_SURFACES: &[&str] = &[
    "ProjectedOverlapCandidatePolicy",
    "ProjectedOverlapFaceSet",
    "ProjectedOverlapFaceGeometry",
    "ProjectedOverlapFaceDenial",
    "CertifiedProjectedOverlapFace",
    "CertifiedProjectedOverlapFaceSet",
    "CertifiedProjectedOverlapCandidatePairs",
    "ProjectedOverlapExtractionContracts",
    "CoplanarOverlapExtractionBundle",
    "contracts_from_context",
];

const OVERLAP_EDGE_CHAINS_ALLOWED_SURFACES: &[&str] = &[
    "PlanarBooleanOverlapChainBoundaryRole",
    "PlanarBooleanOverlapChainPosture",
    "PlanarBooleanOverlapEdgeChainMember",
    "PlanarBooleanOverlapEdgeChain",
    "PlanarBooleanOverlapEdgeChainSet",
    "PlanarBooleanOverlapEdgeChainCounters",
    "PlanarBooleanOverlapEdgeChainDenialKind",
    "PlanarBooleanOverlapEdgeChainDenial",
    "OverlapChainIndexedInputs",
    "overlap_chain_member_identity",
    "overlap_chain_identity",
    "overlap_chain_set_identity",
    "source_sense_name",
    "interval_kind_name",
    "reject_foreign_fragment_set",
    "reject_ambiguous_chain_basis",
];

const TRAVERSAL_VIEWS_ALLOWED_SURFACES: &[&str] = &[
    "TraversalViewsMigrationCounters",
    "TraversalViewsDiagnosticProjection",
    "TraversalViewsMigrationError",
    "TraversalViewsDerivedProductExecutor",
    "TraversalViewsExecutionInput",
    "TraversalViewsPhaseElevenSeed",
    "TraversalViewsProductRow",
    "TraversalViewsDerivedProductOutput",
    "TraversalViewsReadStageExecutor",
    "TraversalViewsReadStageReceipt",
    "TraversalViewsReadSource",
    "TraversalViewsSourceRow",
    "TraversalViewsOldAuthorityResidueRow",
    "TraversalViewsOldAuthorityResidue",
    "close_traversal_views_migration_slice",
    "TraversalViewsMigrationCloseout",
];

const EVIDENCE_LOOKUP_SOURCE_FIREWALL_ALLOWED_SURFACES: &[&str] = &[
    "EvidenceLookupSourceFirewallCounters",
    "EvidenceLookupSourceFirewallCoveredRootKind",
    "EvidenceLookupSourceFirewallCoveredRoot",
    "EvidenceLookupSourceFirewallErrorKind",
    "EvidenceLookupSourceFirewallError",
    "EvidenceLookupSourceFirewallExceptionSummary",
    "EvidenceLookupSourceFirewallOutcome",
    "EvidenceLookupSourceFirewallReport",
    "current_evidence_lookup_source_firewall_report",
    "EvidenceLookupForbiddenAuthorityKind",
    "EvidenceLookupSourceFirewallExceptionKind",
    "EvidenceLookupSourceFirewallRowPosture",
    "EvidenceLookupSourceFirewallRow",
    "named_exception_for_path",
    "source_firewall_report_for_snapshot_root",
    "SourceFirewallRecord",
    "SourceFirewallSnapshot",
    "current_source_firewall_snapshot",
    "source_firewall_snapshot_for_workspace_root",
    "covered_root_inventory",
    "MatchedSemanticShape",
    "matched_semantic_shapes",
];

const SPATIAL_UNDO_FAMILY_EXECUTION_ALLOWED_SURFACES: &[&str] = &[
    "BooleanEventLedgerRollbackRequest",
    "ProjectionReceiptRollbackRequest",
    "lower_spatial_undo_scope_product_from_boolean_event_ledger_request",
    "lower_spatial_undo_scope_product_from_projection_receipt_request",
    "SpatialUndoFamilyExecutionError",
];

const TOPOLOGY_UNDO_FAMILY_EXECUTION_ALLOWED_SURFACES: &[&str] = &[
    "MaterializedGraphRollbackRequest",
    "TraversalViewsRollbackRequest",
    "lower_topology_undo_scope_product_from_traversal_views_request",
    "lower_topology_undo_scope_product_from_materialized_graph_request",
    "TopologyUndoFamilyExecutionError",
];

const CONFLICT_INPUT_ALLOWED_SURFACES: &[&str] = &[
    "ConflictInputAdmissionErrorKind",
    "ConflictInputAdmissionError",
    "AdmittedSpatialConflictRoute",
    "SpatialConflictInputRequest",
    "AdmittedSpatialConflictInput",
    "admit_spatial_conflict_input",
    "AdmittedTopologyConflictRoute",
    "TopologyConflictInputRequest",
    "AdmittedTopologyConflictInput",
    "admit_topology_conflict_input",
    "require_honest_lookup_handoff",
];

const HIGH_VALENCE_SINGULARITY_ALLOWED_SURFACES: &[&str] = &[
    "HighValenceSingularityCounters",
    "HighValenceSingularityCounterInput",
    "HighValenceSingularityReceipt",
    "HIGH_VALENCE_SINGULARITY_MAX_ADMITTED_VALENCE",
    "HighValenceSingularityWorkload",
    "HighValenceRebuildMotionCompatibility",
    "HighValencePredicateCertification",
    "HighValenceSingularityPolicy",
    "HighValenceEvidenceIntegrity",
    "HighValenceSingularityWorkloadError",
];

const DUPLICATE_SPLIT_NORMALIZATION_ALLOWED_SURFACES: &[&str] = &[
    "PlanarBooleanNormalizedEdgeSplitScheduleCounters",
    "PlanarBooleanDuplicateSplitNormalizationDenialKind",
    "PlanarBooleanDuplicateSplitNormalizationDenial",
    "DuplicateSplitGrouping",
    "PlanarBooleanDuplicateSplitCutKey",
    "duplicate_cut_kind_rank",
    "reject_contradictory_same_parameter_points",
    "normalized_cut_identity",
    "duplicate_report_identity",
    "normalized_schedule_identity",
    "normalized_schedule_set_identity",
    "tests_support",
    "normalized_cut_from_duplicate_point_entries",
    "PlanarBooleanOrderedEdgeSplitScheduleSet",
    "PlanarBooleanNormalizedSplitCut",
    "PlanarBooleanNormalizedEndpointAuthority",
    "PlanarBooleanNormalizedEdgeSplitSchedule",
    "PlanarBooleanNormalizedEdgeSplitScheduleSet",
    "PlanarBooleanRetainedIntervalSplitEntry",
    "raw_set_from_schedules",
    "raw_schedule",
    "raw_point_entry",
    "raw_point_entry_with_posture",
    "raw_point_entry_with_frame_precision",
    "raw_interval_entry",
    "raw_entry",
];

const PHASE_TWELVE_SEMANTIC_SOURCE_COVERAGES: &[SemanticSourceCoverage] = &[
        SemanticSourceCoverage::subtree_with_owned_files_and_allowed(
            Forbidden::EntityOnlyOverlapHelper,
            "crates/worth-spatial/src/workload_platform/projected_overlap_faces",
            PROJECTED_OVERLAP_FACES_OWNED_FILES,
            PROJECTED_OVERLAP_FACES_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::subtree_with_owned_files_and_allowed(
            Forbidden::EntityOnlyOverlapHelper,
            "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/overlap_edge_chains",
            OVERLAP_EDGE_CHAINS_OWNED_FILES,
            OVERLAP_EDGE_CHAINS_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::subtree_with_owned_files_and_allowed(
            Forbidden::BroadTopologyScan,
            "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views",
            TRAVERSAL_VIEWS_OWNED_FILES,
            TRAVERSAL_VIEWS_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::subtree_with_owned_files_and_allowed(
            Forbidden::BroadEvidenceScan,
            "crates/worth-spatial/src/workload_platform/evidence_lookup_source_firewall",
            EVIDENCE_LOOKUP_SOURCE_FIREWALL_OWNED_FILES,
            EVIDENCE_LOOKUP_SOURCE_FIREWALL_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::subtree_with_owned_files_and_allowed(
            Forbidden::LockFirstAdmission,
            "crates/worth-spatial/src/replay_undo_semantic_graph/undo_family_execution",
            SPATIAL_UNDO_FAMILY_EXECUTION_OWNED_FILES,
            SPATIAL_UNDO_FAMILY_EXECUTION_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::subtree_with_owned_files_and_allowed(
            Forbidden::LockFirstAdmission,
            "crates/worth-topo/src/replay_undo_semantic_graph/undo_family_execution",
            TOPOLOGY_UNDO_FAMILY_EXECUTION_OWNED_FILES,
            TOPOLOGY_UNDO_FAMILY_EXECUTION_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::subtree_with_owned_files_and_allowed(
            Forbidden::SpeculativeRollbackAdmission,
            "crates/worth-spatial/src/replay_undo_semantic_graph/undo_family_execution",
            SPATIAL_UNDO_FAMILY_EXECUTION_OWNED_FILES,
            SPATIAL_UNDO_FAMILY_EXECUTION_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::subtree_with_owned_files_and_allowed(
            Forbidden::SpeculativeRollbackAdmission,
            "crates/worth-topo/src/replay_undo_semantic_graph/undo_family_execution",
            TOPOLOGY_UNDO_FAMILY_EXECUTION_OWNED_FILES,
            TOPOLOGY_UNDO_FAMILY_EXECUTION_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::subtree_with_owned_files_and_allowed(
            Forbidden::CallerOwnedSerialization,
            "crates/worth-kernel/src/workload_composition/conflict_input",
            CONFLICT_INPUT_OWNED_FILES,
            CONFLICT_INPUT_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::subtree_with_owned_files_and_allowed(
            Forbidden::CallerOwnedSerialization,
            "crates/worth-kernel/src/workload_composition/worth_workload/lookup_consumed_workload",
            LOOKUP_CONSUMED_WORKLOAD_OWNED_FILES,
            &[
                "LookupConsumedWorkloadComposition",
                "LookupConsumedWorkloadComposition::admit",
                "LookupConsumedWorkloadComposition::workload",
                "LookupConsumedWorkloadComposition::handoff",
                "LookupConsumedWorkloadComposition::admit_spatial_conflict_input",
                "WorthWorkload::admit_lookup_consumed_workload",
            ],
        ),
        SemanticSourceCoverage::subtree_with_owned_files_and_allowed(
            Forbidden::CallerOwnedCompatibility,
            "crates/worth-spatial/src/workload_platform/high_valence_singularity",
            HIGH_VALENCE_SINGULARITY_OWNED_FILES,
            HIGH_VALENCE_SINGULARITY_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::subtree_with_owned_files_and_allowed(
            Forbidden::CallerOwnedCompatibility,
            "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/duplicate_split_normalization",
            DUPLICATE_SPLIT_NORMALIZATION_OWNED_FILES,
            DUPLICATE_SPLIT_NORMALIZATION_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::subtree_with_owned_files_and_allowed(
            Forbidden::GenericOverlapSecondAuthorityLane,
            "crates/worth-spatial/src/workload_platform/projected_overlap_faces",
            PROJECTED_OVERLAP_FACES_OWNED_FILES,
            PROJECTED_OVERLAP_FACES_ALLOWED_SURFACES,
        ),
];

pub(crate) const fn phase_twelve_semantic_source_coverages() -> &'static [SemanticSourceCoverage] {
    PHASE_TWELVE_SEMANTIC_SOURCE_COVERAGES
}
