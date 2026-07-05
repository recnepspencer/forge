use super::forbidden_surface::WorthTouchedGraphConflictForbiddenSurface as Forbidden;
use super::phase_twelve_semantic_source_registry::SemanticSourceCoverage;

const PUBLIC_FACADE_CURRENT_ALLOWED_SURFACES: &[&str] = &[
    "current_worth_touched_graph_conflict_derived_diagnostic_projection_with_artifact_policy",
    "WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy",
    "current_worth_touched_graph_conflict_public_closeout",
    "current_worth_touched_graph_conflict_public_facade",
    "current_worth_touched_graph_conflict_public_facade_with_artifact_policy",
    "public_proof_inspection",
    "require_matching_projection_authority",
    "WorthTouchedGraphConflictPublicFacade",
    "WorthTouchedGraphConflictPublicFacade::new",
    "WorthTouchedGraphConflictPublicFacadeError",
    "WorthTouchedGraphConflictPublicFacadeError::new",
    "WorthTouchedGraphConflictPublicFacadeErrorKind",
    "WorthTouchedGraphConflictPublicProofInspection",
    "WorthTouchedGraphConflictPublicProofInspection::new",
];

const PUBLIC_FACADE_AUTHORITY_ALLOWED_SURFACES: &[&str] = &[
    "require_matching_projection_authority",
    "WorthTouchedGraphConflictDerivedDiagnosticProjection",
    "WorthTouchedGraphConflictPublicCloseout",
];

const PUBLIC_FACADE_INSPECTION_ALLOWED_SURFACES: &[&str] = &[
    "WorthTouchedGraphConflictDerivedDiagnosticProjection",
    "WorthTouchedGraphConflictPublicFacade",
    "WorthTouchedGraphConflictPublicFacade::new",
    "WorthTouchedGraphConflictPublicFacade::derived_diagnostics",
    "WorthTouchedGraphConflictPublicFacade::selected_route_identity_digest",
    "WorthTouchedGraphConflictPublicFacadeError",
    "WorthTouchedGraphConflictPublicFacadeError::new",
    "WorthTouchedGraphConflictPublicFacadeError::kind",
    "WorthTouchedGraphConflictPublicFacadeError::detail",
    "WorthTouchedGraphConflictPublicFacadeErrorKind",
    "WorthTouchedGraphConflictPublicProofInspection",
    "WorthTouchedGraphConflictPublicProofInspection::new",
    "WorthTouchedGraphConflictPublicProofInspection::selected_route_identity_digest",
    "WorthTouchedGraphConflictPublicProofInspection::selected_family_identity",
    "WorthTouchedGraphConflictPublicProofInspection::selected_product_identity_digest",
    "WorthTouchedGraphConflictPublicProofInspection::closeout_digest",
    "WorthTouchedGraphConflictPublicProofInspection::source_firewall_digest",
    "WorthTouchedGraphConflictPublicProofInspection::deletion_closeout_digest",
    "WorthTouchedGraphConflictPublicProofInspection::residue_chain",
    "WorthTouchedGraphConflictPublicProofInspection::architecture_alignment_report",
    "WorthTouchedGraphConflictPublicProofInspection::milestone_fifteen_seed",
];

const PUBLIC_FACADE_MOD_ALLOWED_SURFACES: &[&str] = &[
    "current_worth_touched_graph_conflict_public_facade",
    "current_worth_touched_graph_conflict_public_facade_with_artifact_policy",
    "require_matching_projection_authority",
    "WorthTouchedGraphConflictPublicFacade",
    "WorthTouchedGraphConflictPublicFacadeError",
    "WorthTouchedGraphConflictPublicFacadeErrorKind",
];

const DIAGNOSTIC_CURRENT_ALLOWED_SURFACES: &[&str] = &[
    "current_topology_invalidation_route_input",
    "TopologyInvalidationRouteInputCurrentError",
    "WorthTouchedGraphConflictSelectedRoutePacket",
    "current_worth_touched_graph_conflict_selected_route_packet",
    "PlannerOwnedRoutingError",
    "PlannerOwnedRoutingErrorKind",
    "trace_scope",
    "WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy",
    "WorthTouchedGraphConflictDerivedDiagnosticProjection",
    "select_rich_localization",
    "current_worth_touched_graph_conflict_derived_diagnostic_projection",
    "current_worth_touched_graph_conflict_derived_diagnostic_projection_with_artifact_policy",
    "current_worth_touched_graph_conflict_derived_diagnostic_projection_with_packet_loader",
    "build_current_worth_touched_graph_conflict_derived_diagnostic_projection",
    "current_invalidation_error",
];

pub(super) fn phase_fifteen_public_facade_semantic_source_coverages() -> Vec<SemanticSourceCoverage>
{
    vec![
        SemanticSourceCoverage::exact_file(
            Forbidden::SupportWrapperShortcut,
            "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_facade/current.rs",
            &[],
            PUBLIC_FACADE_CURRENT_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::exact_file(
            Forbidden::SupportWrapperShortcut,
            "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_facade/authority.rs",
            &[],
            PUBLIC_FACADE_AUTHORITY_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::exact_file(
            Forbidden::LegacyExplainerImport,
            "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_facade/inspection.rs",
            &[],
            PUBLIC_FACADE_INSPECTION_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::exact_file(
            Forbidden::LegacyExplainerImport,
            "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_facade/mod.rs",
            &[],
            PUBLIC_FACADE_MOD_ALLOWED_SURFACES,
        ),
        SemanticSourceCoverage::exact_file(
            Forbidden::LocalDiagnosticAuthorityFabrication,
            "crates/worth-kernel/src/workload_composition/planner_owned_routing/derived_diagnostics/current.rs",
            &[],
            DIAGNOSTIC_CURRENT_ALLOWED_SURFACES,
        ),
    ]
}
