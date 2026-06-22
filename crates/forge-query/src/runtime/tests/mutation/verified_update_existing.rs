use super::super::support::*;

#[test]
fn update_existing_verified_preserves_backend_verified_assertion_evidence_on_update_receipt() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.update-existing-verified")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.update-existing-verified-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                    crate::authoring::AspectFieldKey::new("status", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("tasks-update-existing-verified-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect(
                test_aspect_touch("identity.id"),
                test_string_aspect_value("task-1"),
            )
            .aspect(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Seed title"),
            )
            .aspect(
                test_aspect_touch("status.value"),
                test_string_aspect_value("open"),
            )
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
        .update_existing_verified(
            binding,
            |task| {
                task.aspect(
                    test_aspect_touch("status.value"),
                    test_string_aspect_value("open"),
                )
            },
            |task| {
                task.aspect(
                    test_aspect_touch("status.value"),
                    test_string_aspect_value("closed"),
                )
            },
        )
        .expect("backend-verified update should execute");

    assert_eq!(receipt.mutation_family(), ForgeQueryMutationFamily::Update);
    assert_eq!(
        receipt.terminal_target_collection_projection(),
        Some("Task")
    );
    assert_eq!(
        receipt
            .existing_truth_binding_evidence()
            .expect("receipt should retain existing binding evidence")
            .family(),
        ForgeQueryExistingTruthBindingFamily::DirectEntityIdentity
    );
    let assertion = receipt
        .existing_truth_assertion_evidence()
        .expect("update receipt should retain verified assertion evidence");
    assert_eq!(
        assertion.mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    assert_eq!(assertion.asserted_aspect_count(), 1);
    assert_eq!(receipt.deltas().len(), 1);
    assert_eq!(
        receipt.deltas()[0].admitted_touched_aspects(),
        test_aspect_touches(["status.value"]).as_slice()
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
}

#[test]
fn update_existing_verified_denies_mismatch_typed_and_leaves_truth_unchanged() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.update-existing-verified-mismatch")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.update-existing-verified-mismatch-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                    crate::authoring::AspectFieldKey::new("status", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("tasks-update-existing-verified-mismatch-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect(
                test_aspect_touch("identity.id"),
                test_string_aspect_value("task-1"),
            )
            .aspect(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Seed title"),
            )
            .aspect(
                test_aspect_touch("status.value"),
                test_string_aspect_value("open"),
            )
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
        .update_existing_verified(
            binding.clone(),
            |task| {
                task.aspect(
                    test_aspect_touch("status.value"),
                    test_string_aspect_value("closed"),
                )
            },
            |task| {
                task.aspect(
                    test_aspect_touch("status.value"),
                    test_string_aspect_value("done"),
                )
            },
        )
        .expect_err("mismatched verified update should deny");

    match error {
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch
            );
            assert_eq!(
                denial.asserted_aspect_touch(),
                Some(&test_aspect_touch("status.value"))
            );
            assert_eq!(
                denial.expected_native_value_digest(),
                Some("status:value=set:string:6:closed")
            );
            assert_eq!(denial.found_native_value_digest(), Some("string:4:open"));
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }

    let probe = workspace
        .probe_existing(binding, test_aspect_touches(["status.value"]))
        .expect("probe should still succeed after denied update");
    assert_eq!(
        probe
            .field_for_touch(&test_aspect_touch("status.value"))
            .expect("status field should remain present")
            .foundational_value(),
        &test_string_aspect_value("open")
    );
}

#[test]
fn batch_update_existing_verified_preserves_aggregate_assertion_digest() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.batch-update-existing-verified")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.batch-update-existing-verified-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                    crate::authoring::AspectFieldKey::new("status", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("tasks-batch-update-existing-verified-table")
        })
        .expect("live view should declare");

    let seed_one = workspace
        .insert("Task", |task| {
            task.aspect(
                test_aspect_touch("identity.id"),
                test_string_aspect_value("task-1"),
            )
            .aspect(
                test_aspect_touch("title.value"),
                test_string_aspect_value("First"),
            )
            .aspect(
                test_aspect_touch("status.value"),
                test_string_aspect_value("open"),
            )
        })
        .expect("first seed should execute");
    let seed_two = workspace
        .insert("Task", |task| {
            task.aspect(
                test_aspect_touch("identity.id"),
                test_string_aspect_value("task-2"),
            )
            .aspect(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Second"),
            )
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
                .update_existing_verified(
                    binding_one,
                    |task| {
                        task.aspect(
                            test_aspect_touch("status.value"),
                            test_string_aspect_value("open"),
                        )
                    },
                    |task| {
                        task.aspect(
                            test_aspect_touch("status.value"),
                            test_string_aspect_value("closed"),
                        )
                    },
                )
                .delete_existing_with(binding_two, |delete| {
                    delete.touch(test_aspect_touch("title.value"))
                })
        })
        .expect("mixed batch should execute");

    assert_eq!(
        receipt.write_receipts()[0].mutation_family(),
        ForgeQueryMutationFamily::Update
    );
    assert_eq!(
        receipt.write_receipts()[0]
            .existing_truth_assertion_evidence()
            .expect("verified update should retain assertion evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .existing_truth_assertion_count(),
        1
    );
    assert!(receipt
        .batch_mutation_evidence()
        .aggregate_existing_truth_assertion_digest()
        .is_some());
}

