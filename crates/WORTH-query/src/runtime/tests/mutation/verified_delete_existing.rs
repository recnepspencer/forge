use super::super::support::*;

#[test]
fn delete_existing_verified_preserves_backend_verified_assertion_evidence_on_delete_receipt() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.delete-existing-verified")
        .expect("task runtime should open a named workspace");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.delete-existing-verified-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("status", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                )
                .schema_basis("tasks-delete-existing-verified-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.set_aspect(
                test_aspect_touch("identity.id"),
                test_authored_string_aspect_value("task-1"),
            )
            .set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("Seed title"),
            )
            .set_aspect(
                test_aspect_touch("status.value"),
                test_authored_string_aspect_value("open"),
            )
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_entity(
            WorthQueryExistingEntityTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
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
            |task| {
                task.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("open"),
                )
            },
            |delete| {
                delete
                    .touch(test_aspect_touch("status.value"))
                    .touch(test_aspect_touch("title.value"))
            },
        )
        .expect("backend-verified delete should execute");

    assert_eq!(receipt.mutation_family(), WorthQueryMutationFamily::Delete);
    assert_eq!(
        receipt.terminal_target_collection_projection(),
        Some("Task")
    );
    assert_eq!(
        receipt
            .existing_truth_binding_evidence()
            .expect("receipt should retain existing binding evidence")
            .family(),
        WorthQueryExistingTruthBindingFamily::DirectEntityIdentity
    );
    let assertion = receipt
        .existing_truth_assertion_evidence()
        .expect("verified delete should retain assertion evidence");
    assert_eq!(
        assertion.mode(),
        WorthQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    assert_eq!(assertion.asserted_aspect_count(), 1);
    assert_eq!(
        receipt.deltas()[0].admitted_touched_aspects(),
        test_aspect_touches(["status.value", "title.value"]).as_slice()
    );
    assert!(receipt.declared_aspect_value_digest().is_some());

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        WorthQueryInspection::WriteReceipt(inspection) => {
            assert_eq!(
                inspection
                    .existing_truth_assertion_evidence()
                    .expect("inspection should retain verified assertion evidence")
                    .mode(),
                WorthQueryExistingTruthAssertionMode::BackendVerifiedAssertion
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
        .probe_existing(binding, test_aspect_touches(["status.value"]))
        .expect_err("deleted target should no longer probe");
    match error {
        WorthQueryRuntimeError::MutationBindingDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryExistingTruthBindingDenialKind::ResolvedTargetMissing
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
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.delete-existing-verified-mismatch-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("status", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                )
                .schema_basis("tasks-delete-existing-verified-mismatch-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.set_aspect(
                test_aspect_touch("identity.id"),
                test_authored_string_aspect_value("task-1"),
            )
            .set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("Seed title"),
            )
            .set_aspect(
                test_aspect_touch("status.value"),
                test_authored_string_aspect_value("open"),
            )
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_entity(
            WorthQueryExistingEntityTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
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
            |task| {
                task.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("closed"),
                )
            },
            |delete| delete.touch(test_aspect_touch("status.value")),
        )
        .expect_err("mismatched verified delete should deny");

    match error {
        WorthQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryExistingTruthAssertionDenialKind::AssertedValueMismatch
            );
            assert_eq!(
                denial.asserted_aspect_touch(),
                Some(&test_aspect_touch("status.value"))
            );
            assert_eq!(
                denial.expected_terminal_value_digest(),
                Some("status:value=set:string:6:closed")
            );
            assert_eq!(denial.found_terminal_value_digest(), Some("string:4:open"));
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }

    let probe = workspace
        .probe_existing(binding, test_aspect_touches(["status.value"]))
        .expect("probe should still succeed after denied delete");
    assert_eq!(
        probe
            .field_for_touch(&test_aspect_touch("status.value"))
            .expect("status field should remain present")
            .foundational_value(),
        &test_string_aspect_value("open")
    );
}

#[test]
fn batch_delete_existing_verified_preserves_aggregate_assertion_digest() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.batch-delete-existing-verified")
        .expect("task runtime should open a named workspace");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.batch-delete-existing-verified-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("status", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                )
                .schema_basis("tasks-batch-delete-existing-verified-table")
        })
        .expect("live view should declare");

    let seed_one = workspace
        .insert("Task", |task| {
            task.set_aspect(
                test_aspect_touch("identity.id"),
                test_authored_string_aspect_value("task-1"),
            )
            .set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("First"),
            )
            .set_aspect(
                test_aspect_touch("status.value"),
                test_authored_string_aspect_value("open"),
            )
        })
        .expect("first seed should execute");
    let seed_two = workspace
        .insert("Task", |task| {
            task.set_aspect(
                test_aspect_touch("identity.id"),
                test_authored_string_aspect_value("task-2"),
            )
            .set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("Second"),
            )
        })
        .expect("second seed should execute");

    let binding_one = workspace
        .bind_existing_entity(
            WorthQueryExistingEntityTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed_one.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding one should build");
    let binding_two = workspace
        .bind_existing_entity(
            WorthQueryExistingEntityTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
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
                    |task| {
                        task.set_aspect(
                            test_aspect_touch("status.value"),
                            test_authored_string_aspect_value("open"),
                        )
                    },
                    |delete| delete.touch(test_aspect_touch("status.value")),
                )
                .verify_existing(binding_two, |task| {
                    task.set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value("Second"),
                    )
                })
        })
        .expect("mixed batch should execute");

    assert_eq!(
        receipt.write_receipts()[0].mutation_family(),
        WorthQueryMutationFamily::Delete
    );
    assert_eq!(
        receipt.write_receipts()[0]
            .existing_truth_assertion_evidence()
            .expect("verified delete should retain assertion evidence")
            .mode(),
        WorthQueryExistingTruthAssertionMode::BackendVerifiedAssertion
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
    let runtime = bridge_runtime_with_support(WorthQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    ));
    let mut workspace = runtime
        .workspace("tasks.delete-existing-verified-unsupported")
        .expect("workspace should open");
    let binding = workspace
        .bind_existing_entity(
            WorthQueryExistingEntityTarget::new(crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"), test_entity_identity("Task:1"))
                .expect("existing entity target should build")
                .in_target_collection("Task")
                .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = workspace
        .delete_existing_verified(
            binding,
            |task| {
                task.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("open"),
                )
            },
            |delete| delete.touch(test_aspect_touch("status.value")),
        )
        .expect_err("unsupported backend verified delete should deny");

    match error {
        WorthQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
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
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.preview-delete-existing-verified-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("status", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                )
                .schema_basis("tasks-preview-delete-existing-verified-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.set_aspect(
                test_aspect_touch("identity.id"),
                test_authored_string_aspect_value("task-1"),
            )
            .set_aspect(
                test_aspect_touch("status.value"),
                test_authored_string_aspect_value("open"),
            )
        })
        .expect("seed insert should execute");
    let mut preview = workspace
        .preview(test_session_label("delete-existing-verified-preview"))
        .expect("preview should open");
    let binding = preview
        .bind_existing_entity(
            WorthQueryExistingEntityTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
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
            |task| {
                task.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("open"),
                )
            },
            |delete| delete.touch(test_aspect_touch("status.value")),
        )
        .expect_err("preview verified delete should require authoritative lane");

    match error.stop_class() {
        WorthQueryStopClass::ExistingTruthAssertionRequiresAuthorityLane { required_lane } => {
            assert_eq!(required_lane, WorthQueryAuthorityLane::AuthoritativeTruth);
        }
        other => panic!("expected typed authoritative lane stop class, got {other:?}"),
    }
}
