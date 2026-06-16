use super::super::support::*;

#[test]
fn update_existing_preserves_naming_evidence_on_receipt_and_inspection() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.naming-existing")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.naming-existing-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-naming-existing-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Named task")
        })
        .expect("seed insert should execute");
    let binding = ForgeQueryExistingTruthTargetBinding::direct_entity(
        crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity"),
        seed.deltas()[0].entity_identity.clone(),
    )
    .expect("binding should build")
    .in_target_collection("Task")
    .expect("binding collection should build");

    let receipt = workspace
        .update_existing(binding, |task| {
            task.naming_attach_existing_target(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_attachment(
                    crate::runtime::ForgeQueryNamingAttachmentAuthorityLabel::new(
                        "persistent-name:task-1",
                    )
                    .expect("naming attachment authority label"),
                )
                .expect("naming attachment identity"),
                crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_target_authority(
                    crate::runtime::ForgeQueryNamingTargetAuthorityLabel::new(
                        "persistent-name:task-1",
                    )
                    .expect("naming target authority label"),
                )
                .expect("naming target authority identity"),
            )
            .aspect("title.value", "Named task renamed")
        })
        .expect("existing-target naming update should execute");
    let inspection = workspace
        .inspect(&receipt)
        .expect("naming receipt should inspect");

    let naming = receipt
        .naming_mutation_evidence()
        .expect("receipt should retain naming evidence");
    assert_eq!(
        naming.family(),
        ForgeQueryNamingMutationFamily::AttachExistingTarget
    );
    assert_eq!(
        naming.outcome(),
        ForgeQueryNamingMutationOutcome::AttachedToExistingTarget
    );
    assert_eq!(
        naming.attachment_identity().as_str(),
        "persistent-name:task-1"
    );
    assert_eq!(
        naming
            .target_authoritative_identity()
            .map(|identity| identity.as_str()),
        Some("authority:task-1")
    );
    assert_eq!(
        naming.resolved_target_entity_identity(),
        Some(&seed.deltas()[0].entity_identity)
    );

    match inspection {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            let naming = inspection
                .naming_mutation_evidence()
                .expect("inspection should retain naming evidence");
            assert_eq!(
                naming.attachment_identity().as_str(),
                "persistent-name:task-1"
            );
            assert_eq!(
                naming
                    .target_authoritative_identity()
                    .map(|identity| identity.as_str()),
                Some("authority:task-1")
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
}

#[test]
fn batch_naming_evidence_preserves_attach_and_rebind_outcomes() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.naming-batch")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.naming-batch-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-naming-batch-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-existing")
                .aspect("title.value", "Existing")
        })
        .expect("seed insert should execute");
    let existing_binding = ForgeQueryExistingTruthTargetBinding::direct_entity(
        crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity"),
        seed.deltas()[0].entity_identity.clone(),
    )
    .expect("existing binding should build")
    .in_target_collection("Task")
    .expect("existing binding collection should build");
    let symbolic = ForgeQuerySymbolicTargetReference::new("draft-task")
        .expect("symbolic reference should build")
        .in_target_collection("Task")
        .expect("symbolic collection should build");

    let receipt = workspace
        .batch(|batch| {
            batch
                .insert_symbolic("draft-task", "Task", |task| {
                    task.aspect("identity.id", "task-draft")
                        .aspect("title.value", "Draft")
                })
                .update_symbolic(symbolic.clone(), |task| {
                    task.naming_attach_new_target(
                        crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_attachment(crate::runtime::ForgeQueryNamingAttachmentAuthorityLabel::new(
                            "persistent-name:draft",
                        )
                        .expect("naming attachment authority label")).expect("naming attachment identity"),
                    )
                    .aspect("title.value", "Draft named")
                })
                .update_existing(existing_binding, |task| {
                    task.naming_rebind_target(crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_attachment(crate::runtime::ForgeQueryNamingAttachmentAuthorityLabel::new("persistent-name:task-existing").expect("naming attachment authority label")).expect("naming attachment identity"), crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_prior_authority(crate::runtime::ForgeQueryNamingPriorAuthorityLabel::new("authority:task-existing-old").expect("naming prior authority label")).expect("naming prior authority identity"), crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_target_authority(crate::runtime::ForgeQueryNamingTargetAuthorityLabel::new("authority:task-existing").expect("naming target authority label")).expect("naming target authority identity"),
                    )
                    .aspect("title.value", "Existing rebound")
                })
        })
        .expect("naming batch should execute");
    let inspection = workspace.inspect(&receipt).expect("batch should inspect");

    assert_eq!(receipt.batch_mutation_evidence().naming_mutation_count(), 2);

    match inspection {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            assert_eq!(
                inspection.batch_mutation_evidence().naming_mutation_count(),
                2
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
                    .naming_mutation_evidence()
                    .expect("existing naming component should retain naming evidence")
                    .outcome(),
                ForgeQueryNamingMutationOutcome::ReboundTarget
            );
        }
        other => panic!("expected batch inspection, got {other:?}"),
    }
}

#[test]
fn naming_existing_target_denies_missing_binding_typed_and_early() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.naming-denial")
        .expect("task runtime should open a named workspace");

    let error = workspace
        .update(test_entity_identity("entity:0:1:0"), |task| {
            task.naming_attach_existing_target(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_attachment(
                    crate::runtime::ForgeQueryNamingAttachmentAuthorityLabel::new(
                        "persistent-name:task-1",
                    )
                    .expect("naming attachment authority label"),
                )
                .expect("naming attachment identity"),
                crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_target_authority(
                    crate::runtime::ForgeQueryNamingTargetAuthorityLabel::new(
                        "persistent-name:task-1",
                    )
                    .expect("naming target authority label"),
                )
                .expect("naming target authority identity"),
            )
            .aspect("title.value", "No binding")
        })
        .expect_err("naming attach-to-existing should deny without binding");

    match error {
        ForgeQueryRuntimeError::MutationNamingDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryNamingMutationDenialKind::RequiresExistingTruthBinding
            );
            assert!(!denial.denial_digest().is_empty());
        }
        other => panic!("expected typed naming denial, got {other:?}"),
    }
}

