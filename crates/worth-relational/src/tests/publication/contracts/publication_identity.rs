use crate::capabilities::DurabilityRead;
use crate::tests::support::*;

#[test]
fn publication_rejects_envelope_identity_drift_before_any_effect() {
    let mut runtime = runtime_with_test_schema();
    let commit = create_entity_outcome(&mut runtime, "publication-identity");
    let receipt = commit.commit.clone();
    let envelope = commit.publication().envelope.clone();
    let identity = runtime
        .branch_identity(&BranchId("main".to_owned()))
        .expect("main identity");
    let binding = runtime
        .admitted_branch_basis_for_identity(&identity)
        .expect("main binding");
    let before_cells = runtime.history.branch_cells_snapshot();
    let before_catalog = runtime.history.commit_catalog.len();

    let mut mismatched_commit = envelope.as_ref().clone();
    mismatched_commit.commit.commit_id.0 += 100;
    let error = runtime
        .mvcc_publication_authority()
        .validate_versioned_publication(receipt.commit_id, &receipt, &binding, &mismatched_commit)
        .expect_err("envelope commit identity drift must be denied");
    assert!(error.contains("envelope commit identity mismatch"));
    assert_eq!(runtime.history.branch_cells_snapshot(), before_cells);
    assert_eq!(runtime.history.commit_catalog.len(), before_catalog);

    let mut mismatched_branch = envelope.as_ref().clone();
    mismatched_branch.branch_context = BranchId("other".to_owned());
    let error = runtime
        .mvcc_publication_authority()
        .validate_versioned_publication(receipt.commit_id, &receipt, &binding, &mismatched_branch)
        .expect_err("envelope branch context drift must be denied");
    assert!(error.contains("envelope branch context mismatch"));
    assert_eq!(runtime.history.branch_cells_snapshot(), before_cells);
    assert_eq!(runtime.history.commit_catalog.len(), before_catalog);
}

