use super::super::support::*;

fn task_edge_runtime() -> ForgeQueryRuntime {
    stateful_bridge_task_edge_runtime()
}

#[test]
fn compose_graph_supports_symbolic_relation_followup_mutation() {
    let mut workspace = task_edge_runtime()
        .workspace("tasks.graph-composition-relation-followup")
        .expect("runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.graph-composition-relation-followup-tasks", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-graph-composition-relation-followup-tasks")
        })
        .expect("task live view should declare");
    let edges: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.graph-composition-relation-followup-edges", |q| {
            q.from("TaskEdge")
                .select(["edge.kind", "edge.source_identity", "edge.target_identity"])
                .order_by("edge.kind")
                .schema_basis("tasks-graph-composition-relation-followup-edges")
        })
        .expect("edge live view should declare");

    let receipt = workspace
        .compose_graph(|graph| {
            let draft = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect("identity.id", "task-draft")
                    .aspect("title.value", "Draft task")
            })?;
            let edge = graph.insert_symbolic_relation("draft-edge", "TaskEdge", |relation| {
                relation
                    .aspect("edge.kind", "depends_on")
                    .symbolic_entity_identity("edge.source_identity", &draft)
                    .existing_entity_identity("edge.target_identity", "task-existing")
            })?;
            graph.update_relation(&edge, |relation| relation.aspect("edge.kind", "blocks"))?;
            Ok(())
        })
        .expect("graph composition with relation followup should execute");
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

    assert_eq!(receipt.write_receipts().len(), 3);
    assert_eq!(receipt.batch_mutation_evidence().component_count(), 3);
    assert_eq!(receipt.batch_mutation_evidence().resolved_target_count(), 3);
    assert_eq!(receipt.graph_composition_breadth().component_count(), 3);
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
    assert_eq!(program.component_count(), 3);
    assert_eq!(
        program.steps()[0].kind(),
        ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration
    );
    assert_eq!(
        program.steps()[1].kind(),
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationDeclaration
    );
    assert_eq!(program.steps()[1].declared_symbol(), Some("draft-edge"));
    assert_eq!(
        program.steps()[2].kind(),
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationFollowupMutation
    );
    assert_eq!(program.steps()[2].declared_symbol(), Some("draft-edge"));
    assert_eq!(lifecycle.entries().len(), 3);
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
        lifecycle.counter_snapshot(),
        "created=2;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=0;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(composition_evidence.symbolic_resolution_count(), 2);
    assert_eq!(
        composition_evidence.counter_snapshot(),
        "components=3;symbolic_entities=1;symbolic_relations=1;symbolic_resolutions=2;affected_live_views=2;affected_derived_views=0;considered_computed_views=0;created=2;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=0;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(
        composition_evidence.lifecycle_counter_snapshot(),
        "created=2;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=0;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .symbolic_resolution_count(),
        2
    );
    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .symbolic_target_reference_count(),
        1
    );
    assert_eq!(receipt.graph_composition_resolution_map().len(), 2);
    assert_eq!(edge_rows.len(), 1);
    assert_eq!(
        edge_rows[0].external_row()["edge"]["kind"].as_str(),
        Some("blocks")
    );

    let support = workspace.public_authoritative_mutation_evidence_support();
    assert!(support
        .graph_composition_families()
        .iter()
        .any(|family| family == "same_batch_symbolic_relation_followup_mutation"));
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
            assert_eq!(program.component_count(), 3);
            assert_eq!(
                program.steps()[1].kind(),
                ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationDeclaration
            );
            assert_eq!(
                program.steps()[2].kind(),
                ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationFollowupMutation
            );
            assert_eq!(
                lifecycle.entries()[2].outcome_kind(),
                ForgeQueryGraphCompositionLifecycleOutcomeKind::UpdatedIdentityPreserved
            );
            assert_eq!(inspection.batch_mutation_evidence().component_count(), 3);
            assert_eq!(inspection.graph_composition_breadth().component_count(), 3);
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
                2
            );
            assert_eq!(
                inspection
                    .batch_mutation_evidence()
                    .symbolic_target_reference_count(),
                1
            );
            assert_eq!(inspection.component_operations().len(), 3);
            assert_eq!(inspection.component_operations()[0].family(), "insert");
            assert_eq!(inspection.component_operations()[1].family(), "insert");
            assert_eq!(inspection.component_operations()[2].family(), "update");
            assert_eq!(composition_evidence.symbolic_resolution_count(), 2);
            assert_eq!(
                composition_evidence.counter_snapshot(),
                "components=3;symbolic_entities=1;symbolic_relations=1;symbolic_resolutions=2;affected_live_views=2;affected_derived_views=0;considered_computed_views=0;created=2;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=0;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
            );
            assert_eq!(
                composition_evidence.lifecycle_counter_snapshot(),
                "created=2;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=0;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
            );
            assert_eq!(
                inspection.component_operations()[1]
                    .symbolic_aspect_resolution_evidence()
                    .len(),
                1
            );
            assert_eq!(
                inspection.component_operations()[2]
                    .symbolic_target_reference_evidence()
                    .expect("relation followup should retain symbolic relation evidence")
                    .symbol(),
                "draft-edge"
            );
            assert_eq!(
                inspection
                    .graph_composition_resolution_map()
                    .entries()
                    .len(),
                2
            );
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}
