use super::super::support::*;

#[test]
fn mixed_batch_preserves_all_authority_aggregate_digests() {
    let mut workspace = task_runtime()
        .workspace("tasks.mixed-authority-batch")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.mixed-authority-batch-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-mixed-authority-batch-table")
        })
        .expect("live view should declare");

    let continuity_seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-continuity")
                .aspect("title.value", "Continuity seed")
        })
        .expect("continuity seed insert should execute");
    let continuity_binding = ForgeQueryExistingTruthTargetBinding::direct_entity(
        "authority:task-continuity",
        continuity_seed.deltas()[0].entity_identity.clone(),
    )
    .expect("continuity binding should build")
    .in_target_collection("Task")
    .expect("continuity binding collection should build");
    let continuity_binding_digest = continuity_binding.binding_digest();

    let delete_seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-delete")
                .aspect("title.value", "Delete seed")
        })
        .expect("delete seed insert should execute");
    let delete_binding = ForgeQueryExistingTruthTargetBinding::direct_entity(
        "authority:task-delete",
        delete_seed.deltas()[0].entity_identity.clone(),
    )
    .expect("delete binding should build")
    .in_target_collection("Task")
    .expect("delete binding collection should build");

    let receipt = workspace
        .batch(|batch| {
            batch
                .insert_symbolic("draft-task", "Task", |task| {
                    task.aspect("identity.id", "task-draft")
                        .aspect("title.value", "Draft")
                })
                .update_symbolic(
                    ForgeQuerySymbolicTargetReference::new("draft-task")
                        .expect("symbolic reference should build")
                        .in_target_collection("Task")
                        .expect("symbolic collection should build"),
                    |task| {
                        task.naming_attach_new_target("persistent-name:draft")
                            .aspect("title.value", "Draft named")
                    },
                )
                .update_existing(continuity_binding, |task| {
                    task.continuity_rebind_merge_successor(
                        "authority:task-continuity",
                        "authority:task-continuity-merged",
                    )
                    .aspect("title.value", "Continuity merged")
                })
                .delete_existing_with(delete_binding, |delete| {
                    delete
                        .touch("title.value")
                        .naming_remove("persistent-name:delete", "authority:task-delete")
                })
        })
        .expect("mixed authority batch should execute");

    let batch_evidence = receipt.batch_mutation_evidence();
    assert_eq!(batch_evidence.component_count(), 4);
    assert_eq!(batch_evidence.existing_truth_binding_count(), 2);
    assert_eq!(batch_evidence.symbolic_target_reference_count(), 1);
    assert_eq!(batch_evidence.naming_mutation_count(), 2);
    assert_eq!(batch_evidence.continuity_mutation_count(), 1);

    let existing_digest = batch_evidence
        .aggregate_existing_truth_binding_digest()
        .expect("mixed batch should retain existing-truth aggregate digest")
        .to_string();
    let symbolic_digest = batch_evidence
        .aggregate_symbolic_target_reference_digest()
        .expect("mixed batch should retain symbolic aggregate digest")
        .to_string();
    let naming_digest = batch_evidence
        .aggregate_naming_mutation_digest()
        .expect("mixed batch should retain naming aggregate digest")
        .to_string();
    let continuity_digest = batch_evidence
        .aggregate_continuity_mutation_digest()
        .expect("mixed batch should retain continuity aggregate digest")
        .to_string();

    let inspection = workspace.inspect(&receipt).expect("batch should inspect");

    match inspection {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            let batch_evidence = inspection.batch_mutation_evidence();
            assert_eq!(
                batch_evidence.aggregate_existing_truth_binding_digest(),
                Some(existing_digest.as_str())
            );
            assert_eq!(
                batch_evidence.aggregate_symbolic_target_reference_digest(),
                Some(symbolic_digest.as_str())
            );
            assert_eq!(
                batch_evidence.aggregate_naming_mutation_digest(),
                Some(naming_digest.as_str())
            );
            assert_eq!(
                batch_evidence.aggregate_continuity_mutation_digest(),
                Some(continuity_digest.as_str())
            );
            assert_eq!(
                inspection.component_operations()[1]
                    .naming_mutation_evidence()
                    .expect("symbolic naming component should retain naming evidence")
                    .outcome(),
                ForgeQueryNamingMutationOutcome::AttachedToNewTarget
            );
            assert_eq!(
                inspection.component_operations()[2]
                    .continuity_mutation_evidence()
                    .expect("continuity component should retain continuity evidence")
                    .basis_binding_digest(),
                Some(continuity_binding_digest.as_str())
            );
            assert_eq!(
                inspection.component_operations()[3]
                    .naming_mutation_evidence()
                    .expect("delete naming component should retain naming evidence")
                    .outcome(),
                ForgeQueryNamingMutationOutcome::Removed
            );
        }
        other => panic!("expected batch write inspection, got {other:?}"),
    }
}
