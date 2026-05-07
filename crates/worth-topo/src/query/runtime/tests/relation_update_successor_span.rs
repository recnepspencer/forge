use forge_query::facade::{
    ForgeQueryExistingTruthAssertionMode, ForgeQueryGraphCompositionLifecycleOutcomeKind,
    ForgeQueryGraphCompositionProgramStepKind,
};
use worth_schema::facade::topology_authoring::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
};

use super::relation_update_successor_span_support::{
    successor_span_relocation_batch, two_half_edge_span_relocation_batch,
};
use super::relation_update_support::RelationUpdateQuerySupport;
use crate::edit::{
    WorthTopologyEditApplicationMode, WorthTopologyEditFamily, WorthTopologyQueryEditExecutionError,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_two_half_edge_span_relocation_successor_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-rewire-successor-span",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.current-head.query-edit-rewire-successor-span",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = RelationUpdateQuerySupport::load(&workspace, &assembly);
    let moved_start_identity = support.first_source_identity_for_relation_kind(
        worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
    );
    let cycle = support.successor_cycle_identities(&mut workspace, &moved_start_identity, 6);
    let moved_end_identity = cycle[1].as_str();
    let old_successor_identity = cycle[2].as_str();
    let new_predecessor_identity = cycle[3].as_str();
    let new_successor_identity = cycle[4].as_str();
    let moved_start_id = support.find_entity_id_by_identity(&moved_start_identity);
    let moved_end_id = support.find_entity_id_by_identity(moved_end_identity);
    let old_predecessor_id = support.prev_target_half_edge_id(&moved_start_identity);
    let old_successor_id = support.find_entity_id_by_identity(old_successor_identity);
    let new_predecessor_id = support.find_entity_id_by_identity(new_predecessor_identity);
    let new_successor_id = support.find_entity_id_by_identity(new_successor_identity);
    let batch = two_half_edge_span_relocation_batch(
        &mut workspace,
        &support,
        &moved_start_identity,
        new_successor_identity,
    );

    let execution = assembly
        .apply_edit(
            &mut workspace,
            batch,
            WorthTopologyEditApplicationMode::Mainline,
        )
        .expect(
            "two-halfedge span relocation workflow should execute through admitted successor lane",
        );

    assert_eq!(execution.families.len(), 6);
    assert!(execution
        .families
        .iter()
        .all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor));
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_update_count(),
        6
    );
    assert_eq!(
        execution
            .receipt
            .graph_composition_program()
            .expect("two-halfedge successor span should expose graph program")
            .steps()
            .iter()
            .map(|step| step.kind())
            .collect::<Vec<_>>(),
        vec![ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget; 6]
    );
    assert_eq!(
        execution
            .receipt
            .graph_composition_lifecycle_outcomes()
            .expect("two-halfedge successor span should expose graph lifecycle")
            .entries()
            .iter()
            .map(|entry| entry.outcome_kind())
            .collect::<Vec<_>>(),
        vec![ForgeQueryGraphCompositionLifecycleOutcomeKind::RetargetedIdentityPreserved; 6]
    );
    assert_eq!(execution.inspection.component_operations().len(), 6);
    assert!(execution
        .inspection
        .component_operations()
        .iter()
        .all(|operation| {
            operation.family() == "update"
                && operation.target_collection() == Some("WorthTopologyRelation")
                && operation
                    .existing_truth_assertion_evidence()
                    .is_some_and(|evidence| {
                        evidence.mode()
                            == ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
                    })
        }));
    let moved_start = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_start_id)
        .expect("moved start halfedge should remain present");
    assert_eq!(moved_start.prev_half_edge_id, Some(new_predecessor_id));
    assert_eq!(moved_start.next_half_edge_id, Some(moved_end_id));
    let moved_end = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_end_id)
        .expect("moved end halfedge should remain present");
    assert_eq!(moved_end.prev_half_edge_id, Some(moved_start_id));
    assert_eq!(moved_end.next_half_edge_id, Some(new_successor_id));
    let old_predecessor = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == old_predecessor_id)
        .expect("old predecessor should remain present");
    assert_eq!(old_predecessor.next_half_edge_id, Some(old_successor_id));
}

