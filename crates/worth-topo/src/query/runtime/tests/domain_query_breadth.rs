use worth_schema::facade::topology_authoring::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
};
use worth_schema::facade::WorthTopologyRelationKind;

use super::relation_update_support::RelationUpdateQuerySupport;
use crate::query::domain::report::WorthTopologyDomainQueryRequestFamily;
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn relation_update_query_support_reports_domain_query_breadth_aggregate() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.domain-query-breadth",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.domain-query-breadth.runtime")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = RelationUpdateQuerySupport::load(&workspace, &assembly);
    let moved_identity =
        support.first_source_identity_for_relation_kind(WorthTopologyRelationKind::HalfEdgeNext);

    let _ = support.next_target_half_edge_id(&moved_identity);
    let _ = support.prev_target_half_edge_id(&moved_identity);
    let _ = support.successor_cycle_identities(&mut workspace, &moved_identity, 6);

    let aggregate = support.aggregate_report();
    assert_eq!(aggregate.request_count, 2);
    assert_eq!(aggregate.query_native_execution_count, 1);
    assert_eq!(aggregate.row_scan_fallback_count, 1);
    assert_eq!(aggregate.lowered_traversal_count, 3);
    assert_eq!(aggregate.relationship_proof_admission_count, 3);
    assert_eq!(aggregate.whole_view_fallback_count, 1);
    assert_eq!(aggregate.repeated_rediscovery_denied_count, 0);
    assert_eq!(aggregate.debt_rows.len(), 2);
    assert_eq!(aggregate.family_rows.len(), 2);
    assert!(aggregate.family_rows.iter().any(|row| {
        row.request_family == WorthTopologyDomainQueryRequestFamily::LoopCycleNeighborhood
            && row.request_count == 1
            && row.query_native_execution_count == 1
            && row.lowered_traversal_count == 1
            && row.relationship_proof_admission_count == 1
            && row.row_scan_fallback_count == 0
            && row.whole_view_fallback_count == 1
    }));
    assert!(aggregate.family_rows.iter().any(|row| {
        row.request_family == WorthTopologyDomainQueryRequestFamily::LocalRewireNeighborhood
            && row.request_count == 1
            && row.query_native_execution_count == 0
            && row.lowered_traversal_count == 2
            && row.relationship_proof_admission_count == 2
            && row.row_scan_fallback_count == 1
    }));
}

#[test]
fn relation_update_query_support_reports_query_native_radial_breadth_aggregate() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.domain-query-breadth.radial",
        &WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.current-head.domain-query-breadth.radial.runtime",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = RelationUpdateQuerySupport::load(&workspace, &assembly);
    let source_identity = support
        .first_source_identity_for_relation_kind(WorthTopologyRelationKind::HalfEdgeRadialNext);
    let current_target_identity = support.radial_current_target_identity(&source_identity);

    let _ = support.alternate_same_edge_half_edge_id(
        &mut workspace,
        &source_identity,
        &current_target_identity,
    );
    let _ = support.different_edge_half_edge_id(&mut workspace, &source_identity);

    let aggregate = support.aggregate_report();
    assert_eq!(aggregate.request_count, 2);
    assert_eq!(aggregate.query_native_execution_count, 2);
    assert_eq!(aggregate.row_scan_fallback_count, 0);
    assert_eq!(aggregate.whole_view_fallback_count, 2);
    assert_eq!(aggregate.lowered_traversal_count, 4);
    assert_eq!(aggregate.relationship_proof_admission_count, 4);
    assert_eq!(aggregate.debt_rows.len(), 1);
    assert_eq!(aggregate.family_rows.len(), 1);
    assert_eq!(
        aggregate.family_rows[0].request_family,
        WorthTopologyDomainQueryRequestFamily::HalfEdgeRadialNeighborhood
    );
    assert_eq!(aggregate.family_rows[0].request_count, 2);
    assert_eq!(aggregate.family_rows[0].query_native_execution_count, 2);
    assert_eq!(aggregate.family_rows[0].whole_view_fallback_count, 2);
}
