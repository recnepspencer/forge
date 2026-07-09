use super::super::support::*;

fn task_relation_runtime() -> WorthQueryRuntime {
    stateful_bridge_task_relation_runtime()
}

#[test]
fn update_existing_relation_preserves_identity_binding_and_receipt_target() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.update-existing-relation")
        .expect("workspace should open");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.update-existing-relation-table", |q| {
            q.from("TaskRelation")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("kind", "value")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("status", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("kind", "value")
                        .unwrap(),
                )
                .schema_basis("tasks-update-existing-relation-table")
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
                .set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("open"),
                )
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_relation(
            WorthQueryExistingRelationTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:rel-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("relation binding should build");

    let receipt = workspace
        .update_existing(binding, |relation| {
            relation.set_aspect(
                test_aspect_touch("kind.value"),
                test_authored_string_aspect_value("blocks"),
            )
        })
        .expect("relation update should execute");

    assert_eq!(receipt.mutation_family(), WorthQueryMutationFamily::Update);
    assert_eq!(
        receipt.terminal_target_collection_projection(),
        Some("TaskRelation")
    );
    assert_eq!(
        receipt.target_entity_identity(),
        Some(&seed.deltas()[0].entity_identity)
    );
    let evidence = receipt
        .existing_truth_binding_evidence()
        .expect("relation update should retain binding evidence");
    assert_eq!(
        evidence.family(),
        WorthQueryExistingTruthBindingFamily::DirectRelationIdentity
    );
    assert_eq!(
        evidence.authoritative_identity().as_str(),
        "authority:rel-1"
    );
    assert_eq!(
        evidence.resolved_relation_identity(),
        &seed.deltas()[0].entity_identity
    );

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        WorthQueryInspection::WriteReceipt(inspection) => {
            let inspected = inspection
                .existing_truth_binding_evidence()
                .expect("inspection should retain relation binding evidence");
            assert_eq!(
                inspected.family(),
                WorthQueryExistingTruthBindingFamily::DirectRelationIdentity
            );
            assert_eq!(
                inspected.authoritative_identity().as_str(),
                "authority:rel-1"
            );
            assert_eq!(
                inspected.resolved_relation_identity(),
                &seed.deltas()[0].entity_identity
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
}

#[test]
fn update_existing_verified_relation_preserves_relation_identity_and_assertion_mode() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.update-existing-verified-relation")
        .expect("workspace should open");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.update-existing-verified-relation-table", |q| {
            q.from("TaskRelation")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("kind", "value")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("status", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("kind", "value")
                        .unwrap(),
                )
                .schema_basis("tasks-update-existing-verified-relation-table")
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
                .set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("open"),
                )
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_relation(
            WorthQueryExistingRelationTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:rel-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("relation binding should build");

    let receipt = workspace
        .update_existing_verified(
            binding,
            |relation| {
                relation.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("open"),
                )
            },
            |relation| {
                relation.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("closed"),
                )
            },
        )
        .expect("verified relation update should execute");

    assert_eq!(receipt.mutation_family(), WorthQueryMutationFamily::Update);
    assert_eq!(
        receipt.terminal_target_collection_projection(),
        Some("TaskRelation")
    );
    assert_eq!(
        receipt.target_entity_identity(),
        Some(&seed.deltas()[0].entity_identity)
    );
    assert_eq!(
        receipt
            .existing_truth_binding_evidence()
            .expect("verified update should retain binding evidence")
            .family(),
        WorthQueryExistingTruthBindingFamily::DirectRelationIdentity
    );
    assert_eq!(
        receipt
            .existing_truth_assertion_evidence()
            .expect("verified update should retain assertion evidence")
            .mode(),
        WorthQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        WorthQueryInspection::WriteReceipt(inspection) => {
            assert_eq!(
                inspection
                    .existing_truth_binding_evidence()
                    .expect("inspection should retain relation binding evidence")
                    .family(),
                WorthQueryExistingTruthBindingFamily::DirectRelationIdentity
            );
            assert_eq!(
                inspection
                    .existing_truth_assertion_evidence()
                    .expect("inspection should retain verified assertion evidence")
                    .mode(),
                WorthQueryExistingTruthAssertionMode::BackendVerifiedAssertion
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
}

#[test]
fn batch_relation_updates_preserve_identity_binding_aggregate_digest() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.batch-existing-relation-update")
        .expect("workspace should open");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.batch-existing-relation-update-table", |q| {
            q.from("TaskRelation")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("kind", "value")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("status", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("kind", "value")
                        .unwrap(),
                )
                .schema_basis("tasks-batch-existing-relation-update-table")
        })
        .expect("relation live view should declare");

    let first = workspace
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
                .set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("open"),
                )
        })
        .expect("first seed should execute");
    let second = workspace
        .insert("TaskRelation", |relation| {
            relation
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("rel-2"),
                )
                .set_aspect(
                    test_aspect_touch("kind.value"),
                    test_authored_string_aspect_value("blocks"),
                )
                .set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("open"),
                )
        })
        .expect("second seed should execute");

    let first_binding = workspace
        .bind_existing_relation(
            WorthQueryExistingRelationTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:rel-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                first.deltas()[0].entity_identity.clone(),
            )
            .expect("first relation target should build")
            .in_target_collection("TaskRelation")
            .expect("first relation collection should build"),
        )
        .expect("first relation binding should build");
    let second_binding = workspace
        .bind_existing_relation(
            WorthQueryExistingRelationTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:rel-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                second.deltas()[0].entity_identity.clone(),
            )
            .expect("second relation target should build")
            .in_target_collection("TaskRelation")
            .expect("second relation collection should build"),
        )
        .expect("second relation binding should build");

    let receipt = workspace
        .batch(|batch| {
            batch
                .update_existing(first_binding, |relation| {
                    relation.set_aspect(
                        test_aspect_touch("status.value"),
                        test_authored_string_aspect_value("closed"),
                    )
                })
                .update_existing(second_binding, |relation| {
                    relation.set_aspect(
                        test_aspect_touch("kind.value"),
                        test_authored_string_aspect_value("follows"),
                    )
                })
        })
        .expect("relation batch should execute");

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

    match workspace.inspect(&receipt).expect("batch should inspect") {
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
                    .expect("first relation update should retain binding evidence")
                    .family(),
                WorthQueryExistingTruthBindingFamily::DirectRelationIdentity
            );
            assert_eq!(
                inspection.component_operations()[1]
                    .existing_truth_binding_evidence()
                    .expect("second relation update should retain binding evidence")
                    .family(),
                WorthQueryExistingTruthBindingFamily::DirectRelationIdentity
            );
        }
        other => panic!("expected batch write receipt inspection, got {other:?}"),
    }
}

#[test]
fn update_existing_verified_relation_denies_unsupported_backend_typed_and_early() {
    let runtime = bridge_runtime_with_support(WorthQueryRuntimeSupportProfile::bridge_backed(
        "test-relation-live",
        "test-relation-preview",
        "test-relation-inspect",
    ));
    let mut workspace = runtime
        .workspace("tasks.update-existing-verified-relation-unsupported")
        .expect("workspace should open");
    let binding = workspace
        .bind_existing_relation(
            WorthQueryExistingRelationTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:rel-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                test_entity_identity("TaskRelation:1"),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation collection should build"),
        )
        .expect("relation binding should build");

    let error = workspace
        .update_existing_verified(
            binding,
            |relation| {
                relation.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("open"),
                )
            },
            |relation| {
                relation.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("closed"),
                )
            },
        )
        .expect_err("unsupported bridge-backed verified relation update should deny");

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
