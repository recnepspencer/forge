use forge_query::facade::{
    ForgeQueryContinuityMutationFamily, ForgeQueryContinuityOutcomeClass,
    ForgeQueryExistingTruthAssertionMode, ForgeQueryGraphCompositionLifecycleOutcomeKind,
    ForgeQueryGraphCompositionProgramStepKind,
};
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::super::query_runtime_support::QueryRuntimeSupport;
use crate::certification::support::declaration_runtime::{
    current_head_unsupported_declaration_families, execute_current_head_topology_declaration,
};
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::topology_operators::{LoopSuccessorKind, TopologyMutationFamily};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_half_edge_relocation_successor_program() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        ".current-head.query-mutation-rewire-successor",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 5 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, ".current-head.query-mutation-rewire-successor")
        .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let support = QueryRuntimeSupport::load(&mut workspace, &surfaces);
    let moved_identity = support.first_source_identity_for_relation_kind(
        schema::facade::platform::relations::TopologyRelationKind::HalfEdgeNext,
    );
    let moved_half_edge_id = support.find_entity_id_by_identity(&moved_identity);
    let old_successor_id = support.next_target_half_edge_id(&mut workspace, &moved_identity);
    let old_predecessor_id = support.prev_target_half_edge_id(&mut workspace, &moved_identity);
    let intermediate_successor_identity = support.find_entity_identity_by_id(old_successor_id);
    let intermediate_successor_id =
        support.next_target_half_edge_id(&mut workspace, &intermediate_successor_identity);
    let second_intermediate_identity =
        support.find_entity_identity_by_id(intermediate_successor_id);
    let new_successor_id =
        support.next_target_half_edge_id(&mut workspace, &second_intermediate_identity);
    let new_successor_identity = support.find_entity_identity_by_id(new_successor_id);
    let new_predecessor_id =
        support.prev_target_half_edge_id(&mut workspace, &new_successor_identity);
    let declaration = successor_relocation_declaration(
        &mut workspace,
        &support,
        &moved_identity,
        &new_successor_identity,
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("halfedge relocation program should execute through declaration entry");
    let synopsis = execution.accepted_mutation_projection();

    assert_eq!(synopsis.mutation_families().len(), 6);
    assert!(synopsis
        .mutation_families()
        .iter()
        .all(|family| *family == TopologyMutationFamily::RewireLoopSuccessor));
    assert_eq!(
        execution
            .mutation_evidence()
            .backend_verified_update_count(),
        6
    );
    assert_eq!(
        execution
            .receipt()
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
            .receipt()
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
            .receipt()
            .graph_composition_assumption_summary()
            .expect("successor relocation should expose graph assumption summary")
            .verified_step_count(),
        6
    );
    let lineage = execution
        .receipt()
        .graph_composition_lineage_summary()
        .expect("successor relocation should expose graph lineage summary");
    assert_eq!(
        lineage.counter_snapshot(),
        "continuity_entries=6;single_successors=6;split_successors=0;merge_successors=0;rejections=0"
    );
    assert!(lineage.entries().iter().all(|entry| {
        entry.family() == ForgeQueryContinuityMutationFamily::RebindExistingTarget
            && entry.outcome_class() == ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
            && entry.target_collection() == Some("TopologyRelation")
    }));
    assert_eq!(execution.inspection().component_operations().len(), 6);
    assert!(execution
        .inspection()
        .component_operations()
        .iter()
        .all(|operation| {
            operation.family() == "update"
                && operation.target_collection() == Some("TopologyRelation")
                && operation
                    .existing_truth_assertion_evidence()
                    .is_some_and(|evidence| {
                        evidence.mode()
                            == ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
                    })
        }));
    assert_eq!(
        execution
            .inspection()
            .graph_composition_program()
            .expect("inspection should expose composed program")
            .component_count(),
        6
    );
    assert_eq!(
        execution
            .inspection()
            .graph_composition_lineage_summary()
            .expect("inspection should expose graph lineage summary")
            .lineage_summary_digest(),
        lineage.lineage_summary_digest()
    );
    assert!(execution
        .inspection()
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
        .materialized()
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_half_edge_id)
        .expect("moved halfedge should remain present");
    assert_eq!(moved_half_edge.prev_half_edge_id, Some(new_predecessor_id));
    assert_eq!(moved_half_edge.next_half_edge_id, Some(new_successor_id));
    let old_predecessor = execution
        .materialized()
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == old_predecessor_id)
        .expect("old predecessor should remain present");
    assert_eq!(old_predecessor.next_half_edge_id, Some(old_successor_id));
}

