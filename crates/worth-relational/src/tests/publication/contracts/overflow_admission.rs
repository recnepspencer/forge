use crate::tests::support::*;

#[test]
fn truth_sequence_overflow_is_rejected_before_publication_effects() {
    let mut runtime = runtime_with_test_schema();
    let before_cells = runtime.history.branch_cells_snapshot();
    let before_catalog = runtime.history.commit_catalog.len();
    let before_graph = runtime.history.commit_graph.len();
    let before_envelopes = runtime.history.commit_envelopes.len();
    let before_patch_index = runtime.history.patch_stream_index.len();
    let before_slots: usize = runtime
        .partitions
        .values()
        .map(|partition| partition.entity_arena.slot_count())
        .sum();

    runtime.history.next_commit_id = u64::MAX;
    runtime.history.next_version_id = u64::MAX;
    let mut transaction = test_owner_begin_transaction_for_main(&mut runtime);
    transaction.push_batch(batch_create("overflow-denial"));
    let error = transaction
        .commit()
        .expect_err("sequence overflow must be a typed publication denial");

    match &error {
        TransactionCommitError::Publication { error, .. } => {
            assert_eq!(error.stage, PublicationStage::BundleAssembly);
            assert!(error.detail.contains("sequence overflow"));
        }
        other => panic!("sequence overflow escaped as an untyped error: {other:?}"),
    }
    assert_eq!(runtime.history.branch_cells_snapshot(), before_cells);
    assert_eq!(runtime.history.commit_catalog.len(), before_catalog);
    assert_eq!(runtime.history.commit_graph.len(), before_graph);
    assert_eq!(runtime.history.commit_envelopes.len(), before_envelopes);
    assert_eq!(runtime.history.patch_stream_index.len(), before_patch_index);
    let after_slots: usize = runtime
        .partitions
        .values()
        .map(|partition| partition.entity_arena.slot_count())
        .sum();
    assert_eq!(after_slots, before_slots);
}

#[test]
fn metadata_sequence_overflow_is_rejected_without_advancing_the_allocator() {
    let mut runtime = runtime_with_test_schema();
    runtime.history.next_commit_id = u64::MAX;
    let before_version = runtime.history.next_version_id;
    assert_eq!(
        runtime.history.advance_metadata_commit_sequence(),
        Err("metadata commit id sequence overflow")
    );
    assert_eq!(runtime.history.next_commit_id, u64::MAX);
    assert_eq!(runtime.history.next_version_id, before_version);
}
