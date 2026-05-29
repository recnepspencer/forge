use forge_relational::facade::identity::RelationId;
use schema::facade::topology_authoring::{
    created_ref, seed_milestone_one_primitive, seed_minimal_topology, MilestoneOnePrimitiveCase,
};
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};

use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    BoundaryMembershipKind, TopologyEditApplicationMode, TopologyEditBatch, TopologyEditContract,
    TopologyEditFamily, TopologyEditNamingOutcome,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::{
    ForgeQueryExistingTruthAssertionMode, ForgeQueryGraphCompositionProgramStepKind,
};

#[test]
fn current_head_runtime_executes_create_topology_entity_through_topology_operator_runner() {
    let runtime = build_milestone_one_runtime().expect(" runtime");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, ".current-head.query-edit").expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch = TopologyEditBatch::new(vec![TopologyEditContract::create_topology_entity(
        "query-edit-runtime.added_vertex",
        TopologyEntityKind::Vertex,
    )])
    .expect("non-empty edit batch");

    let execution = assembly
        .apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect("create topology entity should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![TopologyEditFamily::CreateTopologyEntity]
    );
    assert_eq!(execution.topology_edit_digest.contract_count, 1);
    assert_eq!(execution.topology_edit_digest.family_count, 1);
    assert_eq!(execution.naming_continuity_matrix.rows.len(), 1);
    assert_eq!(execution.naming_continuity_matrix.preserved_count, 1);
    assert_eq!(execution.naming_continuity_matrix.ambiguous_count, 0);
    assert_eq!(execution.naming_continuity_matrix.rejected_count, 0);
    assert_eq!(
        execution.naming_continuity_matrix.rows[0].outcome,
        TopologyEditNamingOutcome::Preserved
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
        .any(|vertex| vertex.label == "query-edit-runtime.added_vertex"));
}

#[test]
fn current_head_runtime_executes_retire_topology_entity_through_topology_operator_runner() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "query-edit-runtime-retire").expect("seed topology");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-edit-retire").expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch = TopologyEditBatch::new(vec![TopologyEditContract::retire_topology_entity(
        seeded.vertex,
        TopologyEntityKind::Vertex,
    )])
    .expect("non-empty edit batch");

    let execution = assembly
        .apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect("retire topology entity should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![TopologyEditFamily::RetireTopologyEntity]
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
fn current_head_runtime_executes_detach_boundary_membership_through_topology_operator_runner() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "query-edit-runtime-detach").expect("seed topology");
    let loop_owns_half_edge_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        TopologyRelationKind::LoopOwnsHalfEdge,
    );
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-edit-detach").expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch = TopologyEditBatch::new(vec![TopologyEditContract::detach_boundary_membership(
        loop_owns_half_edge_relation,
        BoundaryMembershipKind::LoopOwnsHalfEdge,
    )])
    .expect("non-empty edit batch");

    let execution = assembly
        .apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect("detach boundary membership should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![TopologyEditFamily::DetachBoundaryMembership]
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
fn current_head_runtime_executes_create_inner_loop_on_existing_face_program() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-edit-runtime-attach-boundary",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("seed topology");
    let face_id = runtime
        .read_truth()
        .read_snapshot(verified.read_basis().snapshot())
        .expect("seeded snapshot should remain readable")
        .entities()
        .iter()
        .find(|record| {
            record.kind.kind_id == EntityKind::Topology(TopologyEntityKind::Face).kind_id()
        })
        .map(|record| record.entity_id)
        .expect("seeded primitive should contain a face");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-edit-attach-boundary").expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let loop_key = CreateKey::new("query-edit-runtime-attach-boundary.inner_loop");
    let batch = TopologyEditBatch::new(vec![
        TopologyEditContract::create_topology_entity(loop_key.as_str(), TopologyEntityKind::Loop),
        TopologyEditContract::attach_boundary_membership(
            "query-edit-runtime-attach-boundary.face-inner-loop",
            BoundaryMembershipKind::FaceInnerLoop,
            face_id,
            created_ref(loop_key.as_str()),
        ),
    ])
    .expect("non-empty edit batch");

    let execution = assembly.apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect("create-inner-loop program should execute through the admitted invariant-complete runtime lane");

    assert_eq!(
        execution.families,
        vec![
            TopologyEditFamily::CreateTopologyEntity,
            TopologyEditFamily::AttachBoundaryMembership,
        ]
    );
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&assembly.materialized().name().to_string()));
    assert_eq!(
        execution
            .receipt
            .graph_composition_program()
            .expect("inner-loop program should expose composed program")
            .steps()
            .iter()
            .map(|step| step.kind())
            .collect::<Vec<_>>(),
        vec![
            ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration,
            ForgeQueryGraphCompositionProgramStepKind::RelationDeclaration,
        ]
    );
    let face = execution
        .materialized
        .topology()
        .faces
        .iter()
        .find(|face| face.entity_id == face_id)
        .expect("seeded face should remain present");
    assert!(
        !face.inner_loop_ids.is_empty(),
        "face should gain an inner loop after admitted program"
    );
}

#[test]
fn current_head_runtime_executes_detach_radial_adjacency_through_topology_operator_runner() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded = seed_minimal_topology(&mut runtime, "query-edit-runtime-detach-radial")
        .expect("seed topology");
    let radial_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        TopologyRelationKind::HalfEdgeRadialNext,
    );
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-edit-detach-radial").expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch = TopologyEditBatch::new(vec![TopologyEditContract::detach_radial_adjacency(
        radial_relation,
    )])
    .expect("non-empty edit batch");

    let execution = assembly
        .apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect("detach radial adjacency should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![TopologyEditFamily::DetachRadialAdjacency]
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
    kind: TopologyRelationKind,
) -> RelationId {
    runtime
        .read_truth()
        .read_snapshot(snapshot)
        .expect("seeded snapshot should remain readable")
        .relations()
        .iter()
        .find(|record| record.kind.kind_id == RelationKind::Topology(kind).kind_id())
        .map(|record| record.relation_id)
        .expect("seeded topology should contain requested relation kind")
}




