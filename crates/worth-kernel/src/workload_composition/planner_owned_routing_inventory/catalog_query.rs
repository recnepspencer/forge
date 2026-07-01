use super::classification::{
    PlannerOwnedRoutingDisplacedLane as DisplacedLane,
    PlannerOwnedRoutingDisposition as Disposition, PlannerOwnedRoutingLifecycleRole as Role,
    PlannerOwnedRoutingOwner as Owner, PlannerOwnedRoutingQueryGapKind as QueryGap,
    PlannerOwnedRoutingReplacementLane as Lane,
};
use super::row::{
    PlannerOwnedRoutingInventoryRow as Row, PlannerOwnedRoutingSurfaceIdentity as Surface,
};

pub(super) fn rows() -> Vec<Row> {
    vec![
        support_row(
            Surface::QueryWorkspacePublicSupportMatrix,
            "crates/forge-query/docs/foundations/support-matrix-and-admission.md",
            "workspace.public_support_matrix()",
            &["workspace.public_support_matrix()"],
            Lane::TopoQueryBackedReadFamily,
        ),
        support_row(
            Surface::QueryWorkspacePublicApiContract,
            "crates/forge-query/docs/foundations/support-matrix-and-admission.md",
            "workspace.public_api_contract()",
            &["workspace.public_api_contract()"],
            Lane::TopoQueryBackedReadFamily,
        ),
        support_row(
            Surface::QueryWorkspacePublicHandleContract,
            "crates/forge-query/docs/foundations/support-matrix-and-admission.md",
            "workspace.public_handle_contract()",
            &["workspace.public_handle_contract()"],
            Lane::TopoQueryBackedReadFamily,
        ),
        support_row(
            Surface::QueryWorkspaceAdmitPublicApiFamily,
            "crates/forge-query/docs/foundations/support-matrix-and-admission.md",
            "workspace.admit_public_api_family(...)",
            &["workspace.admit_public_api_family(...)"],
            Lane::TopoQueryBackedReadFamily,
        ),
        support_row(
            Surface::QueryProjectWorkspaceSupportSnapshot,
            "crates/forge-query/docs/foundations/consumer-kit.md",
            "project_workspace_support_snapshot(...)",
            &["project_workspace_support_snapshot(...)"],
            Lane::KernelSourceFirewall,
        ),
        support_row(
            Surface::QuerySupportPinningContract,
            "crates/forge-query/docs/foundations/consumer-kit.md",
            "support_pinning_contract(...)",
            &["support_pinning_contract(...)"],
            Lane::KernelSourceFirewall,
        ),
        support_row(
            Surface::QueryHardProhibitionBoundaryAudit,
            "crates/forge-query/docs/foundations/consumer-kit.md",
            "hard_prohibition_boundary_audit()",
            &["hard_prohibition_boundary_audit()"],
            Lane::KernelSourceFirewall,
        ),
        support_row(
            Surface::QueryConsumerResidueAudit,
            "crates/forge-query/docs/foundations/consumer-kit.md",
            "query_consumer_residue_audit()",
            &["query_consumer_residue_audit()"],
            Lane::KernelSourceFirewall,
        ),
        support_row(
            Surface::QueryConsumeProjectionFacts,
            "crates/forge-query/docs/capabilities/projection-consumption.md",
            "consume_projection_facts(...)",
            &["consume_projection_facts(...)"],
            Lane::TopoDiagnosticProjectionInput,
        ),
        support_row(
            Surface::QueryDeclareProjectionFactConsumption,
            "crates/forge-query/docs/capabilities/projection-consumption.md",
            "declare_projection_fact_consumption(...)",
            &["declare_projection_fact_consumption(...)"],
            Lane::TopoDiagnosticProjectionInput,
        ),
        support_row(
            Surface::QueryLowerRuntimeBoundaryEnvelopeSupport,
            "crates/forge-query/docs/domain-capabilities/support/lower-runtime-support-and-boundary-traceability.md",
            "for_lower_runtime_boundary_envelope(...)",
            &["for_lower_runtime_boundary_envelope(...)"],
            Lane::SpatialPublicCloseoutRoute,
        ),
        support_row(
            Surface::QueryLowerRuntimeBoundarySourceSupport,
            "crates/forge-query/docs/domain-capabilities/support/lower-runtime-support-and-boundary-traceability.md",
            "for_lower_runtime_boundary_source(...)",
            &["for_lower_runtime_boundary_source(...)"],
            Lane::SpatialPublicCloseoutRoute,
        ),
        support_row(
            Surface::QueryDeclarationScopedCapabilitySupport,
            "crates/forge-query/docs/domain-capabilities/support/declaration-scoped-support-and-traceability.md",
            "supports_capability(...)",
            &["supports_capability(...)"],
            Lane::KernelSelectedRoute,
        ),
        support_row(
            Surface::QueryDeclarationScopedTraceabilitySupport,
            "crates/forge-query/docs/domain-capabilities/support/declaration-scoped-support-and-traceability.md",
            "supports_traceability(...)",
            &["supports_traceability(...)"],
            Lane::KernelSelectedRoute,
        ),
        support_row(
            Surface::QueryDeclarationEnvelopeInput,
            "crates/forge-query/docs/domain-capabilities/declaration-boundary-envelopes.md",
            "ForgeQueryDeclarationEnvelopeInput",
            &["ForgeQueryDeclarationEnvelopeInput"],
            Lane::KernelAdmittedPublicProofInput,
        ),
        support_row(
            Surface::QueryDeclarationEnvelope,
            "crates/forge-query/docs/domain-capabilities/declaration-boundary-envelopes.md",
            "ForgeQueryDeclarationEnvelope",
            &["ForgeQueryDeclarationEnvelope"],
            Lane::KernelAdmittedPublicProofInput,
        ),
    ]
}

fn support_row(
    surface: Surface,
    source_path: &'static str,
    surface_name: &'static str,
    current_authority_sources: &'static [&'static str],
    replacement_lane: Lane,
) -> Row {
    Row::new(
        surface,
        DisplacedLane::ForgeQueryDocs,
        source_path,
        surface_name,
        current_authority_sources,
        "milestone 15 planner-owned routing consumers",
        Role::PriorProofInputConsumer,
        Disposition::QueryGap,
        Owner::ForgeQuery,
        replacement_lane,
        "the required Query identity or support artifact is still consumed through docs-level surfaces rather than a planner-owned Worth route lane",
        "the replacement lane consumes a real admitted Query artifact with the exact route identity Milestone 15 requires",
        false,
        false,
        Some(QueryGap::MissingIdentitySemantics),
        surface_name,
    )
}
