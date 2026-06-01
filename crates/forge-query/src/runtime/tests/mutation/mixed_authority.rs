use super::super::support::*;

#[test]
fn mixed_batch_preserves_existing_truth_mode_and_neighbor_aggregate_evidence() {
    let mut workspace = stateful_bridge_task_runtime()
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

    let assert_seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-assert")
                .aspect("title.value", "Assert seed")
        })
        .expect("assert seed insert should execute");
    let verify_seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-verify")
                .aspect("title.value", "Verify seed")
        })
        .expect("verify seed insert should execute");
    let update_seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-update")
                .aspect("title.value", "Update seed")
        })
        .expect("update seed insert should execute");
    let delete_seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-delete")
                .aspect("title.value", "Delete seed")
        })
        .expect("delete seed insert should execute");

    let assert_binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                "authority:task-assert",
                assert_seed.deltas()[0].entity_identity.clone(),
            )
            .expect("assert target should build")
            .in_target_collection("Task")
            .expect("assert target collection should build"),
        )
        .expect("assert binding should build");
    let verify_binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                "authority:task-verify",
                verify_seed.deltas()[0].entity_identity.clone(),
            )
            .expect("verify target should build")
            .in_target_collection("Task")
            .expect("verify target collection should build"),
        )
        .expect("verify binding should build");
    let update_binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                "authority:task-update",
                update_seed.deltas()[0].entity_identity.clone(),
            )
            .expect("update target should build")
            .in_target_collection("Task")
            .expect("update target collection should build"),
        )
        .expect("update binding should build");
    let update_binding_digest = update_binding.binding_digest().to_string();
    let update_probe_binding = update_binding.clone();
    let delete_binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                "authority:task-delete",
                delete_seed.deltas()[0].entity_identity.clone(),
            )
            .expect("delete target should build")
            .in_target_collection("Task")
            .expect("delete target collection should build"),
        )
        .expect("delete binding should build");

    let receipt = workspace
        .batch(|batch| {
            batch
                .assert_existing(assert_binding, |task| {
                    task.aspect("title.value", "Assert seed")
                })
                .verify_existing(verify_binding, |task| {
                    task.aspect("title.value", "Verify seed")
                })
                .update_existing_verified(
                    update_binding,
                    |task| task.aspect("title.value", "Update seed"),
                    |task| {
                        task.continuity_rebind_merge_successor(
                            "authority:task-update",
                            "authority:task-update-merged",
                        )
                        .aspect("title.value", "Update merged")
                    },
                )
                .delete_existing_verified(
                    delete_binding,
                    |task| task.aspect("title.value", "Delete seed"),
                    |delete| {
                        delete
                            .touch("title.value")
                            .naming_remove("persistent-name:delete", "authority:task-delete")
                    },
                )
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
        })
        .expect("mixed authority batch should execute");

    let batch_evidence = receipt.batch_mutation_evidence();
    assert_eq!(batch_evidence.component_count(), 6);
    assert_eq!(batch_evidence.existing_truth_assertion_count(), 4);
    assert_eq!(batch_evidence.retained_authoritative_assertion_count(), 1);
    assert_eq!(batch_evidence.backend_verified_assertion_count(), 1);
    assert_eq!(batch_evidence.backend_verified_update_count(), 1);
    assert_eq!(batch_evidence.backend_verified_delete_count(), 1);
    assert_eq!(batch_evidence.existing_truth_binding_count(), 4);
    assert_eq!(batch_evidence.symbolic_target_reference_count(), 1);
    assert_eq!(batch_evidence.naming_mutation_count(), 2);
    assert_eq!(batch_evidence.continuity_mutation_count(), 1);

    let mode_digest = batch_evidence
        .aggregate_existing_truth_mode_digest()
        .expect("mixed batch should retain existing-truth mode digest")
        .to_string();
    let assertion_digest = batch_evidence
        .aggregate_existing_truth_assertion_digest()
        .expect("mixed batch should retain assertion digest")
        .to_string();
    let binding_digest = batch_evidence
        .aggregate_existing_truth_binding_digest()
        .expect("mixed batch should retain binding digest")
        .to_string();
    let symbolic_digest = batch_evidence
        .aggregate_symbolic_target_reference_digest()
        .expect("mixed batch should retain symbolic digest")
        .to_string();
    let naming_digest = batch_evidence
        .aggregate_naming_mutation_digest()
        .expect("mixed batch should retain naming digest")
        .to_string();
    let continuity_digest = batch_evidence
        .aggregate_continuity_mutation_digest()
        .expect("mixed batch should retain continuity digest")
        .to_string();
    let update_probe = workspace
        .probe_existing(update_probe_binding, ["title.value"])
        .expect("updated target should remain probeable after mixed batch");
    assert_eq!(
        update_probe.mode(),
        ForgeQueryExistingTruthProbeMode::BackendVerifiedProbe
    );
    assert_eq!(
        update_probe
            .field("title.value")
            .expect("probe should retain updated title")
            .external_value_json(),
        "\"Update merged\""
    );
    assert_ne!(update_probe.probe_digest(), mode_digest.as_str());

    match workspace.inspect(&receipt).expect("batch should inspect") {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            let batch_evidence = inspection.batch_mutation_evidence();
            assert_eq!(
                batch_evidence.aggregate_existing_truth_mode_digest(),
                Some(mode_digest.as_str())
            );
            assert_eq!(
                batch_evidence.aggregate_existing_truth_assertion_digest(),
                Some(assertion_digest.as_str())
            );
            assert_eq!(
                batch_evidence.aggregate_existing_truth_binding_digest(),
                Some(binding_digest.as_str())
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
                inspection.component_operations()[0].family(),
                ForgeQueryMutationFamily::Assertion.as_str()
            );
            assert_eq!(
                inspection.component_operations()[1].family(),
                ForgeQueryMutationFamily::Assertion.as_str()
            );
            assert_eq!(
                inspection.component_operations()[0]
                    .existing_truth_assertion_evidence()
                    .expect("retained assertion should keep assertion evidence")
                    .mode(),
                ForgeQueryExistingTruthAssertionMode::RetainedAuthoritativeAssertion
            );
            assert_eq!(
                inspection.component_operations()[1]
                    .existing_truth_assertion_evidence()
                    .expect("verified assertion should keep assertion evidence")
                    .mode(),
                ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
            );
            assert_eq!(
                inspection.component_operations()[2].family(),
                ForgeQueryMutationFamily::Update.as_str()
            );
            assert_eq!(
                inspection.component_operations()[3].family(),
                ForgeQueryMutationFamily::Delete.as_str()
            );
            assert_eq!(
                inspection.component_operations()[2]
                    .existing_truth_assertion_evidence()
                    .expect("verified update should keep assertion evidence")
                    .mode(),
                ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
            );
            assert_eq!(
                inspection.component_operations()[3]
                    .existing_truth_assertion_evidence()
                    .expect("verified delete should keep assertion evidence")
                    .mode(),
                ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
            );
            assert_eq!(
                inspection.component_operations()[2]
                    .continuity_mutation_evidence()
                    .expect("verified update should retain continuity evidence")
                    .basis_binding_digest(),
                Some(update_binding_digest.as_str())
            );
            assert_eq!(
                inspection.component_operations()[3]
                    .naming_mutation_evidence()
                    .expect("verified delete should retain naming evidence")
                    .outcome(),
                ForgeQueryNamingMutationOutcome::Removed
            );
        }
        other => panic!("expected batch write inspection, got {other:?}"),
    }
}

