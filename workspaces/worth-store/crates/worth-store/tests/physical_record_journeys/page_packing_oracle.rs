use worth_store::physical_runtime::{
    PhysicalRecordInitialization, RecordAppendBatch, RecordAppendDenial, RecordAppendError,
    RecordByteLimit, RecordReadLimits, RecordWriteSource, RecordWriteSourceError,
};
use worth_store_physical_backend::MediaOperationRole;

use super::{
    media, read_record, scenario_configuration::dense_configuration, stream_fixture::PatternSource,
    success,
};

#[test]
fn batch_packing_matches_an_independent_page_oracle() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let payloads = [b"abc".as_slice(), b"12345".as_slice(), b"".as_slice()];
    let before = serving.media_counters();
    let published = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter(payloads).unwrap(),
            placement,
        )
        .unwrap();
    let after = serving.media_counters();
    assert_eq!(published.observation().records(), 3);
    assert_eq!(published.observation().transfer_count(), 8);
    assert_eq!(
        after.attempts_for(MediaOperationRole::PositionedWrite)
            - before.attempts_for(MediaOperationRole::PositionedWrite),
        7
    );
    assert_eq!(published.observation().explicit_copy_count(), 3);
    assert_eq!(published.observation().copied_bytes(), 8);

    let page = std::fs::read(
        root.join("families/records/segments/segment-0000000000000001-0000000000000001.pages"),
    )
    .unwrap();
    assert_eq!(page.len(), 16_384);
    assert_eq!(&page[..8], b"WRC5FRM\0");
    assert_eq!(page[8], 3);
    assert_eq!(u64::from_le_bytes(page[28..36].try_into().unwrap()), 1);
    assert_eq!(
        u32::from_le_bytes(page[36..40].try_into().unwrap()),
        independent_crc32c(&[&page[..36], &page[40..]])
    );

    let frame_payload = &page[40..];
    assert_eq!(
        u64::from_le_bytes(frame_payload[..8].try_into().unwrap()),
        1
    );
    assert_eq!(
        u64::from_le_bytes(frame_payload[8..16].try_into().unwrap()),
        1
    );
    assert_eq!(
        u16::from_le_bytes(frame_payload[16..18].try_into().unwrap()),
        3
    );
    assert_eq!(&frame_payload[18..24], &[0; 6]);
    let expected_offsets = [
        frame_payload.len() - 3,
        frame_payload.len() - 8,
        frame_payload.len() - 8,
    ];
    for (index, expected_payload) in payloads.iter().enumerate() {
        let base = 24 + index * 40;
        let record = published.record_id(index).unwrap();
        assert_eq!(&frame_payload[base..base + 16], &record.allocation_epoch());
        assert_eq!(
            u64::from_le_bytes(frame_payload[base + 16..base + 24].try_into().unwrap()),
            record.ordinal()
        );
        let offset =
            u32::from_le_bytes(frame_payload[base + 24..base + 28].try_into().unwrap()) as usize;
        let length =
            u32::from_le_bytes(frame_payload[base + 28..base + 32].try_into().unwrap()) as usize;
        assert_eq!(
            (offset, length),
            (expected_offsets[index], expected_payload.len())
        );
        assert_eq!(
            u64::from_le_bytes(frame_payload[base + 32..base + 40].try_into().unwrap()),
            1
        );
        assert_eq!(&frame_payload[offset..offset + length], *expected_payload);
    }
    let directory_end = 24 + payloads.len() * 40;
    assert_eq!(expected_offsets[2] - directory_end, 16_192);
    serving.close();
}

#[test]
fn inline_source_and_delivery_copies_are_counted_at_the_actual_copy_seams() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let published = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::builder()
                .push_source(PatternSource::exact(13))
                .build()
                .unwrap(),
            placement,
        )
        .unwrap();
    assert_eq!(published.observation().explicit_copy_count(), 2);
    assert_eq!(published.observation().copied_bytes(), 26);

    let session = serving
        .records()
        .open(
            published.record_id(0).unwrap(),
            RecordReadLimits::new(RecordByteLimit::new(13).unwrap()),
        )
        .unwrap();
    let (_, observation) = read_record(session, 13);
    assert_eq!(observation.bytes_completed(), 13);
    assert_eq!(observation.explicit_copy_count(), 1);
    assert_eq!(observation.copied_bytes(), 13);
    serving.close();
}

#[test]
fn pre_effect_inline_source_failure_keeps_the_writer_usable() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let before = serving.media_counters();
    let error = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::builder()
                .push_source(PatternSource::truncated(13, 7))
                .build()
                .unwrap(),
            placement,
        )
        .unwrap_err();
    assert!(matches!(error, RecordAppendError::StreamFailed(_)));
    let after = serving.media_counters();
    assert_eq!(
        after.attempts_for(MediaOperationRole::PositionedWrite),
        before.attempts_for(MediaOperationRole::PositionedWrite)
    );
    assert!(serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"still-usable".as_slice()]).unwrap(),
            placement,
        )
        .is_ok());
    serving.close();
}

#[test]
fn published_tail_is_validated_before_an_inline_producer_is_consumed() {
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    struct CountedSource {
        reads: Arc<AtomicUsize>,
        delivered: bool,
    }
    impl RecordWriteSource for CountedSource {
        fn declared_length(&self) -> u64 {
            3
        }
        fn read_next(&mut self, target: &mut [u8]) -> Result<usize, RecordWriteSourceError> {
            self.reads.fetch_add(1, Ordering::SeqCst);
            if self.delivered {
                return Ok(0);
            }
            target[..3].copy_from_slice(b"new");
            self.delivered = true;
            Ok(3)
        }
    }

    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"old".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    let manifest = root.join(
        "families/records/segment-manifests/segments-0000000000000002-block-0000000000000001.manifest",
    );
    let mut damaged = std::fs::read(&manifest).unwrap();
    let last = damaged.len() - 1;
    damaged[last] ^= 1;
    std::fs::write(manifest, damaged).unwrap();
    serving.drain_clean_residency();

    let reads = Arc::new(AtomicUsize::new(0));
    let error = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::builder()
                .push_source(CountedSource {
                    reads: Arc::clone(&reads),
                    delivered: false,
                })
                .build()
                .unwrap(),
            placement,
        )
        .unwrap_err();
    assert!(matches!(
        error,
        RecordAppendError::Denied(RecordAppendDenial::PublishedLayoutDamaged)
    ));
    assert_eq!(reads.load(Ordering::SeqCst), 0);
    serving.abort();
}

pub(super) fn independent_crc32c(parts: &[&[u8]]) -> u32 {
    let mut crc = !0_u32;
    for part in parts {
        for byte in *part {
            crc ^= u32::from(*byte);
            for _ in 0..8 {
                crc = (crc >> 1) ^ (0x82f6_3b78 & 0_u32.wrapping_sub(crc & 1));
            }
        }
    }
    !crc
}
