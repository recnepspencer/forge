use std::collections::BTreeSet;

use forge_query::facade::ForgeQueryExistingTruthAssertionMode;
use worth_schema::facade::{
    created_ref, seed_milestone_one_primitive, WorthCreateKey, WorthEntityKind,
    WorthMilestoneOnePrimitiveCase, WorthTopologyEntityKind,
};

use crate::edit::{
    WorthShellOrWireMembershipKind, WorthTopologyEditApplicationMode, WorthTopologyEditBatch,
    WorthTopologyEditContract, WorthTopologyEditFamily, WorthTopologyQueryEditRunner,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_rehome_all_owned_faces_to_new_shell_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-attach-shell-face-set",
        &WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let (region, shell, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis);
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.current-head.query-edit-attach-shell-face-set",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key =
        WorthCreateKey::new("worth.current-head.query-edit-attach-shell-face-set.new_shell");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            shell_key.as_str(),
            WorthTopologyEntityKind::Shell,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.current-head.query-edit-attach-shell-face-set.region-owns-shell",
            WorthShellOrWireMembershipKind::RegionOwnsShell,
            region,
            created_ref(shell_key.as_str()),
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.current-head.query-edit-attach-shell-face-set.shell-owns-face-1",
            WorthShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            face_ids[0],
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.current-head.query-edit-attach-shell-face-set.shell-owns-face-2",
            WorthShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            face_ids[1],
        ),
        WorthTopologyEditContract::retire_topology_entity(shell, WorthTopologyEntityKind::Shell),
    ])
    .expect("non-empty edit batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("full face-set shell rehome should execute through the admitted shell owner-rehome lane");

    assert_eq!(
        execution.families,
        vec![
            WorthTopologyEditFamily::CreateTopologyEntity,
            WorthTopologyEditFamily::AttachShellOrWireMembership,
            WorthTopologyEditFamily::AttachShellOrWireMembership,
            WorthTopologyEditFamily::AttachShellOrWireMembership,
            WorthTopologyEditFamily::RetireTopologyEntity,
        ]
    );
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_update_count(),
        3
    );
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_delete_count(),
        1
    );
    assert!(execution
        .inspection
        .component_operations()
        .iter()
        .filter(|operation| operation.family() == "update")
        .all(|operation| {
            operation.target_collection() == Some("WorthTopologyRelation")
                && operation
                    .existing_truth_assertion_evidence()
                    .is_some_and(|evidence| {
                        evidence.mode()
                            == ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
                    })
        }));
    let new_shell = execution
        .materialized
        .topology()
        .shells
        .iter()
        .find(|shell_record| shell_record.label == shell_key.as_str())
        .expect("new shell should remain present");
    assert_eq!(
        new_shell.face_ids.iter().copied().collect::<BTreeSet<_>>(),
        face_ids.iter().copied().collect::<BTreeSet<_>>()
    );
    assert!(!execution
        .materialized
        .topology()
        .shells
        .iter()
        .any(|shell_record| shell_record.entity_id == shell));
}

#[test]
fn current_head_runtime_denies_shell_face_set_rehome_with_duplicate_face_membership() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-attach-shell-face-set-duplicate",
        &WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let (region, shell, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis);
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.current-head.query-edit-attach-shell-face-set-duplicate",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key = WorthCreateKey::new(
        "worth.current-head.query-edit-attach-shell-face-set-duplicate.new_shell",
    );
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            shell_key.as_str(),
            WorthTopologyEntityKind::Shell,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.current-head.query-edit-attach-shell-face-set-duplicate.region-owns-shell",
            WorthShellOrWireMembershipKind::RegionOwnsShell,
            region,
            created_ref(shell_key.as_str()),
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.current-head.query-edit-attach-shell-face-set-duplicate.shell-owns-face-1",
            WorthShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            face_ids[0],
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.current-head.query-edit-attach-shell-face-set-duplicate.shell-owns-face-2",
            WorthShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            face_ids[0],
        ),
        WorthTopologyEditContract::retire_topology_entity(shell, WorthTopologyEntityKind::Shell),
    ])
    .expect("non-empty edit batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("shell face-set rehome must fail closed if face attachments do not form a unique full set");

    assert!(matches!(
        error,
        crate::edit::WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachShellOrWireMembership]
    ));
}

fn seeded_patch_region_shell_and_faces(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    read_basis: &worth_schema::facade::DerivedTopologyReadBasis,
) -> (
    forge_relational::facade::identity::EntityId,
    forge_relational::facade::identity::EntityId,
    Vec<forge_relational::facade::identity::EntityId>,
) {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .expect("seeded snapshot should remain readable");
    let mut region = None;
    let mut shell = None;
    let mut face_ids = Vec::new();
    for record in read_view.entities() {
        match WorthEntityKind::from_kind_id(record.kind.kind_id) {
            Some(WorthEntityKind::Topology(WorthTopologyEntityKind::Region)) => {
                region = Some(record.entity_id)
            }
            Some(WorthEntityKind::Topology(WorthTopologyEntityKind::Shell)) => {
                shell = Some(record.entity_id)
            }
            Some(WorthEntityKind::Topology(WorthTopologyEntityKind::Face)) => {
                face_ids.push(record.entity_id)
            }
            _ => {}
        }
    }
    (
        region.expect("seeded patch should contain one region"),
        shell.expect("seeded patch should contain one shell"),
        face_ids,
    )
}
