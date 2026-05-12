use super::super::support::*;

fn task_relation_runtime() -> ForgeQueryRuntime {
    stateful_bridge_task_relation_runtime()
}

#[test]
fn compose_graph_supports_mixed_symbolic_create_and_existing_target_lifecycle() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.graph-composition-mixed-existing")
        .expect("runtime should open a named workspace");
    let tasks: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.graph-composition-mixed-existing-tasks", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-graph-composition-mixed-existing-tasks")
        })
        .expect("task live view should declare");
    let relations: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.graph-composition-mixed-existing-relations", |q| {
            q.from("TaskRelation")
                .select(["identity.id", "kind.value", "status.value"])
                .order_by("identity.id")
                .schema_basis("tasks-graph-composition-mixed-existing-relations")
        })
        .expect("relation live view should declare");

    let update_seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect("identity.id", "rel-update")
                .aspect("kind.value", "depends_on")
                .aspect("status.value", "open")
        })
        .expect("update seed should execute");
    let delete_seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect("identity.id", "rel-delete")
                .aspect("kind.value", "blocks")
                .aspect("status.value", "stale")
        })
        .expect("delete seed should execute");
    let update_binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                "authority:rel-update",
                update_seed.deltas()[0].entity_identity.clone(),
            )
            .expect("update relation target should build")
            .in_target_collection("TaskRelation")
            .expect("update relation collection should build"),
        )
        .expect("update binding should build");
    let delete_binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                "authority:rel-delete",
                delete_seed.deltas()[0].entity_identity.clone(),
            )
            .expect("delete relation target should build")
            .in_target_collection("TaskRelation")
            .expect("delete relation collection should build"),
        )
        .expect("delete binding should build");

    let receipt = workspace
        .compose_graph(|graph| {
            let _ = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect("identity.id", "task-mixed-existing")
                    .aspect("title.value", "Mixed existing task")
            })?;
            graph.update_existing(update_binding, |relation| {
                relation.aspect("status.value", "closed")
            })?;
            graph.delete_existing(delete_binding, |delete| {
                delete.touches(["kind.value", "status.value"])
            })?;
            Ok(())
        })
        .expect("mixed symbolic and existing lifecycle program should execute");

    let task_rows = workspace.read(&tasks);
    let relation_rows = workspace.read(&relations);
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
        0
    );
    assert_eq!(program.component_count(), 3);
    assert_eq!(
        program.steps()[1].kind(),
        ForgeQueryGraphCompositionProgramStepKind::ExistingTargetFollowupMutation
    );
    assert_eq!(program.steps()[1].declared_collection(), "TaskRelation");
    assert_eq!(
        program.steps()[2].kind(),
        ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetirement
    );
    assert_eq!(program.steps()[2].declared_collection(), "TaskRelation");
    assert_eq!(lifecycle.entries().len(), 3);
    assert_eq!(
        lifecycle.entries()[0].outcome_kind(),
        ForgeQueryGraphCompositionLifecycleOutcomeKind::Created
    );
    assert_eq!(
        lifecycle.entries()[1].outcome_kind(),
        ForgeQueryGraphCompositionLifecycleOutcomeKind::UpdatedIdentityPreserved
    );
    assert_eq!(
        lifecycle.entries()[2].outcome_kind(),
        ForgeQueryGraphCompositionLifecycleOutcomeKind::RetiredCurrentTruth
    );
    assert_eq!(
        lifecycle.counter_snapshot(),
        "created=1;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=1;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(composition_evidence.symbolic_resolution_count(), 0);
    assert_eq!(
        composition_evidence.counter_snapshot(),
        "components=3;symbolic_entities=1;symbolic_relations=0;symbolic_resolutions=0;affected_live_views=2;affected_derived_views=0;considered_computed_views=0;created=1;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=1;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(
        composition_evidence.lifecycle_counter_snapshot(),
        "created=1;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=1;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(task_rows.len(), 1);
    assert_eq!(relation_rows.len(), 1);
    assert_eq!(
        relation_rows[0].payload["identity"]["id"].as_str(),
        Some("rel-update")
    );
    assert_eq!(
        relation_rows[0].payload["status"]["value"].as_str(),
        Some("closed")
    );

    let support = workspace.public_authoritative_mutation_evidence_support();
    assert!(support
        .graph_composition_families()
        .iter()
        .any(|family| family == "mixed_existing_target_followup_mutation"));
    assert!(support
        .graph_composition_families()
        .iter()
        .any(|family| family == "mixed_existing_target_retirement"));

    match inspection {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            let program = inspection
                .graph_composition_program()
                .expect("inspection should expose composition program");
            let lifecycle = inspection
                .graph_composition_lifecycle_outcomes()
                .expect("inspection should expose lifecycle outcomes");
            assert_eq!(
                program.steps()[1].kind(),
                ForgeQueryGraphCompositionProgramStepKind::ExistingTargetFollowupMutation
            );
            assert_eq!(
                program.steps()[2].kind(),
                ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetirement
            );
            assert_eq!(
                lifecycle.entries()[1].outcome_kind(),
                ForgeQueryGraphCompositionLifecycleOutcomeKind::UpdatedIdentityPreserved
            );
            assert_eq!(
                lifecycle.entries()[2].outcome_kind(),
                ForgeQueryGraphCompositionLifecycleOutcomeKind::RetiredCurrentTruth
            );
            assert_eq!(inspection.component_operations()[1].family(), "update");
            assert_eq!(inspection.component_operations()[2].family(), "delete");
            assert_eq!(
                inspection.component_operations()[1]
                    .existing_truth_binding_evidence()
                    .expect("existing-target update should retain binding evidence")
                    .family(),
                ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity
            );
            assert_eq!(
                inspection.component_operations()[2]
                    .existing_truth_binding_evidence()
                    .expect("existing-target delete should retain binding evidence")
                    .family(),
                ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity
            );
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}
