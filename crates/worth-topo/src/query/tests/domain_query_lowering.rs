use crate::facade::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::query::domain::error::WorthTopologyDomainQueryErrorKind;
use crate::query::domain::lowering::{
    lower_topology_domain_query, WorthTopologyDomainQueryLoweringPosture,
    WorthTopologyDomainQueryRelationshipProofPosture,
};
use crate::query::domain::report::WorthTopologyDomainQueryExecutionEngine;
use crate::query::domain::request::WorthTopologyDomainQueryRequest;
use crate::query::domain::schema::{
    worth_topology_domain_query_schema_view, WorthTopologyDomainTraversalRelation,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;
use forge_query::facade::PlannedExecutionRoute;
use worth_schema::facade::topology_authoring::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
};
use worth_schema::facade::{WorthQuerySchemaBasis, WorthTopologyRelationKind};

#[test]
fn topology_domain_query_schema_admits_only_declared_traversal_relations() {
    let schema = worth_topology_domain_query_schema_view()
        .expect("domain query schema should build through worth query declarations");

    let comparison_schema = forge_query::facade::QuerySchemaView::new(
        WorthQuerySchemaBasis::TopologyEntityLiveView.as_str(),
        [],
        [],
    );

    for relation in WorthTopologyDomainTraversalRelation::ALL {
        let admitted = schema
            .relation(relation.relation_name().as_str())
            .expect("declared topology domain traversal relation must be admitted");
        assert_eq!(admitted.relation(), relation.relation_name().as_str());
        assert_eq!(admitted.max_depth(), relation.max_depth());
    }
    assert_eq!(
        schema.relation(WorthTopologyRelationKind::LoopOwnsHalfEdge.kind_name()),
        None
    );
    assert_eq!(
        schema.relation(WorthTopologyRelationKind::WireOwnsHalfEdge.kind_name()),
        None
    );
    assert_ne!(schema.basis(), comparison_schema.basis());
}

#[test]
fn topology_domain_request_lowering_is_canonical_for_equivalent_local_rewire_requests() {
    let request = WorthTopologyDomainQueryRequest::LocalRewireNeighborhood {
        moved_half_edge_identity: "worth.topology.half_edge.7".to_string(),
        cycle_depth: 6,
    };

    let left = lower_topology_domain_query(&request).expect("lowering should stay admitted");
    let right = lower_topology_domain_query(&request).expect("lowering should stay admitted");

    assert_eq!(
        left.canonical_query_digest(),
        right.canonical_query_digest()
    );
    assert_eq!(
        left.canonical_result_shape_digest(),
        right.canonical_result_shape_digest()
    );
    assert_eq!(
        left.lowering_posture(),
        WorthTopologyDomainQueryLoweringPosture::CanonicalTraversalLowered
    );
    assert_eq!(left.live_query_family().as_str(), "bounded_materialization");
    assert_eq!(
        left.planned_execution_route(),
        &PlannedExecutionRoute::RuntimeExpandedSnapshotRead
    );
    assert_eq!(left.planned_traversal_depth_limit(), 64);
    assert_eq!(
        left.relationship_proof_posture(),
        WorthTopologyDomainQueryRelationshipProofPosture::Admitted
    );
    assert_eq!(left.relationship_proof_admission_count(), 2);
    assert_eq!(left.relationship_proof_topology_width(), 7);
    assert_eq!(left.relationship_proof_topology_classes().len(), 2);
    assert!(left.relationship_proof_admission_identity().is_some());
    assert_eq!(left.traversal_steps().len(), 2);
    assert_eq!(
        left.traversal_steps()[0].relation_name().as_str(),
        "worth.half_edge_next"
    );
    assert_eq!(
        left.traversal_steps()[1].relation_name().as_str(),
        "worth.half_edge_prev"
    );
}

#[test]
fn topology_domain_lowering_denies_zero_depth_before_canonical_authoring() {
    let loop_cycle = WorthTopologyDomainQueryRequest::LoopCycleNeighborhood {
        start_half_edge_identity: "worth.topology.half_edge.7".to_string(),
        depth: 0,
    };
    let local_rewire = WorthTopologyDomainQueryRequest::LocalRewireNeighborhood {
        moved_half_edge_identity: "worth.topology.half_edge.7".to_string(),
        cycle_depth: 0,
    };

    let loop_cycle_error = lower_topology_domain_query(&loop_cycle)
        .expect_err("zero-depth loop cycle must fail before canonical lowering");
    let local_rewire_error = lower_topology_domain_query(&local_rewire)
        .expect_err("zero-depth local rewire must fail before canonical lowering");

    assert_eq!(
        loop_cycle_error.kind(),
        WorthTopologyDomainQueryErrorKind::UnsupportedTraversalDepth
    );
    assert_eq!(
        local_rewire_error.kind(),
        WorthTopologyDomainQueryErrorKind::UnsupportedTraversalDepth
    );
}

#[test]
fn topology_domain_views_expose_canonical_lowering_and_explicit_debt_rows() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "query.domain-query.lowering-debt",
        &WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "query.domain-query.lowering-debt.runtime")
            .expect("worth topology runtime");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let domain_query = crate::query::domain::WorthTopologyDomainQuery::load(&workspace, &assembly)
        .expect("domain query should load");
    let source_identity = domain_query
        .first_source_identity_for_relation_kind(WorthTopologyRelationKind::HalfEdgeRadialNext)
        .expect("edge fan should expose radial source");

    let shared_vertex = domain_query
        .shared_vertex_half_edge_neighborhood(&mut workspace, &source_identity)
        .expect("shared-vertex neighborhood should load");
    let radial = domain_query
        .radial_half_edge_neighborhood(&mut workspace, &source_identity)
        .expect("radial neighborhood should load");
    let aggregate = domain_query.aggregate_report();

    assert_eq!(
        shared_vertex
            .request_report
            .lowering_artifact
            .lowering_posture(),
        WorthTopologyDomainQueryLoweringPosture::CanonicalTraversalLowered
    );
    assert_eq!(
        shared_vertex
            .request_report
            .lowering_artifact
            .live_query_family()
            .as_str(),
        "bounded_materialization"
    );
    assert_eq!(
        shared_vertex
            .request_report
            .lowering_artifact
            .relationship_proof_posture(),
        WorthTopologyDomainQueryRelationshipProofPosture::Admitted
    );
    assert_eq!(
        radial
            .request_report
            .lowering_artifact
            .canonical_result_shape_digest(),
        shared_vertex
            .request_report
            .lowering_artifact
            .canonical_result_shape_digest()
    );
    assert_eq!(
        shared_vertex
            .request_report
            .relationship_proof_admission_count,
        2
    );
    assert_eq!(radial.request_report.relationship_proof_admission_count, 2);
    assert_eq!(
        shared_vertex.request_report.execution_engine,
        WorthTopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
    );
    assert_eq!(
        radial.request_report.execution_engine,
        WorthTopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
    );
    assert_eq!(aggregate.query_native_execution_count, 2);
    assert_eq!(aggregate.row_scan_fallback_count, 0);
    assert_eq!(aggregate.whole_view_fallback_count, 2);
    assert_eq!(aggregate.lowered_traversal_count, 4);
    assert_eq!(aggregate.relationship_proof_admission_count, 4);
    assert_eq!(aggregate.debt_rows.len(), 2);
    assert!(aggregate.debt_rows.iter().all(|row| row.request_count == 1));
}
