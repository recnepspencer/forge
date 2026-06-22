use super::super::support::*;

#[test]
fn verify_existing_preserves_backend_verified_assertion_evidence_without_mutation_deltas() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.verify-existing")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.verify-existing-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("tasks-verify-existing-table")
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
        .verify_existing(binding, |task| {
            task.aspect(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Seed title"),
            )
        })
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
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.verify-existing-missing-aspect-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("tasks-verify-existing-missing-aspect-table")
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
            task.aspect(
                test_aspect_touch("status.value"),
                test_string_aspect_value("open"),
            )
        })
        .expect_err("missing asserted aspect should deny");

    match error {
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect
            );
            assert_eq!(
                denial.asserted_aspect_touch(),
                Some(&test_aspect_touch("status.value"))
            );
            assert_eq!(
                denial.expected_native_value_digest(),
                Some("status:value=set:string:4:open")
            );
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }
}

#[test]
fn verify_existing_denies_mismatched_value_typed_and_early() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.verify-existing-mismatch")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.verify-existing-mismatch-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("tasks-verify-existing-mismatch-table")
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
            task.aspect(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Different title"),
            )
        })
        .expect_err("mismatched asserted value should deny");

    match error {
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch
            );
            assert_eq!(
                denial.asserted_aspect_touch(),
                Some(&test_aspect_touch("title.value"))
            );
            assert_eq!(
                denial.expected_native_value_digest(),
                Some("title:value=set:string:15:Different title")
            );
            assert_eq!(
                denial.found_native_value_digest(),
                Some("string:10:Seed title")
            );
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }
}

#[test]
fn verify_existing_reports_the_actual_failing_aspect_in_multi_aspect_requests() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.verify-existing-multi-mismatch")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.verify-existing-multi-mismatch-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("tasks-verify-existing-multi-mismatch-table")
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
            task.aspect(
                test_aspect_touch("identity.id"),
                test_string_aspect_value("task-1"),
            )
            .aspect(
                test_aspect_touch("status.value"),
                test_string_aspect_value("open"),
            )
            .aspect(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Seed title"),
            )
        })
        .expect_err("missing asserted aspect should deny");

    match error {
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect
            );
            assert_eq!(
                denial.asserted_aspect_touch(),
                Some(&test_aspect_touch("status.value"))
            );
            assert_eq!(
                denial.expected_native_value_digest(),
                Some("status:value=set:string:4:open")
            );
        }
        other => panic!("expected typed assertion denial, got {other:?}"),
    }
}
