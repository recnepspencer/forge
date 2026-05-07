use forge_query::facade::{
    ForgeQueryContinuityOutcomeClass, ForgeQueryExistingTruthAssertionMode,
    ForgeQueryGraphCompositionProgramStepKind,
};
use worth_schema::facade::topology_authoring::{
    created_ref, seed_milestone_one_primitive, seed_minimal_topology,
    WorthMilestoneOnePrimitiveCase,
};
use worth_schema::facade::{WorthCreateKey, WorthEntityKind, WorthTopologyEntityKind};

use crate::edit::{
    WorthShellOrWireMembershipKind, WorthTopologyEditApplicationMode, WorthTopologyEditBatch,
    WorthTopologyEditContract, WorthTopologyEditFamily,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_rehome_single_face_to_new_shell_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth-query-edit-runtime-attach-shell-face")
        .expect("seed topology");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit-attach-shell-face")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key = WorthCreateKey::new("worth-query-edit-runtime-attach-shell-face.inner_shell");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            shell_key.as_str(),
            WorthTopologyEntityKind::Shell,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth-query-edit-runtime-attach-shell-face.region-owns-shell",
            WorthShellOrWireMembershipKind::RegionOwnsShell,
            seeded.region,
            created_ref(shell_key.as_str()),
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth-query-edit-runtime-attach-shell-face.shell-owns-face",
            WorthShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            seeded.face,
        ),
        WorthTopologyEditContract::retire_topology_entity(
            seeded.shell,
            WorthTopologyEntityKind::Shell,
        ),
    ])
    .expect("non-empty edit batch");

    let execution = assembly
        .apply_edit(
            &mut workspace,
            batch,
            WorthTopologyEditApplicationMode::Mainline,
        )
        .expect(
            "single-face shell rehome should execute through the admitted shell owner-rehome lane",
        );

    assert_eq!(
        execution.families,
        vec![
            WorthTopologyEditFamily::CreateTopologyEntity,
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
        2
    );
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_delete_count(),
        1
    );
    assert_eq!(
        execution
            .receipt
            .graph_composition_program()
            .expect("shell rehome should expose graph program")
            .steps()
            .iter()
            .map(|step| step.kind())
            .collect::<Vec<_>>(),
        vec![
            ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetirement,
        ]
    );
    assert_eq!(
        execution
            .receipt
            .graph_composition_lineage_summary()
            .expect("shell rehome should expose lineage summary")
            .entries()
            .iter()
            .filter(|entry| {
                entry.outcome_class()
                    == ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
            })
            .count(),
        2
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
    assert!(execution
        .inspection
        .component_operations()
        .iter()
        .filter(|operation| operation.family() == "delete")
        .all(|operation| {
            operation.target_collection() == Some("WorthTopologyEntity")
                && operation
                    .existing_truth_assertion_evidence()
                    .is_some_and(|evidence| {
                        evidence.mode()
                            == ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
                    })
        }));
    assert!(!execution
        .materialized
        .topology()
        .shells
        .iter()
        .any(|shell| shell.entity_id == seeded.shell));
    let new_shell = execution
        .materialized
        .topology()
        .shells
        .iter()
        .find(|shell| shell.label == shell_key.as_str())
        .expect("new shell should remain present after admitted rehome");
    assert_eq!(new_shell.face_ids, vec![seeded.face]);
}

#[test]
fn current_head_runtime_denies_single_face_shell_rehome_when_created_shell_keys_diverge() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(
        &mut runtime,
        "worth-query-edit-runtime-attach-shell-created-key-mismatch",
    )
    .expect("seed topology");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.current-head.query-edit-attach-shell-created-key-mismatch",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let region_shell_key =
        WorthCreateKey::new("worth-query-edit-runtime-attach-shell-created-key-mismatch.region");
    let face_shell_key =
        WorthCreateKey::new("worth-query-edit-runtime-attach-shell-created-key-mismatch.face");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            region_shell_key.as_str(),
            WorthTopologyEntityKind::Shell,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth-query-edit-runtime-attach-shell-created-key-mismatch.region-owns-shell",
            WorthShellOrWireMembershipKind::RegionOwnsShell,
            seeded.region,
            created_ref(region_shell_key.as_str()),
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth-query-edit-runtime-attach-shell-created-key-mismatch.shell-owns-face",
            WorthShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(face_shell_key.as_str()),
            seeded.face,
        ),
        WorthTopologyEditContract::retire_topology_entity(
            seeded.shell,
            WorthTopologyEntityKind::Shell,
        ),
    ])
    .expect("non-empty edit batch");

    let error = assembly.apply_edit(&mut workspace, batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err(
            "single-face shell rehome must fail closed if the two owner updates do not target the same created shell",
        );

    assert!(matches!(
        error,
        crate::edit::WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachShellOrWireMembership]
    ));
}

#[test]
fn current_head_runtime_denies_single_face_shell_rehome_when_old_shell_still_owns_other_faces() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth-query-edit-runtime-attach-shell-patch",
        &WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let (region, shell, face) = seeded_patch_region_shell_and_face(&runtime, &verified.read_basis);
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit-attach-shell-patch")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key =
        WorthCreateKey::new("worth-current-head.query-edit-attach-shell-patch.inner_shell");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            shell_key.as_str(),
            WorthTopologyEntityKind::Shell,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth-current-head.query-edit-attach-shell-patch.region-owns-shell",
            WorthShellOrWireMembershipKind::RegionOwnsShell,
            region,
            created_ref(shell_key.as_str()),
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth-current-head.query-edit-attach-shell-patch.shell-owns-face",
            WorthShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            face,
        ),
        WorthTopologyEditContract::retire_topology_entity(shell, WorthTopologyEntityKind::Shell),
    ])
    .expect("non-empty edit batch");

    let error = assembly
        .apply_edit(
            &mut workspace,
            batch,
            WorthTopologyEditApplicationMode::Mainline,
        )
        .expect_err(
            "single-face shell rehome must fail closed if the old shell still owns other faces",
        );

    assert!(matches!(
        error,
        crate::edit::WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachShellOrWireMembership]
    ));
}

fn seeded_patch_region_shell_and_face(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    read_basis: &worth_schema::facade::DerivedTopologyReadBasis,
) -> (
    forge_relational::facade::identity::EntityId,
    forge_relational::facade::identity::EntityId,
    forge_relational::facade::identity::EntityId,
) {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .expect("seeded snapshot should remain readable");
    let mut region = None;
    let mut shell = None;
    let mut face = None;
    for record in read_view.entities() {
        match WorthEntityKind::from_kind_id(record.kind.kind_id) {
            Some(WorthEntityKind::Topology(WorthTopologyEntityKind::Region)) => {
                region = Some(record.entity_id)
            }
            Some(WorthEntityKind::Topology(WorthTopologyEntityKind::Shell)) => {
                shell = Some(record.entity_id)
            }
            Some(WorthEntityKind::Topology(WorthTopologyEntityKind::Face)) if face.is_none() => {
                face = Some(record.entity_id)
            }
            _ => {}
        }
    }
    (
        region.expect("seeded patch should contain one region"),
        shell.expect("seeded patch should contain one shell"),
        face.expect("seeded patch should contain at least one face"),
    )
}
