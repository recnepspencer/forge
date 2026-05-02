use worth_schema::facade::{
    created_ref, seed_milestone_one_primitive, seed_minimal_topology, WorthCreateKey,
    WorthMilestoneOnePrimitiveCase, WorthTopologyEntityKind,
};

use crate::edit::{
    WorthBoundaryMembershipKind, WorthTopologyEditApplicationMode, WorthTopologyEditBatch,
    WorthTopologyEditContract, WorthTopologyEditFamily, WorthTopologyQueryEditExecutionError,
    WorthTopologyQueryEditRunner,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn attach_boundary_membership_contract_preserves_created_member_reference() {
    let loop_key = WorthCreateKey::new("worth.query-native-edit.attach-inner-loop.inner_loop");
    let contract = WorthTopologyEditContract::attach_boundary_membership(
        "worth.query-native-edit.attach-inner-loop.face-inner-loop",
        WorthBoundaryMembershipKind::FaceInnerLoop,
        forge_relational::facade::identity::EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            1,
            1,
        ),
        created_ref(loop_key.as_str()),
    );

    match &contract.lowered_mutations()[0] {
        worth_schema::facade::WorthTopologyMutation::CreateRelation { target, .. } => {
            assert_eq!(target, &created_ref(loop_key.as_str()));
        }
        other => panic!("expected relation create lowering, got {other:?}"),
    }
}

#[test]
fn query_native_edit_runner_executes_create_inner_loop_on_existing_face_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.attach-boundary",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("seed");
    let face = runtime
        .read_truth()
        .read_snapshot(verified.read_basis.snapshot())
        .expect("seeded snapshot should remain readable")
        .entities()
        .iter()
        .find(|record| {
            record.kind.kind_id
                == worth_schema::facade::WorthEntityKind::Topology(WorthTopologyEntityKind::Face)
                    .kind_id()
        })
        .map(|record| record.entity_id)
        .expect("seeded primitive should contain a face");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, "worth.query-native-edit.attach-boundary")
        .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let loop_key = WorthCreateKey::new("worth.query-native-edit.attach-boundary.inner_loop");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            loop_key.as_str(),
            WorthTopologyEntityKind::Loop,
        ),
        WorthTopologyEditContract::attach_boundary_membership(
            "worth.query-native-edit.attach-boundary.face-inner-loop",
            WorthBoundaryMembershipKind::FaceInnerLoop,
            face,
            created_ref(loop_key.as_str()),
        ),
    ])
    .expect("non-empty batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("create-inner-loop workflow should execute once the runtime admits this invariant-complete subgraph");

    assert_eq!(
        execution.families,
        vec![
            WorthTopologyEditFamily::CreateTopologyEntity,
            WorthTopologyEditFamily::AttachBoundaryMembership,
        ]
    );
    assert!(execution
        .materialized
        .topology()
        .loops
        .iter()
        .any(|loop_record| loop_record.label == loop_key.as_str()));
    let face = execution
        .materialized
        .topology()
        .faces
        .iter()
        .find(|face_record| face_record.entity_id == face)
        .expect("seeded face should remain present");
    assert!(
        !face.inner_loop_ids.is_empty(),
        "face should retain an inner loop after admitted workflow"
    );
}

#[test]
fn query_native_edit_runner_denies_create_inner_loop_workflow_when_relation_precedes_symbolic_create(
) {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(
        &mut runtime,
        "worth.query-native-edit.attach-boundary-reversed-order",
    )
    .expect("seed");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.query-native-edit.attach-boundary-reversed-order",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let loop_key = WorthCreateKey::new("worth.query-native-edit.attach-boundary-reversed.inner");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::attach_boundary_membership(
            "worth.query-native-edit.attach-boundary-reversed.face-inner-loop",
            WorthBoundaryMembershipKind::FaceInnerLoop,
            seeded.face,
            created_ref(loop_key.as_str()),
        ),
        WorthTopologyEditContract::create_topology_entity(
            loop_key.as_str(),
            WorthTopologyEntityKind::Loop,
        ),
    ])
    .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("reversed-order create-inner-loop batch must fail closed");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachBoundaryMembership]
    ));
}

#[test]
fn query_native_edit_runner_denies_attach_boundary_membership_missing_created_entity() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(
        &mut runtime,
        "worth.query-native-edit.attach-boundary-missing-created",
    )
    .expect("seed");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.query-native-edit.attach-boundary-missing-created",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::attach_boundary_membership(
            "worth.query-native-edit.attach-boundary-missing-created.face-inner-loop",
            WorthBoundaryMembershipKind::FaceInnerLoop,
            seeded.face,
            created_ref("worth.query-native-edit.attach-boundary-missing-created.missing-loop"),
        )])
        .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("attach boundary membership remains fail-closed at the public runner boundary");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachBoundaryMembership]
    ));
}