#[test]
fn current_head_runtime_executes_three_half_edge_span_relocation_successor_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-rewire-successor-three-span",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.current-head.query-edit-rewire-successor-three-span",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = RelationUpdateQuerySupport::load(&workspace, &assembly);
    let moved_start_identity = support.first_source_identity_for_relation_kind(
        worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
    );
    let cycle = support.successor_cycle_identities(&mut workspace, &moved_start_identity, 6);
    let moved_mid_identity = cycle[1].as_str();
    let moved_end_identity = cycle[2].as_str();
    let old_successor_identity = cycle[3].as_str();
    let new_predecessor_identity = cycle[4].as_str();
    let new_successor_identity = cycle[5].as_str();
    let moved_start_id = support.find_entity_id_by_identity(&moved_start_identity);
    let moved_mid_id = support.find_entity_id_by_identity(moved_mid_identity);
    let moved_end_id = support.find_entity_id_by_identity(moved_end_identity);
    let old_predecessor_id = support.prev_target_half_edge_id(&moved_start_identity);
    let old_successor_id = support.find_entity_id_by_identity(old_successor_identity);
    let new_predecessor_id = support.find_entity_id_by_identity(new_predecessor_identity);
    let new_successor_id = support.find_entity_id_by_identity(new_successor_identity);
    let batch = successor_span_relocation_batch(
        &mut workspace,
        &support,
        &moved_start_identity,
        new_successor_identity,
        3,
    );

    let execution = assembly.apply_edit(&mut workspace, batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("three-halfedge span relocation workflow should execute through admitted successor lane");

    assert_eq!(execution.families.len(), 6);
    assert!(execution
        .families
        .iter()
        .all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor));
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_update_count(),
        6
    );
    assert_eq!(
        execution
            .receipt
            .graph_composition_program()
            .expect("three-halfedge successor span should expose graph program")
            .steps()
            .iter()
            .map(|step| step.kind())
            .collect::<Vec<_>>(),
        vec![ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget; 6]
    );
    assert_eq!(
        execution
            .receipt
            .graph_composition_lifecycle_outcomes()
            .expect("three-halfedge successor span should expose graph lifecycle")
            .entries()
            .iter()
            .map(|entry| entry.outcome_kind())
            .collect::<Vec<_>>(),
        vec![ForgeQueryGraphCompositionLifecycleOutcomeKind::RetargetedIdentityPreserved; 6]
    );
    assert_eq!(execution.inspection.component_operations().len(), 6);
    assert!(execution
        .inspection
        .component_operations()
        .iter()
        .all(|operation| {
            operation.family() == "update"
                && operation.target_collection() == Some("WorthTopologyRelation")
                && operation
                    .existing_truth_assertion_evidence()
                    .is_some_and(|evidence| {
                        evidence.mode()
                            == ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
                    })
        }));
    let moved_start = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_start_id)
        .expect("moved start halfedge should remain present");
    assert_eq!(moved_start.prev_half_edge_id, Some(new_predecessor_id));
    assert_eq!(moved_start.next_half_edge_id, Some(moved_mid_id));
    let moved_mid = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_mid_id)
        .expect("moved middle halfedge should remain present");
    assert_eq!(moved_mid.prev_half_edge_id, Some(moved_start_id));
    assert_eq!(moved_mid.next_half_edge_id, Some(moved_end_id));
    let moved_end = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_end_id)
        .expect("moved end halfedge should remain present");
    assert_eq!(moved_end.prev_half_edge_id, Some(moved_mid_id));
    assert_eq!(moved_end.next_half_edge_id, Some(new_successor_id));
    let old_predecessor = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == old_predecessor_id)
        .expect("old predecessor should remain present");
    assert_eq!(old_predecessor.next_half_edge_id, Some(old_successor_id));
}

#[test]
fn current_head_runtime_denies_cross_loop_two_half_edge_span_relocation_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-rewire-successor-span-cross-loop",
        &WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.current-head.query-edit-rewire-successor-span-cross-loop",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = RelationUpdateQuerySupport::load(&workspace, &assembly);
    let (moved_start_identity, new_successor_identity) =
        support.half_edge_identities_for_different_loops();
    let batch = two_half_edge_span_relocation_batch(
        &mut workspace,
        &support,
        &moved_start_identity,
        &new_successor_identity,
    );

    let error = assembly
        .apply_edit(
            &mut workspace,
            batch,
            WorthTopologyEditApplicationMode::Mainline,
        )
        .expect_err("cross-loop two-halfedge span relocation must fail closed");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families.iter().all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor)
    ));
}

#[test]
fn current_head_runtime_denies_degenerate_two_half_edge_span_relocation_before_current_successor() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-rewire-successor-span-degenerate",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.current-head.query-edit-rewire-successor-span-degenerate",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = RelationUpdateQuerySupport::load(&workspace, &assembly);
    let moved_start_identity = support.first_source_identity_for_relation_kind(
        worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
    );
    let cycle = support.successor_cycle_identities(&mut workspace, &moved_start_identity, 3);
    let old_successor_identity = cycle[2].as_str();
    let batch = two_half_edge_span_relocation_batch(
        &mut workspace,
        &support,
        &moved_start_identity,
        old_successor_identity,
    );

    let error = assembly
        .apply_edit(
            &mut workspace,
            batch,
            WorthTopologyEditApplicationMode::Mainline,
        )
        .expect_err("degenerate same-loop two-halfedge span relocation must fail closed");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families.iter().all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor)
    ));
}

#[test]
fn current_head_runtime_denies_three_half_edge_span_relocation_before_internal_member() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-rewire-successor-three-span-internal",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.current-head.query-edit-rewire-successor-three-span-internal",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = RelationUpdateQuerySupport::load(&workspace, &assembly);
    let moved_start_identity = support.first_source_identity_for_relation_kind(
        worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
    );
    let cycle = support.successor_cycle_identities(&mut workspace, &moved_start_identity, 3);
    let internal_successor_identity = cycle[1].as_str();
    let batch = successor_span_relocation_batch(
        &mut workspace,
        &support,
        &moved_start_identity,
        internal_successor_identity,
        3,
    );

    let error = assembly
        .apply_edit(
            &mut workspace,
            batch,
            WorthTopologyEditApplicationMode::Mainline,
        )
        .expect_err("span relocation before an internal member must fail closed");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families.iter().all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor)
    ));
}
