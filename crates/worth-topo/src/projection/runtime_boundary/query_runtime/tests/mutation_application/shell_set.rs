use std::collections::BTreeSet;

use forge_query::facade::{
    ForgeQueryContinuityOutcomeClass, ForgeQueryExistingTruthAssertionMode,
    ForgeQueryGraphCompositionProgramStepKind,
};
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
<<<<<<< HEAD:crates/worth-topo/src/projection/runtime_boundary/query_runtime/tests/edit_execution/shell_set.rs
use schema::facade::topology_authoring::{
    created_ref, seed_milestone_one_primitive, MilestoneOnePrimitiveCase,
};
=======
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
>>>>>>> origin/master:crates/worth-topo/src/projection/runtime_boundary/query_runtime/tests/mutation_application/shell_set.rs

use crate::certification::support::declaration_runtime::{
    current_head_unsupported_declaration_families, execute_current_head_topology_declaration,
};
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    TopologyMutationFamily, TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    TopologyShellRehomeFaceMember,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_rehome_all_owned_faces_to_new_shell_program() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-mutation-attach-shell-face-set",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let (region, shell, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.query-mutation-attach-shell-face-set",
    )
    .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let shell_key = CreateKey::new(".current-head.query-mutation-attach-shell-face-set.new_shell");
    let declaration = TopologyRehomeAllOwnedFacesToNewShellDeclaration::new(
        shell_key.as_str(),
        ".current-head.query-mutation-attach-shell-face-set.region-owns-shell",
        region,
        shell,
        vec![
            TopologyShellRehomeFaceMember::new(
                ".current-head.query-mutation-attach-shell-face-set.shell-owns-face-1",
                face_ids[0],
            ),
            TopologyShellRehomeFaceMember::new(
                ".current-head.query-mutation-attach-shell-face-set.shell-owns-face-2",
                face_ids[1],
            ),
        ],
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("full face-set shell rehome should execute through declaration entry");

    assert_eq!(
        execution.families,
        vec![
            TopologyMutationFamily::CreateTopologyEntity,
            TopologyMutationFamily::AttachShellOrWireMembership,
            TopologyMutationFamily::AttachShellOrWireMembership,
            TopologyMutationFamily::AttachShellOrWireMembership,
            TopologyMutationFamily::RetireTopologyEntity,
        ]
    );
    assert_eq!(
        execution
            .mutation_evidence()
            .backend_verified_update_count(),
        3
    );
    assert_eq!(
        execution
            .mutation_evidence()
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
        ".current-head.query-mutation-attach-shell-face-set-duplicate",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let (region, shell, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.query-mutation-attach-shell-face-set-duplicate",
    )
    .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let shell_key =
        CreateKey::new(".current-head.query-mutation-attach-shell-face-set-duplicate.new_shell");
    let declaration = TopologyRehomeAllOwnedFacesToNewShellDeclaration::new(
        shell_key.as_str(),
        ".current-head.query-mutation-attach-shell-face-set-duplicate.region-owns-shell",
        region,
        shell,
        vec![
            TopologyShellRehomeFaceMember::new(
                ".current-head.query-mutation-attach-shell-face-set-duplicate.shell-owns-face-1",
                face_ids[0],
            ),
            TopologyShellRehomeFaceMember::new(
                ".current-head.query-mutation-attach-shell-face-set-duplicate.shell-owns-face-2",
                face_ids[0],
            ),
        ],
    );
    assert_eq!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration),
        vec![TopologyMutationFamily::AttachShellOrWireMembership]
    );
}

fn seeded_patch_region_shell_and_faces(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    read_basis: &schema::facade::topology_authoring::DerivedTopologyReadBasis,
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
