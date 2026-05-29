use super::support::{current_lookup_rows, snapshot_basis_workspace};
use crate::facade::{topology_runtime, TopologyQueryAssembly, TopologyRuntimeAdapters};
use crate::projection::read_views::domain::error::TopologyDomainQueryErrorKind;
use crate::projection::read_views::domain::report::{
    TopologyDomainQueryExecutionEngine, TopologyDomainQueryRequestFamily,
};
use crate::projection::read_views::domain::TopologyDomainQuery;
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::{ForgeQueryReadBuiltInOperator, ForgeQueryReadScopeClass};
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use schema::facade::TopologyRelationKind;

#[test]
fn domain_query_reports_request_only_posture() {
    let (mut workspace, _assembly) = seeded_workspace(
        "query.domain-query.edge-fan",
        MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    );
    let domain_query = TopologyDomainQuery::load();

    assert_eq!(domain_query.fallback_posture().as_str(), "none");
    assert_eq!(domain_query.aggregate_report().request_count, 0);
    let _ = &mut workspace;
}

#[test]
fn domain_query_reports_supported_request_families() {
    let (mut workspace, _assembly) = seeded_workspace(
        "query.domain-query.supported-families",
        MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    );
    let domain_query = TopologyDomainQuery::load();

    assert_eq!(
        domain_query.supported_request_families(),
        vec![
            TopologyDomainQueryRequestFamily::HalfEdgeSharedVertexNeighborhood,
            TopologyDomainQueryRequestFamily::HalfEdgeRadialNeighborhood,
            TopologyDomainQueryRequestFamily::LoopCycleNeighborhood,
            TopologyDomainQueryRequestFamily::LocalRewireNeighborhood,
        ]
    );
    let _ = &mut workspace;
}

#[test]
fn domain_query_exposes_shared_vertex_and_radial_half_edge_neighborhoods() {
    let (mut workspace, assembly) = seeded_workspace(
        "query.domain-query.edge-fan-neighborhoods",
        MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    );
    let domain_query = TopologyDomainQuery::load();
    let lookup_rows = current_lookup_rows(&mut workspace, &assembly);
    let source_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeRadialNext)
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
        TopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
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
    assert_eq!(shared_vertex.request_report.query_execution_count, 1);
    assert_eq!(shared_vertex.request_report.row_scan_fallback_count, 0);
    assert_eq!(shared_vertex.request_report.whole_view_fallback_count, 0);
    assert_eq!(shared_vertex.request_report.lowered_traversal_count, 2);
    assert_eq!(
        shared_vertex
            .request_report
            .relationship_proof_admission_count,
        2
    );
    assert_eq!(
        shared_vertex.request_report.lowering_artifact.root_entity(),
        "TopologyEntity"
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
    assert_eq!(
        radial.request_report.execution_engine,
        TopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
    );
    assert_eq!(
        radial.request_report.executed_scope_class,
        Some(ForgeQueryReadScopeClass::LocalNeighborhood)
    );
    assert_eq!(
        radial.request_report.executed_built_in_operator_coverage,
        vec![ForgeQueryReadBuiltInOperator::SharedAttachment]
    );
    assert_eq!(radial.request_report.query_execution_count, 1);
    assert_eq!(radial.request_report.row_scan_fallback_count, 0);
    assert_eq!(radial.request_report.whole_view_fallback_count, 0);
    assert_eq!(radial.request_report.lowered_traversal_count, 2);
    assert_eq!(radial.request_report.relationship_proof_admission_count, 2);
    let aggregate = domain_query.aggregate_report();
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
    assert_eq!(aggregate.family_rows.len(), 2);
    assert!(aggregate.family_rows.iter().any(|row| {
        row.request_family == TopologyDomainQueryRequestFamily::HalfEdgeSharedVertexNeighborhood
            && row.request_count == 1
            && row.query_execution_count == 1
            && row.row_scan_fallback_count == 0
            && row.whole_view_fallback_count == 0
    }));
    assert!(aggregate.family_rows.iter().any(|row| {
        row.request_family == TopologyDomainQueryRequestFamily::HalfEdgeRadialNeighborhood
            && row.request_count == 1
            && row.query_execution_count == 1
            && row.row_scan_fallback_count == 0
            && row.whole_view_fallback_count == 0
    }));
    assert_eq!(aggregate.execution_rows.len(), 2);
}

