use forge_query::facade::ForgeQueryExistingTruthAssertionMode;
use forge_relational::facade::identity::RelationId;
use worth_schema::facade::{
    created_ref, seed_minimal_topology, WorthCreateKey, WorthRelationKind, WorthTopologyEntityKind,
    WorthTopologyRelationKind,
};

use crate::edit::{
    WorthShellOrWireMembershipKind, WorthTopologyEditApplicationMode, WorthTopologyEditBatch,
    WorthTopologyEditContract, WorthTopologyEditFamily, WorthTopologyQueryEditRunner,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_detach_shell_or_wire_membership_through_query_native_edit_runner()
{
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth-query-edit-runtime-detach-wire")
        .expect("seed topology");
    let wire_owns_half_edge_relation =
        seeded_wire_owns_half_edge_relation(&runtime, &seeded.snapshot);
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit-detach-wire")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::detach_shell_or_wire_membership(
            wire_owns_half_edge_relation,
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
        ),
    ])
    .expect("non-empty edit batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("detach shell-or-wire membership should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::DetachShellOrWireMembership]
    );
    assert_eq!(
        execution.inspection.component_operations()[0]
            .existing_truth_assertion_evidence()
            .expect("detach receipt should retain backend verification evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_delete_count(),
        1
    );
    let wire = execution
        .materialized
        .topology()
        .wires
        .iter()
        .find(|wire| wire.entity_id == seeded.wire)
        .expect("seeded wire should remain present");
    assert!(wire.half_edge_ids.is_empty());
}

#[test]
fn current_head_runtime_executes_rehome_single_half_edge_to_new_wire_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "worth-query-edit-runtime-attach-wire").expect("seed");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit-attach-wire")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let wire_key = WorthCreateKey::new("worth-query-edit-runtime-attach-wire.new_wire");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            wire_key.as_str(),
            WorthTopologyEntityKind::Wire,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth-query-edit-runtime-attach-wire.new-wire-owns-half-edge",
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
            created_ref(wire_key.as_str()),
            seeded.half_edge,
        ),
        WorthTopologyEditContract::retire_topology_entity(
            seeded.wire,
            WorthTopologyEntityKind::Wire,
        ),
    ])
    .expect("non-empty edit batch");

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
fn current_head_runtime_denies_region_owns_shell_membership_until_invariant_complete_shell_subgraphs_are_admitted(
) {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth-query-edit-runtime-attach-shell")
        .expect("seed topology");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit-attach-shell")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key = WorthCreateKey::new("worth-query-edit-runtime-attach-shell.inner_shell");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            shell_key.as_str(),
            WorthTopologyEntityKind::Shell,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth-query-edit-runtime-attach-shell.region-owns-shell",
            WorthShellOrWireMembershipKind::RegionOwnsShell,
            seeded.region,
            created_ref(shell_key.as_str()),
        ),
    ])
    .expect("non-empty edit batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("attach shell-or-wire membership must fail closed until invariant-complete shell subgraphs are admitted");

    assert!(matches!(
        error,
        crate::edit::WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachShellOrWireMembership]
    ));
}

fn seeded_wire_owns_half_edge_relation(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    snapshot: &forge_relational::facade::snapshots::SnapshotHandle,
) -> RelationId {
    runtime
        .read_truth()
        .read_snapshot(snapshot)
        .expect("seeded snapshot should remain readable")
        .relations()
        .iter()
        .find(|record| {
            record.kind.kind_id
                == WorthRelationKind::Topology(WorthTopologyRelationKind::WireOwnsHalfEdge)
                    .kind_id()
        })
        .map(|record| record.relation_id)
        .expect("seeded primitive should contain a wire-owns-half-edge relation")
}
