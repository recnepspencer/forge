use super::super::support::*;

#[test]
fn update_existing_preserves_continuity_evidence_on_receipt_and_inspection() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.continuity-existing")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.continuity-existing-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-continuity-existing-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Before continuity rebind")
        })
        .expect("seed insert should execute");
    let binding = ForgeQueryExistingTruthTargetBinding::direct_entity(
        "authority:task-1",
        seed.deltas()[0].entity_identity.clone(),
    )
    .expect("binding should build")
    .in_target_collection("Task")
    .expect("binding collection should build");
    let binding_digest = binding.binding_digest();

    let receipt = workspace
        .update_existing(binding, |task| {
            task.continuity_rebind_existing_target("authority:task-1", "authority:task-1-successor")
                .aspect("title.value", "After continuity rebind")
        })
        .expect("continuity-aware existing-target update should execute");
    let inspection = workspace
        .inspect(&receipt)
        .expect("continuity-aware receipt should inspect");

    let continuity = receipt
        .continuity_mutation_evidence()
        .expect("receipt should retain continuity evidence");
    assert_eq!(
        continuity.family(),
        ForgeQueryContinuityMutationFamily::RebindExistingTarget
    );
    assert_eq!(
        continuity.outcome_class(),
        ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
    );
    assert_eq!(
        continuity.prior_authoritative_identity(),
        "authority:task-1"
    );
    assert_eq!(
        continuity.successor_authoritative_identity(),
        Some("authority:task-1-successor")
    );
    assert_eq!(
        continuity.basis_binding_digest(),
        Some(binding_digest.as_str())
    );
    assert_eq!(
        continuity.resolved_target_entity_identity(),
        Some(seed.deltas()[0].entity_identity.as_str())
    );
    assert_eq!(continuity.target_collection(), Some("Task"));
    assert!(!continuity.lineage_digest().is_empty());
    assert!(!continuity.continuity_resolution_digest().is_empty());

    match inspection {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            let continuity = inspection
                .continuity_mutation_evidence()
                .expect("inspection should retain continuity evidence");
            assert_eq!(
                continuity.prior_authoritative_identity(),
                "authority:task-1"
            );
            assert_eq!(
                continuity.successor_authoritative_identity(),
                Some("authority:task-1-successor")
            );
            assert_eq!(
                continuity.basis_binding_digest(),
                Some(binding_digest.as_str())
            );
            assert_eq!(
                continuity.lineage_digest(),
                receipt
                    .continuity_mutation_evidence()
                    .expect("receipt should retain continuity evidence")
                    .lineage_digest()
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
}

#[test]
fn mixed_batch_preserves_continuity_and_naming_session_evidence() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.continuity-batch")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.continuity-batch-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-continuity-batch-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-existing")
                .aspect("title.value", "Existing")
        })
        .expect("seed insert should execute");
    let binding = ForgeQueryExistingTruthTargetBinding::direct_entity(
        "authority:task-existing",
        seed.deltas()[0].entity_identity.clone(),
    )
    .expect("binding should build")
    .in_target_collection("Task")
    .expect("binding collection should build");
    let binding_digest = binding.binding_digest();

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
                .update_existing(binding, |task| {
                    task.continuity_rebind_merge_successor(
                        "authority:task-existing",
                        "authority:task-existing-merged",
                    )
                    .aspect("title.value", "Existing continuity merged")
                })
        })
        .expect("mixed continuity batch should execute");
    let inspection = workspace.inspect(&receipt).expect("batch should inspect");

    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .continuity_mutation_count(),
        1
    );
    assert_eq!(receipt.batch_mutation_evidence().naming_mutation_count(), 1);
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
    assert!(receipt
        .batch_mutation_evidence()
        .aggregate_continuity_mutation_digest()
        .is_some());

    match inspection {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            assert_eq!(
                inspection
                    .batch_mutation_evidence()
                    .continuity_mutation_count(),
                1
            );
            assert!(inspection
                .batch_mutation_evidence()
                .aggregate_continuity_mutation_digest()
                .is_some());
            assert_eq!(
                inspection.component_operations()[2]
                    .continuity_mutation_evidence()
                    .expect("existing continuity component should retain evidence")
                    .outcome_class(),
                ForgeQueryContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
            );
            assert_eq!(
                inspection.component_operations()[2]
                    .continuity_mutation_evidence()
                    .expect("existing continuity component should retain evidence")
                    .basis_binding_digest(),
                Some(binding_digest.as_str())
            );
        }
        other => panic!("expected batch write inspection, got {other:?}"),
    }
}