#[test]
fn root_capture_sabotage_leaves_storage_index_history_and_reference_unchanged() {
    let mut runtime = runtime_with_test_schema();
    let committed = create_entity_outcome(&mut runtime, "root-capture-sabotage");
    let current_receipt = committed.commit.clone();
    let identity = runtime
        .branch_identity(&BranchId("main".to_owned()))
        .expect("main identity");
    let binding = runtime
        .admitted_branch_basis_for_identity(&identity)
        .expect("main binding");
    let commit_id = runtime.history.preview_next_commit_id();
    let version_id = runtime.history.preview_next_version_id();
    let mut future_envelope = committed.publication().envelope.as_ref().clone();
    future_envelope.commit = crate::history::data::RelationalCommitReceipt {
        commit_id,
        version_id,
        branch_id: BranchId("main".to_owned()),
        parents: vec![current_receipt.commit_id],
    };

    let partition_id = *runtime
        .partitions
        .keys()
        .next()
        .expect("the committed entity installs one partition");
    let mut malformed_partition = runtime.partitions[&partition_id].clone();
    malformed_partition.entity_arena.aspect_versions[0]
        .insert(crate::symbols::data::Symbol(u32::MAX), 1);
    let mut journal = crate::storage::overlay::PartitionMutationJournal::default();
    journal.entity_slots.insert(0);
    let delta = crate::storage::RelationalPublishedPartitionDelta::from_committed_partitions(
        &std::collections::BTreeMap::from([(partition_id, (malformed_partition, journal))]),
    );

    let before_storage = runtime
        .partitions
        .iter()
        .map(|(id, partition)| {
            (
                *id,
                partition
                    .authoritative_content_digest(&runtime.services.symbols)
                    .expect("installed storage symbols resolve"),
            )
        })
        .collect::<Vec<_>>();
    let before_index = runtime.indexes.entity_unique_aspect_field_index.clone();
    let before_cells = runtime.history.branch_cells_snapshot();
    let before_catalog = runtime.history.commit_catalog.len();
    let before_envelopes = runtime.history.commit_envelopes.len();
    let before_patch_index = runtime.history.patch_stream_index.len();
    let before_sequences = (
        runtime.history.next_commit_id,
        runtime.history.next_version_id,
    );

    let selected_branch_state = runtime
        .selected_branch_state(&binding)
        .expect("current binding must select a root");
    let error = runtime
        .mvcc_publication_authority()
        .prepare_commit(
            commit_id,
            future_envelope.commit.clone(),
            &binding,
            &selected_branch_state,
            delta,
            std::sync::Arc::new(future_envelope),
        )
        .err()
        .expect("the unresolved owner symbol sabotages root capture");
    assert!(error.contains("UnresolvedContentSymbol"));
    let after_storage = runtime
        .partitions
        .iter()
        .map(|(id, partition)| {
            (
                *id,
                partition
                    .authoritative_content_digest(&runtime.services.symbols)
                    .expect("failed preparation cannot corrupt installed storage"),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(after_storage, before_storage);
    assert_eq!(
        runtime.indexes.entity_unique_aspect_field_index,
        before_index
    );
    assert_eq!(runtime.history.branch_cells_snapshot(), before_cells);
    assert_eq!(runtime.history.commit_catalog.len(), before_catalog);
    assert_eq!(runtime.history.commit_envelopes.len(), before_envelopes);
    assert_eq!(runtime.history.patch_stream_index.len(), before_patch_index);
    assert_eq!(
        (
            runtime.history.next_commit_id,
            runtime.history.next_version_id
        ),
        before_sequences
    );
}

#[test]
fn production_commit_root_capture_sabotage_precedes_durable_append_and_all_effects() {
    let mut runtime = runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "root-capture-production-anchor");
    let before_storage = runtime
        .partitions
        .iter()
        .map(|(id, partition)| {
            (
                *id,
                partition
                    .authoritative_content_digest(&runtime.services.symbols)
                    .expect("installed storage symbols resolve"),
            )
        })
        .collect::<Vec<_>>();
    let before_index = runtime.indexes.entity_unique_aspect_field_index.clone();
    let before_cells = runtime.history.branch_cells_snapshot();
    let before_catalog = runtime.history.commit_catalog.len();
    let before_durable = runtime.durable_log().len();
    let before_envelopes = runtime.history.commit_envelopes.len();
    let before_patch_index = runtime.history.patch_stream_index.len();
    let before_sequences = (
        runtime.history.next_commit_id,
        runtime.history.next_version_id,
    );

    runtime.history.sabotage_next_root_capture();
    let mut transaction = test_owner_begin_transaction_for_main(&mut runtime);
    transaction.push_batch(batch_create("root-capture-production-sabotage"));
    let error = transaction
        .commit(&mut runtime)
        .expect_err("the test-only root court must reject the real commit path");
    assert!(format!("{error:?}").contains("UnresolvedContentSymbol"));

    let after_storage = runtime
        .partitions
        .iter()
        .map(|(id, partition)| {
            (
                *id,
                partition
                    .authoritative_content_digest(&runtime.services.symbols)
                    .expect("failed publication cannot corrupt installed storage"),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(after_storage, before_storage);
    assert_eq!(
        runtime.indexes.entity_unique_aspect_field_index,
        before_index
    );
    assert_eq!(runtime.history.branch_cells_snapshot(), before_cells);
    assert_eq!(runtime.history.commit_catalog.len(), before_catalog);
    assert_eq!(runtime.durable_log().len(), before_durable);
    assert_eq!(runtime.history.commit_envelopes.len(), before_envelopes);
    assert_eq!(runtime.history.patch_stream_index.len(), before_patch_index);
    assert_eq!(
        (
            runtime.history.next_commit_id,
            runtime.history.next_version_id
        ),
        before_sequences
    );
}

#[test]
fn new_partition_publication_reuses_every_prior_region() {
    let mut runtime = runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "prior-main-region");
    let identity = runtime
        .branch_identity(&BranchId("main".to_owned()))
        .expect("main branch identity exists");
    let scope =
        crate::facade::inspection::RelationalMvccCostScope::capture(&runtime, vec![identity]);

    create_entity_in_partition(&mut runtime, "new-partition-region", PartitionId(29));
    let costs = runtime
        .observe_mvcc_cost(&scope)
        .expect("main sharing remains inspectable")
        .sharing_cost_delta();

    assert_eq!(costs.publication_touched_region_count, 1);
    assert_eq!(costs.publication_reused_region_count, 1);
}
