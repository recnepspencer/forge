use forge_query::facade::{
    ForgeQueryContinuityOutcomeClass, ForgeQueryExistingTruthAssertionMode,
    ForgeQueryGraphCompositionProgramStepKind,
};
use forge_relational::facade::identity::RelationId;
use schema::facade::topology_authoring::{created_ref, seed_minimal_topology};
use schema::facade::{CreateKey, RelationKind, TopologyEntityKind, TopologyRelationKind};

use crate::edit::{
    RejectedEditScopeRow, ShellOrWireMembershipKind, TopologyEditApplicationMode,
    TopologyEditBatch, TopologyEditContract, TopologyEditFamily, TopologyEditRejectionClass,
};
use crate::query::{topology_runtime, TopologyQueryAssembly, TopologyRuntimeAdapters};
use crate::runtime_invariants::build_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_detach_shell_or_wire_membership_through_query_native_edit_runner()
{
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded = seed_minimal_topology(&mut runtime, "query-edit-runtime-detach-wire")
        .expect("seed topology");
    let wire_owns_half_edge_relation =
        seeded_wire_owns_half_edge_relation(&runtime, &seeded.snapshot);
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-edit-detach-wire").expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        TopologyEditBatch::new(vec![TopologyEditContract::detach_shell_or_wire_membership(
            wire_owns_half_edge_relation,
            ShellOrWireMembershipKind::WireOwnsHalfEdge,
        )])
        .expect("non-empty edit batch");

    let execution = assembly
        .apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect("detach shell-or-wire membership should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![TopologyEditFamily::DetachShellOrWireMembership]
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
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "query-edit-runtime-attach-wire").expect("seed");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-edit-attach-wire").expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let wire_key = CreateKey::new("query-edit-runtime-attach-wire.new_wire");
    let batch = TopologyEditBatch::new(vec![
        TopologyEditContract::create_topology_entity(wire_key.as_str(), TopologyEntityKind::Wire),
        TopologyEditContract::attach_shell_or_wire_membership(
            "query-edit-runtime-attach-wire.new-wire-owns-half-edge",
            ShellOrWireMembershipKind::WireOwnsHalfEdge,
            created_ref(wire_key.as_str()),
            seeded.half_edge,
        ),
        TopologyEditContract::retire_topology_entity(seeded.wire, TopologyEntityKind::Wire),
    ])
    .expect("non-empty edit batch");

    let execution = assembly
        .apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect("wire rehome should execute through the admitted created-wire owner-rehome lane");

    assert_eq!(
        execution.families,
        vec![
            TopologyEditFamily::CreateTopologyEntity,
            TopologyEditFamily::AttachShellOrWireMembership,
            TopologyEditFamily::RetireTopologyEntity,
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
    assert_eq!(
        execution
            .receipt
            .graph_composition_program()
            .expect("wire rehome should expose graph program")
            .steps()
            .iter()
            .map(|step| step.kind())
            .collect::<Vec<_>>(),
        vec![
            ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetirement,
        ]
    );
    assert_eq!(
        execution
            .receipt
            .graph_composition_lineage_summary()
            .expect("wire rehome should expose lineage summary")
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
        .any(|operation| {
            operation.family() == "update"
                && operation.target_collection() == Some("TopologyRelation")
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
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded = seed_minimal_topology(&mut runtime, "query-edit-runtime-attach-shell")
        .expect("seed topology");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-edit-attach-shell").expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key = CreateKey::new("query-edit-runtime-attach-shell.inner_shell");
    let batch = TopologyEditBatch::new(vec![
        TopologyEditContract::create_topology_entity(shell_key.as_str(), TopologyEntityKind::Shell),
        TopologyEditContract::attach_shell_or_wire_membership(
            "query-edit-runtime-attach-shell.region-owns-shell",
            ShellOrWireMembershipKind::RegionOwnsShell,
            seeded.region,
            created_ref(shell_key.as_str()),
        ),
    ])
    .expect("non-empty edit batch");

    let error = assembly.apply_edit(&mut workspace, batch.clone(), TopologyEditApplicationMode::Mainline)
        .expect_err("attach shell-or-wire membership must fail closed until invariant-complete shell subgraphs are admitted");

    assert!(matches!(
        error,
        crate::edit::TopologyQueryEditExecutionError::UnsupportedFamilies(ref families)
            if families == &vec![TopologyEditFamily::AttachShellOrWireMembership]
    ));
    assert_eq!(
        error.rejection_class(),
        Some(TopologyEditRejectionClass::OutOfClassEdit)
    );
    assert_eq!(
        error.rejected_edit_scope_report(&batch),
        Some(crate::edit::RejectedEditScopeReport {
            rows: vec![RejectedEditScopeRow {
                family: TopologyEditFamily::AttachShellOrWireMembership,
                rejection_class: TopologyEditRejectionClass::OutOfClassEdit,
                changed_scopes: vec![
                    crate::edit::TopologyEditChangedScope::Relation,
                    crate::edit::TopologyEditChangedScope::Shell,
                    crate::edit::TopologyEditChangedScope::LocalNeighborhood,
                ],
                naming_scopes: vec![crate::edit::TopologyEditNamingScope::AdjacentEntityNames],
                derived_regions: vec![
                    crate::edit::TopologyDerivedRegion::ShellRegion,
                    crate::edit::TopologyDerivedRegion::EditLocalNeighborhoodRegion,
                    crate::edit::TopologyDerivedRegion::NamingContinuityRegion,
                ],
                detail: "topology query edit execution does not admit families `[AttachShellOrWireMembership]` yet".to_string(),
            }],
        })
    );
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
                == RelationKind::Topology(TopologyRelationKind::WireOwnsHalfEdge).kind_id()
        })
        .map(|record| record.relation_id)
        .expect("seeded primitive should contain a wire-owns-half-edge relation")
}
