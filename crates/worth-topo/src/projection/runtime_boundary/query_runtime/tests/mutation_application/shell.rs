use forge_query::facade::{
    ForgeQueryContinuityOutcomeClass, ForgeQueryExistingTruthAssertionMode,
    ForgeQueryGraphCompositionProgramStepKind,
};
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::topology_authoring::{
    seed_milestone_one_primitive, seed_minimal_topology, MilestoneOnePrimitiveCase,
};

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
fn current_head_runtime_executes_rehome_single_face_to_new_shell_program() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded = seed_minimal_topology(&mut runtime, "query-mutation-runtime-attach-shell-face")
        .expect("seed topology");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-mutation-attach-shell-face")
            .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let shell_key = CreateKey::new("query-mutation-runtime-attach-shell-face.inner_shell");
    let declaration = TopologyRehomeAllOwnedFacesToNewShellDeclaration::new(
        shell_key.as_str(),
        "query-mutation-runtime-attach-shell-face.region-owns-shell",
        seeded.region,
        seeded.shell,
        vec![TopologyShellRehomeFaceMember::new(
            "query-mutation-runtime-attach-shell-face.shell-owns-face",
            seeded.face,
        )],
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("single-face shell rehome should execute through declaration entry");

    assert_eq!(
        execution.families,
        vec![
            TopologyMutationFamily::CreateTopologyEntity,
            TopologyMutationFamily::AttachShellOrWireMembership,
            TopologyMutationFamily::AttachShellOrWireMembership,
            TopologyMutationFamily::RetireTopologyEntity,
        ]
    );
    assert_eq!(
        execution
            .mutation_evidence()
            .backend_verified_update_count(),
        2
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
            .expect("shell rehome should expose composed program")
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
fn current_head_runtime_denies_single_face_shell_rehome_when_old_shell_still_owns_other_faces() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-mutation-runtime-attach-shell-patch",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let (region, shell, face) =
        seeded_patch_region_shell_and_face(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-mutation-attach-shell-patch")
            .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let shell_key = CreateKey::new("current-head.query-mutation-attach-shell-patch.inner_shell");
    let declaration = TopologyRehomeAllOwnedFacesToNewShellDeclaration::new(
        shell_key.as_str(),
        "current-head.query-mutation-attach-shell-patch.region-owns-shell",
        region,
        shell,
        vec![TopologyShellRehomeFaceMember::new(
            "current-head.query-mutation-attach-shell-patch.shell-owns-face",
            face,
        )],
    );

    assert_eq!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration),
        vec![TopologyMutationFamily::AttachShellOrWireMembership]
    );
}

fn seeded_patch_region_shell_and_face(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    read_basis: &schema::facade::topology_authoring::DerivedTopologyReadBasis,
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