#[test]
fn domain_query_exposes_local_rewire_cycle_from_sheet_disk() {
    let (mut workspace, assembly) = seeded_workspace(
        "query.domain-query.local-rewire",
        MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    );
    let domain_query = TopologyDomainQuery::load();
    let lookup_rows = current_lookup_rows(&mut workspace, &assembly);
    let moved_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");
    let local_rewire = domain_query
        .local_rewire_neighborhood(&mut workspace, &moved_identity, 6)
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
        local_rewire.request_report.execution_engine,
        TopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
    );
    assert_eq!(
        local_rewire.request_report.fallback_posture.as_str(),
        "none"
    );
    assert_eq!(local_rewire.request_report.query_execution_count, 1);
    assert_eq!(local_rewire.request_report.row_scan_fallback_count, 0);
    assert_eq!(local_rewire.request_report.whole_view_fallback_count, 0);
    assert_eq!(local_rewire.request_report.lowered_traversal_count, 2);
    assert_eq!(
        local_rewire
            .request_report
            .relationship_proof_admission_count,
        2
    );
    let aggregate = domain_query.aggregate_report();
    assert_eq!(aggregate.request_count, 1);
    assert_eq!(aggregate.query_runtime_current_execution_count, 1);
    assert_eq!(aggregate.local_neighborhood_execution_count, 0);
    assert_eq!(aggregate.anchored_expansion_execution_count, 1);
    assert_eq!(aggregate.explicit_broad_search_execution_count, 0);
    assert_eq!(aggregate.locality_claim_mismatch_count, 0);
    assert_eq!(aggregate.query_execution_count, 1);
    assert_eq!(aggregate.row_scan_fallback_count, 0);
    assert_eq!(aggregate.whole_view_fallback_count, 0);
    assert_eq!(aggregate.lowered_traversal_count, 2);
    assert_eq!(aggregate.relationship_proof_admission_count, 2);
    assert_eq!(aggregate.family_rows.len(), 1);
    assert_eq!(aggregate.debt_rows.len(), 0);
    let _ = &mut workspace;
}

#[test]
fn domain_query_moves_loop_cycle_onto_query_runtime_without_decode_debt() {
    let (mut workspace, assembly) = seeded_workspace(
        "query.domain-query.loop-cycle",
        MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 5 },
    );
    let domain_query = TopologyDomainQuery::load();
    let lookup_rows = current_lookup_rows(&mut workspace, &assembly);
    let start_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("wire should expose successor source");
    let loop_cycle = domain_query
        .loop_cycle(&mut workspace, &start_identity, 5)
        .expect("loop cycle should load through the query kernel");

    assert_eq!(loop_cycle.start_half_edge_identity, start_identity);
    assert_eq!(loop_cycle.cycle_identities.len(), 5);
    assert_eq!(loop_cycle.cycle_identities.first(), Some(&start_identity));
    assert_eq!(
        loop_cycle.request_report.execution_engine,
        TopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
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
    assert_eq!(loop_cycle.request_report.fallback_posture.as_str(), "none");
    assert_eq!(loop_cycle.request_report.query_execution_count, 1);
    assert_eq!(loop_cycle.request_report.row_scan_fallback_count, 0);
    assert_eq!(loop_cycle.request_report.whole_view_fallback_count, 0);
    let aggregate = domain_query.aggregate_report();
    assert_eq!(aggregate.request_count, 1);
    assert_eq!(aggregate.query_runtime_current_execution_count, 1);
    assert_eq!(aggregate.local_neighborhood_execution_count, 0);
    assert_eq!(aggregate.anchored_expansion_execution_count, 0);
    assert_eq!(aggregate.explicit_broad_search_execution_count, 1);
    assert_eq!(aggregate.locality_claim_mismatch_count, 0);
    assert_eq!(aggregate.query_execution_count, 1);
    assert_eq!(aggregate.row_scan_fallback_count, 0);
    assert_eq!(aggregate.whole_view_fallback_count, 0);
    assert_eq!(aggregate.family_rows.len(), 1);
    assert_eq!(aggregate.family_rows[0].query_execution_count, 1);
}

