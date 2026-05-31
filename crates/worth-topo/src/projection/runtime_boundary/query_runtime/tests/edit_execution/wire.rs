use forge_query::facade::{
    ForgeQueryContinuityOutcomeClass, ForgeQueryExistingTruthAssertionMode,
    ForgeQueryGraphCompositionProgramStepKind,
};
use forge_relational::facade::identity::RelationId;
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::topology_authoring::{created_ref, seed_minimal_topology};

use super::super::declaration_runtime_support::{
    current_head_unsupported_declaration_families, execute_current_head_topology_declaration,
};
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    RejectedEditScopeRow, ShellOrWireMembershipKind,
    TopologyAttachShellOrWireMembershipDeclaration, TopologyDetachShellOrWireMembershipDeclaration,
    TopologyEditFamily, TopologyEditRejectionClass,
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration, TopologyWireRehomeHalfEdgeMember,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_detach_shell_or_wire_membership_through_topology_operator_runner()
{
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded = seed_minimal_topology(&mut runtime, "query-edit-runtime-detach-wire")
        .expect("seed topology");
    let wire_owns_half_edge_relation =
        seeded_wire_owns_half_edge_relation(&runtime, &seeded.snapshot);
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-edit-detach-wire").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let declaration = TopologyDetachShellOrWireMembershipDeclaration::new(
        wire_owns_half_edge_relation,
        ShellOrWireMembershipKind::WireOwnsHalfEdge,
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("detach shell-or-wire membership should execute through declaration entry");

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
fn current_head_runtime_executes_rehome_single_half_edge_to_new_wire_program() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "query-edit-runtime-attach-wire").expect("seed");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-edit-attach-wire").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let wire_key = CreateKey::new("query-edit-runtime-attach-wire.new_wire");
    let declaration = TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::new(
        wire_key.as_str(),
        seeded.wire,
        vec![TopologyWireRehomeHalfEdgeMember::new(
            "query-edit-runtime-attach-wire.new-wire-owns-half-edge",
            seeded.half_edge,
        )],
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("wire rehome should execute through declaration entry");

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
            .expect("wire rehome should expose composed program")
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
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let shell_key = CreateKey::new("query-edit-runtime-attach-shell.inner_shell");
    let declaration = TopologyAttachShellOrWireMembershipDeclaration::new(
        "query-edit-runtime-attach-shell.region-owns-shell",
        ShellOrWireMembershipKind::RegionOwnsShell,
        seeded.region,
        created_ref(shell_key.as_str()),
    );
    assert_eq!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration),
        vec![TopologyEditFamily::AttachShellOrWireMembership]
    );
    assert_eq!(
        crate::topology_operators::TopologyOperatorExecutionError::UnsupportedFamilies(vec![
            TopologyEditFamily::AttachShellOrWireMembership
        ])
        .rejection_class(),
        Some(TopologyEditRejectionClass::OutOfClassEdit)
    );
    assert_eq!(
        crate::topology_operators::TopologyOperatorExecutionError::UnsupportedFamilies(
            vec![TopologyEditFamily::AttachShellOrWireMembership]
        )
        .rejected_declaration_scope_report(&declaration),
        Some(crate::topology_operators::RejectedEditScopeReport {
            rows: vec![RejectedEditScopeRow {
                family: TopologyEditFamily::AttachShellOrWireMembership,
                rejection_class: TopologyEditRejectionClass::OutOfClassEdit,
                changed_scopes: vec![
                    crate::topology_operators::TopologyEditChangedScope::Relation,
                    crate::topology_operators::TopologyEditChangedScope::Shell,
                    crate::topology_operators::TopologyEditChangedScope::LocalNeighborhood,
                ],
                naming_scopes: vec![crate::topology_operators::TopologyEditNamingScope::AdjacentEntityNames],
                derived_regions: vec![
                    crate::topology_operators::TopologyDerivedRegion::ShellRegion,
                    crate::topology_operators::TopologyDerivedRegion::EditLocalNeighborhoodRegion,
                    crate::topology_operators::TopologyDerivedRegion::NamingContinuityRegion,
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
