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
fn query_native_edit_runner_executes_rehome_all_owned_half_edges_to_new_wire_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.attach-wire-set",
        &WorthMilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("seed");
    let (wire, half_edge_ids) = seeded_wire_and_half_edges(&runtime, &verified.read_basis);
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, "worth.query-native-edit.attach-wire-set")
        .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let wire_key = WorthCreateKey::new("worth.query-native-edit.attach-wire-set.new_wire");
    let mut contracts = vec![WorthTopologyEditContract::create_topology_entity(
        wire_key.as_str(),
        WorthTopologyEntityKind::Wire,
    )];
    for (index, half_edge_id) in half_edge_ids.iter().enumerate() {
        contracts.push(WorthTopologyEditContract::attach_shell_or_wire_membership(
            &format!(
                "worth.query-native-edit.attach-wire-set.new-wire-owns-half-edge-{}",
                index + 1
            ),
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
            created_ref(wire_key.as_str()),
            *half_edge_id,
        ));
    }
    contracts.push(WorthTopologyEditContract::retire_topology_entity(
        wire,
        WorthTopologyEntityKind::Wire,
    ));
    let batch = WorthTopologyEditBatch::new(contracts).expect("non-empty batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("full half-edge-set wire rehome should execute through the admitted wire owner-rehome lane");

    assert_eq!(
        execution.families,
        vec![
            WorthTopologyEditFamily::CreateTopologyEntity,
            WorthTopologyEditFamily::AttachShellOrWireMembership,
            WorthTopologyEditFamily::AttachShellOrWireMembership,
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
        4
    );
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_delete_count(),
        1
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
        .any(|operation| {
            operation.family() == "delete"
                && operation.target_collection() == Some("WorthTopologyEntity")
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
fn query_native_edit_runner_denies_wire_rehome_without_moving_the_full_owned_half_edge_set() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.attach-wire-set-partial",
        &WorthMilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("seed");
    let (wire, half_edge_ids) = seeded_wire_and_half_edges(&runtime, &verified.read_basis);
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.attach-wire-set-partial")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let wire_key = WorthCreateKey::new("worth.query-native-edit.attach-wire-set-partial.new_wire");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            wire_key.as_str(),
            WorthTopologyEntityKind::Wire,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.attach-wire-set-partial.new-wire-owns-half-edge-1",
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
            created_ref(wire_key.as_str()),
            half_edge_ids[0],
        ),
        WorthTopologyEditContract::retire_topology_entity(wire, WorthTopologyEntityKind::Wire),
    ])
    .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err(
            "wire set rehome must fail closed unless it moves the wire's full owned half-edge set",
        );

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachShellOrWireMembership]
    ));
}

#[test]
fn query_native_edit_runner_denies_wire_rehome_when_created_wire_keys_diverge() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth.query-native-edit.attach-wire-set-diverged-key",
        &WorthMilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("seed");
    let (wire, half_edge_ids) = seeded_wire_and_half_edges(&runtime, &verified.read_basis);
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.query-native-edit.attach-wire-set-diverged-key",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let wire_key =
        WorthCreateKey::new("worth.query-native-edit.attach-wire-set-diverged-key.new_wire");
    let wrong_wire_key =
        WorthCreateKey::new("worth.query-native-edit.attach-wire-set-diverged-key.other_wire");
    let mut contracts = vec![WorthTopologyEditContract::create_topology_entity(
        wire_key.as_str(),
        WorthTopologyEntityKind::Wire,
    )];
    for (index, half_edge_id) in half_edge_ids.iter().enumerate() {
        let owner = if index + 1 == half_edge_ids.len() {
            created_ref(wrong_wire_key.as_str())
        } else {
            created_ref(wire_key.as_str())
        };
        contracts.push(WorthTopologyEditContract::attach_shell_or_wire_membership(
            &format!(
                "worth.query-native-edit.attach-wire-set-diverged-key.new-wire-owns-half-edge-{}",
                index + 1
            ),
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
            owner,
            *half_edge_id,
        ));
    }
    contracts.push(WorthTopologyEditContract::retire_topology_entity(
        wire,
        WorthTopologyEntityKind::Wire,
    ));
    let batch = WorthTopologyEditBatch::new(contracts).expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("wire set rehome must fail closed when created wire keys diverge");

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
