use std::collections::BTreeSet;

use forge_query::facade::{
    ForgeQueryContinuityOutcomeClass, ForgeQueryExistingTruthAssertionMode,
    ForgeQueryGraphCompositionProgramStepKind,
};
use schema::facade::topology_authoring::{
    created_ref, seed_milestone_one_primitive, MilestoneOnePrimitiveCase,
};
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};

use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    ShellOrWireMembershipKind, TopologyEditApplicationMode, TopologyEditBatch,
    TopologyEditContract, TopologyEditFamily,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_rehome_all_owned_faces_to_new_shell_program() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-edit-attach-shell-face-set",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let (region, shell, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis);
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-edit-attach-shell-face-set")
            .expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key = CreateKey::new(".current-head.query-edit-attach-shell-face-set.new_shell");
    let batch = TopologyEditBatch::new(vec![
        TopologyEditContract::create_topology_entity(shell_key.as_str(), TopologyEntityKind::Shell),
        TopologyEditContract::attach_shell_or_wire_membership(
            ".current-head.query-edit-attach-shell-face-set.region-owns-shell",
            ShellOrWireMembershipKind::RegionOwnsShell,
            region,
            created_ref(shell_key.as_str()),
        ),
        TopologyEditContract::attach_shell_or_wire_membership(
            ".current-head.query-edit-attach-shell-face-set.shell-owns-face-1",
            ShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            face_ids[0],
        ),
        TopologyEditContract::attach_shell_or_wire_membership(
            ".current-head.query-edit-attach-shell-face-set.shell-owns-face-2",
            ShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            face_ids[1],
        ),
        TopologyEditContract::retire_topology_entity(shell, TopologyEntityKind::Shell),
    ])
    .expect("non-empty edit batch");

    let execution = assembly.apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect("full face-set shell rehome should execute through the admitted shell owner-rehome lane");

    assert_eq!(
        execution.families,
        vec![
            TopologyEditFamily::CreateTopologyEntity,
            TopologyEditFamily::AttachShellOrWireMembership,
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
        3
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
            .expect("shell face-set rehome should expose composed program")
            .steps()
            .iter()
            .map(|step| step.kind())
            .collect::<Vec<_>>(),
        vec![
            ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetirement,
        ]
    );
    assert_eq!(
        execution
            .receipt
            .graph_composition_lineage_summary()
            .expect("shell face-set rehome should expose lineage summary")
            .entries()
            .iter()
            .filter(|entry| {
                entry.outcome_class()
                    == ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
            })
            .count(),
        3
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
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-edit-attach-shell-face-set-duplicate",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let (region, shell, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis);
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.query-edit-attach-shell-face-set-duplicate",
    )
    .expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key =
        CreateKey::new(".current-head.query-edit-attach-shell-face-set-duplicate.new_shell");
    let batch = TopologyEditBatch::new(vec![
        TopologyEditContract::create_topology_entity(shell_key.as_str(), TopologyEntityKind::Shell),
        TopologyEditContract::attach_shell_or_wire_membership(
            ".current-head.query-edit-attach-shell-face-set-duplicate.region-owns-shell",
            ShellOrWireMembershipKind::RegionOwnsShell,
            region,
            created_ref(shell_key.as_str()),
        ),
        TopologyEditContract::attach_shell_or_wire_membership(
            ".current-head.query-edit-attach-shell-face-set-duplicate.shell-owns-face-1",
            ShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            face_ids[0],
        ),
        TopologyEditContract::attach_shell_or_wire_membership(
            ".current-head.query-edit-attach-shell-face-set-duplicate.shell-owns-face-2",
            ShellOrWireMembershipKind::ShellOwnsFace,
            created_ref(shell_key.as_str()),
            face_ids[0],
        ),
        TopologyEditContract::retire_topology_entity(shell, TopologyEntityKind::Shell),
    ])
    .expect("non-empty edit batch");

    let error = assembly.apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect_err("shell face-set rehome must fail closed if face attachments do not form a unique full set");

    assert!(matches!(
        error,
        crate::topology_operators::TopologyOperatorExecutionError::UnsupportedFamilies(families)
            if families == vec![TopologyEditFamily::AttachShellOrWireMembership]
    ));
}

fn seeded_patch_region_shell_and_faces(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    read_basis: &schema::facade::platform::authority::DerivedTopologyReadBasis,
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
        match EntityKind::from_kind_id(record.kind.kind_id) {
            Some(EntityKind::Topology(TopologyEntityKind::Region)) => {
                region = Some(record.entity_id)
            }
            Some(EntityKind::Topology(TopologyEntityKind::Shell)) => shell = Some(record.entity_id),
            Some(EntityKind::Topology(TopologyEntityKind::Face)) => face_ids.push(record.entity_id),
            _ => {}
        }
    }
    (
        region.expect("seeded patch should contain one region"),
        shell.expect("seeded patch should contain one shell"),
        face_ids,
    )
}




