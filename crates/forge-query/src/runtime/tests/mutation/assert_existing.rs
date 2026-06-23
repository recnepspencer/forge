use super::super::support::*;

#[test]
fn assert_existing_preserves_binding_evidence_without_mutation_deltas() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.assert-existing")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.assert-existing-table", |q| {
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
                .schema_basis("tasks-assert-existing-table")
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
        .assert_existing(binding, |task| {
            task.set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("Seed title"),
            )
        })
        .expect("existing-truth assertion should execute");

    assert_eq!(
        receipt.mutation_family(),
        ForgeQueryMutationFamily::Assertion
    );
    assert!(receipt.deltas().is_empty());
    assert_eq!(
        receipt.target_entity_identity(),
        Some(&seed.deltas()[0].entity_identity)
    );
    assert_eq!(
        receipt.terminal_target_collection_projection(),
        Some("Task")
    );
    assert!(receipt.declared_aspect_value_digest().is_some());

    let binding = receipt
        .existing_truth_binding_evidence()
        .expect("assertion should retain existing-binding evidence");
    assert_eq!(
        binding.family(),
        ForgeQueryExistingTruthBindingFamily::DirectEntityIdentity
    );
    assert_eq!(
        binding.authoritative_identity().as_str(),
        "authority:task-1"
    );
    let assertion = receipt
        .existing_truth_assertion_evidence()
        .expect("assertion should retain assertion evidence");
    assert_eq!(
        assertion.mode(),
        ForgeQueryExistingTruthAssertionMode::RetainedAuthoritativeAssertion
    );

    let inspection = workspace
        .inspect(&receipt)
        .expect("assertion receipt should inspect");
    match inspection {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            assert!(inspection.live_patch_artifacts().is_empty());
            assert_eq!(
                inspection
                    .existing_truth_assertion_evidence()
                    .expect("inspection should retain assertion evidence")
                    .mode(),
                ForgeQueryExistingTruthAssertionMode::RetainedAuthoritativeAssertion
            );
            assert_eq!(
                inspection.declared_aspect_value_digest(),
                receipt.declared_aspect_value_digest()
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
}

#[test]
fn assert_existing_inspection_digest_changes_with_asserted_value() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.assert-existing-digest")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.assert-existing-digest-table", |q| {
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
                .schema_basis("tasks-assert-existing-digest-table")
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
        })
        .expect("seed insert should execute");

    let left_binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("left binding should build");
    let right_binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("right binding should build");

    let left = workspace
        .assert_existing(left_binding, |task| {
            task.set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("Seed title"),
            )
        })
        .expect("left assertion should execute");
    let right = workspace
        .assert_existing(right_binding, |task| {
            task.set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("Different title"),
            )
        })
        .expect("right assertion should execute");

    assert_ne!(
        left.declared_aspect_value_digest(),
        right.declared_aspect_value_digest()
    );

    let left_inspection = workspace
        .inspect(&left)
        .expect("left assertion should inspect");
    let right_inspection = workspace
        .inspect(&right)
        .expect("right assertion should inspect");

    match (left_inspection, right_inspection) {
        (ForgeQueryInspection::WriteReceipt(left), ForgeQueryInspection::WriteReceipt(right)) => {
            assert_ne!(left.inspection_digest(), right.inspection_digest())
        }
        other => panic!("expected paired write receipt inspections, got {other:?}"),
    }
}

#[test]
fn batch_assert_existing_mixes_with_existing_delete_and_retains_binding_evidence() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.batch-assert-existing")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.batch-assert-existing-table", |q| {
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
                .schema_basis("tasks-batch-assert-existing-table")
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
                .assert_existing(binding_one, |task| {
                    task.set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value("First"),
                    )
                })
                .delete_existing_with(binding_two, |delete| {
                    delete.touch(test_aspect_touch("title.value"))
                })
        })
        .expect("mixed existing-target batch should execute");

    assert_eq!(receipt.write_count(), 2);
    assert_eq!(
        receipt.write_receipts()[0].mutation_family(),
        ForgeQueryMutationFamily::Assertion
    );
    assert!(receipt.write_receipts()[0].deltas().is_empty());
    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .existing_truth_assertion_count(),
        1
    );
    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .existing_truth_binding_count(),
        2
    );
    assert!(receipt
        .batch_mutation_evidence()
        .aggregate_existing_truth_assertion_digest()
        .is_some());
}

#[test]
fn preview_assert_existing_requires_authoritative_lane() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.preview-assert-existing")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.preview-assert-existing-table", |q| {
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
                .schema_basis("tasks-preview-assert-existing-table")
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
        })
        .expect("seed insert should execute");
    let mut preview = workspace
        .preview(test_session_label("assert-existing-preview"))
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
        .assert_existing(binding, |task| {
            task.set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("Seed title"),
            )
        })
        .expect_err("preview assertion should require authoritative lane");

    match error.stop_class() {
        ForgeQueryStopClass::ExistingTruthAssertionRequiresAuthorityLane { required_lane } => {
            assert_eq!(required_lane, ForgeQueryAuthorityLane::AuthoritativeTruth);
        }
        other => panic!("expected typed authoritative lane stop class, got {other:?}"),
    }
}