#[test]
fn delete_existing_preserves_naming_removal_evidence_on_receipt_and_inspection() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.naming-remove")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.naming-remove-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-naming-remove-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Named task")
        })
        .expect("seed insert should execute");
    let binding = ForgeQueryExistingTruthTargetBinding::direct_entity(
        crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity"),
        seed.deltas()[0].entity_identity.clone(),
    )
    .expect("binding should build")
    .in_target_collection("Task")
    .expect("binding collection should build");

    let receipt = workspace
        .delete_existing_with(binding, |delete| {
            delete.touch("title.value").naming_remove(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_attachment(
                    crate::runtime::ForgeQueryNamingAttachmentAuthorityLabel::new(
                        "persistent-name:task-1",
                    )
                    .expect("naming attachment authority label"),
                )
                .expect("naming attachment identity"),
                crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_prior_authority(
                    crate::runtime::ForgeQueryNamingPriorAuthorityLabel::new(
                        "persistent-name:task-1",
                    )
                    .expect("naming prior authority label"),
                )
                .expect("naming prior authority identity"),
            )
        })
        .expect("existing-target naming delete should execute");
    let inspection = workspace
        .inspect(&receipt)
        .expect("naming delete receipt should inspect");

    let naming = receipt
        .naming_mutation_evidence()
        .expect("receipt should retain naming removal evidence");
    assert_eq!(naming.family(), ForgeQueryNamingMutationFamily::Remove);
    assert_eq!(naming.outcome(), ForgeQueryNamingMutationOutcome::Removed);
    assert_eq!(
        naming.attachment_identity().as_str(),
        "persistent-name:task-1"
    );
    assert_eq!(
        naming
            .prior_authoritative_identity()
            .map(|identity| identity.as_str()),
        Some("authority:task-1")
    );
    assert_eq!(naming.target_authoritative_identity(), None);
    assert_eq!(
        naming.resolved_target_entity_identity(),
        Some(&seed.deltas()[0].entity_identity)
    );

    match inspection {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            let naming = inspection
                .naming_mutation_evidence()
                .expect("inspection should retain naming removal evidence");
            assert_eq!(naming.family(), ForgeQueryNamingMutationFamily::Remove);
            assert_eq!(naming.outcome(), ForgeQueryNamingMutationOutcome::Removed);
            assert_eq!(
                naming.attachment_identity().as_str(),
                "persistent-name:task-1"
            );
            assert_eq!(
                naming
                    .prior_authoritative_identity()
                    .map(|identity| identity.as_str()),
                Some("authority:task-1")
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
}

#[test]
fn naming_remove_denies_non_delete_family_typed_and_early() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.naming-remove-denial")
        .expect("task runtime should open a named workspace");

    let command = ForgeQueryWriteCommand::UpdateAspects {
        entity_identity: crate::memory_workspace::admit_authored_entity_label(
            "entity:0:1:0",
        ),
        aspects: Vec::new(),
        metadata: ForgeQueryMutationMetadata::default(),
        naming_intent: Some(ForgeQueryNamingMutationIntent::remove(
            crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_attachment(
                crate::runtime::ForgeQueryNamingAttachmentAuthorityLabel::new(
                    "persistent-name:task-1",
                )
                .expect("naming attachment authority label"),
            )
            .expect("naming attachment identity"),
            crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_prior_authority(
                crate::runtime::ForgeQueryNamingPriorAuthorityLabel::new("persistent-name:task-1")
                    .expect("naming prior authority label"),
            )
            .expect("naming prior authority identity"),
        )),
        continuity_intent: None,
    };
    let error = workspace
        .write(command)
        .expect_err("naming remove should deny on non-delete mutation family");

    match error {
        ForgeQueryRuntimeError::MutationNamingDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryNamingMutationDenialKind::RequiresDeleteFamily
            );
            assert!(!denial.denial_digest().is_empty());
        }
        other => panic!("expected typed naming denial, got {other:?}"),
    }
}

#[test]
fn preview_batch_symbolic_naming_preserves_typed_evidence() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.preview-naming")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.preview-naming-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-preview-naming-table")
        })
        .expect("live view should declare");

    let mut preview = workspace
        .preview(test_session_label("naming-preview"))
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
                        .expect("symbolic collection should build"),
                    |task| {
                        task.naming_attach_new_target(
                            crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_attachment(
                                crate::runtime::ForgeQueryNamingAttachmentAuthorityLabel::new(
                                    "persistent-name:draft",
                                )
                                .expect("naming attachment authority label"),
                            )
                            .expect("naming attachment identity"),
                        )
                        .aspect("title.value", "Preview named")
                    },
                )
        })
        .expect("preview naming batch should execute");

    let naming = receipt.write_receipts()[1]
        .naming_mutation_evidence()
        .expect("preview symbolic component should retain naming evidence");
    assert_eq!(
        naming.outcome(),
        ForgeQueryNamingMutationOutcome::AttachedToNewTarget
    );
    assert_eq!(
        naming.attachment_identity().as_str(),
        "persistent-name:draft"
    );
}
