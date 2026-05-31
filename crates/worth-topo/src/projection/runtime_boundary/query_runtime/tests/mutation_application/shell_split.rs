use std::collections::BTreeSet;

use forge_query::facade::{
    ForgeQueryContinuityOutcomeClass, ForgeQueryExistingTruthAssertionMode,
    ForgeQueryGraphCompositionProgramStepKind,
};
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
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
    TopologyMutationFamily, TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_single_face_two_face_shell_split_program() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-mutation.split-shell",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let (region, shell, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis());
    let moved_face_id = face_ids[0];
    let retained_face_id = face_ids[1];
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-mutation.split-shell").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let shell_key = CreateKey::new(".current-head.query-mutation.split-shell.new_shell");
    let declaration = TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration::new(
        shell_key.as_str(),
        ".current-head.query-mutation.split-shell.region-owns-shell",
        ".current-head.query-mutation.split-shell.shell-owns-face",
        region,
        moved_face_id,
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("single-face two-face shell split should execute through declaration entry");

    assert_eq!(
        execution.families,
        vec![
            TopologyMutationFamily::CreateTopologyEntity,
            TopologyMutationFamily::AttachShellOrWireMembership,
            TopologyMutationFamily::AttachShellOrWireMembership,
        ]
    );
    assert_eq!(
        execution
            .mutation_evidence()
            .backend_verified_update_count(),
        1
    );
    assert_eq!(
        execution
            .mutation_evidence()
            .backend_verified_delete_count(),
        0
    );
    assert_eq!(
        execution
            .receipt
            .graph_composition_program()
            .expect("shell split should expose composed program")
            .steps()
            .iter()
            .map(|step| step.kind())
            .collect::<Vec<_>>(),
        vec![
            ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration,
            ForgeQueryGraphCompositionProgramStepKind::RelationDeclaration,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget,
        ]
    );
    assert_eq!(
        execution
            .receipt
            .graph_composition_lineage_summary()
            .expect("shell split should expose lineage summary")
            .entries()
            .iter()
            .filter(|entry| {
                entry.outcome_class()
                    == ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
            })
            .count(),
        1
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
fn current_head_runtime_denies_shell_split_when_old_shell_owns_more_than_two_faces() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-mutation.split-shell-large",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
    )
    .expect("seed topology");
    let (region, _shell, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-mutation.split-shell-large")
            .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let shell_key = CreateKey::new(".current-head.query-mutation.split-shell-large.new_shell");
    let declaration = TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration::new(
        shell_key.as_str(),
        ".current-head.query-mutation.split-shell-large.region-owns-shell",
        ".current-head.query-mutation.split-shell-large.shell-owns-face",
        region,
        face_ids[0],
    );

    assert_eq!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration),
        vec![TopologyMutationFamily::AttachShellOrWireMembership]
    );
}

#[test]
fn current_head_runtime_denies_shell_split_when_region_does_not_own_source_shell() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-mutation.split-shell-wrong-region",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let (_region, _shell, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis());
    let wrong_region = seed_minimal_topology(
        &mut runtime,
        ".current-head.query-mutation.split-shell-wrong-region.other",
    )
    .expect("extra seed")
    .region;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.query-mutation.split-shell-wrong-region",
    )
    .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let shell_key = CreateKey::new(".current-head.query-mutation.split-shell-wrong-region.new");
    let declaration = TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration::new(
        shell_key.as_str(),
        ".current-head.query-mutation.split-shell-wrong-region.region-owns-shell",
        ".current-head.query-mutation.split-shell-wrong-region.shell-owns-face",
        wrong_region,
        face_ids[0],
    );

    assert_eq!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration),
        vec![TopologyMutationFamily::AttachShellOrWireMembership]
    );
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
