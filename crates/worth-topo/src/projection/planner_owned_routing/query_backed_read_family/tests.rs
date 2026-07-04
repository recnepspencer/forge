use super::admitted_route::{
    admit_topology_query_backed_read_family_route_with_selected_route_authority,
    TopologyQueryBackedReadFamilyAdmissionAuthority,
};
use super::{
    admit_topology_query_backed_consumer_cutover, admit_topology_query_backed_read_family_route,
    current_query_backed_consumer_residue_manifest, current_topology_query_backed_consumer_cutover,
    current_topology_query_backed_read_family_route_input, QueryBackedConsumerResidueDisposition,
    QueryBackedConsumerResidueOwner, TopologyQueryBackedConsumerCutover,
    TopologyReadModelReusePosture,
};
use crate::certification::support::historical_query_snapshot::historical_query_snapshot_for_read_basis;
use crate::certification::support::read_basis_query_runtime::HistoricalReadBasisQueryRuntime;
use crate::derived_topology::compiled_product_consumer_cutover::DerivedEquivalenceContractReport;
use crate::facade::{topology_runtime, TopologyRuntimeAdapters};
use crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces;
use crate::projection::TopologyQueryRowLookup;
use crate::query_domain::{
    topology_current_head_authoritative_context, topology_current_head_query_basis_evidence,
    topology_query_domain_entry, TopologyCurrentHeadReadHandleExt,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::ForgeQueryApplicationFacade;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;
use serde_json::Value;

#[test]
fn query_backed_read_route_explanation_uses_real_query_artifacts() {
    let route_input =
        current_topology_query_backed_read_family_route_input().expect("route input should build");
    let cutover =
        admit_topology_query_backed_read_family_route(&route_input).expect("route should admit");
    let observed_row = cutover
        .family_rows()
        .iter()
        .find(|row| row.query_execution_count() > 0)
        .expect("query-backed route should expose an observed family row");

    assert!(!route_input.handle_identity_digest().is_empty());
    assert!(!route_input.support_snapshot_digest().is_empty());
    assert!(!route_input.operating_context_identity_digest().is_empty());
    assert_eq!(
        cutover.handle_identity_digest(),
        route_input.handle_identity_digest()
    );
    assert_eq!(
        cutover.support_snapshot_digest(),
        route_input.support_snapshot_digest()
    );
    assert_eq!(
        cutover.operating_context_identity_digest(),
        route_input.operating_context_identity_digest()
    );
    assert_eq!(
        cutover.parity_verified_count(),
        route_input.parity_verified_count()
    );
    assert!(cutover.query_executed_debt_free_family_count() > 0);
    assert!(observed_row
        .selected_equivalence_family_identity()
        .is_some());
    assert!(observed_row
        .selected_compatibility_basis_identity_digest()
        .is_some());
}

#[test]
fn foreign_query_posture_cannot_explain_read_route() {
    let route_input =
        current_topology_query_backed_read_family_route_input().expect("route input should build");
    let error = admit_topology_query_backed_read_family_route_with_selected_route_authority(
        &route_input,
        &TopologyQueryBackedReadFamilyAdmissionAuthority::from_route_input(&route_input)
            .with_support_snapshot_digest("foreign-support-snapshot"),
    )
    .expect_err("foreign query posture should be rejected");

    assert!(error.detail().contains("query support snapshot"));
}

#[test]
fn current_public_closeout_cutover_exposes_loop_cycle_row_with_typed_authority() {
    let cutover = current_topology_query_backed_consumer_cutover()
        .expect("current topology query-backed cutover should build");
    let loop_cycle_row = cutover
        .family_rows()
        .iter()
        .find(|row| {
            row.request_family()
                == crate::projection::read_views::domain::TopologyReadRequestFamily::LoopCycleNeighborhood
        })
        .expect("loop-cycle family row");

    assert_ne!(
        loop_cycle_row.reuse_posture(),
        TopologyReadModelReusePosture::Denied
    );
    assert!(loop_cycle_row
        .selected_equivalence_family_identity()
        .is_some());
    assert!(loop_cycle_row
        .selected_equivalence_basis_identity_digest()
        .is_some());
    assert!(loop_cycle_row
        .selected_compatibility_basis_identity_digest()
        .is_some());
    assert!(loop_cycle_row
        .selected_reuse_basis_identity_digest()
        .is_some());
    assert!(!cutover.support_snapshot_digest().is_empty());
}

#[test]
fn hostile_query_backed_cutover_carries_typed_rebuild_denial_for_forced_non_reuse() {
    let cutover = build_hostile_query_backed_cutover(
        "phase13.query-backed-consumer-cutover.denied",
        DerivedEquivalenceContractReport::with_test_selected_family_contract_removed,
    );
    let denied_row = cutover
        .family_rows()
        .iter()
        .find(|row| {
            row.request_family()
                == crate::projection::read_views::domain::TopologyReadRequestFamily::HalfEdgeRadialNeighborhood
        })
        .expect("radial family row");

    assert_eq!(
        denied_row.reuse_posture(),
        TopologyReadModelReusePosture::Denied
    );
    assert!(denied_row.compiled_product_identity().is_some());
    assert!(denied_row.equivalence_policy_identity().is_some());
    assert!(denied_row.compiled_product_identity_digest().is_some());
    assert!(denied_row.equivalence_policy_identity_digest().is_some());
    assert!(denied_row.selected_equivalence_family_identity().is_none());
    assert!(denied_row
        .selected_equivalence_basis_identity_digest()
        .is_none());
    assert!(denied_row
        .selected_compatibility_basis_identity_digest()
        .is_none());
    assert!(denied_row.selected_reuse_basis_identity_digest().is_none());
    assert!(denied_row.reuse_decision_identity_digest().is_none());
    assert!(denied_row.rebuild_denial_identity().is_some());
    assert!(denied_row.rebuild_denial_identity_digest().is_some());
    assert_eq!(denied_row.query_execution_count(), 1);
    assert_eq!(denied_row.row_scan_fallback_count(), 0);
    assert_eq!(denied_row.whole_view_fallback_count(), 0);
    assert_eq!(denied_row.repeated_rediscovery_denied_count(), 0);
}

#[test]
fn query_boundary_residue_rows_are_exact() {
    let residue = current_query_backed_consumer_residue_manifest();

    assert_eq!(residue.len(), 2);
    assert!(residue.iter().all(|row| !row.source_path().is_empty()));
    assert!(residue.iter().all(|row| !row.current_surface().is_empty()));
    assert!(residue.iter().all(|row| !row.blocker().is_empty()));
    assert!(residue.iter().all(|row| !row.removal_trigger().is_empty()));
    assert!(residue.iter().any(|row| {
        row.owner() == QueryBackedConsumerResidueOwner::WorthTopo
            && row.disposition() == QueryBackedConsumerResidueDisposition::ExplicitResidue
    }));
    assert!(residue
        .iter()
        .any(|row| { row.current_surface() == "TopologyReadViewParityArtifact::view_digest_hex" }));
    assert!(residue.iter().any(|row| {
        row.current_surface()
            == "historical_context_for_family(... HistoricalPathReuseDescriptor::retained_reuse())"
            && row.owner() == QueryBackedConsumerResidueOwner::WorthTopo
            && row.disposition() == QueryBackedConsumerResidueDisposition::ExplicitResidue
            && row.query_gap_kind().is_none()
    }));
}

fn build_hostile_query_backed_cutover(
    scenario: &str,
    mutate_contract: impl FnOnce(DerivedEquivalenceContractReport) -> DerivedEquivalenceContractReport,
) -> TopologyQueryBackedConsumerCutover {
    let mut runtime = build_milestone_one_runtime().expect("runtime should build");
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        scenario,
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("primitive should seed");
    let mut historical_query_runtime = HistoricalReadBasisQueryRuntime::open(
        &runtime,
        verified.read_basis().clone(),
        &format!("{scenario}.historical"),
    )
    .expect("historical query runtime should open");
    let historical_snapshot =
        historical_query_snapshot_for_read_basis(&mut historical_query_runtime)
            .expect("historical query snapshot should decode");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, &format!("{scenario}.runtime"))
        .expect("current-head query runtime should open");
    let surfaces =
        declare_topology_query_surfaces(&mut workspace).expect("declared query surfaces");
    let entity_rows = workspace.read::<Value>(surfaces.entities());
    let relation_rows = workspace.read::<Value>(surfaces.relations());
    let source_identity = TopologyQueryRowLookup::new(&entity_rows, &relation_rows)
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeRadialNext)
        .expect("edge fan should expose a radial source");
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let basis_evidence =
        topology_current_head_query_basis_evidence(&facade).expect("basis evidence");
    let handle = topology_query_domain_entry(&facade)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .expect("query domain should validate")
        .admit()
        .expect("query domain should admit");
    let mut reads = handle.topology_reads(&mut workspace);
    let anchor =
        crate::projection::read_views::domain::TopologyReadAnchorIdentity::from_runtime_row_label(
            &source_identity,
        );
    let _radial = reads
        .radial_half_edge_neighborhood(&anchor)
        .expect("radial neighborhood should execute through Query");

    admit_topology_query_backed_consumer_cutover(
        &reads,
        &basis_evidence,
        &mutate_contract(historical_snapshot.equivalence_contract().clone()),
    )
}
