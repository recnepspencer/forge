use std::collections::BTreeSet;

use forge_query::facade::ForgeQueryExistingTruthAssertionMode;
use worth_schema::facade::{
    created_ref, seed_milestone_one_primitive, DerivedTopologyReadBasis, WorthCreateKey,
    WorthEntityKind, WorthMilestoneOnePrimitiveCase, WorthRelationKind, WorthTopologyEntityKind,
    WorthTopologyRelationKind,
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
fn query_native_edit_runner_executes_connected_wire_split_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.split-wire",
        &WorthMilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("seed");
    let (wire, half_edge_ids) = seeded_wire_and_half_edges(&runtime, &verified.read_basis);
    let moved_half_edge_ids = vec![half_edge_ids[2], half_edge_ids[3]];
    let retained_half_edge_ids = vec![half_edge_ids[0], half_edge_ids[1]];
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.split-wire").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let wire_key = WorthCreateKey::new("worth.query-native-edit.split-wire.new_wire");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            wire_key.as_str(),
            WorthTopologyEntityKind::Wire,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.split-wire.new-wire-owns-half-edge-1",
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
            created_ref(wire_key.as_str()),
            moved_half_edge_ids[0],
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.split-wire.new-wire-owns-half-edge-2",
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
            created_ref(wire_key.as_str()),
            moved_half_edge_ids[1],
        ),
    ])
    .expect("non-empty batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("connected wire split should execute through the admitted owner-preserving lane");

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
        2
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
fn query_native_edit_runner_denies_wire_split_when_moved_subset_is_disconnected() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.split-wire-disconnected",
        &WorthMilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("seed");
    let (_wire, half_edge_ids) = seeded_wire_and_half_edges(&runtime, &verified.read_basis);
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.split-wire-disconnected")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let wire_key = WorthCreateKey::new("worth.query-native-edit.split-wire-disconnected.new_wire");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            wire_key.as_str(),
            WorthTopologyEntityKind::Wire,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.split-wire-disconnected.new-wire-owns-half-edge-1",
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
            created_ref(wire_key.as_str()),
            half_edge_ids[0],
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.split-wire-disconnected.new-wire-owns-half-edge-2",
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
            created_ref(wire_key.as_str()),
            half_edge_ids[2],
        ),
    ])
    .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("wire split must fail closed when the moved half-edge subset is disconnected");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachShellOrWireMembership]
    ));
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
            WorthEntityKind::from_kind_id(record.kind.kind_id)
                == Some(WorthEntityKind::Topology(WorthTopologyEntityKind::Wire))
        })
        .map(|record| record.entity_id)
        .expect("seeded wire primitive should contain a wire");
    let half_edge_ids = read_view
        .relations()
        .iter()
        .filter(|record| {
            record.source == wire
                && WorthRelationKind::from_kind_id(record.kind.kind_id)
                    == Some(WorthRelationKind::Topology(
                        WorthTopologyRelationKind::WireOwnsHalfEdge,
                    ))
        })
        .map(|record| record.target)
        .collect::<Vec<_>>();
    (wire, half_edge_ids)
}
