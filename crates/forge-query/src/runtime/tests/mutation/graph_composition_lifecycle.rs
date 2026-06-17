use super::super::support::*;

fn task_edge_runtime() -> ForgeQueryRuntime {
    stateful_bridge_task_edge_runtime()
}

#[test]
fn compose_graph_supports_symbolic_entity_followup_and_relation_retirement() {
    let mut workspace = task_edge_runtime()
        .workspace("tasks.graph-composition-lifecycle")
        .expect("runtime should open a named workspace");
    let tasks: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.graph-composition-lifecycle-tasks", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-graph-composition-lifecycle-tasks")
        })
        .expect("task live view should declare");
    let edges: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.graph-composition-lifecycle-edges", |q| {
            q.from("TaskEdge")
                .select(["edge.kind", "edge.source_identity", "edge.target_identity"])
                .order_by("edge.kind")
                .schema_basis("tasks-graph-composition-lifecycle-edges")
        })
        .expect("edge live view should declare");

    let receipt = workspace
        .compose_graph(|graph| {
            let draft = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect("identity.id", "task-lifecycle")
                    .aspect("title.value", "Draft task")
            })?;
            let edge = graph.insert_symbolic_relation("draft-edge", "TaskEdge", |relation| {
                relation
                    .aspect("edge.kind", "depends_on")
                    .symbolic_entity_identity("edge.source_identity", &draft)
                    .existing_entity_identity(
                        "edge.target_identity",
                        test_entity_identity("task-existing"),
                    )
            })?;
            graph.update_entity(&draft, |task| task.aspect("title.value", "Published task"))?;
            graph.delete_relation(&edge, |delete| {
                delete.touches(["edge.kind", "edge.source_identity", "edge.target_identity"])
            })?;
            Ok(())
        })
        .expect("graph composition lifecycle should execute");
    let task_rows = workspace.read(&tasks);
    let edge_rows = workspace.read(&edges);
    let inspection = workspace.inspect(&receipt).expect("receipt should inspect");
    let program = receipt
        .graph_composition_program()
        .expect("graph composition receipt should expose composition program");
    let lifecycle = receipt
        .graph_composition_lifecycle_outcomes()
        .expect("graph composition receipt should expose lifecycle outcomes");
    let composition_evidence = receipt
        .graph_composition_evidence()
        .expect("graph composition receipt should expose composition evidence");

    assert_eq!(receipt.write_receipts().len(), 4);
    assert_eq!(receipt.batch_mutation_evidence().component_count(), 4);
    assert_eq!(receipt.batch_mutation_evidence().resolved_target_count(), 4);
    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .symbolic_resolution_count(),
        3
    );
    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .symbolic_target_reference_count(),
        2
    );
    assert_eq!(receipt.graph_composition_breadth().component_count(), 4);
    assert_eq!(
        receipt
            .graph_composition_breadth()
            .symbolic_entity_declaration_count(),
        1
    );
    assert_eq!(
        receipt
            .graph_composition_breadth()
            .symbolic_relation_declaration_count(),
        1
    );
    assert_eq!(program.component_count(), 4);
    assert_eq!(
        program.steps()[2].kind(),
        ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityFollowupMutation
    );
    assert_eq!(program.steps()[2].declared_symbol(), Some("draft-task"));
    assert_eq!(
        program.steps()[3].kind(),
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement
    );
    assert_eq!(program.steps()[3].declared_symbol(), Some("draft-edge"));
    assert_eq!(lifecycle.entries().len(), 4);
    assert_eq!(
        lifecycle.entries()[0].outcome_kind(),
        ForgeQueryGraphCompositionLifecycleOutcomeKind::Created
    );
    assert_eq!(
        lifecycle.entries()[1].outcome_kind(),
        ForgeQueryGraphCompositionLifecycleOutcomeKind::Created
    );
    assert_eq!(
        lifecycle.entries()[2].outcome_kind(),
        ForgeQueryGraphCompositionLifecycleOutcomeKind::UpdatedIdentityPreserved
    );
    assert_eq!(
        lifecycle.entries()[3].outcome_kind(),
        ForgeQueryGraphCompositionLifecycleOutcomeKind::RetiredCurrentTruth
    );
    assert_eq!(
        lifecycle.counter_snapshot(),
        "created=2;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=1;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(receipt.graph_composition_resolution_map().len(), 3);
    assert_eq!(composition_evidence.symbolic_resolution_count(), 3);
    assert_eq!(composition_evidence.affected_live_view_count(), 2);
    assert_eq!(composition_evidence.affected_derived_view_count(), 0);
    assert_eq!(composition_evidence.considered_computed_view_count(), 0);
    assert_eq!(
        composition_evidence.counter_snapshot(),
        "components=4;symbolic_entities=1;symbolic_relations=1;symbolic_resolutions=3;affected_live_views=2;affected_derived_views=0;considered_computed_views=0;created=2;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=1;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(
        composition_evidence.lifecycle_counter_snapshot(),
        "created=2;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=1;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(task_rows.len(), 1);
    assert_eq!(edge_rows.len(), 0);
    assert_eq!(
        task_rows[0].external_row()["title"]["value"].as_str(),
        Some("Published task")
    );

    let support = workspace.public_authoritative_mutation_evidence_support();
    assert!(support
        .graph_composition_families()
        .iter()
        .any(|family| family == "same_batch_symbolic_entity_followup_mutation"));
    assert!(support
        .graph_composition_families()
        .iter()
        .any(|family| family == "same_batch_symbolic_relation_retirement"));

    match inspection {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            let program = inspection
                .graph_composition_program()
                .expect("inspection should expose composition program");
            let lifecycle = inspection
                .graph_composition_lifecycle_outcomes()
                .expect("inspection should expose lifecycle outcomes");
            let composition_evidence = inspection
                .graph_composition_evidence()
                .expect("inspection should expose composition evidence");
            assert_eq!(
                program.steps()[2].kind(),
                ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityFollowupMutation
            );
            assert_eq!(
                program.steps()[3].kind(),
                ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement
            );
            assert_eq!(
                lifecycle.entries()[2].outcome_kind(),
                ForgeQueryGraphCompositionLifecycleOutcomeKind::UpdatedIdentityPreserved
            );
            assert_eq!(
                lifecycle.entries()[3].outcome_kind(),
                ForgeQueryGraphCompositionLifecycleOutcomeKind::RetiredCurrentTruth
            );
            assert_eq!(inspection.component_operations().len(), 4);
            assert_eq!(inspection.batch_mutation_evidence().component_count(), 4);
            assert_eq!(
                inspection.batch_mutation_evidence().resolved_target_count(),
                4
            );
            assert_eq!(
                inspection
                    .batch_mutation_evidence()
                    .symbolic_resolution_count(),
                3
            );
            assert_eq!(
                inspection
                    .batch_mutation_evidence()
                    .symbolic_target_reference_count(),
                2
            );
            assert_eq!(inspection.graph_composition_breadth().component_count(), 4);
            assert_eq!(
                inspection
                    .graph_composition_breadth()
                    .symbolic_entity_declaration_count(),
                1
            );
            assert_eq!(
                inspection
                    .graph_composition_breadth()
                    .symbolic_relation_declaration_count(),
                1
            );
            assert_eq!(inspection.component_operations()[2].family(), "update");
            assert_eq!(inspection.component_operations()[3].family(), "delete");
            assert_eq!(inspection.graph_composition_resolution_map().len(), 3);
            assert_eq!(composition_evidence.symbolic_resolution_count(), 3);
            assert_eq!(composition_evidence.affected_live_view_count(), 2);
            assert_eq!(composition_evidence.affected_derived_view_count(), 0);
            assert_eq!(composition_evidence.considered_computed_view_count(), 0);
            assert_eq!(
                composition_evidence.counter_snapshot(),
                "components=4;symbolic_entities=1;symbolic_relations=1;symbolic_resolutions=3;affected_live_views=2;affected_derived_views=0;considered_computed_views=0;created=2;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=1;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
            );
            assert_eq!(
                composition_evidence.lifecycle_counter_snapshot(),
                "created=2;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=1;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
            );
            assert_eq!(
                inspection.component_operations()[2]
                    .symbolic_target_reference_evidence()
                    .expect("symbolic entity update should retain target evidence")
                    .symbol()
                    .as_str(),
                "draft-task"
            );
            assert_eq!(
                inspection.component_operations()[3]
                    .symbolic_target_reference_evidence()
                    .expect("symbolic relation delete should retain target evidence")
                    .symbol()
                    .as_str(),
                "draft-edge"
            );
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}
