use super::support::current_head_query_handle;
use super::support::current_lookup_rows;
use crate::facade::{topology_runtime, TopologyCurrentHeadReadHandleExt, TopologyRuntimeAdapters};
use crate::projection::read_views::domain::error::TopologyDomainQueryErrorKind;
use crate::projection::read_views::domain::report::TopologyDomainQueryExecutionEngine;
use crate::projection::read_views::domain::request::TopologyDomainQueryRequest;
use crate::projection::runtime_boundary::read_lowering::schema::{
    topology_domain_query_schema_view, TopologyDomainTraversalRelation,
};
use crate::projection::runtime_boundary::read_lowering::{
    lower_topology_domain_query, TopologyDomainQueryLoweringPosture,
    TopologyDomainQueryRelationshipProofPosture,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::PlannedExecutionRoute;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use schema::facade::QuerySchemaBasis;

#[test]
fn topology_domain_query_schema_admits_only_declared_traversal_relations() {
    let schema = topology_domain_query_schema_view()
        .expect("domain query schema should build through  query declarations");

    let comparison_schema = forge_query::facade::QuerySchemaView::new(
        QuerySchemaBasis::TopologyEntityLiveView.as_str(),
        [],
        [],
    );

    for relation in TopologyDomainTraversalRelation::ALL {
        let admitted = schema
            .relation(relation.relation_name().as_str())
            .expect("declared topology domain traversal relation must be admitted");
        assert_eq!(admitted.relation(), relation.relation_name().as_str());
        assert_eq!(admitted.max_depth(), relation.max_depth());
    }
    assert_eq!(
        schema.relation(TopologyRelationKind::LoopOwnsHalfEdge.kind_name()),
        None
    );
    assert_eq!(
        schema.relation(TopologyRelationKind::WireOwnsHalfEdge.kind_name()),
        None
    );
    assert_ne!(schema.basis(), comparison_schema.basis());
}

#[test]
fn topology_domain_request_lowering_is_canonical_for_equivalent_local_rewire_requests() {
    let request = TopologyDomainQueryRequest::LocalRewireNeighborhood {
        moved_half_edge_identity: ".topology.half_edge.7".to_string(),
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
        TopologyDomainQueryLoweringPosture::CanonicalTraversalLowered
    );
    assert_eq!(left.live_query_family().as_str(), "bounded_materialization");
    assert_eq!(
        left.planned_execution_route(),
        &PlannedExecutionRoute::RuntimeExpandedSnapshotRead
    );
    assert_eq!(left.planned_traversal_depth_limit(), 64);
    assert_eq!(
        left.relationship_proof_posture(),
        TopologyDomainQueryRelationshipProofPosture::Admitted
    );
    assert_eq!(left.relationship_proof_admission_count(), 2);
    assert_eq!(left.relationship_proof_topology_width(), 7);
    assert_eq!(left.relationship_proof_topology_classes().len(), 2);
    assert!(left.relationship_proof_admission_identity().is_some());
    assert_eq!(left.traversal_steps().len(), 2);
    assert_eq!(
        left.traversal_steps()[0].relation_name().as_str(),
        ".half_edge_next"
    );
    assert_eq!(
        left.traversal_steps()[1].relation_name().as_str(),
        ".half_edge_prev"
    );
}

#[test]
fn topology_domain_lowering_denies_zero_depth_before_canonical_authoring() {
    let loop_cycle = TopologyDomainQueryRequest::LoopCycleNeighborhood {
        start_half_edge_identity: ".topology.half_edge.7".to_string(),
        depth: 0,
    };
    let local_rewire = TopologyDomainQueryRequest::LocalRewireNeighborhood {
        moved_half_edge_identity: ".topology.half_edge.7".to_string(),
        cycle_depth: 0,
    };

    let loop_cycle_error = lower_topology_domain_query(&loop_cycle)
        .expect_err("zero-depth loop cycle must fail before canonical lowering");
    let local_rewire_error = lower_topology_domain_query(&local_rewire)
        .expect_err("zero-depth local rewire must fail before canonical lowering");

    assert_eq!(
        loop_cycle_error.kind(),
        TopologyDomainQueryErrorKind::UnsupportedTraversalDepth
    );
    assert_eq!(
        local_rewire_error.kind(),
        TopologyDomainQueryErrorKind::UnsupportedTraversalDepth
    );
}

#[test]
fn topology_domain_views_expose_canonical_lowering_and_explicit_debt_rows() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "query.domain-query.lowering-debt",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, "query.domain-query.lowering-debt.runtime")
        .expect(" topology runtime");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let handle = current_head_query_handle();
    let lookup_rows = current_lookup_rows(&mut workspace, &surfaces);
    let source_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeRadialNext)
        .expect("edge fan should expose radial source");
    let mut reads = handle.topology_reads(&mut workspace);

    let shared_vertex = reads
        .shared_vertex_half_edge_neighborhood(&source_identity)
        .expect("shared-vertex neighborhood should load");
    let radial = reads
        .radial_half_edge_neighborhood(&source_identity)
        .expect("radial neighborhood should load");
    let aggregate = reads.aggregate_report();

    assert_eq!(
        shared_vertex
            .request_report
            .lowering_artifact
            .lowering_posture(),
        TopologyDomainQueryLoweringPosture::CanonicalTraversalLowered
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
        TopologyDomainQueryRelationshipProofPosture::Admitted
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
        TopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
    );
    assert_eq!(
        radial.request_report.execution_engine,
        TopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
    );
    assert_eq!(aggregate.query_execution_count, 2);
    assert_eq!(aggregate.row_scan_fallback_count, 0);
    assert_eq!(aggregate.whole_view_fallback_count, 0);
    assert_eq!(aggregate.lowered_traversal_count, 4);
    assert_eq!(aggregate.relationship_proof_admission_count, 4);
    assert_eq!(aggregate.debt_rows.len(), 0);
    assert!(aggregate.debt_rows.iter().all(|row| row.request_count == 1));
}
