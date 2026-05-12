use super::super::support::*;

#[test]
fn delete_existing_verified_preserves_backend_verified_assertion_evidence_on_delete_receipt() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.delete-existing-verified")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.delete-existing-verified-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value", "status.value"])
                .order_by("title.value")
                .schema_basis("tasks-delete-existing-verified-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Seed title")
                .aspect("status.value", "open")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                "authority:task-1",
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let receipt = workspace
        .delete_existing_verified(
            binding.clone(),
            |task| task.aspect("status.value", "open"),
            |delete| delete.touch("status.value").touch("title.value"),
        )
        .expect("backend-verified delete should execute");

    assert_eq!(receipt.mutation_family(), ForgeQueryMutationFamily::Delete);
    assert_eq!(receipt.target_collection(), Some("Task"));
    assert_eq!(
        receipt
            .existing_truth_binding_evidence()
            .expect("receipt should retain existing binding evidence")
            .family(),
        ForgeQueryExistingTruthBindingFamily::DirectEntityIdentity
    );
    let assertion = receipt
        .existing_truth_assertion_evidence()
        .expect("verified delete should retain assertion evidence");
    assert_eq!(
        assertion.mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    assert_eq!(assertion.asserted_aspect_count(), 1);
    assert_eq!(
        receipt.deltas()[0].aspect_paths,
        vec!["status.value".to_string(), "title.value".to_string()]
    );
    assert!(receipt.declared_aspect_value_digest().is_some());

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            assert_eq!(
                inspection
                    .existing_truth_assertion_evidence()
                    .expect("inspection should retain verified assertion evidence")
                    .mode(),
                ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
            );
            assert_eq!(
                inspection
                    .existing_truth_assertion_evidence()
                    .expect("inspection should retain verified assertion evidence")
                    .verification_digest(),
                assertion.verification_digest()
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }

    let error = workspace
        .probe_existing(binding, ["status.value"])
        .expect_err("deleted target should no longer probe");
    match error {
        ForgeQueryRuntimeError::MutationBindingDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthBindingDenialKind::ResolvedTargetMissing
            );
        }
        other => panic!("expected missing-binding denial, got {other:?}"),
    }
}

#[test]
fn delete_existing_verified_denies_mismatch_typed_and_leaves_truth_unchanged() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.delete-existing-verified-mismatch")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.delete-existing-verified-mismatch-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value", "status.value"])
                .order_by("title.value")
                .schema_basis("tasks-delete-existing-verified-mismatch-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Seed title")
                .aspect("status.value", "open")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                "authority:task-1",
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = workspace
        .delete_existing_verified(
            binding.clone(),
            |task| task.aspect("status.value", "closed"),
            |delete| delete.touch("status.value"),
        )
        .expect_err("mismatched verified delete should deny");

    match error {
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch
            );
            assert_eq!(denial.asserted_aspect_path(), Some("status.value"));
            assert_eq!(denial.expected_value_json(), Some("\"closed\""));
            assert_eq!(denial.found_value_json(), Some("\"open\""));
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }

    let probe = workspace
        .probe_existing(binding, ["status.value"])
        .expect("probe should still succeed after denied delete");
    assert_eq!(
        probe
            .field("status.value")
            .expect("status field should remain present")
            .value_json(),
        "\"open\""
    );
}

#[test]
fn batch_delete_existing_verified_preserves_aggregate_assertion_digest() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.batch-delete-existing-verified")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.batch-delete-existing-verified-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value", "status.value"])
                .order_by("title.value")
                .schema_basis("tasks-batch-delete-existing-verified-table")
        })
        .expect("live view should declare");

    let seed_one = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "First")
                .aspect("status.value", "open")
        })
        .expect("first seed should execute");
    let seed_two = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-2")
                .aspect("title.value", "Second")
        })
        .expect("second seed should execute");

    let binding_one = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                "authority:task-1",
                seed_one.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding one should build");
    let binding_two = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                "authority:task-2",
                seed_two.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding two should build");

    let receipt = workspace
        .batch(|batch| {
            batch
                .delete_existing_verified(
                    binding_one,
                    |task| task.aspect("status.value", "open"),
                    |delete| delete.touch("status.value"),
                )
                .verify_existing(binding_two, |task| task.aspect("title.value", "Second"))
        })
        .expect("mixed batch should execute");

    assert_eq!(
        receipt.write_receipts()[0].mutation_family(),
        ForgeQueryMutationFamily::Delete
    );
    assert_eq!(
        receipt.write_receipts()[0]
            .existing_truth_assertion_evidence()
            .expect("verified delete should retain assertion evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .existing_truth_assertion_count(),
        2
    );
    assert!(receipt
        .batch_mutation_evidence()
        .aggregate_existing_truth_assertion_digest()
        .is_some());
}

#[test]
fn delete_existing_verified_denies_unsupported_backend_typed_and_early() {
    let runtime = bridge_runtime_with_support(ForgeQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    ));
    let mut workspace = runtime
        .workspace("tasks.delete-existing-verified-unsupported")
        .expect("workspace should open");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new("authority:task-1", "Task:1")
                .expect("existing entity target should build")
                .in_target_collection("Task")
                .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = workspace
        .delete_existing_verified(
            binding,
            |task| task.aspect("status.value", "open"),
            |delete| delete.touch("status.value"),
        )
        .expect_err("unsupported backend verified delete should deny");

    match error {
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
            );
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }
}

#[test]
fn preview_delete_existing_verified_requires_authoritative_lane() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.preview-delete-existing-verified")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.preview-delete-existing-verified-table", |q| {
            q.from("Task")
                .select(["identity.id", "status.value"])
                .order_by("identity.id")
                .schema_basis("tasks-preview-delete-existing-verified-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("status.value", "open")
        })
        .expect("seed insert should execute");
    let mut preview = workspace
        .preview("delete-existing-verified-preview")
        .expect("preview should open");
    let binding = preview
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                "authority:task-1",
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = preview
        .delete_existing_verified(
            binding,
            |task| task.aspect("status.value", "open"),
            |delete| delete.touch("status.value"),
        )
        .expect_err("preview verified delete should require authoritative lane");

    match error {
        ForgeQueryRuntimeError::UnsupportedAuthority(message) => {
            assert!(message.contains("authoritative lane"));
        }
        other => panic!("expected unsupported authority denial, got {other:?}"),
    }
}
