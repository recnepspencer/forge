use super::*;

pub(super) fn milestone_3_5_failures() -> Vec<ObservedPublicationFailure> {
    let record =
        crate::wal::WalRecord::durable_mutation_intent(1, crate::DurableMutationId(77), "rt", "x")
            .unwrap();
    let classified = record
        .classify_media_barrier(crate::DurabilityBarrierClass::FileContentDurable)
        .unwrap();
    let framed = classified.record().framed_record().as_bytes().to_vec();
    let truncated_error =
        crate::wal::WalRecord::decode_from_media_bytes(framed[..framed.len() - 5].to_vec())
            .unwrap_err();

    let mut torn_bytes = framed.clone();
    let payload_index = torn_bytes.iter().position(|byte| *byte == b'r').unwrap();
    torn_bytes[payload_index] = b'X';
    let torn_error = crate::wal::WalRecord::decode_from_media_bytes(torn_bytes).unwrap_err();
    let source_error = StoreError::external_runtime_artifact_rejection(
        "integrity-valid family failed local source admission",
    );

    vec![
        ObservedPublicationFailure::from_error(&truncated_error),
        ObservedPublicationFailure::from_error(&torn_error),
        ObservedPublicationFailure::from_error(&source_error),
    ]
}

pub(super) fn quarantined_recovery_handle() -> crate::DurableStoreHandle {
    let path = unique_test_store_path("worth-store-m36-quarantine-certification");
    let mut durable = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();
    durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .unwrap();
    drop(durable);
    force_branch_head_gap(&path);
    WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap()
}

pub(super) fn retained_without_ack_recovery_handle() -> crate::DurableStoreHandle {
    let path = unique_test_store_path("worth-store-m36-retained-without-ack");
    let mut durable = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();
    durable
        .execute_mutation_until_crash(
            DurableMutationRequest::new("create-alpha", create_alpha_commit),
            crate::modes::SimulatedCrashPoint::AfterAuthoritativeAppendPublished,
        )
        .unwrap();
    drop(durable);
    WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap()
}

pub(super) fn milestone_3_6_failures() -> Vec<ObservedRecoveryFailure356> {
    let path = unique_test_store_path("worth-store-m36-source-conflict");
    let mut durable = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();
    let acknowledged = durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .unwrap();
    drop(durable);

    crate::tests::harness::corruption::local_file::force_publication_commit_id_conflict(
        &path,
        worth_relational::facade::history::CommitId(
            acknowledged.persisted().envelope().commit.commit_id.0 + 999,
        ),
    );

    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap_err();

    vec![ObservedRecoveryFailure356::from_error(&error)]
}