#[test]
fn existing_truth_cluster_unsupported_backend_denials_remain_typed_and_distinct() {
    let runtime = bridge_runtime_with_support(ForgeQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    ));
    let mut workspace = runtime
        .workspace("tasks.existing-truth-cluster-unsupported")
        .expect("workspace should open");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new("authority:task-1", "Task:1")
                .expect("existing entity target should build")
                .in_target_collection("Task")
                .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let probe_error = workspace
        .probe_existing(binding.clone(), ["title.value"])
        .expect_err("unsupported backend probe should deny");
    match probe_error {
        ForgeQueryRuntimeError::ExistingTruthProbeDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthProbeDenialKind::BackendProbeUnsupported
            );
        }
        other => panic!("expected typed probe denial, got {other:?}"),
    }

    let verify_error = workspace
        .verify_existing(binding.clone(), |task| {
            task.aspect("title.value", "Seed title")
        })
        .expect_err("unsupported backend verification should deny");
    match verify_error {
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
            );
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }

    let update_error = workspace
        .update_existing_verified(
            binding.clone(),
            |task| task.aspect("title.value", "Seed title"),
            |task| task.aspect("title.value", "Updated title"),
        )
        .expect_err("unsupported backend verified update should deny");
    match update_error {
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
            );
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }

    let delete_error = workspace
        .delete_existing_verified(
            binding,
            |task| task.aspect("title.value", "Seed title"),
            |delete| delete.touch("title.value"),
        )
        .expect_err("unsupported backend verified delete should deny");
    match delete_error {
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
            );
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }
}