#[test]
fn continuity_update_denies_missing_binding_typed_and_early() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.continuity-denial")
        .expect("task runtime should open a named workspace");

    let error = workspace
        .update("entity:0:1:0", |task| {
            task.continuity_rebind_existing_target("authority:task-1", "authority:task-1-successor")
                .aspect("title.value", "No binding")
        })
        .expect_err("continuity-aware update should deny without existing binding");

    match error {
        ForgeQueryRuntimeError::MutationContinuityDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryContinuityMutationDenialKind::RequiresExistingTruthBinding
            );
            assert!(!denial.denial_digest().is_empty());
        }
        other => panic!("expected typed continuity denial, got {other:?}"),
    }
}

#[test]
fn continuity_insert_denies_non_update_family_typed_and_early() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.continuity-insert-denial")
        .expect("task runtime should open a named workspace");

    let error = workspace
        .insert("Task", |task| {
            task.continuity_rebind_existing_target("authority:task-1", "authority:task-1-successor")
                .aspect("identity.id", "task-2")
        })
        .expect_err("continuity-aware insert should deny on non-update family");

    match error {
        ForgeQueryRuntimeError::MutationContinuityDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryContinuityMutationDenialKind::RequiresUpdateFamily
            );
            assert!(!denial.denial_digest().is_empty());
        }
        other => panic!("expected typed continuity denial, got {other:?}"),
    }
}

#[test]
fn preview_update_existing_denies_continuity_without_authoritative_lane() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.preview-continuity-denial")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.preview-continuity-denial-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-preview-continuity-denial-table")
        })
        .expect("live view should declare");
    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-preview")
                .aspect("title.value", "Preview continuity")
        })
        .expect("seed insert should execute");
    let binding = ForgeQueryExistingTruthTargetBinding::direct_entity(
        "authority:task-preview",
        seed.deltas()[0].entity_identity.clone(),
    )
    .expect("binding should build")
    .in_target_collection("Task")
    .expect("binding collection should build");
    let binding_digest = binding.binding_digest();
    let mut preview = workspace
        .preview("continuity denial")
        .expect("preview should open");

    let error = preview
        .update_existing(binding, |task| {
            task.continuity_rebind_existing_target(
                "authority:task-preview",
                "authority:task-preview-successor",
            )
            .aspect("title.value", "Preview continuity denied")
        })
        .expect_err("preview continuity should deny outside authoritative lane");

    match error {
        ForgeQueryRuntimeError::MutationContinuityDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryContinuityMutationDenialKind::RequiresAuthoritativeLane
            );
            assert_eq!(denial.basis_binding_digest(), Some(binding_digest.as_str()));
            assert!(!denial.denial_digest().is_empty());
        }
        other => panic!("expected typed continuity denial, got {other:?}"),
    }
}

#[test]
fn preview_batch_denies_continuity_without_authoritative_lane() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.preview-batch-continuity-denial")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.preview-batch-continuity-denial-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-preview-batch-continuity-denial-table")
        })
        .expect("live view should declare");
    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-preview-batch")
                .aspect("title.value", "Preview continuity batch")
        })
        .expect("seed insert should execute");
    let binding = ForgeQueryExistingTruthTargetBinding::direct_entity(
        "authority:task-preview-batch",
        seed.deltas()[0].entity_identity.clone(),
    )
    .expect("binding should build")
    .in_target_collection("Task")
    .expect("binding collection should build");
    let binding_digest = binding.binding_digest();
    let mut preview = workspace
        .preview("continuity batch denial")
        .expect("preview should open");

    let error = preview
        .batch(|batch| {
            batch.update_existing(binding, |task| {
                task.continuity_rebind_existing_target(
                    "authority:task-preview-batch",
                    "authority:task-preview-batch-successor",
                )
                .aspect("title.value", "Preview continuity batch denied")
            })
        })
        .expect_err("preview batch continuity should deny outside authoritative lane");

    match error {
        ForgeQueryRuntimeError::MutationContinuityDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryContinuityMutationDenialKind::RequiresAuthoritativeLane
            );
            assert_eq!(denial.basis_binding_digest(), Some(binding_digest.as_str()));
            assert!(!denial.denial_digest().is_empty());
        }
        other => panic!("expected typed continuity denial, got {other:?}"),
    }
}
