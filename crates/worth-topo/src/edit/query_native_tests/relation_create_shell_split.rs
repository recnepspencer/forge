use std::collections::BTreeSet;

use forge_query::facade::ForgeQueryExistingTruthAssertionMode;
use worth_schema::facade::{
    created_ref, seed_milestone_one_primitive, seed_minimal_topology, DerivedTopologyReadBasis,
    WorthCreateKey, WorthEntityKind, WorthMilestoneOnePrimitiveCase, WorthTopologyEntityKind,
};

use crate::edit::{
    WorthShellOrWireMembershipKind, WorthTopologyEditApplicationMode, WorthTopologyEditBatch,
    WorthTopologyEditContract, WorthTopologyEditFamily, WorthTopologyQueryEditExecutionError,
    WorthTopologyQueryEditRunner,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn query_native_edit_runner_executes_single_face_two_face_shell_split_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.split-shell",
        &WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed");
    let (region, shell, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis);
    let moved_face_id = face_ids[0];
    let retained_face_id = face_ids[1];
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.split-shell").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key = WorthCreateKey::new("worth.query-native-edit.split-shell.new_shell");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            shell_key.as_str(),
            WorthTopologyEntityKind::Shell,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.split-shell.region-owns-shell",
            WorthShellOrWireMembershipKind::RegionOwnsShell,
            region,
            created_ref(shell_key.as_str()),
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.split-shell.shell-owns-face",
            WorthShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            moved_face_id,
        ),
    ])
    .expect("non-empty batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("single-face two-face shell split should execute through the admitted owner-preserving shell lane");

    assert_eq!(
        execution.families,
        vec![
            WorthTopologyEditFamily::CreateTopologyEntity,
            WorthTopologyEditFamily::AttachShellOrWireMembership,
            WorthTopologyEditFamily::AttachShellOrWireMembership,
        ]
    );
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_update_count(),
        1
    );
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_delete_count(),
        0
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
    let retained_shell = execution
        .materialized
        .topology()
        .shells
        .iter()
        .find(|shell_record| shell_record.entity_id == shell)
        .expect("retained shell should remain present");
    assert_eq!(new_shell.face_ids, vec![moved_face_id]);
    assert_eq!(retained_shell.face_ids, vec![retained_face_id]);
    let region_record = execution
        .materialized
        .topology()
        .regions
        .iter()
        .find(|region_record| region_record.entity_id == region)
        .expect("region should remain present");
    assert_eq!(
        region_record
            .shell_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        [shell, new_shell.entity_id]
            .into_iter()
            .collect::<BTreeSet<_>>()
    );
}

#[test]
fn query_native_edit_runner_denies_shell_split_when_old_shell_owns_more_than_two_faces() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.split-shell-large",
        &WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
    )
    .expect("seed");
    let (region, _shell, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis);
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.split-shell-large")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key = WorthCreateKey::new("worth.query-native-edit.split-shell-large.new_shell");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            shell_key.as_str(),
            WorthTopologyEntityKind::Shell,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.split-shell-large.region-owns-shell",
            WorthShellOrWireMembershipKind::RegionOwnsShell,
            region,
            created_ref(shell_key.as_str()),
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.split-shell-large.shell-owns-face",
            WorthShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            face_ids[0],
        ),
    ])
    .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("owner-preserving shell split must stay fail-closed beyond the admitted two-face shell lane");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachShellOrWireMembership]
    ));
}

#[test]
fn query_native_edit_runner_denies_shell_split_when_created_shell_keys_diverge() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.split-shell-diverged-key",
        &WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed");
    let (region, _shell, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis);
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.split-shell-diverged-key")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let region_shell_key =
        WorthCreateKey::new("worth.query-native-edit.split-shell-diverged-key.region");
    let face_shell_key =
        WorthCreateKey::new("worth.query-native-edit.split-shell-diverged-key.face");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            region_shell_key.as_str(),
            WorthTopologyEntityKind::Shell,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.split-shell-diverged-key.region-owns-shell",
            WorthShellOrWireMembershipKind::RegionOwnsShell,
            region,
            created_ref(region_shell_key.as_str()),
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.split-shell-diverged-key.shell-owns-face",
            WorthShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(face_shell_key.as_str()),
            face_ids[0],
        ),
    ])
    .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err(
            "owner-preserving shell split must fail closed when the two owner updates target different created shells",
        );

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachShellOrWireMembership]
    ));
}

#[test]
fn query_native_edit_runner_denies_shell_split_when_region_does_not_own_source_shell() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.split-shell-wrong-region",
        &WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed");
    let (_region, _shell, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis);
    let wrong_region = seed_minimal_topology(
        &mut runtime,
        "worth.query-native-edit.split-shell-wrong-region.other",
    )
    .expect("extra seed")
    .region;
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.split-shell-wrong-region")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key = WorthCreateKey::new("worth.query-native-edit.split-shell-wrong-region.new");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            shell_key.as_str(),
            WorthTopologyEntityKind::Shell,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.split-shell-wrong-region.region-owns-shell",
            WorthShellOrWireMembershipKind::RegionOwnsShell,
            wrong_region,
            created_ref(shell_key.as_str()),
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.split-shell-wrong-region.shell-owns-face",
            WorthShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            face_ids[0],
        ),
    ])
    .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err(
            "owner-preserving shell split must fail closed when the nominated region does not own the source shell",
        );

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachShellOrWireMembership]
    ));
}

fn seeded_patch_region_shell_and_faces(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
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