#[test]
fn snapshot_domain_query_uses_historical_basis_context_receipt() {
    let stem = "query.domain-query.snapshot-loop-cycle";
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        stem,
        &MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 5 },
    )
    .expect("seed primitive");
    let (mut workspace, assembly) =
        snapshot_basis_workspace(&runtime, &format!("{stem}.snapshot"), &verified.read_basis);
    let domain_query = TopologyDomainQuery::load();
    let lookup_rows = current_lookup_rows(&mut workspace, &assembly);
    let start_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("wire should expose successor source");

    let loop_cycle = domain_query
        .loop_cycle(&mut workspace, &start_identity, 5)
        .expect("snapshot loop cycle should load through historical query context");

    assert_eq!(
        loop_cycle.request_report.execution_engine,
        TopologyDomainQueryExecutionEngine::QueryRuntimeHistorical
    );
    assert_eq!(
        loop_cycle.request_report.executed_snapshot_token.as_deref(),
        Some(workspace.snapshot_token().as_str())
    );
    assert!(loop_cycle
        .request_report
        .executed_basis_digest
        .as_ref()
        .is_some_and(|digest| !digest.is_empty()));
    assert_eq!(loop_cycle.request_report.query_execution_count, 1);
    assert_eq!(loop_cycle.request_report.row_scan_fallback_count, 0);
    assert_eq!(loop_cycle.request_report.whole_view_fallback_count, 0);
    let aggregate = domain_query.aggregate_report();
    assert_eq!(aggregate.query_runtime_current_execution_count, 0);
    assert_eq!(aggregate.query_runtime_historical_execution_count, 1);
    assert_eq!(aggregate.query_execution_count, 1);
    assert_eq!(aggregate.debt_rows.len(), 0);
}

#[test]
fn domain_query_denies_zero_and_oversized_cycle_depths_typed_and_early() {
    let (mut workspace, assembly) = seeded_workspace(
        "query.domain-query.depth-denial",
        MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    );
    let domain_query = TopologyDomainQuery::load();
    let lookup_rows = current_lookup_rows(&mut workspace, &assembly);
    let moved_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");

    let zero_depth_error = domain_query
        .loop_cycle(&mut workspace, &moved_identity, 0)
        .expect_err("zero-depth loop cycle should fail typed and early");
    assert_eq!(
        zero_depth_error.kind(),
        TopologyDomainQueryErrorKind::UnsupportedTraversalDepth
    );
    let oversized_depth_error = domain_query
        .local_rewire_neighborhood(&mut workspace, &moved_identity, 65)
        .expect_err("oversized local rewire traversal should fail typed and early");
    assert_eq!(
        oversized_depth_error.kind(),
        TopologyDomainQueryErrorKind::UnsupportedTraversalDepth
    );
    let aggregate = domain_query.aggregate_report();
    assert_eq!(aggregate.request_count, 0);
    let _ = &mut workspace;
}

fn seeded_workspace(
    stem: &str,
    primitive: MilestoneOnePrimitiveCase,
) -> (
    forge_query::facade::ForgeQueryWorkspace,
    TopologyQueryAssembly,
) {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive(&mut runtime, stem, &primitive).expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, &format!("{stem}.runtime")).expect(" topology runtime");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    (workspace, assembly)
}
