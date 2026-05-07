use forge_query::facade::{
    ForgeQueryContinuityMutationFamily, ForgeQueryContinuityOutcomeClass,
    ForgeQueryExistingTruthAssertionMode, ForgeQueryGraphCompositionLifecycleOutcomeKind,
    ForgeQueryGraphCompositionProgramStepKind,
};
use worth_schema::facade::topology_authoring::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
};

use super::relation_update_support::RelationUpdateQuerySupport;
use crate::edit::{
    WorthLoopSuccessorKind, WorthTopologyEditApplicationMode, WorthTopologyEditBatch,
    WorthTopologyEditContract, WorthTopologyEditFamily, WorthTopologyQueryEditExecutionError,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_half_edge_relocation_successor_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-rewire-successor",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 5 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit-rewire-successor")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = RelationUpdateQuerySupport::load(&workspace, &assembly);
    let moved_identity = support.first_source_identity_for_relation_kind(
        worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
    );
    let moved_half_edge_id = support.find_entity_id_by_identity(&moved_identity);
    let old_successor_id = support.next_target_half_edge_id(&moved_identity);
    let old_predecessor_id = support.prev_target_half_edge_id(&moved_identity);
    let intermediate_successor_identity = support.find_entity_identity_by_id(old_successor_id);
    let intermediate_successor_id =
        support.next_target_half_edge_id(&intermediate_successor_identity);
    let second_intermediate_identity =
        support.find_entity_identity_by_id(intermediate_successor_id);
    let new_successor_id = support.next_target_half_edge_id(&second_intermediate_identity);
    let new_successor_identity = support.find_entity_identity_by_id(new_successor_id);
    let new_predecessor_id = support.prev_target_half_edge_id(&new_successor_identity);
    let batch = successor_relocation_batch(&support, &moved_identity, &new_successor_identity);

    let execution = assembly
        .apply_edit(
            &mut workspace,
            batch,
            WorthTopologyEditApplicationMode::Mainline,
        )
        .expect("halfedge relocation workflow should execute through admitted successor lane");

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
            .expect("successor relocation should expose graph composition program")
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
            .expect("successor relocation should expose graph lifecycle")
            .entries()
            .iter()
            .map(|entry| entry.outcome_kind())
            .collect::<Vec<_>>(),
        vec![ForgeQueryGraphCompositionLifecycleOutcomeKind::RetargetedIdentityPreserved; 6]
    );
    assert_eq!(
        execution
            .receipt
            .graph_composition_assumption_summary()
            .expect("successor relocation should expose graph assumption summary")
            .verified_step_count(),
        6
    );
    let lineage = execution
        .receipt
        .graph_composition_lineage_summary()
        .expect("successor relocation should expose graph lineage summary");
    assert_eq!(
        lineage.counter_snapshot(),
        "continuity_entries=6;single_successors=6;split_successors=0;merge_successors=0;rejections=0"
    );
    assert!(lineage.entries().iter().all(|entry| {
        entry.family() == ForgeQueryContinuityMutationFamily::RebindExistingTarget
            && entry.outcome_class() == ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
            && entry.target_collection() == Some("WorthTopologyRelation")
    }));
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
    assert_eq!(
        execution
            .inspection
            .graph_composition_program()
            .expect("inspection should expose graph program")
            .component_count(),
        6
    );
    assert_eq!(
        execution
            .inspection
            .graph_composition_lineage_summary()
            .expect("inspection should expose graph lineage summary")
            .lineage_summary_digest(),
        lineage.lineage_summary_digest()
    );
    assert!(execution
        .inspection
        .component_operations()
        .iter()
        .all(|operation| {
            operation
                .continuity_mutation_evidence()
                .is_some_and(|evidence| {
                    evidence.family() == ForgeQueryContinuityMutationFamily::RebindExistingTarget
                        && evidence.outcome_class()
                            == ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
                })
        }));
    let moved_half_edge = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_half_edge_id)
        .expect("moved halfedge should remain present");
    assert_eq!(moved_half_edge.prev_half_edge_id, Some(new_predecessor_id));
    assert_eq!(moved_half_edge.next_half_edge_id, Some(new_successor_id));
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
fn current_head_runtime_denies_cross_loop_successor_relocation_workflow() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-rewire-successor-cross-loop",
        &WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.current-head.query-edit-rewire-successor-cross-loop",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = RelationUpdateQuerySupport::load(&workspace, &assembly);
    let (moved_identity, new_successor_identity) =
        support.half_edge_identities_for_different_loops();
    let batch = successor_relocation_batch(&support, &moved_identity, &new_successor_identity);

    let error = assembly
        .apply_edit(
            &mut workspace,
            batch,
            WorthTopologyEditApplicationMode::Mainline,
        )
        .expect_err("cross-loop successor relocation must fail closed");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families.iter().all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor)
    ));
}

fn successor_relocation_batch(
    support: &RelationUpdateQuerySupport,
    moved_identity: &str,
    new_successor_identity: &str,
) -> WorthTopologyEditBatch {
    let moved_half_edge_id = support.find_entity_id_by_identity(moved_identity);
    let old_successor_id = support.next_target_half_edge_id(moved_identity);
    let old_predecessor_id = support.prev_target_half_edge_id(moved_identity);
    let new_successor_id = support.find_entity_id_by_identity(new_successor_identity);
    let new_predecessor_id = support.prev_target_half_edge_id(new_successor_identity);
    let old_successor_identity = support.find_entity_identity_by_id(old_successor_id);
    let old_predecessor_identity = support.find_entity_identity_by_id(old_predecessor_id);
    let new_predecessor_identity = support.find_entity_identity_by_id(new_predecessor_id);

    WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::rewire_loop_successor(
            support.relation_id_for_source_kind(
                moved_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
            ),
            WorthLoopSuccessorKind::Next,
            moved_half_edge_id,
            new_successor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            support.relation_id_for_source_kind(
                moved_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgePrev,
            ),
            WorthLoopSuccessorKind::Prev,
            moved_half_edge_id,
            new_predecessor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            support.relation_id_for_source_kind(
                &old_predecessor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
            ),
            WorthLoopSuccessorKind::Next,
            old_predecessor_id,
            old_successor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            support.relation_id_for_source_kind(
                &old_successor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgePrev,
            ),
            WorthLoopSuccessorKind::Prev,
            old_successor_id,
            old_predecessor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            support.relation_id_for_source_kind(
                &new_predecessor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
            ),
            WorthLoopSuccessorKind::Next,
            new_predecessor_id,
            moved_half_edge_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            support.relation_id_for_source_kind(
                new_successor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgePrev,
            ),
            WorthLoopSuccessorKind::Prev,
            new_successor_id,
            moved_half_edge_id,
        ),
    ])
    .expect("non-empty successor relocation batch")
}
