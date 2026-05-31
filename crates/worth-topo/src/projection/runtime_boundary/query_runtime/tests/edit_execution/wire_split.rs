use std::collections::BTreeSet;

use forge_query::facade::{
    ForgeQueryContinuityOutcomeClass, ForgeQueryExistingTruthAssertionMode,
    ForgeQueryGraphCompositionProgramStepKind,
};
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

use super::super::declaration_runtime_support::{
    current_head_unsupported_declaration_families, execute_current_head_topology_declaration,
};
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    TopologyEditFamily, TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    TopologyWireSplitHalfEdgeMember,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_connected_wire_split_program() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-edit.split-wire",
        &MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("seed topology");
    let (wire, half_edge_ids) = seeded_wire_and_half_edges(&runtime, &verified.read_basis());
    let moved_half_edge_ids = vec![half_edge_ids[2], half_edge_ids[3]];
    let retained_half_edge_ids = vec![half_edge_ids[0], half_edge_ids[1]];
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-edit.split-wire").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let wire_key = CreateKey::new(".current-head.query-edit.split-wire.new_wire");
    let declaration = TopologySplitConnectedHalfEdgeSetToNewWireDeclaration::new(
        wire_key.as_str(),
        vec![
            TopologyWireSplitHalfEdgeMember::new(
                ".current-head.query-edit.split-wire.new-wire-owns-half-edge-1",
                moved_half_edge_ids[0],
            ),
            TopologyWireSplitHalfEdgeMember::new(
                ".current-head.query-edit.split-wire.new-wire-owns-half-edge-2",
                moved_half_edge_ids[1],
            ),
        ],
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("connected wire split should execute through declaration entry");

    assert_eq!(
        execution.families,
        vec![
            TopologyEditFamily::CreateTopologyEntity,
            TopologyEditFamily::AttachShellOrWireMembership,
            TopologyEditFamily::AttachShellOrWireMembership,
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
        0
    );
    assert_eq!(
        execution
            .receipt
            .graph_composition_program()
            .expect("wire split should expose composed program")
            .steps()
            .iter()
            .map(|step| step.kind())
            .collect::<Vec<_>>(),
        vec![
            ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget,
        ]
    );
    assert_eq!(
        execution
            .receipt
            .graph_composition_lineage_summary()
            .expect("wire split should expose lineage summary")
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
    let new_wire = execution
        .materialized
        .topology()
        .wires
        .iter()
        .find(|wire_record| wire_record.label == wire_key.as_str())
        .expect("new wire should remain present");
    let retained_wire = execution
        .materialized
        .topology()
        .wires
        .iter()
        .find(|wire_record| wire_record.entity_id == wire)
        .expect("retained wire should remain present");
    assert_eq!(
        new_wire
            .half_edge_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        moved_half_edge_ids.iter().copied().collect::<BTreeSet<_>>()
    );
    assert_eq!(
        retained_wire
            .half_edge_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        retained_half_edge_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
    );
}

#[test]
fn current_head_runtime_denies_wire_split_when_moved_subset_is_disconnected() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-edit.split-wire-disconnected",
        &MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("seed topology");
    let (_wire, half_edge_ids) = seeded_wire_and_half_edges(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-edit.split-wire-disconnected")
            .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let wire_key = CreateKey::new(".current-head.query-edit.split-wire-disconnected.new_wire");
    let declaration = TopologySplitConnectedHalfEdgeSetToNewWireDeclaration::new(
        wire_key.as_str(),
        vec![
            TopologyWireSplitHalfEdgeMember::new(
                ".current-head.query-edit.split-wire-disconnected.new-wire-owns-half-edge-1",
                half_edge_ids[0],
            ),
            TopologyWireSplitHalfEdgeMember::new(
                ".current-head.query-edit.split-wire-disconnected.new-wire-owns-half-edge-2",
                half_edge_ids[2],
            ),
        ],
    );

    assert_eq!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration),
        vec![TopologyEditFamily::AttachShellOrWireMembership]
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
