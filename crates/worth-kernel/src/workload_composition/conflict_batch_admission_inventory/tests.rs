use super::closeout::ConflictBatchAdmissionInventory;
use super::error::ConflictBatchAdmissionInventoryError;
use super::row::{
    ConflictBatchAdmissionAuthorityKind, ConflictBatchAdmissionCertificationPosture,
    ConflictBatchAdmissionCostPosture, ConflictBatchAdmissionDisposition,
    ConflictBatchAdmissionOwner, ConflictBatchAdmissionQuerySurface,
    ConflictBatchAdmissionReplacementPhase, ConflictBatchAdmissionRowScope,
    ConflictBatchAdmissionSurfaceIdentity,
};
use super::test_support::assert_distinct_source_paths;
use super::{
    current_conflict_batch_admission_inventory, ConflictBatchAdmissionInventoryRowBuilder,
    ConflictBatchAdmissionSourceFirewallReport,
};

#[test]
fn conflict_inventory_has_no_keep_or_unclassified_rows() {
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");

    assert_eq!(inventory.unclassified_count(), 0);
    assert_eq!(inventory.keep_disposition_count(), 0);
    assert!(inventory.counters().migrate_rows() > 0);
    assert!(inventory.counters().query_gap_rows() > 0);
    assert!(inventory.cut_line().ready_for_aspect_routed_replacement());
}

#[test]
fn seeded_operational_surfaces_have_exact_identity_rows() {
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");

    assert_seed(
        &inventory,
        ConflictBatchAdmissionSurfaceIdentity::WorthWorkloadCompose,
        "WorthWorkload::compose",
        ConflictBatchAdmissionOwner::WorthKernel,
    );
    assert_seed(
        &inventory,
        ConflictBatchAdmissionSurfaceIdentity::LookupConsumedWorkloadCompositionAdmit,
        "LookupConsumedWorkloadComposition::admit",
        ConflictBatchAdmissionOwner::WorthKernel,
    );
    assert_seed(
        &inventory,
        ConflictBatchAdmissionSurfaceIdentity::CoplanarOverlapWorkloadOperatorExecute,
        "CoplanarOverlapWorkloadOperator::execute",
        ConflictBatchAdmissionOwner::WorthSpatial,
    );
}

#[test]
fn query_proof_surfaces_are_query_gaps_not_conflict_authority() {
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");

    let query_rows = inventory
        .rows()
        .iter()
        .filter(|row| row.query_surface() != ConflictBatchAdmissionQuerySurface::NotQuery)
        .collect::<Vec<_>>();

    assert!(!query_rows.is_empty());
    for row in query_rows {
        assert_eq!(
            row.disposition(),
            ConflictBatchAdmissionDisposition::QueryGap
        );
        assert_eq!(
            row.certification_posture(),
            ConflictBatchAdmissionCertificationPosture::QuerySupportOnlyCannotMintConflictAuthority
        );
    }
}

#[test]
fn query_support_inventory_names_spec_public_surfaces() {
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");

    let query_surfaces = [
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryForgeQueryWorkspace,
            "ForgeQueryWorkspace",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryWorkspacePublicSupportMatrix,
            "ForgeQueryWorkspace::public_support_matrix",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryWorkspacePublicApiContract,
            "ForgeQueryWorkspace::public_api_contract",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryWorkspacePublicHandleContract,
            "ForgeQueryWorkspace::public_handle_contract",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryWorkspaceAdmitPublicApiFamily,
            "ForgeQueryWorkspace::admit_public_api_family",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryEvidenceReportDeclaration,
            "EvidenceReportDeclaration",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryHardProhibitionBoundaryAudit,
            "hard_prohibition_boundary_audit",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryConsumerResidueAudit,
            "query_consumer_residue_audit",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryInMemoryTestRuntime,
            "in_memory_test_runtime",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryConsumeProjectionFacts,
            "consume_projection_facts",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryDeclareProjectionFactConsumption,
            "declare_projection_fact_consumption",
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
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryDeclarationEnvelopeInput,
            "ForgeQueryDeclarationEnvelopeInput",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryDeclarationEnvelope,
            "ForgeQueryDeclarationEnvelope",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryDeclarationEnvelopeChecked,
            "ForgeQueryDeclarationEnvelopeChecked",
        ),
        (
            ConflictBatchAdmissionSurfaceIdentity::QueryProjectionConsumptionBindContract,
            "forge_query_projection_consumption_intent(...).admit()?.bind_contract()",
        ),
    ];

    for (identity, surface_name) in query_surfaces {
        assert_query_gap(&inventory, identity, surface_name);
    }

    assert!(inventory
        .rows()
        .iter()
        .all(|row| row.surface_name() != "forge_query::facade::consumer_kit::*"));
}