#[test]
fn current_head_runtime_denies_cross_loop_successor_relocation_program() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        ".current-head.query-mutation-rewire-successor-cross-loop",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.query-mutation-rewire-successor-cross-loop",
    )
    .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let support = QueryRuntimeSupport::load(&mut workspace, &surfaces);
    let (moved_identity, new_successor_identity) =
        support.half_edge_identities_for_different_loops();
    let declaration = successor_relocation_declaration(
        &mut workspace,
        &support,
        &moved_identity,
        &new_successor_identity,
    );
    assert_eq!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration),
        vec![TopologyMutationFamily::RewireLoopSuccessor]
    );
}

fn successor_relocation_declaration(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    support: &QueryRuntimeSupport,
    moved_identity: &str,
    new_successor_identity: &str,
) -> crate::topology_operators::TopologyRewireLoopSuccessorProgramDeclaration {
    let moved_half_edge_id = support.find_entity_id_by_identity(moved_identity);
    let old_successor_id = support.next_target_half_edge_id(workspace, moved_identity);
    let old_predecessor_id = support.prev_target_half_edge_id(workspace, moved_identity);
    let new_successor_id = support.find_entity_id_by_identity(new_successor_identity);
    let new_predecessor_id = support.prev_target_half_edge_id(workspace, new_successor_identity);
    let old_successor_identity = support.find_entity_identity_by_id(old_successor_id);
    let old_predecessor_identity = support.find_entity_identity_by_id(old_predecessor_id);
    let new_predecessor_identity = support.find_entity_identity_by_id(new_predecessor_id);

    crate::topology_operators::TopologyRewireLoopSuccessorProgramDeclaration::new(vec![
        crate::topology_operators::TopologyLoopSuccessorRewireMember::new(
            support.relation_id_for_source_kind(
                moved_identity,
                schema::facade::platform::relations::TopologyRelationKind::HalfEdgeNext,
            ),
            LoopSuccessorKind::Next,
            moved_half_edge_id,
            new_successor_id,
        ),
        crate::topology_operators::TopologyLoopSuccessorRewireMember::new(
            support.relation_id_for_source_kind(
                moved_identity,
                schema::facade::platform::relations::TopologyRelationKind::HalfEdgePrev,
            ),
            LoopSuccessorKind::Prev,
            moved_half_edge_id,
            new_predecessor_id,
        ),
        crate::topology_operators::TopologyLoopSuccessorRewireMember::new(
            support.relation_id_for_source_kind(
                &old_predecessor_identity,
                schema::facade::platform::relations::TopologyRelationKind::HalfEdgeNext,
            ),
            LoopSuccessorKind::Next,
            old_predecessor_id,
            old_successor_id,
        ),
        crate::topology_operators::TopologyLoopSuccessorRewireMember::new(
            support.relation_id_for_source_kind(
                &old_successor_identity,
                schema::facade::platform::relations::TopologyRelationKind::HalfEdgePrev,
            ),
            LoopSuccessorKind::Prev,
            old_successor_id,
            old_predecessor_id,
        ),
        crate::topology_operators::TopologyLoopSuccessorRewireMember::new(
            support.relation_id_for_source_kind(
                &new_predecessor_identity,
                schema::facade::platform::relations::TopologyRelationKind::HalfEdgeNext,
            ),
            LoopSuccessorKind::Next,
            new_predecessor_id,
            moved_half_edge_id,
        ),
        crate::topology_operators::TopologyLoopSuccessorRewireMember::new(
            support.relation_id_for_source_kind(
                new_successor_identity,
                schema::facade::platform::relations::TopologyRelationKind::HalfEdgePrev,
            ),
            LoopSuccessorKind::Prev,
            new_successor_id,
            moved_half_edge_id,
        ),
    ])
}
