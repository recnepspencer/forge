use crate::facade::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::query::domain::error::WorthTopologyDomainQueryErrorKind;
use crate::query::domain::report::{
    WorthTopologyDomainQueryExecutionEngine, WorthTopologyDomainQueryRequestFamily,
};
use crate::query::domain::WorthTopologyDomainQuery;
use crate::runtime_invariants::build_worth_milestone_one_runtime;
use forge_query::facade::{ForgeQueryReadBuiltInOperator, ForgeQueryReadScopeClass};
use worth_schema::facade::topology_authoring::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
};
use worth_schema::facade::WorthTopologyRelationKind;

#[test]
fn domain_query_reports_snapshot_indexed_fallback_posture() {
    let (mut workspace, assembly) = seeded_workspace(
        "query.domain-query.edge-fan",
        WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    );
    let domain_query =
        WorthTopologyDomainQuery::load(&workspace, &assembly).expect("domain query should load");

    assert_eq!(
        domain_query.fallback_posture().as_str(),
        "snapshot_indexed_fallback"
    );
    let _ = &mut workspace;
}

#[test]
fn domain_query_reports_supported_request_families() {
    let (mut workspace, assembly) = seeded_workspace(
        "query.domain-query.supported-families",
        WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    );
    let domain_query =
        WorthTopologyDomainQuery::load(&workspace, &assembly).expect("domain query should load");

    assert_eq!(
        domain_query.supported_request_families(),
        vec![
            WorthTopologyDomainQueryRequestFamily::HalfEdgeSharedVertexNeighborhood,
            WorthTopologyDomainQueryRequestFamily::HalfEdgeRadialNeighborhood,
            WorthTopologyDomainQueryRequestFamily::LoopCycleNeighborhood,
            WorthTopologyDomainQueryRequestFamily::LocalRewireNeighborhood,
        ]
    );
    let _ = &mut workspace;
}

