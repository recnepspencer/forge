use super::super::support::*;

#[test]
fn update_existing_preserves_authoritative_binding_evidence() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.update-existing")
        .expect("task runtime should open a named workspace");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.update-existing-table", |q| {
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
                .schema_basis("tasks-update-existing-table")
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
                test_authored_string_aspect_value("Before existing update"),
            )
        })
        .expect("seed insert should execute");
    let binding_authority =
        crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-1")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity");
    let binding = workspace
        .bind_existing_entity(
            WorthQueryExistingEntityTarget::new(
                binding_authority.clone(),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let receipt = workspace
        .update_existing(binding, |task| {
            task.set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("After existing update"),
            )
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
        WorthQueryExistingTruthBindingFamily::DirectEntityIdentity
    );
    assert_eq!(
        evidence.outcome(),
        WorthQueryExistingTruthBindingOutcome::ExistingAuthoritativeTarget
    );
    assert_eq!(
        evidence.authoritative_identity().as_str(),
        "authority:task-1"
    );
    assert_eq!(
        evidence.resolved_entity_identity(),
        &seed.deltas()[0].entity_identity
    );
    assert_eq!(
        evidence
            .target_collection()
            .map(|collection| collection.as_str()),
        Some("Task")
    );
    assert!(!evidence.binding_digest().is_empty());

    match inspection {
        WorthQueryInspection::WriteReceipt(inspection) => {
            let evidence = inspection
                .existing_truth_binding_evidence()
                .expect("inspection should retain existing-truth evidence");
            assert_eq!(
                evidence.authoritative_identity().as_str(),
                "authority:task-1"
            );
            assert_eq!(
                evidence.resolved_entity_identity(),
                &seed.deltas()[0].entity_identity
            );
            assert_eq!(
                evidence
                    .target_collection()
                    .map(|collection| collection.as_str()),
                Some("Task")
            );
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
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.update-existing-denial")
        .expect("task runtime should open a named workspace");
    let binding = workspace
        .bind_existing_entity(
            WorthQueryExistingEntityTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                test_entity_identity("task:missing"),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = workspace
        .update_existing(binding, |task| {
            task.set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("No target"),
            )
        })
        .expect_err("missing existing target should deny early");

    match error {
        WorthQueryRuntimeError::MutationBindingDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryExistingTruthBindingDenialKind::ResolvedTargetMissing
            );
            assert!(!denial.denial_digest().is_empty());
        }
        other => panic!("expected typed mutation binding denial, got {other:?}"),
    }
}

#[test]
fn batch_existing_targets_preserve_component_and_aggregate_binding_evidence() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.batch-existing")
        .expect("task runtime should open a named workspace");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.batch-existing-table", |q| {
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
                .schema_basis("tasks-batch-existing-table")
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

    let binding_one_authority =
        crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-1")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity");
    let binding_two_authority =
        crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-2")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity");
    let binding_one = workspace
        .bind_existing_entity(
            WorthQueryExistingEntityTarget::new(
                binding_one_authority.clone(),
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
                binding_two_authority.clone(),
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
                .update_existing(binding_one, |task| {
                    task.set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value("First renamed"),
                    )
                })
                .delete_existing_with(binding_two, |delete| {
                    delete.touch(test_aspect_touch("title.value"))
                })
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
        WorthQueryInspection::BatchWriteReceipt(inspection) => {
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
                    .authoritative_identity()
                    .as_str(),
                "authority:task-1"
            );
            assert_eq!(
                inspection.component_operations()[1]
                    .existing_truth_binding_evidence()
                    .expect("second component should retain existing binding")
                    .authoritative_identity()
                    .as_str(),
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
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.batch-existing-symbolic")
        .expect("task runtime should open a named workspace");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.batch-existing-symbolic-table", |q| {
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
                .schema_basis("tasks-batch-existing-symbolic-table")
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
        .expect("seed should execute");
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
                .update_existing(binding, |task| {
                    task.set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value("Existing renamed"),
                    )
                })
                .update_symbolic(
                    WorthQuerySymbolicTargetReference::new("draft-task")
                        .expect("symbolic reference should build")
                        .in_target_collection("Task")
                        .expect("symbolic collection should build"),
                    |task| {
                        task.set_aspect(
                            test_aspect_touch("title.value"),
                            test_authored_string_aspect_value("Draft renamed"),
                        )
                    },
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

#[test]
fn delete_existing_relation_preserves_relation_binding_family() {
    let runtime = stateful_bridge_task_relation_runtime();
    let mut workspace = runtime
        .workspace("tasks.delete-existing-relation")
        .expect("workspace should open");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.relation-table", |q| {
            q.from("TaskRelation")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("kind", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("kind", "value")
                        .unwrap(),
                )
                .schema_basis("tasks-relation-table")
        })
        .expect("relation live view should declare");

    let seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("rel-1"),
                )
                .set_aspect(
                    test_aspect_touch("kind.value"),
                    test_authored_string_aspect_value("depends_on"),
                )
        })
        .expect("seed insert should execute");
    let binding_authority =
        crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-1")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity");
    let binding = workspace
        .bind_existing_relation(
            WorthQueryExistingRelationTarget::new(
                binding_authority.clone(),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("relation binding should build");

    let receipt = workspace
        .delete_existing(binding)
        .expect("existing relation delete should execute");
    let evidence = receipt
        .existing_truth_binding_evidence()
        .expect("receipt should retain relation binding evidence");

    assert_eq!(
        evidence.family(),
        WorthQueryExistingTruthBindingFamily::DirectRelationIdentity
    );
    assert_eq!(
        evidence.authoritative_identity().as_str(),
        "authority:task-1"
    );
    assert_eq!(
        evidence.resolved_relation_identity(),
        &seed.deltas()[0].entity_identity
    );
    assert_eq!(
        evidence.resolved_target_identity(),
        &seed.deltas()[0].entity_identity
    );
    assert_eq!(
        evidence
            .target_collection()
            .map(|collection| collection.as_str()),
        Some("TaskRelation")
    );
}
