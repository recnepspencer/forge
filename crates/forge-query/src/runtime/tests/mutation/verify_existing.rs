use super::super::support::*;

#[test]
fn verify_existing_preserves_backend_verified_assertion_evidence_without_mutation_deltas() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.verify-existing")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.verify-existing-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-verify-existing-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Seed title")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let receipt = workspace
        .verify_existing(binding, |task| task.aspect("title.value", "Seed title"))
        .expect("backend-verified assertion should execute");

    assert_eq!(
        receipt.mutation_family(),
        ForgeQueryMutationFamily::Assertion
    );
    assert!(receipt.deltas().is_empty());
    let evidence = receipt
        .existing_truth_assertion_evidence()
        .expect("verified assertion should retain assertion evidence");
    assert_eq!(
        evidence.mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    assert_eq!(evidence.asserted_aspect_count(), 1);
    assert!(!evidence.verification_digest().is_empty());

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            let evidence = inspection
                .existing_truth_assertion_evidence()
                .expect("inspection should retain assertion evidence");
            assert_eq!(
                evidence.mode(),
                ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
            );
            assert_eq!(
                evidence.verification_digest(),
                receipt
                    .existing_truth_assertion_evidence()
                    .expect("receipt should retain assertion evidence")
                    .verification_digest()
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
}

#[test]
fn verify_existing_denies_missing_asserted_aspect_typed_and_early() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.verify-existing-missing-aspect")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.verify-existing-missing-aspect-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-verify-existing-missing-aspect-table")
        })
        .expect("live view should declare");
    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Seed title")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = workspace
        .verify_existing(binding, |task| task.aspect("status.value", "open"))
        .expect_err("missing asserted aspect should deny");

    match error {
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect
            );
            assert_eq!(denial.asserted_aspect_path(), Some("status.value"));
            assert_eq!(denial.expected_external_value_json(), Some("\"open\""));
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }
}

#[test]
fn verify_existing_denies_mismatched_value_typed_and_early() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.verify-existing-mismatch")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.verify-existing-mismatch-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-verify-existing-mismatch-table")
        })
        .expect("live view should declare");
    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Seed title")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = workspace
        .verify_existing(binding, |task| {
            task.aspect("title.value", "Different title")
        })
        .expect_err("mismatched asserted value should deny");

    match error {
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch
            );
            assert_eq!(denial.asserted_aspect_path(), Some("title.value"));
            assert_eq!(
                denial.expected_external_value_json(),
                Some("\"Different title\"")
            );
            assert_eq!(denial.found_external_value_json(), Some("\"Seed title\""));
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }
}

#[test]
fn verify_existing_reports_the_actual_failing_aspect_in_multi_aspect_requests() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.verify-existing-multi-mismatch")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.verify-existing-multi-mismatch-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-verify-existing-multi-mismatch-table")
        })
        .expect("live view should declare");
    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Seed title")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = workspace
        .verify_existing(binding, |task| {
            task.aspect("identity.id", "task-1")
                .aspect("status.value", "open")
                .aspect("title.value", "Seed title")
        })
        .expect_err("missing asserted aspect should deny");

    match error {
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect
            );
            assert_eq!(denial.asserted_aspect_path(), Some("status.value"));
            assert_eq!(denial.expected_external_value_json(), Some("\"open\""));
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }
}

#[test]
fn batch_verify_existing_preserves_aggregate_assertion_digest() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.batch-verify-existing")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.batch-verify-existing-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-batch-verify-existing-table")
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
    let binding_one = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
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
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
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
                .verify_existing(binding_one, |task| task.aspect("title.value", "First"))
                .delete_existing_with(binding_two, |delete| delete.touch("title.value"))
        })
        .expect("mixed batch should execute");

    let evidence = receipt.batch_mutation_evidence();
    assert_eq!(evidence.existing_truth_assertion_count(), 1);
    assert!(evidence
        .aggregate_existing_truth_assertion_digest()
        .is_some());

    match workspace.inspect(&receipt).expect("batch should inspect") {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            assert_eq!(
                inspection.component_operations()[0]
                    .existing_truth_assertion_evidence()
                    .expect("first component should retain assertion evidence")
                    .mode(),
                ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
            );
            assert_eq!(
                inspection
                    .batch_mutation_evidence()
                    .aggregate_existing_truth_assertion_digest(),
                evidence.aggregate_existing_truth_assertion_digest()
            );
        }
        other => panic!("expected batch write receipt inspection, got {other:?}"),
    }
}

#[test]
fn preview_verify_existing_requires_authoritative_lane() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.preview-verify-existing")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.preview-verify-existing-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-preview-verify-existing-table")
        })
        .expect("live view should declare");
    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Seed title")
        })
        .expect("seed insert should execute");
    let mut preview = workspace
        .preview(test_session_label("verify-existing-preview"))
        .expect("preview should open");
    let binding = preview
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = preview
        .verify_existing(binding, |task| task.aspect("title.value", "Seed title"))
        .expect_err("preview verification should require authoritative lane");

    match error.stop_class() {
        ForgeQueryStopClass::ExistingTruthAssertionRequiresAuthorityLane { required_lane } => {
            assert_eq!(required_lane, ForgeQueryAuthorityLane::AuthoritativeTruth);
        }
        other => panic!("expected typed authoritative lane stop class, got {other:?}"),
    }
}

#[test]
fn verify_existing_relation_preserves_backend_verified_assertion_evidence() {
    let runtime = stateful_bridge_task_relation_runtime();
    let mut workspace = runtime
        .workspace("tasks.verify-existing-relation")
        .expect("workspace should open");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.verify-existing-relation-table", |q| {
            q.from("TaskRelation")
                .select(["identity.id", "kind.value"])
                .order_by("kind.value")
                .schema_basis("tasks-verify-existing-relation-table")
        })
        .expect("relation live view should declare");

    let seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect("identity.id", "rel-1")
                .aspect("kind.value", "depends_on")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("relation binding should build");

    let receipt = workspace
        .verify_existing(binding, |relation| {
            relation.aspect("kind.value", "depends_on")
        })
        .expect("backend-verified relation assertion should execute");

    assert_eq!(
        receipt
            .existing_truth_binding_evidence()
            .expect("receipt should retain relation binding evidence")
            .family(),
        ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity
    );
    assert_eq!(
        receipt
            .existing_truth_assertion_evidence()
            .expect("receipt should retain assertion evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    assert!(receipt.deltas().is_empty());
}