#[test]
fn query_support_inventory_rejects_ambiguous_fragment_names() {
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");

    for forbidden_name in [
        "bind_contract",
        "for_lower_runtime_boundary_envelope",
        "for_lower_runtime_boundary_source",
        "for_intent(...).supports_capability",
        "for_intent(...).supports_traceability",
    ] {
        assert!(
            inventory
                .rows()
                .iter()
                .all(|row| row.surface_name() != forbidden_name),
            "query support inventory must not collapse exact public identities into {forbidden_name}",
        );
    }
}

#[test]
fn inventory_rows_preserve_source_identity_for_near_collision_surfaces() {
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
        ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanOverlapEdgeChainMember,
    );
}

#[test]
fn row_contract_rejects_query_surface_as_local_authority() {
    let error = ConflictBatchAdmissionInventoryRowBuilder::default()
        .surface_identity(ConflictBatchAdmissionSurfaceIdentity::WorthWorkloadCompose)
        .source_path("crates/worth-kernel/src/workload_composition/worth_workload.rs")
        .surface_name("WorthWorkload::compose")
        .owner(ConflictBatchAdmissionOwner::WorthKernel)
        .current_caller("test")
        .authority_kind(ConflictBatchAdmissionAuthorityKind::WorkloadCompositionAdmission)
        .disposition(ConflictBatchAdmissionDisposition::Migrate)
        .replacement_phase(ConflictBatchAdmissionReplacementPhase::PhaseFourAdmittedConflictInput)
        .blocker("test blocker")
        .removal_trigger("test trigger")
        .certification_posture(
            ConflictBatchAdmissionCertificationPosture::OrdinaryProductionReachable,
        )
        .cost_posture(ConflictBatchAdmissionCostPosture::LocalTypedAdmission)
        .query_surface(ConflictBatchAdmissionQuerySurface::ConsumerKitProof)
        .row_scope(ConflictBatchAdmissionRowScope::ConcreteSource)
        .build()
        .expect_err("local authority cannot claim Query proof surface");

    assert!(matches!(
        error,
        ConflictBatchAdmissionInventoryError::QuerySurfaceCannotMintAuthority(
            ConflictBatchAdmissionSurfaceIdentity::WorthWorkloadCompose
        )
    ));
}

#[test]
fn default_workspace_firewall_is_clean_against_inventory() {
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");
    let report =
        ConflictBatchAdmissionSourceFirewallReport::scan_default_workspace_against_inventory(
            &inventory,
        )
        .expect("default firewall scan should complete");

    assert!(report.scanned_file_count() > 0);
    assert_eq!(report.violations().len(), 0);
    report
        .ensure_clean()
        .expect("current inventoried source should pass");
}

fn assert_seed(
    inventory: &ConflictBatchAdmissionInventory,
    identity: ConflictBatchAdmissionSurfaceIdentity,
    expected_name: &str,
    expected_owner: ConflictBatchAdmissionOwner,
) {
    let row = inventory
        .row_for_surface(identity)
        .expect("required seed row should exist");
    assert_eq!(row.surface_name(), expected_name);
    assert_eq!(row.owner(), expected_owner);
    assert_eq!(
        row.disposition(),
        ConflictBatchAdmissionDisposition::Migrate
    );
}

fn assert_query_gap(
    inventory: &ConflictBatchAdmissionInventory,
    identity: ConflictBatchAdmissionSurfaceIdentity,
    expected_name: &str,
) {
    let row = inventory
        .row_for_surface(identity)
        .expect("required Query support row should exist");
    assert_eq!(row.surface_name(), expected_name);
    assert_eq!(row.owner(), ConflictBatchAdmissionOwner::ForgeQuery);
    assert_eq!(
        row.disposition(),
        ConflictBatchAdmissionDisposition::QueryGap
    );
    assert_eq!(
        row.certification_posture(),
        ConflictBatchAdmissionCertificationPosture::QuerySupportOnlyCannotMintConflictAuthority
    );
}
