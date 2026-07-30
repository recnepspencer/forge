use std::path::Path;

use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, FramePortCounterSnapshot, ManifestEntryCapacity,
    PageFillPercent, PhysicalRecordInitialization, PhysicalRecordPlacementPolicy,
    PhysicalWorkCounterSnapshot, PhysicalWorkCounterStage, PhysicalWorkEffectFate,
    PhysicalWorkOperationFamily, PhysicalWorkSignalFamily, PhysicalWritebackCounterSnapshot,
    PublishedRecordBatch, RecordAppendBatch, RecordByteLimit, RecordReadLimits, SegmentPageCount,
    ServingPhysicalRuntime,
};
use worth_store_offline_verifier::OfflineRecordPlacement;
use worth_store_physical_backend::{MediaCounterSnapshot, MediaOperationRole};
use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, PhysicalRecordFormatDeclaration,
};

use super::{
    media, read_record, scenario_configuration::dense_configuration, stream_fixture::hex, success,
};

#[test]
fn one_batch_rolls_across_four_segments_and_routes_without_scans() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(2);
    let serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let records = (0_u8..15)
        .map(|value| vec![value; 3_000])
        .collect::<Vec<_>>();
    let writeback_baseline = SegmentWritebackBaseline::capture(&serving);
    let published = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter(records.iter()).unwrap(),
            placement,
        )
        .expect("C5_PREDICATE:identity-placement-seam");
    assert_eq!(published.record_ids().len(), 15);
    assert_eq!(published.observation().segment_artifacts(), 4);
    assert_segment_frame_and_media_evidence(&serving, &writeback_baseline);
    assert_segment_work_and_signal_evidence(&serving, &writeback_baseline);
    assert_segment_artifact_lengths(&root);
    assert_inline_placement_truth(&root, format.declaration());
    let epoch = published.record_id(0).unwrap().allocation_epoch();
    for (index, id) in published.record_ids().iter().enumerate() {
        assert_eq!(id.allocation_epoch(), epoch);
        assert_eq!(id.ordinal(), index as u64 + 1);
    }
    let request = segment_reader_request(serving.store_identity(), &published);
    serving.close();
    assert_fresh_process_records(&root, &request);
}

struct SegmentWritebackBaseline {
    media: MediaCounterSnapshot,
    writebacks: PhysicalWritebackCounterSnapshot,
    frames: FramePortCounterSnapshot,
    work: PhysicalWorkCounterSnapshot,
    causal_records: usize,
    causal_overflow: u64,
}

impl SegmentWritebackBaseline {
    fn capture(serving: &ServingPhysicalRuntime) -> Self {
        Self {
            media: serving.media_counters(),
            writebacks: serving.residency_observation().writebacks(),
            frames: serving.certification_frame_port_observer().snapshot(),
            work: serving.physical_work_counters(),
            causal_records: serving.physical_work_observer().causal().records().len(),
            causal_overflow: serving.physical_work_observer().causal().overflow(),
        }
    }
}

fn assert_segment_frame_and_media_evidence(
    serving: &ServingPhysicalRuntime,
    before: &SegmentWritebackBaseline,
) {
    let media = serving.media_counters();
    let residency = serving.residency_observation();
    let frames = serving.certification_frame_port_observer().snapshot();
    let candidate_frames = frames.candidate_frames() - before.frames.candidate_frames();
    let writeback_frames = residency.writebacks().attempts() - before.writebacks.attempts();
    assert_eq!(media.replacements(), before.media.replacements() + 1);
    assert_eq!(
        frames.declared_candidate_frames() - before.frames.declared_candidate_frames(),
        candidate_frames
    );
    assert_eq!(
        frames.candidate_publications() - before.frames.candidate_publications(),
        candidate_frames
    );
    assert_eq!(candidate_frames, 14);
    assert_eq!(writeback_frames, 4);
    assert_eq!(candidate_frames - writeback_frames, 10);
    assert_eq!(frames.writebacks() - before.frames.writebacks(), 4);
    assert_eq!(
        residency.writebacks().exact_receipts() - before.writebacks.exact_receipts(),
        4
    );
    assert_eq!(
        residency.writebacks().retryable() - before.writebacks.retryable(),
        0
    );
    assert_eq!(
        residency.writebacks().inspection_required() - before.writebacks.inspection_required(),
        0
    );
    assert_eq!(
        media.attempts_for(MediaOperationRole::PositionedWrite)
            - before
                .media
                .attempts_for(MediaOperationRole::PositionedWrite),
        candidate_frames
    );
    assert_eq!(residency.counters().dirty_frames(), 0);
    assert_eq!(residency.counters().candidate_frames(), 0);
    assert_eq!(residency.counters().active_writeback_claims(), 0);
}

fn assert_segment_work_and_signal_evidence(
    serving: &ServingPhysicalRuntime,
    before: &SegmentWritebackBaseline,
) {
    assert_eq!(
        serving.physical_work_counters().count(
            PhysicalWorkOperationFamily::ArtifactRangeWrite,
            PhysicalWorkCounterStage::Terminal,
        ) - before.work.count(
            PhysicalWorkOperationFamily::ArtifactRangeWrite,
            PhysicalWorkCounterStage::Terminal,
        ),
        4
    );
    assert_eq!(
        serving.physical_work_observer().causal().overflow(),
        before.causal_overflow
    );
    let causal = serving.physical_work_observer().causal().records();
    let writebacks = causal[before.causal_records..]
        .iter()
        .filter(|record| record.operation() == PhysicalWorkOperationFamily::ArtifactRangeWrite)
        .collect::<Vec<_>>();
    assert_eq!(writebacks.len(), 4);
    let signal_bindings = serving.physical_signal_aspect_binding_observations();
    for writeback in writebacks {
        assert_eq!(
            writeback.effect_fate(),
            PhysicalWorkEffectFate::WriteCompleted
        );
        assert!(writeback.backend_operation().is_some());
        assert!(signal_bindings.iter().any(|binding| {
            binding.digest() == writeback.signal_binding()
                && binding
                    .families()
                    .contains(PhysicalWorkSignalFamily::ExactWriteback)
        }));
    }
}

