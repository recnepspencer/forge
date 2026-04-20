use super::*;

#[test]
fn repeated_bulk_recovery_loops_converge_after_hosted_result_with_checkpoint_intent() {
    let path = unique_test_store_path("forge-store-bulk-repeat-hosted-checkpoint");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .expect("bulk store should build");
    let (plan, envelope, _runtime_session_id, durable_mutation_id) =
        prepare_pending_bulk_ingest_mutation(
            &mut store,
            "bulk-program-repeat-hosted",
            "bulk-source-repeat-hosted",
            true,
        );
    drop(store);

    let recovered_once = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("first recovery should complete");
    assert!(recovered_once
        .last_recovery()
        .decisions
        .iter()
        .any(|decision| {
            decision.durable_mutation_id == durable_mutation_id
                && decision.decision == RecoveryDecisionClass::FinishPublicationFromCanonicalResult
        }));
    assert_eq!(
        recovered_once
            .store()
            .fetch_program_chunk_witness_index("bulk-program-repeat-hosted", plan.plan_id())
            .expect("witness index should be reconstructed")
            .latest_checkpoint_sequence(),
        Some(1)
    );
    assert_eq!(
        recovered_once.resolve_retry(durable_mutation_id),
        Ok(
            crate::DurableRetryResolution::PreviouslyAcknowledgedEquivalentCommit {
                commit_id: envelope.commit.commit_id
            }
        )
    );
    let export_once = recovered_once.store().export_authoritative_records();
    drop(recovered_once);

    let recovered_twice = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("second recovery handle should build")
        .recover()
        .expect("second recovery should complete");
    assert!(recovered_twice.last_recovery().decisions.is_empty());
    assert_eq!(
        export_once.canonical_json(),
        recovered_twice
            .store()
            .export_authoritative_records()
            .canonical_json()
    );
}

#[test]
fn repeated_bulk_recovery_loops_converge_after_published_truth_with_existing_witness_only() {
    let path = unique_test_store_path("forge-store-bulk-repeat-published-witness");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .expect("bulk store should build");
    let (plan, envelope, runtime_session_id, durable_mutation_id) =
        prepare_pending_bulk_ingest_mutation(
            &mut store,
            "bulk-program-repeat-published-witness",
            "bulk-source-repeat-published-witness",
            true,
        );
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::AuthoritativeAppendPublished,
            Some(envelope.commit.commit_id),
        )
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 1)
        .unwrap();
    store
        .publish_bulk_chunk_witness(&admitted, envelope.commit.commit_id)
        .unwrap();
    drop(store);

    let recovered_once = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("first recovery should complete");
    assert!(recovered_once
        .last_recovery()
        .decisions
        .iter()
        .any(|decision| {
            decision.durable_mutation_id == durable_mutation_id
                && decision.decision == RecoveryDecisionClass::RetainPublishedTruth
        }));
    let witness_index = recovered_once
        .store()
        .fetch_program_chunk_witness_index("bulk-program-repeat-published-witness", plan.plan_id())
        .expect("witness index should exist");
    assert_eq!(
        witness_index.highest_committed_chunk_ordinal(),
        ChunkOrdinal::new(0)
    );
    assert_eq!(witness_index.latest_checkpoint_sequence(), Some(1));
    let export_once = recovered_once.store().export_authoritative_records();
    drop(recovered_once);

    let recovered_twice = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("second recovery handle should build")
        .recover()
        .expect("second recovery should complete");
    assert!(recovered_twice.last_recovery().decisions.is_empty());
    assert_eq!(
        export_once.canonical_json(),
        recovered_twice
            .store()
            .export_authoritative_records()
            .canonical_json()
    );
    assert_eq!(
        recovered_twice
            .store()
            .fetch_bulk_progress_checkpoint("bulk-program-repeat-published-witness", plan.plan_id())
            .expect("checkpoint should be reconstructed once")
            .checkpoint_sequence(),
        1
    );
}

#[test]
fn repeated_bulk_recovery_loops_converge_after_published_truth_with_existing_witness_and_checkpoint(
) {
    let path = unique_test_store_path("forge-store-bulk-repeat-published-checkpoint");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .expect("bulk store should build");
    let (plan, envelope, runtime_session_id, durable_mutation_id) =
        prepare_pending_bulk_ingest_mutation(
            &mut store,
            "bulk-program-repeat-published-checkpoint",
            "bulk-source-repeat-published-checkpoint",
            true,
        );
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::AuthoritativeAppendPublished,
            Some(envelope.commit.commit_id),
        )
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 1)
        .unwrap();
    let witness = store
        .publish_bulk_chunk_witness(&admitted, envelope.commit.commit_id)
        .unwrap();
    let checkpoint = store.publish_bulk_progress_checkpoint(&witness).unwrap();
    assert_eq!(checkpoint.checkpoint_sequence(), 1);
    drop(store);

    let recovered_once = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("first recovery should complete");
    assert!(recovered_once
        .last_recovery()
        .decisions
        .iter()
        .any(|decision| {
            decision.durable_mutation_id == durable_mutation_id
                && decision.decision == RecoveryDecisionClass::RetainPublishedTruth
        }));
    assert_eq!(
        recovered_once
            .store()
            .fetch_program_chunk_witness_index(
                "bulk-program-repeat-published-checkpoint",
                plan.plan_id()
            )
            .expect("witness index should exist")
            .latest_checkpoint_sequence(),
        Some(1)
    );
    let export_once = recovered_once.store().export_authoritative_records();
    drop(recovered_once);

    let recovered_twice = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("second recovery handle should build")
        .recover()
        .expect("second recovery should complete");
    assert!(recovered_twice.last_recovery().decisions.is_empty());
    assert_eq!(
        export_once.canonical_json(),
        recovered_twice
            .store()
            .export_authoritative_records()
            .canonical_json()
    );
    assert_eq!(
        recovered_twice
            .store()
            .fetch_bulk_progress_checkpoint(
                "bulk-program-repeat-published-checkpoint",
                plan.plan_id()
            )
            .expect("checkpoint should still be singular and present")
            .checkpoint_sequence(),
        1
    );
}
