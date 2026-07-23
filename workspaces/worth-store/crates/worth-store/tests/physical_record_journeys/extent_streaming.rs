use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, PhysicalRecordInitialization, PhysicalRecordOpen,
    RecordAppendBatch, RecordAppendDenial, RecordAppendError, RecordByteLimit, RecordReadLimits,
    RecordServingTerminalPosture, RecordStreamFailureKind, UnpublishedRecordEffectFate,
};

use super::{
    media,
    scenario_configuration::dense_configuration,
    stream_fixture::{hex, pattern_digest, PatternSource},
    success,
};

#[test]
fn extent_allocation_peak_is_independent_of_logical_record_length() {
    let short = measured_extent_append(17 * (16_384 - 104) as u64 + 7);
    let long = measured_extent_append(65 * (16_384 - 104) as u64 + 7);
    for (operation, short, long) in [
        ("append", short.append, long.append),
        ("read", short.read, long.read),
    ] {
        assert!(
            short >= 16_384,
            "C5_PREDICATE:transfer-allocation-slope the bounded {operation} frame allocation must be visible"
        );
        assert_eq!(
            long, short,
            "C5_PREDICATE:transfer-allocation-slope {operation}: {short} -> {long}"
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct ExtentAllocationObservation {
    append: usize,
    read: usize,
}

fn measured_extent_append(logical_bytes: u64) -> ExtentAllocationObservation {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let output = super::child_process::run_child(
        "allocation_writer",
        &root,
        Some(&logical_bytes.to_string()),
    );
    let row = output
        .lines()
        .find_map(|line| line.strip_prefix("C5_ALLOC "))
        .expect("allocation child must report its operation evidence");
    let mut fields = row.split_whitespace();
    let append = fields.next().unwrap().parse().unwrap();
    let scratch: u64 = fields.next().unwrap().parse().unwrap();
    let locator = fields.next().unwrap();
    assert_eq!(
        scratch, 16_384,
        "append scratch remains a bounded mutable frame"
    );
    let output = super::child_process::run_child("allocation_reader", &root, Some(locator));
    let row = output
        .lines()
        .find_map(|line| line.strip_prefix("C5_READ_ALLOC "))
        .expect("allocation child must report its read evidence");
    let mut fields = row.split_whitespace();
    let read = fields.next().unwrap().parse().unwrap();
    let scratch: u64 = fields.next().unwrap().parse().unwrap();
    assert_eq!(
        scratch, 0,
        "resident leases are not operation scratch allocations"
    );
    ExtentAllocationObservation { append, read }
}

#[test]
fn mixed_batch_streams_extent_and_a_fresh_process_reads_with_seventeen_widths() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let mut serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let logical_bytes = 1_048_613_u64;
    let batch = RecordAppendBatch::builder()
        .push_bytes(b"inline")
        .push_source(PatternSource::exact(logical_bytes))
        .build()
        .unwrap();
    let published = serving
        .records_mut()
        .append_batch(batch, placement)
        .unwrap();
    let observation = published.observation();
    let chunks = logical_bytes.div_ceil(16_384 - 104);
    assert_eq!(observation.extent_artifacts(), 1);
    assert_eq!(observation.bytes_requested(), logical_bytes + 6);
    assert_eq!(observation.bytes_completed(), logical_bytes + 6);
    assert_eq!(observation.transfer_count(), chunks + 9);
    assert_eq!(observation.peak_transfer_width(), 16_384);
    assert_eq!(observation.peak_scratch_bytes(), 16_384);
    assert_eq!(observation.explicit_copy_count(), chunks + 1);
    assert_eq!(observation.copied_bytes(), logical_bytes + 6);
    let locator = ExternalPhysicalRecordLocator::new(
        serving.store_identity(),
        published.record_id(1).unwrap(),
    );
    serving.close();
    let extent =
        root.join("families/records/extents/extent-0000000000000001-0000000000000001.data");
    assert_eq!(
        std::fs::metadata(extent).unwrap().len(),
        logical_bytes + chunks * 104
    );
    let output =
        super::child_process::run_child("extent_reader", &root, Some(&hex(&locator.encode())));
    assert!(output.contains(&format!(
        "C5_EXTENT {logical_bytes} {}",
        pattern_digest(logical_bytes)
    )));
    assert!(output.contains(&format!(
        "C5_EXTENT_OBS {logical_bytes} {logical_bytes} {} 16384 {logical_bytes} {} 0",
        chunks + 1,
        chunks + 1,
    )));
}

#[test]
fn source_length_drift_reports_exact_completed_range_without_publishing() {
    for (source, expected_kind, completed) in [
        (
            PatternSource::truncated(20_000, 12_345),
            RecordStreamFailureKind::SourceEndedEarly,
            12_345,
        ),
        (
            PatternSource::overlong(20_000),
            RecordStreamFailureKind::SourceExceededDeclaredLength,
            20_000,
        ),
    ] {
        let parent = tempfile::tempdir().unwrap();
        let root = parent.path().join("store");
        let (format, placement, access) = dense_configuration(4);
        let mut serving = success(
            media(&root).initialize_record_store(PhysicalRecordInitialization::new(
                format, placement, access,
            )),
        );
        let catalog_before =
            std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap();
        let error = serving
            .records_mut()
            .append_batch(
                RecordAppendBatch::builder()
                    .push_source(source)
                    .build()
                    .unwrap(),
                placement,
            )
            .unwrap_err();
        let worth_store::physical_runtime::RecordAppendError::Unpublished(failure) = error else {
            panic!("known-unpublished stream failure expected")
        };
        let worth_store::physical_runtime::UnpublishedRecordBatchCause::Stream(stream_failure) =
            failure.cause()
        else {
            panic!("stream cause expected")
        };
        assert_eq!(stream_failure.kind(), expected_kind);
        assert_eq!(stream_failure.completed_range(), 0..completed);
        assert_eq!(
            failure.effect_fate(),
            UnpublishedRecordEffectFate::EffectPossible
        );
        assert_eq!(
            std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap(),
            catalog_before
        );
        assert_eq!(
            serving
                .records_mut()
                .append_batch(
                    RecordAppendBatch::try_from_iter([b"retry".as_slice()]).unwrap(),
                    placement,
                )
                .unwrap_err(),
            RecordAppendError::Denied(RecordAppendDenial::ServingRequiresInspection)
        );
        let shutdown = serving.abort();
        assert_eq!(
            shutdown.records().posture(),
            RecordServingTerminalPosture::InspectionRequired
        );
        let mut reopened =
            success(media(&root).open_record_store(PhysicalRecordOpen::new(format, access)));
        assert!(reopened.observed_non_authoritative_residue());
        assert_eq!(
            reopened
                .records_mut()
                .append_batch(
                    RecordAppendBatch::try_from_iter([b"retry".as_slice()]).unwrap(),
                    placement,
                )
                .unwrap_err(),
            RecordAppendError::Denied(RecordAppendDenial::ServingRequiresInspection)
        );
        reopened.abort();
    }
}

#[test]
fn abandoned_candidate_identity_is_never_reused_by_a_later_publication() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let mut serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let error = serving
        .records_mut()
        .append_batch(
            RecordAppendBatch::builder()
                .push_source(PatternSource::overlong(20_000))
                .build()
                .unwrap(),
            placement,
        )
        .unwrap_err();
    assert!(matches!(
        error,
        worth_store::physical_runtime::RecordAppendError::Unpublished(ref failure)
            if matches!(failure.cause(), worth_store::physical_runtime::UnpublishedRecordBatchCause::Stream(_))
    ));
    serving.abort();

    let orphan =
        root.join("families/records/extents/extent-0000000000000001-0000000000000001.data");
    let orphan_bytes = std::fs::read(&orphan).unwrap();
    let abandoned_epoch: [u8; 16] = orphan_bytes[40..56].try_into().unwrap();
    std::fs::remove_file(orphan).unwrap();

    let mut reopened =
        success(media(&root).open_record_store(PhysicalRecordOpen::new(format, access)));
    let published = reopened
        .records_mut()
        .append_batch(
            RecordAppendBatch::builder()
                .push_source(PatternSource::exact(20_000))
                .build()
                .unwrap(),
            placement,
        )
        .unwrap();
    assert_ne!(
        published.record_id(0).unwrap().allocation_epoch(),
        abandoned_epoch,
        "C5_PREDICATE:identity-authority"
    );
    reopened.close();
}

#[test]
fn fresh_open_reports_known_unpublished_extent_residue_and_blocks_collision() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let mut serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let error = serving
        .records_mut()
        .append_batch(
            RecordAppendBatch::builder()
                .push_source(PatternSource::overlong(20_000))
                .build()
                .unwrap(),
            placement,
        )
        .unwrap_err();
    assert!(matches!(
        error,
        worth_store::physical_runtime::RecordAppendError::Unpublished(_)
    ));
    serving.abort();

    let mut reopened =
        success(media(&root).open_record_store(PhysicalRecordOpen::new(format, access)));
    let residue = reopened.publication_residue();
    assert!(residue.next_extent_artifacts());
    assert!(!residue.successor_root());
    assert!(matches!(
        reopened.records_mut().append_batch(
            RecordAppendBatch::try_from_iter([b"must not collide".as_slice()]).unwrap(),
            placement,
        ),
        Err(worth_store::physical_runtime::RecordAppendError::Denied(
            worth_store::physical_runtime::RecordAppendDenial::ServingRequiresInspection
        ))
    ));
    reopened.close();
}

