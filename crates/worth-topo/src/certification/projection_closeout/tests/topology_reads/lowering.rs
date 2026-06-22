use super::support::current_head_query_handle;
use super::support::current_lookup_rows;
use crate::facade::{topology_runtime, TopologyRuntimeAdapters};
use crate::projection::read_views::domain::error::TopologyReadErrorKind;
use crate::projection::read_views::domain::report::TopologyReadExecutionEngine;
use crate::projection::read_views::domain::request::{
    TopologyReadAnchorIdentity, TopologyReadRequest,
};
use crate::projection::runtime_boundary::read_lowering::schema::{
    topology_read_schema_view, TopologyDomainTraversalRelation,
};
use crate::projection::runtime_boundary::read_lowering::{
    lower_topology_read, TopologyReadLoweringPosture, TopologyReadRelationshipProofPosture,
};
use crate::query_domain::TopologyCurrentHeadReadHandleExt;
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::{ForgeQueryEntityIdentity, PlannedExecutionRoute};
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityParts;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;
use schema::facade::QuerySchemaBasis;

#[test]
fn topology_read_schema_admits_only_declared_traversal_relations() {
    let schema = topology_read_schema_view()
        .expect("topology read schema should build through  query declarations");

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
fn topology_read_request_lowering_is_canonical_for_equivalent_local_rewire_requests() {
    let request = TopologyReadRequest::LocalRewireNeighborhood {
        moved_half_edge_identity: TopologyReadAnchorIdentity::from_runtime_row_label(
            ".topology.half_edge.7",
        ),
        cycle_depth: 6,
    };

    let left = lower_topology_read(&request).expect("lowering should stay admitted");
    let right = lower_topology_read(&request).expect("lowering should stay admitted");

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
        TopologyReadLoweringPosture::CanonicalTraversalLowered
    );
    assert_eq!(left.live_query_family().as_str(), "bounded_materialization");
    assert_eq!(
        left.planned_execution_route(),
        &PlannedExecutionRoute::RuntimeExpandedSnapshotRead
    );
    assert_eq!(left.planned_traversal_depth_limit(), 64);
    assert_eq!(
        left.relationship_proof_posture(),
        TopologyReadRelationshipProofPosture::Deferred
    );
    assert_eq!(left.relationship_proof_admission_count(), 0);
    assert_eq!(left.relationship_proof_topology_width(), 0);
    assert_eq!(left.relationship_proof_topology_classes().len(), 0);
    assert!(left.relationship_proof_admission_identity().is_none());
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
fn topology_read_lowering_denies_zero_depth_before_canonical_authoring() {
    let loop_cycle = TopologyReadRequest::LoopCycleNeighborhood {
        start_half_edge_identity: TopologyReadAnchorIdentity::from_runtime_row_label(
            ".topology.half_edge.7",
        ),
        depth: 0,
    };
    let local_rewire = TopologyReadRequest::LocalRewireNeighborhood {
        moved_half_edge_identity: TopologyReadAnchorIdentity::from_runtime_row_label(
            ".topology.half_edge.7",
        ),
        cycle_depth: 0,
    };

    let loop_cycle_error = lower_topology_read(&loop_cycle)
        .expect_err("zero-depth loop cycle must fail before canonical lowering");
    let local_rewire_error = lower_topology_read(&local_rewire)
        .expect_err("zero-depth local rewire must fail before canonical lowering");

    assert_eq!(
        loop_cycle_error.kind(),
        TopologyReadErrorKind::UnsupportedTraversalDepth
    );
    assert_eq!(
        local_rewire_error.kind(),
        TopologyReadErrorKind::UnsupportedTraversalDepth
    );
}

#[test]
fn topology_read_lowering_denies_projection_reconstructed_anchor_before_canonical_authoring() {
    let request = TopologyReadRequest::LocalRewireNeighborhood {
        moved_half_edge_identity: TopologyReadAnchorIdentity::from_runtime_row_label(
            "projection:.topology.half_edge.7",
        ),
        cycle_depth: 6,
    };

    let error = lower_topology_read(&request)
        .expect_err("projection-reconstructed identity must fail before canonical lowering");

    assert_eq!(
        error.kind(),
        TopologyReadErrorKind::RuntimeBoundaryAuthorityDenied
    );
    assert!(error
        .to_string()
        .contains("worth-topo/runtime_boundary/read_lowering"));
    assert!(error.to_string().contains("runtime_row_label"));
}

#[test]
fn topology_read_anchor_admits_only_typed_query_entity_identity() {
    let entity_identity = ForgeQueryEntityIdentity::from_relational_record(
        RelationalBridgeRecordIdentityParts::entity(0, 7, 0),
    );
    let anchor = TopologyReadAnchorIdentity::from_query_entity_identity(&entity_identity)
        .expect("relational query entity identity should admit as topology read anchor");

    let request = TopologyReadRequest::LoopCycleNeighborhood {
        start_half_edge_identity: anchor,
        depth: 2,
    };
    let artifact = lower_topology_read(&request).expect("typed query entity anchor lowers");

    assert_eq!(
        artifact.relationship_proof_posture(),
        TopologyReadRelationshipProofPosture::Deferred
    );
    assert_eq!(artifact.traversal_steps().len(), 1);
}

#[test]
fn topology_read_anchor_denies_typed_query_relation_identity_before_lowering() {
    let relation_identity = ForgeQueryEntityIdentity::from_relational_record(
        RelationalBridgeRecordIdentityParts::relation(0, 7, 0),
    );
    let error = TopologyReadAnchorIdentity::from_query_entity_identity(&relation_identity)
        .expect_err("relation identity must not promote to topology read anchor");

    assert_eq!(
        error.kind(),
        TopologyReadErrorKind::RuntimeBoundaryAuthorityDenied
    );
    assert!(error
        .to_string()
        .contains("worth-topo/runtime_boundary/read_lowering"));
    assert!(error.to_string().contains("query_entity_identity"));
}

#[test]
fn topology_read_views_expose_canonical_lowering_and_explicit_debt_rows() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "query.topology-read.lowering-debt",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, "query.topology-read.lowering-debt.runtime")
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
    let source_anchor = TopologyReadAnchorIdentity::from_runtime_row_label(&source_identity);
    let mut reads = handle.topology_reads(&mut workspace);

    let shared_vertex = reads
        .shared_vertex_half_edge_neighborhood(&source_anchor)
        .expect("shared-vertex neighborhood should load");
    let radial = reads
        .radial_half_edge_neighborhood(&source_anchor)
        .expect("radial neighborhood should load");
    let aggregate = reads.aggregate_report();

    assert_eq!(
        shared_vertex
            .request_report
            .lowering_artifact
            .lowering_posture(),
        TopologyReadLoweringPosture::CanonicalTraversalLowered
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
        TopologyReadRelationshipProofPosture::Admitted
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
        3
    );
    assert_eq!(radial.request_report.relationship_proof_admission_count, 3);
    assert_eq!(
        shared_vertex.request_report.execution_engine,
        TopologyReadExecutionEngine::QueryRuntimeCurrent
    );
    assert_eq!(
        radial.request_report.execution_engine,
        TopologyReadExecutionEngine::QueryRuntimeCurrent
    );
    assert_eq!(aggregate.query_execution_count, 2);
    assert_eq!(aggregate.row_scan_fallback_count, 0);
    assert_eq!(aggregate.whole_view_fallback_count, 0);
    assert_eq!(aggregate.lowered_traversal_count, 4);
    assert_eq!(aggregate.relationship_proof_admission_count, 6);
    assert_eq!(aggregate.debt_rows.len(), 0);
    assert!(aggregate.debt_rows.iter().all(|row| row.request_count == 1));
}
