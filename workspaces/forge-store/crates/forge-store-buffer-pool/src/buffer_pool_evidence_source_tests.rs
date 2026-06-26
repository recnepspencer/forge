use crate::{
    AllocationCounterSnapshot, BufferPoolCounterSnapshot, BufferPoolEvidenceSourceDenial,
    RecordCopyCounterSnapshot, ResidentFrameCounterSnapshot,
};

#[test]
fn executed_evidence_source_rejects_empty_counter_snapshots() {
    let denial = BufferPoolCounterSnapshot::from_executed_store_counters(
        ResidentFrameCounterSnapshot::empty(),
        AllocationCounterSnapshot::default(),
        RecordCopyCounterSnapshot::empty(),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        BufferPoolEvidenceSourceDenial::NoExecutedStoreCounters
    );
}