#[test]
fn primary_multi_verified_update_batch_shares_one_commit_boundary() {
    let attempted_writes = std::rc::Rc::new(std::cell::Cell::new(0usize));
    let attempted_batches = std::rc::Rc::new(std::cell::Cell::new(0usize));
    let runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(TestSnapshotIdentityAdapter)
        .existing_truth_verification(PermissiveExistingTruthVerificationAdapter)
        .write_authority(AtomicBatchCountingWriteAuthority {
            attempted_writes: attempted_writes.clone(),
            attempted_batches: attempted_batches.clone(),
        })
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .support_profile(ForgeQueryRuntimeSupportProfile::bridge_backed(
            "test-subscription-activation",
            "test-preview-basis",
            "test-inspector-evidence",
        ))
        .build_backend_from_parts()
        .build()
        .expect("primary bridge-backed runtime should build");
    let mut workspace = runtime
        .workspace("tasks.batch-update-existing-verified-atomic")
        .expect("task runtime should open a named workspace");

    let binding_one = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                test_entity_identity("Task:1"),
            )
            .expect("first existing entity target should build")
            .in_target_collection("Task")
            .expect("first existing entity target collection should build"),
        )
        .expect("first binding should build");
    let binding_two = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                test_entity_identity("Task:2"),
            )
            .expect("second existing entity target should build")
            .in_target_collection("Task")
            .expect("second existing entity target collection should build"),
        )
        .expect("second binding should build");

    let receipt = workspace
        .batch(|batch| {
            batch
                .update_existing_verified(
                    binding_one,
                    |task| {
                        task.aspect(
                            test_aspect_touch("status.value"),
                            test_string_aspect_value("open"),
                        )
                    },
                    |task| {
                        task.aspect(
                            test_aspect_touch("status.value"),
                            test_string_aspect_value("closed"),
                        )
                    },
                )
                .update_existing_verified(
                    binding_two,
                    |task| {
                        task.aspect(
                            test_aspect_touch("status.value"),
                            test_string_aspect_value("open"),
                        )
                    },
                    |task| {
                        task.aspect(
                            test_aspect_touch("status.value"),
                            test_string_aspect_value("closed"),
                        )
                    },
                )
        })
        .expect("verified update batch should execute atomically");

    assert_eq!(receipt.write_receipts().len(), 2);
    assert_eq!(attempted_batches.get(), 1);
    assert_eq!(attempted_writes.get(), 0);
    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .backend_verified_update_count(),
        2
    );
}

#[test]
fn update_existing_verified_denies_unsupported_backend_typed_and_early() {
    let runtime = bridge_runtime_with_support(ForgeQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    ));
    let mut workspace = runtime
        .workspace("tasks.update-existing-verified-unsupported")
        .expect("workspace should open");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"), test_entity_identity("Task:1"))
                .expect("existing entity target should build")
                .in_target_collection("Task")
                .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = workspace
        .update_existing_verified(
            binding,
            |task| {
                task.aspect(
                    test_aspect_touch("status.value"),
                    test_string_aspect_value("open"),
                )
            },
            |task| {
                task.aspect(
                    test_aspect_touch("status.value"),
                    test_string_aspect_value("closed"),
                )
            },
        )
        .expect_err("unsupported backend verified update should deny");

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
fn preview_update_existing_verified_requires_authoritative_lane() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.preview-update-existing-verified")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.preview-update-existing-verified-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("status", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("identity", "id").unwrap())
                .schema_basis("tasks-preview-update-existing-verified-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect(
                test_aspect_touch("identity.id"),
                test_string_aspect_value("task-1"),
            )
            .aspect(
                test_aspect_touch("status.value"),
                test_string_aspect_value("open"),
            )
        })
        .expect("seed insert should execute");
    let mut preview = workspace
        .preview(test_session_label("update-existing-verified-preview"))
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
        .update_existing_verified(
            binding,
            |task| {
                task.aspect(
                    test_aspect_touch("status.value"),
                    test_string_aspect_value("open"),
                )
            },
            |task| {
                task.aspect(
                    test_aspect_touch("status.value"),
                    test_string_aspect_value("closed"),
                )
            },
        )
        .expect_err("preview verified update should require authoritative lane");

    match error.stop_class() {
        ForgeQueryStopClass::ExistingTruthAssertionRequiresAuthorityLane { required_lane } => {
            assert_eq!(required_lane, ForgeQueryAuthorityLane::AuthoritativeTruth);
        }
        other => panic!("expected typed authoritative lane stop class, got {other:?}"),
    }
}
