use forge_query::facade::ForgeQueryExistingTruthAssertionMode;
use worth_schema::facade::{
    created_ref, seed_minimal_topology, WorthCreateKey, WorthTopologyEntityKind,
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
fn query_native_edit_runner_executes_rehome_single_half_edge_to_new_wire_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "worth.query-native-edit.attach-shell").expect("seed");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, "worth.query-native-edit.attach-shell")
        .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let wire_key = WorthCreateKey::new("worth.query-native-edit.attach-shell.inner_wire");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            wire_key.as_str(),
            WorthTopologyEntityKind::Wire,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.attach-shell.new-wire-owns-half-edge",
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
            created_ref(wire_key.as_str()),
            seeded.half_edge,
        ),
        WorthTopologyEditContract::retire_topology_entity(
            seeded.wire,
            WorthTopologyEntityKind::Wire,
        ),
    ])
    .expect("non-empty batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("wire rehome should execute through the admitted created-wire owner-rehome lane");

    assert_eq!(
        execution.families,
        vec![
            WorthTopologyEditFamily::CreateTopologyEntity,
            WorthTopologyEditFamily::AttachShellOrWireMembership,
            WorthTopologyEditFamily::RetireTopologyEntity,
        ]
    );
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_update_count(),
        1
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
        .any(|operation| {
            operation.family() == "update"
                && operation.target_collection() == Some("WorthTopologyRelation")
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
        .wires
        .iter()
        .any(|wire| wire.entity_id == seeded.wire));
    let new_wire = execution
        .materialized
        .topology()
        .wires
        .iter()
        .find(|wire| wire.label == wire_key.as_str())
        .expect("new wire should remain present after admitted rehome");
    assert_eq!(new_wire.half_edge_ids, vec![seeded.half_edge]);
}

#[test]
fn query_native_edit_runner_denies_wire_rehome_without_retiring_the_emptied_wire() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(
        &mut runtime,
        "worth.query-native-edit.attach-wire-without-retire",
    )
    .expect("seed");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.query-native-edit.attach-wire-without-retire",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let wire_key = WorthCreateKey::new("worth.query-native-edit.attach-wire-without-retire.wire");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            wire_key.as_str(),
            WorthTopologyEntityKind::Wire,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth.query-native-edit.attach-wire-without-retire.new-wire-owns-half-edge",
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
            created_ref(wire_key.as_str()),
            seeded.half_edge,
        ),
    ])
    .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("wire rehome must fail closed if the emptied old wire is left behind");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachShellOrWireMembership]
    ));
}
