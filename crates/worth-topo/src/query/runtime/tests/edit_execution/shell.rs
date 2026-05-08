use forge_query::facade::{
    ForgeQueryContinuityOutcomeClass, ForgeQueryExistingTruthAssertionMode,
    ForgeQueryGraphCompositionProgramStepKind,
};
use schema::facade::topology_authoring::{
    created_ref, seed_milestone_one_primitive, seed_minimal_topology, MilestoneOnePrimitiveCase,
};
use schema::facade::{CreateKey, EntityKind, TopologyEntityKind};

use crate::edit::{
    ShellOrWireMembershipKind, TopologyEditApplicationMode, TopologyEditBatch,
    TopologyEditContract, TopologyEditFamily,
};
use crate::query::{topology_runtime, TopologyQueryAssembly, TopologyRuntimeAdapters};
use crate::runtime_invariants::build_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_rehome_single_face_to_new_shell_workflow() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded = seed_minimal_topology(&mut runtime, "query-edit-runtime-attach-shell-face")
        .expect("seed topology");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, ".current-head.query-edit-attach-shell-face")
        .expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key = CreateKey::new("query-edit-runtime-attach-shell-face.inner_shell");
    let batch = TopologyEditBatch::new(vec![
        TopologyEditContract::create_topology_entity(shell_key.as_str(), TopologyEntityKind::Shell),
        TopologyEditContract::attach_shell_or_wire_membership(
            "query-edit-runtime-attach-shell-face.region-owns-shell",
            ShellOrWireMembershipKind::RegionOwnsShell,
            seeded.region,
            created_ref(shell_key.as_str()),
        ),
        TopologyEditContract::attach_shell_or_wire_membership(
            "query-edit-runtime-attach-shell-face.shell-owns-face",
            ShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            seeded.face,
        ),
        TopologyEditContract::retire_topology_entity(seeded.shell, TopologyEntityKind::Shell),
    ])
    .expect("non-empty edit batch");

    let execution = assembly
        .apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect(
            "single-face shell rehome should execute through the admitted shell owner-rehome lane",
        );

    assert_eq!(
        execution.families,
        vec![
            TopologyEditFamily::CreateTopologyEntity,
            TopologyEditFamily::AttachShellOrWireMembership,
            TopologyEditFamily::AttachShellOrWireMembership,
            TopologyEditFamily::RetireTopologyEntity,
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
            operation.target_collection() == Some("TopologyRelation")
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
            operation.target_collection() == Some("TopologyEntity")
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
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded = seed_minimal_topology(
        &mut runtime,
        "query-edit-runtime-attach-shell-created-key-mismatch",
    )
    .expect("seed topology");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.query-edit-attach-shell-created-key-mismatch",
    )
    .expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let region_shell_key =
        CreateKey::new("query-edit-runtime-attach-shell-created-key-mismatch.region");
    let face_shell_key =
        CreateKey::new("query-edit-runtime-attach-shell-created-key-mismatch.face");
    let batch = TopologyEditBatch::new(vec![
        TopologyEditContract::create_topology_entity(
            region_shell_key.as_str(),
            TopologyEntityKind::Shell,
        ),
        TopologyEditContract::attach_shell_or_wire_membership(
            "query-edit-runtime-attach-shell-created-key-mismatch.region-owns-shell",
            ShellOrWireMembershipKind::RegionOwnsShell,
            seeded.region,
            created_ref(region_shell_key.as_str()),
        ),
        TopologyEditContract::attach_shell_or_wire_membership(
            "query-edit-runtime-attach-shell-created-key-mismatch.shell-owns-face",
            ShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(face_shell_key.as_str()),
            seeded.face,
        ),
        TopologyEditContract::retire_topology_entity(seeded.shell, TopologyEntityKind::Shell),
    ])
    .expect("non-empty edit batch");

    let error = assembly.apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect_err(
            "single-face shell rehome must fail closed if the two owner updates do not target the same created shell",
        );

    assert!(matches!(
        error,
        crate::edit::TopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![TopologyEditFamily::AttachShellOrWireMembership]
    ));
}

#[test]
fn current_head_runtime_denies_single_face_shell_rehome_when_old_shell_still_owns_other_faces() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-edit-runtime-attach-shell-patch",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let (region, shell, face) = seeded_patch_region_shell_and_face(&runtime, &verified.read_basis);
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, ".current-head.query-edit-attach-shell-patch")
        .expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key = CreateKey::new("current-head.query-edit-attach-shell-patch.inner_shell");
    let batch = TopologyEditBatch::new(vec![
        TopologyEditContract::create_topology_entity(shell_key.as_str(), TopologyEntityKind::Shell),
        TopologyEditContract::attach_shell_or_wire_membership(
            "current-head.query-edit-attach-shell-patch.region-owns-shell",
            ShellOrWireMembershipKind::RegionOwnsShell,
            region,
            created_ref(shell_key.as_str()),
        ),
        TopologyEditContract::attach_shell_or_wire_membership(
            "current-head.query-edit-attach-shell-patch.shell-owns-face",
            ShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            face,
        ),
        TopologyEditContract::retire_topology_entity(shell, TopologyEntityKind::Shell),
    ])
    .expect("non-empty edit batch");

    let error = assembly
        .apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect_err(
            "single-face shell rehome must fail closed if the old shell still owns other faces",
        );

    assert!(matches!(
        error,
        crate::edit::TopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![TopologyEditFamily::AttachShellOrWireMembership]
    ));
}

fn seeded_patch_region_shell_and_face(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    read_basis: &schema::facade::DerivedTopologyReadBasis,
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
        match EntityKind::from_kind_id(record.kind.kind_id) {
            Some(EntityKind::Topology(TopologyEntityKind::Region)) => {
                region = Some(record.entity_id)
            }
            Some(EntityKind::Topology(TopologyEntityKind::Shell)) => shell = Some(record.entity_id),
            Some(EntityKind::Topology(TopologyEntityKind::Face)) if face.is_none() => {
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
