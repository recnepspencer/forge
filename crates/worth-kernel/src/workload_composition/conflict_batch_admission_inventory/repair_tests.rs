use super::test_support::{
    assert_distinct_source_paths, assert_violation_signatures, temp_firewall_root, write_source,
};
use super::{
    current_conflict_batch_admission_inventory, ConflictBatchAdmissionDisposition,
    ConflictBatchAdmissionOwner, ConflictBatchAdmissionQuerySurface,
    ConflictBatchAdmissionScanPattern, ConflictBatchAdmissionSourceFirewallReport,
    ConflictBatchAdmissionSurfaceIdentity,
};

#[test]
fn query_support_inventory_names_loaded_stable_docs_surfaces() {
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");

    for (identity, surface_name) in [
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryWorkspacePublicDownstreamDeliveryContract,
            "ForgeQueryWorkspace::public_downstream_delivery_contract",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryWorkspacePublicMutationSurfaceReport,
            "ForgeQueryWorkspace::public_mutation_surface_report",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryHardProhibitionRegistry,
            "hard_prohibition_registry",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryHardProhibitionDocumentationRows,
            "hard_prohibition_documentation_rows",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryBoundaryAuditSourceSet,
            "ForgeQueryBoundaryAuditSourceSet",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryTestBackendSchema,
            "ForgeQueryTestBackendSchema",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryConsumerResidueReport,
            "ForgeQueryConsumerResidueReport",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryTestBackendResidueAudit,
            "query_test_backend_residue_audit",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryProjectionConsumptionBindContract,
            "forge_query_projection_consumption_intent(...).admit()?.bind_contract()",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryLowerRuntimeBoundaryEnvelopeSupport,
            "forge_query_domain(...).for_lower_runtime_boundary_envelope(...).supports_boundary_traceability(...).because(...).materialize()",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryLowerRuntimeBoundarySourceSupport,
            "forge_query_domain(...).for_lower_runtime_boundary_source(...).supports_boundary_traceability(...).because(...).materialize()",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryDeclarationScopedCapabilitySupport,
            "forge_query_domain(...).for_intent(...).supports_capability(...).because(...).materialize()",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryDeclarationScopedTraceabilitySupport,
            "forge_query_domain(...).for_intent(...).supports_traceability(...).because(...).materialize()",
        ),
    ] {
        let row = inventory
            .row_for_surface(identity)
            .expect("Query support row should exist");
        assert_eq!(row.surface_name(), surface_name);
        assert_eq!(row.owner(), ConflictBatchAdmissionOwner::ForgeQuery);
        assert_eq!(
            row.disposition(),
            ConflictBatchAdmissionDisposition::QueryGap
        );
        assert_ne!(
            row.query_surface(),
            ConflictBatchAdmissionQuerySurface::NotQuery
        );
    }
}

#[test]
fn overlap_chain_authority_surfaces_have_exact_inventory_rows() {
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");

    for (identity, surface_name) in [
        (
            ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanBuildOverlapEdgeChains,
            "PlanarBooleanIntervalSubdivisionNormalizedScheduleSet::build_overlap_edge_chains",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanOverlapEdgeChainSet,
            "PlanarBooleanOverlapEdgeChainSet",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanOverlapEdgeChain,
            "PlanarBooleanOverlapEdgeChain",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanOverlapEdgeChainMember,
            "PlanarBooleanOverlapEdgeChainMember",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanOverlapEdgeChainDenial,
            "PlanarBooleanOverlapEdgeChainDenial",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanOverlapChainPosture,
            "PlanarBooleanOverlapChainPosture",
        ),
    ] {
        let row = inventory
            .row_for_surface(identity)
            .expect("overlap-chain row should exist");
        assert_eq!(row.surface_name(), surface_name);
        assert_eq!(row.owner(), ConflictBatchAdmissionOwner::WorthSpatial);
        assert_eq!(
            row.disposition(),
            ConflictBatchAdmissionDisposition::Migrate
        );
    }
}

#[test]
fn repair_rows_preserve_distinct_source_paths() {
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");

    assert_distinct_source_paths(
        &inventory,
        ConflictBatchAdmissionSurfaceIdentity::QueryProjectionConsumptionBindContract,
        ConflictBatchAdmissionSurfaceIdentity::QueryDeclarationScopedCapabilitySupport,
    );
    assert_distinct_source_paths(
        &inventory,
        ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanOverlapEdgeChainSet,
        ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanOverlapEdgeChain,
    );
}

#[test]
fn overlap_builders_and_products_without_helper_names_fail_closeout() {
    let root = temp_firewall_root("ordinary_overlap_builder_product");
    write_source(
        &root,
        "ordinary_overlap.rs",
        "#[allow(dead_code)]\n\
         pub fn build_overlap_edge_chains(\n\
             input: usize,\n\
         ) -> usize {\n\
             input\n\
         }\n\
         pub struct PlanarBooleanOverlapEdgeChainSet;\n",
    );
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");

    let report =
        ConflictBatchAdmissionSourceFirewallReport::scan_root_against_inventory(&root, &inventory)
            .expect("firewall scan should complete");

    assert_eq!(report.violations().len(), 2);
    assert_violation_signatures(
        &report,
        &[
            (
                "ordinary_overlap.rs",
                "build_overlap_edge_chains",
                ConflictBatchAdmissionScanPattern::OrdinaryOverlapHelper,
            ),
            (
                "ordinary_overlap.rs",
                "PlanarBooleanOverlapEdgeChainSet",
                ConflictBatchAdmissionScanPattern::OrdinaryOverlapHelper,
            ),
        ],
    );
    assert!(report.ensure_clean().is_err());
}
