use super::*;

#[test]
fn complexity_budget_unique_entity_invariant_uses_changed_set_lookup() {
    let mut runtime = runtime_with_declared_aspect_schema_and_invariants(InvariantCatalog {
        registrations: vec![InvariantRegistration::mutation_sensitive_blocking(
            InvariantRule::unique_entity_aspect_field(aspect_key("name"), field_key("name")),
        )],
        ..InvariantCatalog::default()
    });
    let target = create_entity(&mut runtime, "target");
    let _other = create_entity(&mut runtime, "other");
    runtime
        .index_authority()
        .rebuild_unique_entity_aspect_field_indexes();

    runtime.performance_access().reset_counters();
    let mut txn = crate::tests::support::test_owner_begin_transaction_for_main(&mut runtime);
    txn.push_batch(
        WorkerIntentBatch::new("duplicate-name").push(MutationIntent::Entity(
            EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                entity_id: target,
                fields: crate::tests::support::single_string_aspect_field_patch(
                    crate::tests::support::aspect_key("name"),
                    crate::tests::support::field_key("name"),
                    "other",
                ),
            }),
        )),
    );
    let error = txn.commit().unwrap_err();
    let counters = runtime.performance_access().counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
            if conflict.code == DiagnosticCode::InvariantViolation
    ));
    assert_eq!(counters.invariant_entity_slot_scans, 1);
    assert_eq!(
        counters.invariant_authoritative_entity_records_materialized,
        0
    );
}

#[test]
fn complexity_budget_commit_boundary_unique_invariant_uses_merged_plan_lookup() {
    let mut runtime = runtime_with_declared_aspect_schema_and_invariants(InvariantCatalog {
        registrations: vec![InvariantRegistration::commit_boundary_blocking(
            InvariantRule::unique_entity_aspect_field(aspect_key("name"), field_key("name")),
        )],
        ..InvariantCatalog::default()
    });
    let target = create_entity(&mut runtime, "target");
    let _other = create_entity(&mut runtime, "other");
    runtime
        .index_authority()
        .rebuild_unique_entity_aspect_field_indexes();

    runtime.performance_access().reset_counters();
    let mut txn = crate::tests::support::test_owner_begin_transaction_for_main(&mut runtime);
    txn.push_batch(
        WorkerIntentBatch::new("duplicate-name").push(MutationIntent::Entity(
            EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                entity_id: target,
                fields: crate::tests::support::single_string_aspect_field_patch(
                    crate::tests::support::aspect_key("name"),
                    crate::tests::support::field_key("name"),
                    "other",
                ),
            }),
        )),
    );
    let error = txn.commit().unwrap_err();
    let counters = runtime.performance_access().counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
            if conflict.code == DiagnosticCode::InvariantViolation
    ));
    assert_eq!(counters.invariant_entity_slot_scans, 1);
    assert_eq!(
        counters.invariant_authoritative_entity_records_materialized,
        0
    );
}