#[test]
fn domain_query_exposes_shared_vertex_and_radial_half_edge_neighborhoods() {
    let (mut workspace, assembly) = seeded_workspace(
        "query.domain-query.edge-fan-neighborhoods",
        WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    );
    let domain_query =
        WorthTopologyDomainQuery::load(&workspace, &assembly).expect("domain query should load");
    let source_identity = domain_query
        .first_source_identity_for_relation_kind(WorthTopologyRelationKind::HalfEdgeRadialNext)
        .expect("edge fan should expose radial source");
    let shared_vertex = domain_query
        .shared_vertex_half_edge_neighborhood(&mut workspace, &source_identity)
        .expect("shared-vertex neighborhood should load");
    let radial = domain_query
        .radial_half_edge_neighborhood(&mut workspace, &source_identity)
        .expect("radial neighborhood should load");

    assert!(!shared_vertex.source_vertex_identities.is_empty());
    assert!(!shared_vertex
        .vertex_adjacent_half_edge_identities
        .is_empty());
    assert!(!shared_vertex
        .vertex_adjacent_different_edge_half_edge_identities
        .is_empty());
    assert_eq!(
        shared_vertex.request_report.execution_engine,
        WorthTopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
    );
    assert_eq!(
        shared_vertex.request_report.executed_scope_class,
        Some(ForgeQueryReadScopeClass::LocalNeighborhood)
    );
    assert_eq!(
        shared_vertex
            .request_report
            .executed_built_in_operator_coverage,
        vec![ForgeQueryReadBuiltInOperator::SharedEndpoint]
    );
    assert_eq!(shared_vertex.request_report.query_native_execution_count, 1);
    assert_eq!(shared_vertex.request_report.row_scan_fallback_count, 0);
    assert_eq!(shared_vertex.request_report.whole_view_fallback_count, 1);
    assert_eq!(shared_vertex.request_report.lowered_traversal_count, 2);
    assert_eq!(
        shared_vertex
            .request_report
            .relationship_proof_admission_count,
        2
    );
    assert_eq!(
        shared_vertex.request_report.lowering_artifact.root_entity(),
        "WorthTopologyEntity"
    );
    assert_eq!(
        shared_vertex
            .request_report
            .lowering_artifact
            .canonical_result_shape_digest(),
        radial
            .request_report
            .lowering_artifact
            .canonical_result_shape_digest()
    );
    assert_eq!(radial.source_half_edge_identity, source_identity);
    assert!(!radial.same_edge_half_edge_identities.is_empty());
    assert!(!radial.different_edge_half_edge_identities.is_empty());
    assert_eq!(
        radial.request_report.execution_engine,
        WorthTopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
    );
    assert_eq!(
        radial.request_report.executed_scope_class,
        Some(ForgeQueryReadScopeClass::LocalNeighborhood)
    );
    assert_eq!(
        radial.request_report.executed_built_in_operator_coverage,
        vec![ForgeQueryReadBuiltInOperator::SharedAttachment]
    );
    assert_eq!(radial.request_report.query_native_execution_count, 1);
    assert_eq!(radial.request_report.row_scan_fallback_count, 0);
    assert_eq!(radial.request_report.whole_view_fallback_count, 1);
    assert_eq!(radial.request_report.lowered_traversal_count, 2);
    assert_eq!(radial.request_report.relationship_proof_admission_count, 2);
    let aggregate = domain_query.aggregate_report();
    assert_eq!(aggregate.request_count, 2);
    assert_eq!(aggregate.query_native_execution_count, 2);
    assert_eq!(aggregate.row_scan_fallback_count, 0);
    assert_eq!(aggregate.whole_view_fallback_count, 2);
    assert_eq!(aggregate.lowered_traversal_count, 4);
    assert_eq!(aggregate.relationship_proof_admission_count, 4);
    assert_eq!(aggregate.debt_rows.len(), 2);
    assert_eq!(aggregate.family_rows.len(), 2);
    assert!(aggregate.family_rows.iter().any(|row| {
        row.request_family
            == WorthTopologyDomainQueryRequestFamily::HalfEdgeSharedVertexNeighborhood
            && row.request_count == 1
            && row.query_native_execution_count == 1
            && row.row_scan_fallback_count == 0
            && row.whole_view_fallback_count == 1
    }));
    assert!(aggregate.family_rows.iter().any(|row| {
        row.request_family == WorthTopologyDomainQueryRequestFamily::HalfEdgeRadialNeighborhood
            && row.request_count == 1
            && row.query_native_execution_count == 1
            && row.row_scan_fallback_count == 0
            && row.whole_view_fallback_count == 1
    }));
}

