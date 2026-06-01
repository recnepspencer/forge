use std::collections::BTreeSet;

use forge_query::facade::{
    ForgeQueryContinuityOutcomeClass, ForgeQueryExistingTruthAssertionMode,
    ForgeQueryGraphCompositionProgramStepKind,
};
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
<<<<<<< HEAD:crates/worth-topo/src/projection/runtime_boundary/query_runtime/tests/edit_execution/wire_set.rs
use schema::facade::topology_authoring::{
    created_ref, seed_milestone_one_primitive, MilestoneOnePrimitiveCase,
};
=======
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
>>>>>>> origin/master:crates/worth-topo/src/projection/runtime_boundary/query_runtime/tests/mutation_application/wire_set.rs

use crate::certification::support::declaration_runtime::{
    current_head_unsupported_declaration_families, execute_current_head_topology_declaration,
};
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    TopologyMutationFamily, TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    TopologyWireRehomeHalfEdgeMember,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_rehome_all_owned_half_edges_to_new_wire_program() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-mutation-attach-wire-set",
        &MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("seed topology");
    let (wire, half_edge_ids) = seeded_wire_and_half_edges(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, ".current-head.query-mutation-attach-wire-set")
        .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let wire_key = CreateKey::new(".current-head.query-mutation-attach-wire-set.new_wire");
    let declaration = TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::new(
        wire_key.as_str(),
        wire,
        half_edge_ids
            .iter()
            .enumerate()
            .map(|(index, half_edge_id)| {
                TopologyWireRehomeHalfEdgeMember::new(
                    format!(
                        ".current-head.query-mutation-attach-wire-set.new-wire-owns-half-edge-{}",
                        index + 1
                    ),
                    *half_edge_id,
                )
            })
            .collect(),
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("full half-edge-set wire rehome should execute through declaration entry");

    assert_eq!(
        execution.families,
        vec![
            TopologyMutationFamily::CreateTopologyEntity,
            TopologyMutationFamily::AttachShellOrWireMembership,
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
        4
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
            .expect("wire half-edge-set rehome should expose composed program")
            .steps()
            .iter()
            .map(|step| step.kind())
            .collect::<Vec<_>>(),
        vec![
            ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget,
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
            .expect("wire half-edge-set rehome should expose lineage summary")
            .entries()
            .iter()
            .filter(|entry| {
                entry.outcome_class()
                    == ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
            })
            .count(),
        4
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
        .any(|operation| {
            operation.family() == "delete"
                && operation.target_collection() == Some("TopologyEntity")
                && operation
                    .existing_truth_assertion_evidence()
                    .is_some_and(|evidence| {
                        evidence.mode()
                            == ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
                    })
        }));
    let new_wire = execution
        .materialized
        .topology()
        .wires
        .iter()
        .find(|wire_record| wire_record.label == wire_key.as_str())
        .expect("new wire should remain present");
    assert_eq!(
        new_wire
            .half_edge_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        half_edge_ids.iter().copied().collect::<BTreeSet<_>>()
    );
    assert!(!execution
        .materialized
        .topology()
        .wires
        .iter()
        .any(|wire_record| wire_record.entity_id == wire));
}

#[test]
fn current_head_runtime_denies_wire_rehome_without_moving_the_full_owned_half_edge_set() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-mutation-attach-wire-set-partial",
        &MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("seed topology");
    let (wire, half_edge_ids) = seeded_wire_and_half_edges(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.query-mutation-attach-wire-set-partial",
    )
    .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let wire_key = CreateKey::new(".current-head.query-mutation-attach-wire-set-partial.new_wire");
    let declaration = TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::new(
        wire_key.as_str(),
        wire,
        vec![TopologyWireRehomeHalfEdgeMember::new(
            ".current-head.query-mutation-attach-wire-set-partial.new-wire-owns-half-edge-1",
            half_edge_ids[0],
        )],
    );
    assert_eq!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration),
        vec![TopologyMutationFamily::AttachShellOrWireMembership]
    );
}

fn seeded_wire_and_half_edges(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> (
    forge_relational::facade::identity::EntityId,
    Vec<forge_relational::facade::identity::EntityId>,
) {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .expect("seeded snapshot should remain readable");
    let wire = read_view
        .entities()
        .iter()
        .find(|record| {
            EntityKind::from_kind_id(record.kind.kind_id)
                == Some(EntityKind::Topology(TopologyEntityKind::Wire))
        })
        .map(|record| record.entity_id)
        .expect("seeded wire primitive should contain a wire");
    let half_edge_ids = read_view
        .relations()
        .iter()
        .filter(|record| {
            record.source == wire
                && RelationKind::from_kind_id(record.kind.kind_id)
                    == Some(RelationKind::Topology(
                        TopologyRelationKind::WireOwnsHalfEdge,
                    ))
        })
        .map(|record| record.target)
        .collect::<Vec<_>>();
    (wire, half_edge_ids)
}