fn assert_segment_artifact_lengths(root: &Path) {
    for segment in 1..=4_u64 {
        let path = root.join(format!(
            "families/records/segments/segment-{segment:016x}-0000000000000001.pages"
        ));
        assert_eq!(std::fs::metadata(path).unwrap().len(), 32_768);
    }
}

fn assert_inline_placement_truth(root: &Path, format: PhysicalRecordFormatDeclaration) {
    let root_manifest = super::manifest_fixture::decode_routing_tree(root, 2, format, 128);
    for (index, entry) in root_manifest.placements().iter().enumerate() {
        let OfflineRecordPlacement::Inline {
            segment,
            page,
            slot,
            ..
        } = entry
        else {
            panic!("the rollover fixture contains only inline records")
        };
        assert_eq!(*segment, index as u64 / 4 + 1);
        assert_eq!(*page, index as u64 / 2 + 1);
        assert_eq!(*slot, index as u16 % 2 + 1);
    }
}

fn segment_reader_request(
    store_identity: StableStoreIdentity,
    published: &PublishedRecordBatch,
) -> String {
    let order = [14_usize, 0, 7, 3, 12, 1, 9, 5, 13, 2, 11, 6, 4, 10, 8];
    order
        .iter()
        .map(|index| {
            let locator = ExternalPhysicalRecordLocator::new(
                store_identity,
                published.record_id(*index).unwrap(),
            );
            format!("{index}:{}", hex(&locator.encode()))
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn assert_fresh_process_records(root: &Path, request: &str) {
    let order = [14_usize, 0, 7, 3, 12, 1, 9, 5, 13, 2, 11, 6, 4, 10, 8];
    let output = super::child_process::run_child("segment_reader", root, Some(request));
    for index in order {
        assert!(
            output
                .lines()
                .any(|line| line == format!("C5_SEGMENT {index} {index} 3000 1 1")),
            "C5_PREDICATE:identity-placement-seam"
        );
    }
}

#[test]
fn multi_block_manifest_lookup_has_logarithmic_path_and_exact_parity() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, _, access) = dense_configuration(2);
    let placement = PhysicalRecordPlacementPolicy::builder()
        .segment_pages(SegmentPageCount::new(2).unwrap())
        .extent_threshold(RecordByteLimit::new(8_000).unwrap())
        .page_fill(PageFillPercent::new(50).unwrap())
        .manifest_capacity(ManifestEntryCapacity::new(2).unwrap())
        .admit(format)
        .unwrap();
    let serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let payloads = (0_u8..9).map(|value| vec![value; 100]).collect::<Vec<_>>();
    let published = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter(payloads.iter()).unwrap(),
            placement,
        )
        .unwrap();
    let offline = super::manifest_fixture::decode_routing_tree(&root, 2, format.declaration(), 2);
    assert_eq!(offline.routing_level(), Some(3));
    assert_eq!(offline.placements().len(), payloads.len());
    assert!(offline.manifest_blocks() > 5);
    assert_eq!(offline.segment_pages().len(), 1);
    assert_eq!(offline.free_space().len(), 2);
    for (index, expected) in payloads.iter().enumerate().rev() {
        let session = serving
            .records()
            .open(
                published.record_id(index).unwrap(),
                RecordReadLimits::new(RecordByteLimit::new(100).unwrap()),
            )
            .unwrap();
        let observation = session.observation();
        assert_eq!(observation.manifest_blocks(), 5);
        assert_eq!(
            observation.manifest_comparisons(),
            if index == 8 { 6 } else { 9 }
        );
        assert_eq!(
            observation.manifest_bytes(),
            if index == 8 { 816 } else { 1_048 }
        );
        assert_eq!(observation.touched_segments(), 1);
        assert_eq!(observation.touched_pages(), 1);
        assert_eq!(observation.touched_extents(), 0);
        assert_eq!(observation.payload_bytes(), 0);
        assert_eq!(read_record(session, 100).0, *expected);
        assert_eq!(
            offline.placements()[index].record().ordinal(),
            index as u64 + 1
        );
    }
    serving.close();
}

#[test]
fn cross_batch_page_reuse_is_cow_and_does_not_rebase_old_slots() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let first = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"alpha".as_slice(), b"beta".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    let old_page = std::fs::read(
        root.join("families/records/segments/segment-0000000000000001-0000000000000001.pages"),
    )
    .unwrap();
    let old_offset = old_page[88..92].to_vec();
    serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"gamma".as_slice(), b"delta".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    let new_page = std::fs::read(
        root.join("families/records/segments/segment-0000000000000001-0000000000000002.pages"),
    )
    .unwrap();
    assert_eq!(&new_page[88..92], old_offset, "C5_PREDICATE:page-layout");
    assert!(root
        .join("families/records/segments/segment-0000000000000001-0000000000000001.pages")
        .is_file());
    let old_record = serving
        .records()
        .open(
            first.record_id(0).unwrap(),
            RecordReadLimits::new(RecordByteLimit::new(32).unwrap()),
        )
        .unwrap();
    assert_eq!(read_record(old_record, 5).0, b"alpha");
    serving.close();
}