#[test]
fn domain_query_exposes_local_rewire_cycle_from_sheet_disk() {
    let (mut workspace, assembly) = seeded_workspace(
        "query.domain-query.local-rewire",
        WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    );
    let domain_query =
        WorthTopologyDomainQuery::load(&workspace, &assembly).expect("domain query should load");
    let moved_identity = domain_query
        .first_source_identity_for_relation_kind(WorthTopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");
    let local_rewire = domain_query
        .local_rewire_neighborhood(&moved_identity, 6)
        .expect("local rewire neighborhood should load");

    assert_eq!(local_rewire.moved_half_edge_identity, moved_identity);
    assert_eq!(local_rewire.cycle_identities.len(), 6);
    assert_eq!(
        local_rewire.cycle_identities.first(),
        Some(&moved_identity),
        "cycle should remain anchored to the requested moved halfedge"
    );
    assert_ne!(
        local_rewire.old_successor_identity,
        local_rewire.old_predecessor_identity
    );
    assert_eq!(
        local_rewire.request_report.fallback_posture.as_str(),
        "snapshot_indexed_fallback"
    );
    assert_eq!(local_rewire.request_report.row_scan_fallback_count, 1);
    assert_eq!(local_rewire.request_report.lowered_traversal_count, 2);
    assert_eq!(
        local_rewire
            .request_report
            .relationship_proof_admission_count,
        2
    );
    let aggregate = domain_query.aggregate_report();
    assert_eq!(aggregate.request_count, 1);
    assert_eq!(aggregate.row_scan_fallback_count, 1);
    assert_eq!(aggregate.lowered_traversal_count, 2);
    assert_eq!(aggregate.relationship_proof_admission_count, 2);
    assert_eq!(aggregate.family_rows.len(), 1);
    assert_eq!(aggregate.debt_rows.len(), 1);
    let _ = &mut workspace;
}

#[test]
fn domain_query_moves_loop_cycle_onto_query_runtime_with_explicit_decode_debt() {
    let (mut workspace, assembly) = seeded_workspace(
        "query.domain-query.loop-cycle",
        WorthMilestoneOnePrimitiveCase::WireClosed { half_edge_count: 5 },
    );
    let domain_query =
        WorthTopologyDomainQuery::load(&workspace, &assembly).expect("domain query should load");
    let start_identity = domain_query
        .first_source_identity_for_relation_kind(WorthTopologyRelationKind::HalfEdgeNext)
        .expect("wire should expose successor source");
    let loop_cycle = domain_query
        .loop_cycle(&mut workspace, &start_identity, 5)
        .expect("loop cycle should load through the query kernel");

    assert_eq!(loop_cycle.start_half_edge_identity, start_identity);
    assert_eq!(loop_cycle.cycle_identities.len(), 5);
    assert_eq!(loop_cycle.cycle_identities.first(), Some(&start_identity));
    assert_eq!(
        loop_cycle.request_report.execution_engine,
        WorthTopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
    );
    assert_eq!(
        loop_cycle.request_report.executed_scope_class,
        Some(ForgeQueryReadScopeClass::ExplicitBroadSearch)
    );
    assert_eq!(
        loop_cycle
            .request_report
            .executed_built_in_operator_coverage,
        vec![ForgeQueryReadBuiltInOperator::FrontierSearch]
    );
    assert_eq!(
        loop_cycle.request_report.fallback_posture.as_str(),
        "whole_view_debt"
    );
    assert_eq!(loop_cycle.request_report.query_native_execution_count, 1);
    assert_eq!(loop_cycle.request_report.row_scan_fallback_count, 0);
    assert_eq!(loop_cycle.request_report.whole_view_fallback_count, 1);
    let aggregate = domain_query.aggregate_report();
    assert_eq!(aggregate.request_count, 1);
    assert_eq!(aggregate.query_native_execution_count, 1);
    assert_eq!(aggregate.row_scan_fallback_count, 0);
    assert_eq!(aggregate.whole_view_fallback_count, 1);
    assert_eq!(aggregate.family_rows.len(), 1);
    assert_eq!(aggregate.family_rows[0].query_native_execution_count, 1);
}

#[test]
fn domain_query_denies_zero_and_oversized_cycle_depths_typed_and_early() {
    let (mut workspace, assembly) = seeded_workspace(
        "query.domain-query.depth-denial",
        WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    );
    let domain_query =
        WorthTopologyDomainQuery::load(&workspace, &assembly).expect("domain query should load");
    let moved_identity = domain_query
        .first_source_identity_for_relation_kind(WorthTopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");

    let zero_depth_error = domain_query
        .loop_cycle(&mut workspace, &moved_identity, 0)
        .expect_err("zero-depth loop cycle should fail typed and early");
    assert_eq!(
        zero_depth_error.kind(),
        WorthTopologyDomainQueryErrorKind::UnsupportedTraversalDepth
    );
    let oversized_depth_error = domain_query
        .local_rewire_neighborhood(&moved_identity, 65)
        .expect_err("oversized local rewire traversal should fail typed and early");
    assert_eq!(
        oversized_depth_error.kind(),
        WorthTopologyDomainQueryErrorKind::UnsupportedTraversalDepth
    );
    assert_eq!(domain_query.aggregate_report().request_count, 0);
    let _ = &mut workspace;
}

fn seeded_workspace(
    stem: &str,
    primitive: WorthMilestoneOnePrimitiveCase,
) -> (
    forge_query::facade::ForgeQueryWorkspace,
    WorthTopologyQueryAssembly,
) {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(&mut runtime, stem, &primitive).expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, &format!("{stem}.runtime"))
        .expect("worth topology runtime");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    (workspace, assembly)
}
