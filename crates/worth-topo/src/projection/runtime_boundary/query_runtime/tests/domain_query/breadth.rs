use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

use super::super::query_runtime_support::QueryRuntimeSupport;
use crate::projection::read_views::domain::report::TopologyDomainQueryRequestFamily;
use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn relation_update_query_support_reports_domain_query_breadth_aggregate() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.domain-query-breadth",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, ".current-head.domain-query-breadth.runtime")
        .expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = QueryRuntimeSupport::load(&mut workspace, &assembly);
    let moved_identity =
        support.first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext);

    let _ = support.next_target_half_edge_id(&mut workspace, &moved_identity);
    let _ = support.prev_target_half_edge_id(&mut workspace, &moved_identity);
    let _ = support.successor_cycle_identities(&mut workspace, &moved_identity, 6);

    let aggregate = support.aggregate_report();
    assert_eq!(aggregate.request_count, 3);
    assert_eq!(aggregate.query_runtime_current_execution_count, 3);
    assert_eq!(aggregate.local_neighborhood_execution_count, 0);
    assert_eq!(aggregate.anchored_expansion_execution_count, 2);
    assert_eq!(aggregate.explicit_broad_search_execution_count, 1);
    assert_eq!(aggregate.locality_claim_mismatch_count, 0);
    assert_eq!(aggregate.query_execution_count, 3);
    assert_eq!(aggregate.row_scan_fallback_count, 0);
    assert_eq!(aggregate.lowered_traversal_count, 5);
    assert_eq!(aggregate.relationship_proof_admission_count, 5);
    assert_eq!(aggregate.whole_view_fallback_count, 0);
    assert_eq!(aggregate.repeated_rediscovery_denied_count, 0);
    assert_eq!(aggregate.debt_rows.len(), 0);
    assert_eq!(aggregate.family_rows.len(), 2);
    assert!(aggregate.family_rows.iter().any(|row| {
        row.request_family == TopologyDomainQueryRequestFamily::LoopCycleNeighborhood
            && row.request_count == 1
            && row.query_execution_count == 1
            && row.lowered_traversal_count == 1
            && row.relationship_proof_admission_count == 1
            && row.row_scan_fallback_count == 0
            && row.whole_view_fallback_count == 0
    }));
    assert!(aggregate.family_rows.iter().any(|row| {
        row.request_family == TopologyDomainQueryRequestFamily::LocalRewireNeighborhood
            && row.request_count == 2
            && row.query_execution_count == 2
            && row.lowered_traversal_count == 4
            && row.relationship_proof_admission_count == 4
            && row.row_scan_fallback_count == 0
            && row.whole_view_fallback_count == 0
    }));
}

#[test]
fn relation_update_query_support_reports_topology_operator_radial_breadth_aggregate() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.domain-query-breadth.radial",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.domain-query-breadth.radial.runtime",
    )
    .expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = QueryRuntimeSupport::load(&mut workspace, &assembly);
    let source_identity =
        support.first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeRadialNext);
    let current_target_identity =
        support.radial_current_target_identity(&mut workspace, &source_identity);

    let _ = support.alternate_same_edge_half_edge_id(
        &mut workspace,
        &source_identity,
        &current_target_identity,
    );
    let _ = support.different_edge_half_edge_id(&mut workspace, &source_identity);

    let aggregate = support.aggregate_report();
    assert_eq!(aggregate.request_count, 2);
    assert_eq!(aggregate.query_runtime_current_execution_count, 2);
    assert_eq!(aggregate.local_neighborhood_execution_count, 2);
    assert_eq!(aggregate.anchored_expansion_execution_count, 0);
    assert_eq!(aggregate.explicit_broad_search_execution_count, 0);
    assert_eq!(aggregate.locality_claim_mismatch_count, 0);
    assert_eq!(aggregate.query_execution_count, 2);
    assert_eq!(aggregate.row_scan_fallback_count, 0);
    assert_eq!(aggregate.whole_view_fallback_count, 0);
    assert_eq!(aggregate.lowered_traversal_count, 4);
    assert_eq!(aggregate.relationship_proof_admission_count, 4);
    assert_eq!(aggregate.debt_rows.len(), 0);
    assert_eq!(aggregate.family_rows.len(), 1);
    assert!(aggregate.family_rows.iter().any(|row| {
        row.request_family == TopologyDomainQueryRequestFamily::HalfEdgeRadialNeighborhood
            && row.request_count == 2
            && row.query_execution_count == 2
            && row.whole_view_fallback_count == 0
    }));
}
