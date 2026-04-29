use super::super::support::*;

#[test]
fn mixed_batch_symbolic_and_existing_targets_preserve_distinct_evidence() {
    let mut workspace = task_runtime()
        .workspace("tasks.mixed-target-batch")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.mixed-target-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-mixed-target-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-existing")
                .aspect("title.value", "Existing")
        })
        .expect("seed insert should execute");
    let existing_binding = ForgeQueryExistingTruthTargetBinding::direct_entity(
        "authority:task-existing",
        seed.deltas()[0].entity_identity.clone(),
    )
    .expect("existing binding should build")
    .in_target_collection("Task")
    .expect("existing binding collection should build");
    let symbolic = ForgeQuerySymbolicTargetReference::new("draft-task")
        .expect("symbolic reference should build")
        .in_target_collection("Task")
        .expect("symbolic reference collection should build");

    let receipt = workspace
        .batch(|batch| {
            batch
                .insert_symbolic("draft-task", "Task", |task| {
                    task.aspect("identity.id", "task-draft")
                        .aspect("title.value", "Draft")
                })
                .update_symbolic(symbolic.clone(), |task| {
                    task.aspect("title.value", "Draft renamed")
                })
                .update_existing(existing_binding, |task| {
                    task.aspect("title.value", "Existing renamed")
                })
        })
        .expect("mixed target batch should execute");
    let inspection = workspace.inspect(&receipt).expect("batch should inspect");

    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .existing_truth_binding_count(),
        1
    );
    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .symbolic_target_reference_count(),
        1
    );

    match inspection {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            assert_eq!(
                inspection
                    .batch_mutation_evidence()
                    .symbolic_target_reference_count(),
                1
            );
            assert_eq!(
                inspection.component_operations()[1]
                    .symbolic_target_reference_evidence()
                    .expect("symbolic component should retain symbolic evidence")
                    .symbol(),
                "draft-task"
            );
            assert_eq!(
                inspection.component_operations()[2]
                    .existing_truth_binding_evidence()
                    .expect("existing component should retain existing-target evidence")
                    .authoritative_identity(),
                "authority:task-existing"
            );
        }
        other => panic!("expected batch inspection, got {other:?}"),
    }
}

#[test]
fn symbolic_target_reference_denies_missing_same_batch_target() {
    let mut workspace = task_runtime()
        .workspace("tasks.symbolic-denial")
        .expect("task runtime should open a named workspace");
    let symbolic = ForgeQuerySymbolicTargetReference::new("missing-task")
        .expect("symbolic reference should build")
        .in_target_collection("Task")
        .expect("symbolic reference collection should build");

    let error = workspace
        .batch(|batch| {
            batch.update_symbolic(symbolic, |task| task.aspect("title.value", "No target"))
        })
        .expect_err("missing same-batch symbolic target should deny");

    match error {
        ForgeQueryRuntimeError::MutationTargetReferenceDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQuerySymbolicTargetReferenceDenialKind::UnresolvedSameBatchTarget
            );
            assert!(!denial.denial_digest().is_empty());
        }
        other => panic!("expected symbolic target denial, got {other:?}"),
    }
}

#[test]
fn preview_batch_symbolic_target_preserves_symbolic_evidence() {
    let runtime = task_runtime();
    let mut workspace = runtime
        .workspace("tasks.preview-symbolic")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.preview-symbolic-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-preview-symbolic-table")
        })
        .expect("live view should declare");

    let mut preview = workspace
        .preview("symbolic-preview")
        .expect("preview should open");
    let receipt = preview
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
                        .expect("symbolic reference collection should build"),
                    |task| task.aspect("title.value", "Draft preview renamed"),
                )
        })
        .expect("preview batch should execute");

    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .symbolic_target_reference_count(),
        1
    );
    assert_eq!(
        receipt.write_receipts()[1]
            .symbolic_target_reference_evidence()
            .expect("preview component should retain symbolic evidence")
            .symbol(),
        "draft-task"
    );
}