#[test]
fn streamed_read_damage_retains_the_completed_logical_range() {
    use std::io::{Seek, SeekFrom, Write};

    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let mut serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let published = serving
        .records_mut()
        .append_batch(
            RecordAppendBatch::builder()
                .push_source(PatternSource::exact(40_000))
                .build()
                .unwrap(),
            placement,
        )
        .unwrap();
    let path = root.join("families/records/extents/extent-0000000000000001-0000000000000001.data");
    let mut file = std::fs::OpenOptions::new().write(true).open(path).unwrap();
    file.seek(SeekFrom::Start((16_384 + 120) as u64)).unwrap();
    file.write_all(&[0xa5]).unwrap();
    file.sync_all().unwrap();
    assert!(serving.drain_clean_residency() > 0);
    let mut session = serving
        .records()
        .open(
            published.record_id(0).unwrap(),
            RecordReadLimits::new(RecordByteLimit::new(40_000).unwrap()),
        )
        .unwrap();
    let mut first = vec![0_u8; 16_384 - 104];
    assert_eq!(session.read_next(&mut first).unwrap(), 16_384 - 104);
    let failure = session.read_next(&mut [0_u8; 1]).unwrap_err();
    assert_eq!(failure.kind(), RecordStreamFailureKind::ArtifactDamaged);
    assert_eq!(failure.completed_range(), 0..16_280);
    assert_eq!(session.observation().generation_checks(), 2);
    assert_eq!(session.observation().generation_rejections(), 0);
    drop(session);
    assert_eq!(
        serving
            .records_mut()
            .append_batch(
                RecordAppendBatch::try_from_iter([b"retry".as_slice()]).unwrap(),
                placement,
            )
            .unwrap_err(),
        RecordAppendError::Denied(RecordAppendDenial::ServingRequiresInspection)
    );
    assert_eq!(
        serving.abort().records().posture(),
        RecordServingTerminalPosture::InspectionRequired
    );
}
