use super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
    stores::{unique_test_sqlite_path, unique_test_store_path},
};
use crate::wal::WalRecord;
use crate::{
    media::{
        barriers::{validate_barrier_satisfies_requirement, DurabilityBarrierClass},
        framing::{scan_tail, TailValidationOutcome},
    },
    DurableBackendFamily, ForgeStoreBuilder, StoreErrorKind,
};

#[test]
fn wal_record_media_frame_roundtrip_is_lossless() {
    let record =
        WalRecord::durable_mutation_intent(1, crate::DurableMutationId(7), "runtime-a", "create")
            .expect("wal record should frame");

    let classified = record
        .classify_media_barrier(DurabilityBarrierClass::TransactionalCommitDurable)
        .expect("wal record should classify media barrier");

    let decoded =
        WalRecord::decode_from_media_bytes(classified.record().framed_record().as_bytes().to_vec())
            .expect("framed wal bytes should decode");

    assert_eq!(decoded, record);
    assert_eq!(
        classified.barrier_class(),
        DurabilityBarrierClass::TransactionalCommitDurable
    );
}

#[test]
fn tail_scan_reports_truncated_tail() {
    let record =
        WalRecord::durable_mutation_intent(1, crate::DurableMutationId(9), "runtime-a", "create")
            .expect("wal record should frame");
    let classified = record
        .classify_media_barrier(DurabilityBarrierClass::FileAndRequiredMetadataDurable)
        .expect("wal record should classify media barrier");
    let bytes = classified.record().framed_record().as_bytes();
    let truncated = bytes[..bytes.len() - 5].to_vec();

    let report = scan_tail(&truncated).expect("truncated tail should classify cleanly");

    assert_eq!(report.outcome(), TailValidationOutcome::TruncatedTail);
    assert_eq!(report.valid_record_count(), 0);
    assert_eq!(report.trailing_byte_count(), truncated.len());
}

#[test]
fn tampered_payload_is_detected_as_torn_write() {
    let record =
        WalRecord::durable_mutation_intent(1, crate::DurableMutationId(11), "runtime-a", "create")
            .expect("wal record should frame");
    let classified = record
        .classify_media_barrier(DurabilityBarrierClass::FileContentDurable)
        .expect("wal record should classify media barrier");
    let mut bytes = classified.record().framed_record().as_bytes().to_vec();
    let payload_index = bytes
        .iter()
        .position(|byte| *byte == b'r')
        .expect("payload should contain runtime session bytes");
    bytes[payload_index] = b'X';

    let error = WalRecord::decode_from_media_bytes(bytes).unwrap_err();
    assert_eq!(error.kind(), &StoreErrorKind::DurableTornWriteDetected);
}

#[test]
fn weaker_barrier_is_rejected_against_required_barrier() {
    let error = validate_barrier_satisfies_requirement(
        DurabilityBarrierClass::KernelBufferResident,
        DurabilityBarrierClass::FileContentDurable,
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        &StoreErrorKind::DurableBarrierContractViolation
    );
}

#[test]
fn backend_durable_media_reports_are_explicit() {
    let local = ForgeStoreBuilder::new()
        .local_file(unique_test_store_path("forge-store-media-report"))
        .build()
        .expect("local-file store should build");
    let sqlite = ForgeStoreBuilder::new()
        .sqlite_file(unique_test_sqlite_path("forge-store-media-report"))
        .build()
        .expect("sqlite store should build");

    let local_report = local.durable_media_report();
    let sqlite_report = sqlite.durable_media_report();

    assert_eq!(
        local_report.backend_family(),
        DurableBackendFamily::LocalFileAtomicRewrite
    );
    assert_eq!(
        local_report.content_barrier(),
        DurabilityBarrierClass::FileContentDurable
    );
    assert_eq!(
        local_report.metadata_barrier(),
        DurabilityBarrierClass::RenameOrPublicationMarkerDurable
    );
    assert_eq!(
        sqlite_report.backend_family(),
        DurableBackendFamily::SqliteTransactional
    );
    assert_eq!(
        sqlite_report.content_barrier(),
        DurabilityBarrierClass::TransactionalCommitDurable
    );
    assert_eq!(
        sqlite_report.ack_required_barrier(),
        DurabilityBarrierClass::TransactionalCommitDurable
    );
}

#[test]
fn append_records_barrier_verification_without_ack_violation() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new()
        .local_file(unique_test_store_path("forge-store-media-append"))
        .build()
        .expect("local-file store should build");
    store
        .append_canonical_commit(envelope)
        .expect("append should succeed with explicit barrier report");

    let counters = store.counters();
    assert!(counters.durable_barrier_verified_count >= 1);
    assert_eq!(counters.durable_ack_barrier_violation_count, 0);
}
