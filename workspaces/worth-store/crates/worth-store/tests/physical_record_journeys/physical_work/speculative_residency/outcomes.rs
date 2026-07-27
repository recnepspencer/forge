use worth_store::physical_runtime::{
    PhysicalPrefetchIntent, PhysicalPrefetchOutcome, PhysicalReadAheadFrameOutcome,
    PhysicalReadAheadIntent, PhysicalReadAheadOutcome, PhysicalSpeculativeWorkKind,
    PhysicalWorkOperationFamily, PhysicalWorkSignalFamily,
};

use super::fixture::{causal_record, coordinate, initialize_store, open_store, positioned_reads};

#[test]
fn cold_hot_and_mixed_speculation_reconcile_work_media_and_residency_truth() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("speculative-outcomes");
    initialize_store(&root);
    let serving = open_store(&root, 4, 4);
    let residency = serving.certification_physical_residency();
    residency.drain_unpinned_clean_frames();
    let first = coordinate(0);

    let media_before = positioned_reads(&serving);
    let causal_before = serving.physical_work_observer().causal().records().len();
    let counters_before = residency.counters();
    let cold_work = match residency.prefetch(PhysicalPrefetchIntent::new(first)) {
        PhysicalPrefetchOutcome::Loaded { coordinate, work } => {
            assert_eq!(coordinate, first);
            work
        }
        outcome => panic!("cold prefetch did not use canonical work: {outcome:?}"),
    };
    assert_eq!(positioned_reads(&serving), media_before + 1);
    assert_eq!(
        serving.physical_work_observer().causal().records().len(),
        causal_before + 1
    );
    let cold_record = causal_record(&serving, cold_work);
    assert_eq!(
        cold_record.operation(),
        PhysicalWorkOperationFamily::ArtifactRangeRead
    );
    assert_eq!(
        cold_record.signal_family(),
        PhysicalWorkSignalFamily::ReadFault
    );
    assert!(cold_record.backend_operation().is_some());
    assert_kind_completion(
        counters_before,
        residency.counters(),
        PhysicalSpeculativeWorkKind::Prefetch,
        1,
    );

    let hot_media = serving.media_counters();
    let hot_work = serving.physical_work_counters();
    let hot_causal = serving.physical_work_observer().causal().records().len();
    assert_eq!(
        residency.prefetch(PhysicalPrefetchIntent::new(first)),
        PhysicalPrefetchOutcome::Hit { coordinate: first }
    );
    assert_eq!(serving.media_counters(), hot_media);
    assert_eq!(serving.physical_work_counters(), hot_work);
    assert_eq!(
        serving.physical_work_observer().causal().records().len(),
        hot_causal
    );

    let second = coordinate(1);
    let read_ahead_before = residency.counters();
    let mixed_media = positioned_reads(&serving);
    let coordinates = [first, second];
    let batch = match residency.read_ahead(PhysicalReadAheadIntent::new(&coordinates).unwrap()) {
        PhysicalReadAheadOutcome::Complete(batch) => batch,
        outcome => panic!("mixed read-ahead did not complete exactly: {outcome:?}"),
    };
    assert_eq!(batch.hits(), 1);
    assert_eq!(batch.coalesced(), 0);
    assert_eq!(batch.loaded(), 1);
    assert_eq!(batch.failed(), 0);
    assert!(matches!(
        batch.frames(),
        [
            PhysicalReadAheadFrameOutcome::Hit { coordinate } ,
            PhysicalReadAheadFrameOutcome::Loaded { coordinate: loaded, .. }
        ] if *coordinate == first && *loaded == second
    ));
    let mixed_work = batch.frames()[1]
        .work()
        .expect("the cold read-ahead frame reports its canonical identity");
    let mixed_record = causal_record(&serving, mixed_work);
    assert_eq!(
        mixed_record.operation(),
        PhysicalWorkOperationFamily::ArtifactRangeRead
    );
    assert_eq!(
        mixed_record.signal_family(),
        PhysicalWorkSignalFamily::ReadFault
    );
    assert_eq!(positioned_reads(&serving), mixed_media + 1);
    assert_kind_completion(
        read_ahead_before,
        residency.counters(),
        PhysicalSpeculativeWorkKind::ReadAhead,
        1,
    );
    assert!(!serving.close().residency().requires_inspection());
}

fn assert_kind_completion(
    before: worth_store_buffer_pool::PhysicalResidencyCounters,
    after: worth_store_buffer_pool::PhysicalResidencyCounters,
    kind: PhysicalSpeculativeWorkKind,
    operations: u64,
) {
    assert_eq!(
        after.speculative_attempts(kind),
        before.speculative_attempts(kind) + operations
    );
    assert_eq!(
        after.speculative_admissions(kind),
        before.speculative_admissions(kind) + operations
    );
    assert_eq!(
        after.speculative_completions(kind),
        before.speculative_completions(kind) + operations
    );
    assert_eq!(after.active_speculative_frames(kind), 0);
    assert_eq!(after.active_operation_bytes(), 0);
}
