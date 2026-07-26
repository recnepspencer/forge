use worth_store::physical_runtime::{
    RecordAppendBatch, RecordByteLimit, RecordReadDenial, RecordReadLimits,
};

use super::fixture;

#[test]
fn caller_maximum_payload_denies_before_session_delivery_and_releases_allocation() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("caller-limit");
    let (serving, placement) = fixture::initialize(&root);
    let expected = fixture::payload(2 * fixture::CHUNK_PAYLOAD_BYTES + 17);
    let record = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([expected.as_slice()]).unwrap(),
            placement,
        )
        .unwrap()
        .record_id(0)
        .unwrap();

    let denial = match serving.records().open(
        record,
        RecordReadLimits::new(RecordByteLimit::new(expected.len() as u32 - 1).unwrap()),
    ) {
        Err(error) => error,
        Ok(_) => panic!("a caller limit below the logical payload must deny the read session"),
    };

    assert_eq!(denial.denial(), RecordReadDenial::CallerLimitExceeded);
    assert_eq!(
        denial.observation().bytes_requested(),
        expected.len() as u64
    );
    assert_eq!(denial.observation().bytes_completed(), 0);
    assert_eq!(serving.observer().record_counters().read_sessions_live(), 0);
    let residency = serving.residency_observation().counters();
    assert_eq!(residency.pin_leases(), 0);
    assert_eq!(residency.pinned_frames(), 0);
    assert_eq!(residency.active_operation_bytes(), 0);

    let mut admitted = serving
        .records()
        .open(
            record,
            RecordReadLimits::new(RecordByteLimit::new(expected.len() as u32).unwrap()),
        )
        .unwrap();
    {
        let first = admitted.next_chunk().unwrap().unwrap();
        assert_eq!(first.bytes(), &expected[..fixture::CHUNK_PAYLOAD_BYTES]);
    }
    let cancellation = admitted.cancel();
    assert_eq!(
        cancellation.observation().bytes_requested(),
        expected.len() as u64
    );
    assert_eq!(
        cancellation.observation().bytes_completed(),
        fixture::CHUNK_PAYLOAD_BYTES as u64
    );
    assert_eq!(
        cancellation.unread_payload_bytes(),
        (expected.len() - fixture::CHUNK_PAYLOAD_BYTES) as u64
    );
    fixture::assert_clean_close(serving);
}
