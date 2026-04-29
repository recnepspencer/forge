use super::super::support::*;

#[test]
fn update_existing_preserves_authoritative_binding_evidence() {
    let mut workspace = task_runtime()
        .workspace("tasks.update-existing")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.update-existing-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-update-existing-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Before existing update")
        })
        .expect("seed insert should execute");
    let binding = ForgeQueryExistingTruthTargetBinding::direct_entity(
        "authority:task-1",
        seed.deltas()[0].entity_identity.clone(),
    )
    .expect("binding should build")
    .in_target_collection("Task")
    .expect("binding collection should build");

    let receipt = workspace
        .update_existing(binding, |task| {
            task.aspect("title.value", "After existing update")
        })
        .expect("existing-target update should execute");
    let inspection = workspace
        .inspect(&receipt)
        .expect("existing-target receipt should inspect");

    let evidence = receipt
        .existing_truth_binding_evidence()
        .expect("receipt should retain existing-truth evidence");
    assert_eq!(
        evidence.family(),
        ForgeQueryExistingTruthBindingFamily::DirectEntityIdentity
    );
    assert_eq!(
        evidence.outcome(),
        ForgeQueryExistingTruthBindingOutcome::ExistingAuthoritativeTarget
    );
    assert_eq!(evidence.authoritative_identity(), "authority:task-1");
    assert_eq!(
        evidence.resolved_entity_identity(),
        seed.deltas()[0].entity_identity
    );
    assert_eq!(evidence.target_collection(), Some("Task"));
    assert!(!evidence.binding_digest().is_empty());

    match inspection {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            let evidence = inspection
                .existing_truth_binding_evidence()
                .expect("inspection should retain existing-truth evidence");
            assert_eq!(evidence.authoritative_identity(), "authority:task-1");
            assert_eq!(
                evidence.resolved_entity_identity(),
                seed.deltas()[0].entity_identity
            );
            assert_eq!(evidence.target_collection(), Some("Task"));
            assert_eq!(
                evidence.binding_digest(),
                receipt
                    .existing_truth_binding_evidence()
                    .expect("receipt should retain existing-truth evidence")
                    .binding_digest()
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
}

#[test]
fn update_existing_denies_missing_target_typed_and_early() {
    let mut workspace = task_runtime()
        .workspace("tasks.update-existing-denial")
        .expect("task runtime should open a named workspace");
    let binding =
        ForgeQueryExistingTruthTargetBinding::direct_entity("authority:missing", "task:missing")
            .expect("binding should build")
            .in_target_collection("Task")
            .expect("binding collection should build");

    let error = workspace
        .update_existing(binding, |task| task.aspect("title.value", "No target"))
        .expect_err("missing existing target should deny early");

    match error {
        ForgeQueryRuntimeError::MutationBindingDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthBindingDenialKind::ResolvedTargetMissing
            );
            assert!(!denial.denial_digest().is_empty());
        }
        other => panic!("expected typed mutation binding denial, got {other:?}"),
    }
}

#[test]
fn batch_existing_targets_preserve_component_and_aggregate_binding_evidence() {
    let mut workspace = task_runtime()
        .workspace("tasks.batch-existing")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.batch-existing-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-batch-existing-table")
        })
        .expect("live view should declare");

    let seed_one = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "First")
        })
        .expect("first seed should execute");
    let seed_two = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-2")
                .aspect("title.value", "Second")
        })
        .expect("second seed should execute");

    let binding_one = ForgeQueryExistingTruthTargetBinding::direct_entity(
        "authority:task-1",
        seed_one.deltas()[0].entity_identity.clone(),
    )
    .expect("binding one should build")
    .in_target_collection("Task")
    .expect("binding one collection should build");
    let binding_two = ForgeQueryExistingTruthTargetBinding::direct_entity(
        "authority:task-2",
        seed_two.deltas()[0].entity_identity.clone(),
    )
    .expect("binding two should build")
    .in_target_collection("Task")
    .expect("binding two collection should build");

    let receipt = workspace
        .batch(|batch| {
            batch
                .update_existing(binding_one, |task| {
                    task.aspect("title.value", "First renamed")
                })
                .delete_existing_with(binding_two, |delete| delete.touch("title.value"))
        })
        .expect("existing-target batch should execute");
    let inspection = workspace
        .inspect(&receipt)
        .expect("batch receipt should inspect");

    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .existing_truth_binding_count(),
        2
    );
    assert!(receipt
        .batch_mutation_evidence()
        .aggregate_existing_truth_binding_digest()
        .is_some());

    match inspection {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            assert_eq!(
                inspection
                    .batch_mutation_evidence()
                    .existing_truth_binding_count(),
                2
            );
            assert_eq!(
                inspection.component_operations()[0]
                    .existing_truth_binding_evidence()
                    .expect("first component should retain existing binding")
                    .authoritative_identity(),
                "authority:task-1"
            );
            assert_eq!(
                inspection.component_operations()[1]
                    .existing_truth_binding_evidence()
                    .expect("second component should retain existing binding")
                    .authoritative_identity(),
                "authority:task-2"
            );
            assert_eq!(
                inspection
                    .batch_mutation_evidence()
                    .aggregate_existing_truth_binding_digest(),
                receipt
                    .batch_mutation_evidence()
                    .aggregate_existing_truth_binding_digest()
            );
        }
        other => panic!("expected batch write receipt inspection, got {other:?}"),
    }
}

#[test]
fn mixed_existing_and_symbolic_batch_preserves_aggregate_session_digests() {
    let mut workspace = task_runtime()
        .workspace("tasks.batch-existing-symbolic")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.batch-existing-symbolic-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-batch-existing-symbolic-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-existing")
                .aspect("title.value", "Existing")
        })
        .expect("seed should execute");
    let binding = ForgeQueryExistingTruthTargetBinding::direct_entity(
        "authority:task-existing",
        seed.deltas()[0].entity_identity.clone(),
    )
    .expect("binding should build")
    .in_target_collection("Task")
    .expect("binding collection should build");

    let receipt = workspace
        .batch(|batch| {
            batch
                .insert_symbolic("draft-task", "Task", |task| {
                    task.aspect("identity.id", "task-draft")
                        .aspect("title.value", "Draft")
                })
                .update_existing(binding, |task| {
                    task.aspect("title.value", "Existing renamed")
                })
                .update_symbolic(
                    ForgeQuerySymbolicTargetReference::new("draft-task")
                        .expect("symbolic reference should build")
                        .in_target_collection("Task")
                        .expect("symbolic collection should build"),
                    |task| task.aspect("title.value", "Draft renamed"),
                )
        })
        .expect("mixed existing/symbolic batch should execute");

    let batch_evidence = receipt.batch_mutation_evidence();
    assert_eq!(batch_evidence.existing_truth_binding_count(), 1);
    assert_eq!(batch_evidence.symbolic_target_reference_count(), 1);
    assert!(batch_evidence
        .aggregate_existing_truth_binding_digest()
        .is_some());
    assert!(batch_evidence
        .aggregate_symbolic_target_reference_digest()
        .is_some());
}
