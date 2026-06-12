use super::super::support::*;

fn task_edge_runtime() -> ForgeQueryRuntime {
    stateful_bridge_task_edge_runtime()
}

#[test]
fn compose_graph_preserves_symbolic_resolution_and_mixed_edge_meaning() {
    let mut workspace = task_edge_runtime()
        .workspace("tasks.graph-composition")
        .expect("runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.graph-composition-tasks", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-graph-composition-tasks")
        })
        .expect("task live view should declare");
    let edges: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.graph-composition-edges", |q| {
            q.from("TaskEdge")
                .select(["edge.kind", "edge.source_identity", "edge.target_identity"])
                .order_by("edge.kind")
                .schema_basis("tasks-graph-composition-edges")
        })
        .expect("edge live view should declare");

    let receipt = workspace
        .compose_graph(|graph| {
            let draft = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect("identity.id", "task-draft")
                    .aspect("title.value", "Draft task")
            })?;
            graph.insert_relation("TaskEdge", |edge| {
                edge.aspect("edge.kind", "depends_on")
                    .symbolic_entity_identity("edge.source_identity", &draft)
                    .existing_entity_identity(
                        "edge.target_identity",
                        test_entity_identity("task-existing"),
                    )
            })?;
            Ok(())
        })
        .expect("graph composition should execute");
    let draft_identity = receipt.write_receipts()[0].deltas()[0]
        .entity_identity
        .clone();
    let edge_rows = workspace.read(&edges);
    let inspection = workspace.inspect(&receipt).expect("receipt should inspect");

    assert_eq!(receipt.write_receipts().len(), 2);
    assert_eq!(receipt.batch_mutation_evidence().component_count(), 2);
    assert_eq!(receipt.batch_mutation_evidence().resolved_target_count(), 2);
    assert_eq!(receipt.graph_composition_breadth().component_count(), 2);
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
    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .symbolic_resolution_count(),
        1
    );
    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .symbolic_target_reference_count(),
        0
    );
    let program = receipt
        .graph_composition_program()
        .expect("graph composition receipt should expose composition program");
    let lifecycle = receipt
        .graph_composition_lifecycle_outcomes()
        .expect("graph composition receipt should expose lifecycle outcomes");
    let resolution_map = receipt.graph_composition_resolution_map();
    let composition_evidence = receipt
        .graph_composition_evidence()
        .expect("graph composition receipt should expose composition evidence");
    assert_eq!(program.component_count(), 2);
    assert_eq!(
        program.steps()[0].kind(),
        ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration
    );
    assert_eq!(program.steps()[0].declared_symbol(), Some("draft-task"));
    assert_eq!(program.steps()[0].declared_collection(), "Task");
    assert_eq!(
        program.steps()[1].kind(),
        ForgeQueryGraphCompositionProgramStepKind::RelationDeclaration
    );
    assert_eq!(program.steps()[1].declared_symbol(), None);
    assert_eq!(program.steps()[1].declared_collection(), "TaskEdge");
    assert_eq!(lifecycle.entries().len(), 2);
    assert_eq!(
        lifecycle.entries()[0].outcome_kind(),
        ForgeQueryGraphCompositionLifecycleOutcomeKind::Created
    );
    assert_eq!(
        lifecycle.entries()[1].outcome_kind(),
        ForgeQueryGraphCompositionLifecycleOutcomeKind::Created
    );
    assert_eq!(
        lifecycle.counter_snapshot(),
        "created=2;updated_identity_preserved=0;retargeted_identity_preserved=0;retired_current_truth=0;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(resolution_map.len(), 1);
    assert_eq!(resolution_map.entries()[0].component_index(), 1);
    assert_eq!(
        resolution_map.entries()[0].aspect_path(),
        Some("edge.source_identity")
    );
    assert_eq!(resolution_map.entries()[0].symbol().as_str(), "draft-task");
    assert_eq!(
        resolution_map.entries()[0].resolved_entity_identity(),
        &draft_identity
    );
    assert_eq!(composition_evidence.symbolic_resolution_count(), 1);
    assert_eq!(composition_evidence.affected_live_view_count(), 2);
    assert_eq!(composition_evidence.affected_derived_view_count(), 0);
    assert_eq!(composition_evidence.considered_computed_view_count(), 0);
    assert!(composition_evidence
        .counter_snapshot()
        .contains("symbolic_entities=1"));
    assert_eq!(
        composition_evidence.lifecycle_counter_snapshot(),
        "created=2;updated_identity_preserved=0;retargeted_identity_preserved=0;retired_current_truth=0;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert!(
        !composition_evidence.graph_composition_digest().is_empty()
            && !composition_evidence
                .graph_symbolic_resolution_digest()
                .is_empty()
    );
    assert_eq!(edge_rows.len(), 1);
    assert_eq!(
        edge_rows[0].external_row()["edge"]["source_identity"].as_str(),
        Some(draft_identity.evidence_identity().as_str())
    );
    assert_eq!(
        edge_rows[0].external_row()["edge"]["target_identity"].as_str(),
        Some(
            test_entity_identity("task-existing")
                .evidence_identity()
                .as_str()
        )
    );

    let support = workspace.public_authoritative_mutation_evidence_support();
    assert!(support
        .symbolic_aspect_reference_families()
        .iter()
        .any(|family| family == "same_batch_declared_entity_identity"));
    assert!(support
        .graph_composition_families()
        .iter()
        .any(|family| family == "mixed_existing_and_symbolic_entity_identity_edges"));
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
            assert_eq!(program.component_count(), 2);
            assert_eq!(
                program.steps()[0].kind(),
                ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration
            );
            assert_eq!(program.steps()[0].declared_symbol(), Some("draft-task"));
            assert_eq!(
                program.steps()[1].kind(),
                ForgeQueryGraphCompositionProgramStepKind::RelationDeclaration
            );
            assert_eq!(
                lifecycle.entries()[0].outcome_kind(),
                ForgeQueryGraphCompositionLifecycleOutcomeKind::Created
            );
            assert_eq!(
                lifecycle.entries()[1].outcome_kind(),
                ForgeQueryGraphCompositionLifecycleOutcomeKind::Created
            );
            assert_eq!(inspection.batch_mutation_evidence().component_count(), 2);
            assert_eq!(inspection.graph_composition_breadth().component_count(), 2);
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
            assert_eq!(
                inspection
                    .batch_mutation_evidence()
                    .symbolic_resolution_count(),
                1
            );
            assert_eq!(
                inspection
                    .batch_mutation_evidence()
                    .symbolic_target_reference_count(),
                0
            );
            assert_eq!(inspection.component_operations().len(), 2);
            assert_eq!(inspection.component_operations()[0].family(), "insert");
            assert_eq!(inspection.component_operations()[1].family(), "insert");
            assert_eq!(composition_evidence.symbolic_resolution_count(), 1);
            assert!(composition_evidence
                .counter_snapshot()
                .contains("affected_live_views=2"));
            assert_eq!(
                composition_evidence.lifecycle_counter_snapshot(),
                "created=2;updated_identity_preserved=0;retargeted_identity_preserved=0;retired_current_truth=0;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
            );
            assert_eq!(
                inspection
                    .graph_composition_resolution_map()
                    .entries()
                    .len(),
                1
            );
            assert_eq!(
                inspection.component_operations()[1].declared_collection(),
                Some("TaskEdge")
            );
            assert_eq!(
                inspection.component_operations()[1].target_collection(),
                Some("TaskEdge")
            );
            assert_eq!(
                inspection.component_operations()[1].touched_aspect_paths(),
                &[
                    "edge.kind".to_string(),
                    "edge.source_identity".to_string(),
                    "edge.target_identity".to_string()
                ]
            );
            assert_eq!(
                inspection.component_operations()[1]
                    .symbolic_aspect_resolution_evidence()
                    .len(),
                1
            );
            assert_eq!(
                inspection.component_operations()[1].symbolic_aspect_resolution_evidence()[0]
                    .aspect_path(),
                "edge.source_identity"
            );
            assert!(inspection.component_operations()[1]
                .existing_truth_binding_evidence()
                .is_none());
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}
