use super::*;

#[test]
fn tail_replay_consumes_canonical_reused_record_allocation() {
    let mut runtime = persisted_runtime_with_test_schema_profile(
        crate::facade::config::RelationalRuntimeProfile::AiWorkflow,
    );
    let created = create_entity_outcome(&mut runtime, "allocation-before");
    let original = changed_entities(&created)[0];
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&created.snapshot));
    let deleted = delete_entity(&mut runtime, original);
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&deleted.snapshot));
    assert!(runtime.retention().run_pass().entity_reclaimed <= 1);
    assert_eq!(
        runtime
            .storage_access()
            .storage_stats()
            .reusable_entity_slots,
        1
    );
    runtime.durability_authority().checkpoint().unwrap();

    let replacement = create_entity(&mut runtime, "allocation-after");
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema_profile(
        crate::facade::config::RelationalRuntimeProfile::AiWorkflow,
    );
    recovered.durability_authority().recover(plan).unwrap();

    assert_eq!(replacement.local_slot, original.local_slot);
    assert!(replacement.generation.0 > original.generation.0);
    let current = recovered
        .read_truth()
        .read_version(recovered.current_version_id());
    assert_eq!(
        read_entity_name(current.get_entity(replacement).unwrap()),
        Some("allocation-after".into())
    );
    let historical = recovered.read_truth().read_version(created.version_id);
    assert_eq!(
        read_entity_name(historical.get_entity(original).unwrap()),
        Some("allocation-before".into())
    );
}

#[test]
fn checkpoint_restores_burned_append_frontier_beyond_all_retained_roots() {
    let mut runtime = persisted_runtime_with_test_schema();
    let original = create_entity(&mut runtime, "frontier-root");
    let mut burned = runtime.record_identity.begin_allocations();
    let reservation = burned
        .reserve(
            crate::history::data::RecordAllocationClass::Entity,
            original.partition_id,
        )
        .unwrap();
    assert_eq!(reservation.slot as u64, original.local_slot.0 + 1);
    drop(burned);
    runtime.durability_authority().checkpoint().unwrap();

    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();
    let after_recovery = create_entity(&mut recovered, "frontier-after-recovery");

    assert_eq!(after_recovery.local_slot.0, original.local_slot.0 + 2);
    assert_eq!(after_recovery.generation.0, 1);
}

#[test]
fn tail_replay_rejects_canonical_allocation_targeting_a_live_slot() {
    let mut runtime = persisted_runtime_with_test_schema();
    let original = create_entity(&mut runtime, "allocation-live");
    runtime.durability_authority().checkpoint().unwrap();
    create_entity(&mut runtime, "allocation-tail");
    let mut plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let tail = plan.tail_log.last_mut().expect("tail commit is selected");
    let forged = crate::identity::data::EntityId::new(
        original.partition_id,
        original.local_slot.0,
        original.generation.0.saturating_add(1),
    );
    tail.install_record_allocations(vec![
        crate::history::data::CanonicalRecordAllocation::with_origin(
            0,
            crate::transactions::data::RecordRef::Entity(forged),
            crate::history::data::RecordAllocationOrigin::Reclaimed {
                prior_generation: original.generation.0,
            },
        ),
    ]);

    let mut recovered = persisted_runtime_with_test_schema();
    let error = recovered
        .durability_authority()
        .recover(plan)
        .expect_err("replay must not overwrite a live checkpoint slot");

    assert_eq!(error.class, RecoveryFailureClass::ReplayFailure);
    assert!(error.detail.contains("selects unavailable Entity"));
    assert_eq!(recovered.history().immutable_commit_count(), 0);
}

#[test]
fn tail_replay_rejects_missing_canonical_allocation_evidence() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "allocation-checkpoint");
    runtime.durability_authority().checkpoint().unwrap();
    create_entity(&mut runtime, "allocation-tail");
    let mut plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    plan.tail_log
        .last_mut()
        .expect("tail commit is selected")
        .install_record_allocations(Vec::new());

    let mut recovered = persisted_runtime_with_test_schema();
    let error = recovered
        .durability_authority()
        .recover(plan)
        .expect_err("replay must require canonical allocation evidence");

    assert_eq!(error.class, RecoveryFailureClass::ReplayFailure);
    assert!(error.detail.contains("missing ordinal 0"));
    assert_eq!(recovered.history().immutable_commit_count(), 0);
}

#[test]
fn metadata_only_merge_rejects_unconsumable_allocation_evidence() {
    let mut runtime = persisted_runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "allocation-shared");
    create_branch_from_main(&mut runtime, "allocation-feature");
    delete_entity(&mut runtime, entity);
    delete_entity_on_branch(
        &mut runtime,
        entity,
        BranchId("allocation-feature".to_owned()),
    );
    runtime.durability_authority().checkpoint().unwrap();
    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_owned()),
            source_branch: BranchId("allocation-feature".to_owned()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .unwrap();
    runtime.execute_prepared_merge(prepared).unwrap();
    let mut plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let tail = plan.tail_log.last_mut().expect("merge tail is selected");
    tail.install_record_allocations(vec![crate::history::data::CanonicalRecordAllocation::new(
        0,
        crate::transactions::data::RecordRef::Entity(entity),
    )]);

    let mut recovered = persisted_runtime_with_test_schema();
    let error = recovered
        .durability_authority()
        .recover(plan)
        .expect_err("a metadata-only merge cannot carry allocation evidence");

    assert_eq!(error.class, RecoveryFailureClass::ReplayFailure);
    assert!(error.detail.contains("without a mutation replay path"));
    assert_eq!(recovered.history().immutable_commit_count(), 0);
}
