use super::super::support::*;

fn task_relation_runtime() -> ForgeQueryRuntime {
    stateful_bridge_task_relation_runtime()
}

#[test]
fn update_existing_relation_preserves_identity_binding_and_receipt_target() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.update-existing-relation")
        .expect("workspace should open");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.update-existing-relation-table", |q| {
            q.from("TaskRelation")
                .select(["identity.id", "kind.value", "status.value"])
                .order_by("kind.value")
                .schema_basis("tasks-update-existing-relation-table")
        })
        .expect("relation live view should declare");

    let seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect("identity.id", "rel-1")
                .aspect("kind.value", "depends_on")
                .aspect("status.value", "open")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                "authority:rel-1",
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("relation binding should build");

    let receipt = workspace
        .update_existing(binding, |relation| relation.aspect("kind.value", "blocks"))
        .expect("relation update should execute");

    assert_eq!(receipt.mutation_family(), ForgeQueryMutationFamily::Update);
    assert_eq!(receipt.target_collection(), Some("TaskRelation"));
    assert_eq!(
        receipt.target_entity_identity(),
        Some(seed.deltas()[0].entity_identity.as_str())
    );
    let evidence = receipt
        .existing_truth_binding_evidence()
        .expect("relation update should retain binding evidence");
    assert_eq!(
        evidence.family(),
        ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity
    );
    assert_eq!(evidence.authoritative_identity(), "authority:rel-1");
    assert_eq!(
        evidence.resolved_relation_identity(),
        seed.deltas()[0].entity_identity
    );

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            let inspected = inspection
                .existing_truth_binding_evidence()
                .expect("inspection should retain relation binding evidence");
            assert_eq!(
                inspected.family(),
                ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity
            );
            assert_eq!(inspected.authoritative_identity(), "authority:rel-1");
            assert_eq!(
                inspected.resolved_relation_identity(),
                seed.deltas()[0].entity_identity
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
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.update-existing-verified-relation-table", |q| {
            q.from("TaskRelation")
                .select(["identity.id", "kind.value", "status.value"])
                .order_by("kind.value")
                .schema_basis("tasks-update-existing-verified-relation-table")
        })
        .expect("relation live view should declare");

    let seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect("identity.id", "rel-1")
                .aspect("kind.value", "depends_on")
                .aspect("status.value", "open")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                "authority:rel-1",
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
            |relation| relation.aspect("status.value", "open"),
            |relation| relation.aspect("status.value", "closed"),
        )
        .expect("verified relation update should execute");

    assert_eq!(receipt.mutation_family(), ForgeQueryMutationFamily::Update);
    assert_eq!(receipt.target_collection(), Some("TaskRelation"));
    assert_eq!(
        receipt.target_entity_identity(),
        Some(seed.deltas()[0].entity_identity.as_str())
    );
    assert_eq!(
        receipt
            .existing_truth_binding_evidence()
            .expect("verified update should retain binding evidence")
            .family(),
        ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity
    );
    assert_eq!(
        receipt
            .existing_truth_assertion_evidence()
            .expect("verified update should retain assertion evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            assert_eq!(
                inspection
                    .existing_truth_binding_evidence()
                    .expect("inspection should retain relation binding evidence")
                    .family(),
                ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity
            );
            assert_eq!(
                inspection
                    .existing_truth_assertion_evidence()
                    .expect("inspection should retain verified assertion evidence")
                    .mode(),
                ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
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
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.batch-existing-relation-update-table", |q| {
            q.from("TaskRelation")
                .select(["identity.id", "kind.value", "status.value"])
                .order_by("kind.value")
                .schema_basis("tasks-batch-existing-relation-update-table")
        })
        .expect("relation live view should declare");

    let first = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect("identity.id", "rel-1")
                .aspect("kind.value", "depends_on")
                .aspect("status.value", "open")
        })
        .expect("first seed should execute");
    let second = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect("identity.id", "rel-2")
                .aspect("kind.value", "blocks")
                .aspect("status.value", "open")
        })
        .expect("second seed should execute");

    let first_binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                "authority:rel-1",
                first.deltas()[0].entity_identity.clone(),
            )
            .expect("first relation target should build")
            .in_target_collection("TaskRelation")
            .expect("first relation collection should build"),
        )
        .expect("first relation binding should build");
    let second_binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                "authority:rel-2",
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
                    relation.aspect("status.value", "closed")
                })
                .update_existing(second_binding, |relation| {
                    relation.aspect("kind.value", "follows")
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
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
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
                ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity
            );
            assert_eq!(
                inspection.component_operations()[1]
                    .existing_truth_binding_evidence()
                    .expect("second relation update should retain binding evidence")
                    .family(),
                ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity
            );
        }
        other => panic!("expected batch write receipt inspection, got {other:?}"),
    }
}

#[test]
fn update_existing_verified_relation_denies_unsupported_backend_typed_and_early() {
    let runtime = bridge_runtime_with_support(ForgeQueryRuntimeSupportProfile::bridge_backed(
        "test-relation-live",
        "test-relation-preview",
        "test-relation-inspect",
    ));
    let mut workspace = runtime
        .workspace("tasks.update-existing-verified-relation-unsupported")
        .expect("workspace should open");
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new("authority:rel-1", "TaskRelation:1")
                .expect("existing relation target should build")
                .in_target_collection("TaskRelation")
                .expect("existing relation collection should build"),
        )
        .expect("relation binding should build");

    let error = workspace
        .update_existing_verified(
            binding,
            |relation| relation.aspect("status.value", "open"),
            |relation| relation.aspect("status.value", "closed"),
        )
        .expect_err("unsupported bridge-backed verified relation update should deny");

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
