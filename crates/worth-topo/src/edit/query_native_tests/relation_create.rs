use worth_schema::facade::{
    created_ref, seed_minimal_topology, WorthCreateKey, WorthTopologyEntityKind,
};

use crate::edit::{
    WorthBoundaryMembershipKind, WorthShellOrWireMembershipKind, WorthTopologyEditApplicationMode,
    WorthTopologyEditBatch, WorthTopologyEditContract, WorthTopologyEditFamily,
    WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner,
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
fn attach_shell_or_wire_membership_contract_preserves_created_member_reference() {
    let shell_key = WorthCreateKey::new("worth.query-native-edit.attach-region-shell.shell");
    let contract = WorthTopologyEditContract::attach_shell_or_wire_membership(
        "worth.query-native-edit.attach-region-shell.region-owns-shell",
        WorthShellOrWireMembershipKind::RegionOwnsShell,
        forge_relational::facade::identity::EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            1,
            1,
        ),
        created_ref(shell_key.as_str()),
    );

    match &contract.lowered_mutations()[0] {
        worth_schema::facade::WorthTopologyMutation::CreateRelation { target, .. } => {
            assert_eq!(target, &created_ref(shell_key.as_str()));
        }
        other => panic!("expected relation create lowering, got {other:?}"),
    }
}

#[test]
fn query_native_edit_runner_denies_create_inner_loop_on_existing_face_workflow_until_invariant_complete_subgraphs_are_admitted(
) {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth.query-native-edit.attach-boundary")
        .expect("seed");
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
            seeded.face,
            created_ref(loop_key.as_str()),
        ),
    ])
    .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("create-inner-loop workflow must fail closed until the real runtime admits an invariant-complete subgraph");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachBoundaryMembership]
    ));
}

#[test]
fn query_native_edit_runner_denies_attach_shell_or_wire_membership_on_production_runtime_until_invariant_complete_subgraphs_are_admitted(
) {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "worth.query-native-edit.attach-shell").expect("seed");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, "worth.query-native-edit.attach-shell")
        .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key = WorthCreateKey::new("worth.query-native-edit.attach-shell.inner_shell");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            shell_key.as_str(),
            WorthTopologyEntityKind::Shell,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.attach-shell.region-owns-shell",
            WorthShellOrWireMembershipKind::RegionOwnsShell,
            seeded.region,
            created_ref(shell_key.as_str()),
        ),
    ])
    .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("attach shell membership must fail closed until invariant-complete topology subgraphs are admitted");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachShellOrWireMembership]
    ));
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

#[test]
fn query_native_edit_runner_denies_attach_shell_or_wire_kind_mismatch_for_created_entity() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(
        &mut runtime,
        "worth.query-native-edit.attach-shell-created-kind",
    )
    .expect("seed");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.query-native-edit.attach-shell-created-kind",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let face_key = WorthCreateKey::new("worth.query-native-edit.attach-shell-created-kind.face");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            face_key.as_str(),
            WorthTopologyEntityKind::Face,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.attach-shell-created-kind.region-owns-shell",
            WorthShellOrWireMembershipKind::RegionOwnsShell,
            seeded.region,
            created_ref(face_key.as_str()),
        ),
    ])
    .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("attach shell membership remains fail-closed at the public runner boundary");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachShellOrWireMembership]
    ));
}
