use forge_relational::facade::identity::RelationId;
use worth_schema::facade::{
    created_ref, seed_milestone_one_primitive, seed_minimal_topology, WorthCreateKey,
    WorthEntityKind, WorthMilestoneOnePrimitiveCase, WorthRelationKind, WorthTopologyEntityKind,
    WorthTopologyRelationKind,
};

use crate::edit::{
    WorthBoundaryMembershipKind, WorthTopologyEditApplicationMode, WorthTopologyEditBatch,
    WorthTopologyEditContract, WorthTopologyEditFamily, WorthTopologyQueryEditRunner,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;
use forge_query::facade::ForgeQueryExistingTruthAssertionMode;

#[test]
fn current_head_runtime_executes_create_topology_entity_through_query_native_edit_runner() {
    let runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::create_topology_entity(
            "worth-query-edit-runtime.added_vertex",
            WorthTopologyEntityKind::Vertex,
        )])
        .expect("non-empty edit batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("create topology entity should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::CreateTopologyEntity]
    );
    assert_eq!(execution.naming_report.rows.len(), 1);
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&assembly.materialized().name().to_string()));
    assert_eq!(
        execution.inspection.affected_derived_view_ids(),
        execution.receipt.affected_derived_view_ids()
    );
    assert!(execution
        .materialized
        .topology()
        .vertices
        .iter()
        .any(|vertex| vertex.label == "worth-query-edit-runtime.added_vertex"));
}

#[test]
fn current_head_runtime_executes_retire_topology_entity_through_query_native_edit_runner() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth-query-edit-runtime-retire")
        .expect("seed topology");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, "worth.current-head.query-edit-retire")
        .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::retire_topology_entity(
            seeded.vertex,
            WorthTopologyEntityKind::Vertex,
        )])
        .expect("non-empty edit batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("retire topology entity should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::RetireTopologyEntity]
    );
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&assembly.materialized().name().to_string()));
    assert_eq!(
        execution.inspection.affected_derived_view_ids(),
        execution.receipt.affected_derived_view_ids()
    );
    assert_eq!(
        execution.inspection.component_operations()[0]
            .existing_truth_assertion_evidence()
            .expect("retire receipt should retain backend verification evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_delete_count(),
        1
    );
    assert!(!execution
        .materialized
        .topology()
        .vertices
        .iter()
        .any(|vertex| vertex.entity_id == seeded.vertex));
}

#[test]
fn current_head_runtime_executes_detach_boundary_membership_through_query_native_edit_runner() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth-query-edit-runtime-detach")
        .expect("seed topology");
    let loop_owns_half_edge_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        WorthTopologyRelationKind::LoopOwnsHalfEdge,
    );
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, "worth.current-head.query-edit-detach")
        .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::detach_boundary_membership(
            loop_owns_half_edge_relation,
            WorthBoundaryMembershipKind::LoopOwnsHalfEdge,
        )])
        .expect("non-empty edit batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("detach boundary membership should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::DetachBoundaryMembership]
    );
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&assembly.materialized().name().to_string()));
    assert_eq!(
        execution.inspection.affected_derived_view_ids(),
        execution.receipt.affected_derived_view_ids()
    );
    assert_eq!(
        execution.inspection.component_operations()[0]
            .existing_truth_assertion_evidence()
            .expect("detach receipt should retain backend verification evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_delete_count(),
        1
    );
    let loop_record = execution
        .materialized
        .topology()
        .loops
        .iter()
        .find(|loop_record| loop_record.entity_id == seeded.outer_loop)
        .expect("seeded outer loop should remain present");
    assert!(loop_record.half_edge_ids.is_empty());
}

#[test]
fn current_head_runtime_executes_create_inner_loop_on_existing_face_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth-query-edit-runtime-attach-boundary",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("seed topology");
    let face_id = runtime
        .read_truth()
        .read_snapshot(verified.read_basis.snapshot())
        .expect("seeded snapshot should remain readable")
        .entities()
        .iter()
        .find(|record| {
            record.kind.kind_id
                == WorthEntityKind::Topology(WorthTopologyEntityKind::Face).kind_id()
        })
        .map(|record| record.entity_id)
        .expect("seeded primitive should contain a face");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit-attach-boundary")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let loop_key = WorthCreateKey::new("worth-query-edit-runtime-attach-boundary.inner_loop");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            loop_key.as_str(),
            WorthTopologyEntityKind::Loop,
        ),
        WorthTopologyEditContract::attach_boundary_membership(
            "worth-query-edit-runtime-attach-boundary.face-inner-loop",
            WorthBoundaryMembershipKind::FaceInnerLoop,
            face_id,
            created_ref(loop_key.as_str()),
        ),
    ])
    .expect("non-empty edit batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("create-inner-loop workflow should execute through the admitted invariant-complete runtime lane");

    assert_eq!(
        execution.families,
        vec![
            WorthTopologyEditFamily::CreateTopologyEntity,
            WorthTopologyEditFamily::AttachBoundaryMembership,
        ]
    );
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&assembly.materialized().name().to_string()));
    let face = execution
        .materialized
        .topology()
        .faces
        .iter()
        .find(|face| face.entity_id == face_id)
        .expect("seeded face should remain present");
    assert!(
        !face.inner_loop_ids.is_empty(),
        "face should gain an inner loop after admitted workflow"
    );
}

#[test]
fn current_head_runtime_executes_detach_radial_adjacency_through_query_native_edit_runner() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth-query-edit-runtime-detach-radial")
        .expect("seed topology");
    let radial_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        WorthTopologyRelationKind::HalfEdgeRadialNext,
    );
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit-detach-radial")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::detach_radial_adjacency(
            radial_relation,
        )])
        .expect("non-empty edit batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("detach radial adjacency should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::DetachRadialAdjacency]
    );
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&assembly.materialized().name().to_string()));
    assert_eq!(
        execution.inspection.affected_derived_view_ids(),
        execution.receipt.affected_derived_view_ids()
    );
    assert_eq!(
        execution.inspection.component_operations()[0]
            .existing_truth_assertion_evidence()
            .expect("detach receipt should retain backend verification evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_delete_count(),
        1
    );
    let half_edge = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == seeded.half_edge)
        .expect("seeded half-edge should remain present");
    assert_eq!(half_edge.radial_next_half_edge_id, None);
}

fn seeded_relation_id(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    snapshot: &forge_relational::facade::snapshots::SnapshotHandle,
    kind: WorthTopologyRelationKind,
) -> RelationId {
    runtime
        .read_truth()
        .read_snapshot(snapshot)
        .expect("seeded snapshot should remain readable")
        .relations()
        .iter()
        .find(|record| record.kind.kind_id == WorthRelationKind::Topology(kind).kind_id())
        .map(|record| record.relation_id)
        .expect("seeded topology should contain requested relation kind")
}
