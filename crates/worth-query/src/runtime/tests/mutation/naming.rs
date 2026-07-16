use super::super::support::*;

#[test]
fn update_existing_preserves_naming_evidence_on_receipt_and_inspection() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.naming-existing")
        .expect("task runtime should open a named workspace");
    let _: WorthQueryLiveView<WorthQueryUnrefinedLiveShape> = workspace
        .live_view("tasks.naming-existing-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                )
                .schema_basis("tasks-naming-existing-table")
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
                test_authored_string_aspect_value("Named task"),
            )
        })
        .expect("seed insert should execute");
    let attachment_authority =
        crate::runtime::WorthQueryMutationAuthorityIdentity::naming_attachment(
            crate::runtime::WorthQueryNamingAttachmentAuthorityLabel::new("persistent-name:task-1")
                .expect("naming attachment authority label"),
        )
        .expect("naming attachment identity");
    let target_authority =
        crate::runtime::WorthQueryMutationAuthorityIdentity::naming_target_authority(
            crate::runtime::WorthQueryNamingTargetAuthorityLabel::new("persistent-name:task-1")
                .expect("naming target authority label"),
        )
        .expect("naming target authority identity");
    let binding_authority =
        crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-1")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity");
    let binding = WorthQueryExistingTruthTargetBinding::direct_entity(
        binding_authority.clone(),
        seed.deltas()[0].entity_identity.clone(),
    )
    .expect("binding should build")
    .in_target_collection("Task")
    .expect("binding collection should build");

    let receipt = workspace
        .update_existing(binding, |task| {
            task.naming_attach_existing_target(
                attachment_authority.clone(),
                target_authority.clone(),
            )
            .set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("Named task renamed"),
            )
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
        WorthQueryNamingMutationFamily::AttachExistingTarget
    );
    assert_eq!(
        naming.outcome(),
        WorthQueryNamingMutationOutcome::AttachedToExistingTarget
    );
    assert_eq!(
        naming.attachment_identity().as_str(),
        expected_bridge_naming_attachment_label(&attachment_authority).as_str()
    );
    assert_eq!(
        naming
            .target_authoritative_identity()
            .map(|identity| identity.as_str()),
        Some(expected_bridge_naming_authority_label(&binding_authority).as_str())
    );
    assert_eq!(
        naming.resolved_target_entity_identity(),
        Some(&seed.deltas()[0].entity_identity)
    );

    match inspection {
        WorthQueryInspection::WriteReceipt(inspection) => {
            let naming = inspection
                .naming_mutation_evidence()
                .expect("inspection should retain naming evidence");
            assert_eq!(
                naming.attachment_identity().as_str(),
                expected_bridge_naming_attachment_label(&attachment_authority).as_str()
            );
            assert_eq!(
                naming
                    .target_authoritative_identity()
                    .map(|identity| identity.as_str()),
                Some(expected_bridge_naming_authority_label(&binding_authority).as_str())
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
    let _: WorthQueryLiveView<WorthQueryUnrefinedLiveShape> = workspace
        .live_view("tasks.naming-batch-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                )
                .schema_basis("tasks-naming-batch-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.set_aspect(
                test_aspect_touch("identity.id"),
                test_authored_string_aspect_value("task-existing"),
            )
            .set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("Existing"),
            )
        })
        .expect("seed insert should execute");
    let existing_binding = WorthQueryExistingTruthTargetBinding::direct_entity(
        crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-1")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity"),
        seed.deltas()[0].entity_identity.clone(),
    )
    .expect("existing binding should build")
    .in_target_collection("Task")
    .expect("existing binding collection should build");
    let symbolic = WorthQuerySymbolicTargetReference::new("draft-task")
        .expect("symbolic reference should build")
        .in_target_collection("Task")
        .expect("symbolic collection should build");

    let receipt = workspace
        .batch(|batch| {
            batch
                .insert_symbolic("draft-task", "Task", |task| {
                    task.set_aspect(test_aspect_touch("identity.id"), test_authored_string_aspect_value("task-draft"))
                        .set_aspect(test_aspect_touch("title.value"), test_authored_string_aspect_value("Draft"))
                })
                .update_symbolic(symbolic.clone(), |task| {
                    task.naming_attach_new_target(
                        crate::runtime::WorthQueryMutationAuthorityIdentity::naming_attachment(crate::runtime::WorthQueryNamingAttachmentAuthorityLabel::new(
                            "persistent-name:draft",
                        )
                        .expect("naming attachment authority label")).expect("naming attachment identity"),
                    )
                    .set_aspect(test_aspect_touch("title.value"), test_authored_string_aspect_value("Draft named"))
                })
                .update_existing(existing_binding, |task| {
                    task.naming_rebind_target(crate::runtime::WorthQueryMutationAuthorityIdentity::naming_attachment(crate::runtime::WorthQueryNamingAttachmentAuthorityLabel::new("persistent-name:task-existing").expect("naming attachment authority label")).expect("naming attachment identity"), crate::runtime::WorthQueryMutationAuthorityIdentity::naming_prior_authority(crate::runtime::WorthQueryNamingPriorAuthorityLabel::new("authority:task-existing-old").expect("naming prior authority label")).expect("naming prior authority identity"), crate::runtime::WorthQueryMutationAuthorityIdentity::naming_target_authority(crate::runtime::WorthQueryNamingTargetAuthorityLabel::new("authority:task-existing").expect("naming target authority label")).expect("naming target authority identity"),
                    )
                    .set_aspect(test_aspect_touch("title.value"), test_authored_string_aspect_value("Existing rebound"))
                })
        })
        .expect("naming batch should execute");
    let inspection = workspace.inspect(&receipt).expect("batch should inspect");

    assert_eq!(receipt.batch_mutation_evidence().naming_mutation_count(), 2);

    match inspection {
        WorthQueryInspection::BatchWriteReceipt(inspection) => {
            assert_eq!(
                inspection.batch_mutation_evidence().naming_mutation_count(),
                2
            );
            assert_eq!(
                inspection.component_operations()[1]
                    .naming_mutation_evidence()
                    .expect("symbolic naming component should retain naming evidence")
                    .outcome(),
                WorthQueryNamingMutationOutcome::AttachedToNewTarget
            );
            assert_eq!(
                inspection.component_operations()[2]
                    .naming_mutation_evidence()
                    .expect("existing naming component should retain naming evidence")
                    .outcome(),
                WorthQueryNamingMutationOutcome::ReboundTarget
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
                crate::runtime::WorthQueryMutationAuthorityIdentity::naming_attachment(
                    crate::runtime::WorthQueryNamingAttachmentAuthorityLabel::new(
                        "persistent-name:task-1",
                    )
                    .expect("naming attachment authority label"),
                )
                .expect("naming attachment identity"),
                crate::runtime::WorthQueryMutationAuthorityIdentity::naming_target_authority(
                    crate::runtime::WorthQueryNamingTargetAuthorityLabel::new(
                        "persistent-name:task-1",
                    )
                    .expect("naming target authority label"),
                )
                .expect("naming target authority identity"),
            )
            .set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("No binding"),
            )
        })
        .expect_err("naming attach-to-existing should deny without binding");

    match error {
        WorthQueryRuntimeError::MutationNamingDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryNamingMutationDenialKind::RequiresExistingTruthBinding
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
    let _: WorthQueryLiveView<WorthQueryUnrefinedLiveShape> = workspace
        .live_view("tasks.naming-remove-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                )
                .schema_basis("tasks-naming-remove-table")
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
                test_authored_string_aspect_value("Named task"),
            )
        })
        .expect("seed insert should execute");
    let attachment_authority =
        crate::runtime::WorthQueryMutationAuthorityIdentity::naming_attachment(
            crate::runtime::WorthQueryNamingAttachmentAuthorityLabel::new("persistent-name:task-1")
                .expect("naming attachment authority label"),
        )
        .expect("naming attachment identity");
    let prior_authority =
        crate::runtime::WorthQueryMutationAuthorityIdentity::naming_prior_authority(
            crate::runtime::WorthQueryNamingPriorAuthorityLabel::new("persistent-name:task-1")
                .expect("naming prior authority label"),
        )
        .expect("naming prior authority identity");
    let binding_authority =
        crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-1")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity");
    let binding = WorthQueryExistingTruthTargetBinding::direct_entity(
        binding_authority.clone(),
        seed.deltas()[0].entity_identity.clone(),
    )
    .expect("binding should build")
    .in_target_collection("Task")
    .expect("binding collection should build");

    let receipt = workspace
        .delete_existing_with(binding, |delete| {
            delete
                .touch(test_aspect_touch("title.value"))
                .naming_remove(attachment_authority.clone(), prior_authority.clone())
        })
        .expect("existing-target naming delete should execute");
    let inspection = workspace
        .inspect(&receipt)
        .expect("naming delete receipt should inspect");

    let naming = receipt
        .naming_mutation_evidence()
        .expect("receipt should retain naming removal evidence");
    assert_eq!(naming.family(), WorthQueryNamingMutationFamily::Remove);
    assert_eq!(naming.outcome(), WorthQueryNamingMutationOutcome::Removed);
    assert_eq!(
        naming.attachment_identity().as_str(),
        expected_bridge_naming_attachment_label(&attachment_authority).as_str()
    );
    assert_eq!(
        naming
            .prior_authoritative_identity()
            .map(|identity| identity.as_str()),
        Some(expected_bridge_naming_authority_label(&binding_authority).as_str())
    );
    assert_eq!(naming.target_authoritative_identity(), None);
    assert_eq!(
        naming.resolved_target_entity_identity(),
        Some(&seed.deltas()[0].entity_identity)
    );

    match inspection {
        WorthQueryInspection::WriteReceipt(inspection) => {
            let naming = inspection
                .naming_mutation_evidence()
                .expect("inspection should retain naming removal evidence");
            assert_eq!(naming.family(), WorthQueryNamingMutationFamily::Remove);
            assert_eq!(naming.outcome(), WorthQueryNamingMutationOutcome::Removed);
            assert_eq!(
                naming.attachment_identity().as_str(),
                expected_bridge_naming_attachment_label(&attachment_authority).as_str()
            );
            assert_eq!(
                naming
                    .prior_authoritative_identity()
                    .map(|identity| identity.as_str()),
                Some(expected_bridge_naming_authority_label(&binding_authority).as_str())
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

    let command = WorthQueryWriteCommand::UpdateAspects {
        entity_identity: crate::memory_workspace::admit_authored_entity_label("entity:0:1:0"),
        aspects: Vec::new(),
        metadata: WorthQueryMutationMetadata::default(),
        naming_intent: Some(WorthQueryNamingMutationIntent::remove(
            crate::runtime::WorthQueryMutationAuthorityIdentity::naming_attachment(
                crate::runtime::WorthQueryNamingAttachmentAuthorityLabel::new(
                    "persistent-name:task-1",
                )
                .expect("naming attachment authority label"),
            )
            .expect("naming attachment identity"),
            crate::runtime::WorthQueryMutationAuthorityIdentity::naming_prior_authority(
                crate::runtime::WorthQueryNamingPriorAuthorityLabel::new("persistent-name:task-1")
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
        WorthQueryRuntimeError::MutationNamingDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryNamingMutationDenialKind::RequiresDeleteFamily
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
    let _: WorthQueryLiveView<WorthQueryUnrefinedLiveShape> = workspace
        .live_view("tasks.preview-naming-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                )
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
                    task.set_aspect(
                        test_aspect_touch("identity.id"),
                        test_authored_string_aspect_value("task-draft"),
                    )
                    .set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value("Draft"),
                    )
                })
                .update_symbolic(
                    WorthQuerySymbolicTargetReference::new("draft-task")
                        .expect("symbolic reference should build")
                        .in_target_collection("Task")
                        .expect("symbolic collection should build"),
                    |task| {
                        task.naming_attach_new_target(
                            crate::runtime::WorthQueryMutationAuthorityIdentity::naming_attachment(
                                crate::runtime::WorthQueryNamingAttachmentAuthorityLabel::new(
                                    "persistent-name:draft",
                                )
                                .expect("naming attachment authority label"),
                            )
                            .expect("naming attachment identity"),
                        )
                        .set_aspect(
                            test_aspect_touch("title.value"),
                            test_authored_string_aspect_value("Preview named"),
                        )
                    },
                )
        })
        .expect("preview naming batch should execute");

    let naming = receipt.write_receipts()[1]
        .naming_mutation_evidence()
        .expect("preview symbolic component should retain naming evidence");
    assert_eq!(
        naming.outcome(),
        WorthQueryNamingMutationOutcome::AttachedToNewTarget
    );
    assert_eq!(
        naming.attachment_identity().as_str(),
        "persistent-name:draft"
    );
}
