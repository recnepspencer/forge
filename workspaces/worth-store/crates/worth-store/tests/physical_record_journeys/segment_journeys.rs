use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, ManifestEntryCapacity, PageFillPercent,
    PhysicalRecordInitialization, PhysicalRecordPlacementPolicy, RecordAppendBatch,
    RecordByteLimit, RecordReadLimits, SegmentPageCount,
};
use worth_store_offline_verifier::OfflineRecordPlacement;

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
    let before = serving.media_counters();
    let published = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter(records.iter()).unwrap(),
            placement,
        )
        .expect("C5_PREDICATE:identity-placement-seam");
    let after = serving.media_counters();
    assert_eq!(published.record_ids().len(), 15);
    assert_eq!(published.observation().segment_artifacts(), 4);
    assert_eq!(after.replacements(), before.replacements() + 1);
    let epoch = published.record_id(0).unwrap().allocation_epoch();
    for (index, id) in published.record_ids().iter().enumerate() {
        assert_eq!(id.allocation_epoch(), epoch);
        assert_eq!(id.ordinal(), index as u64 + 1);
    }
    for segment in 1..=4_u64 {
        let path = root.join(format!(
            "families/records/segments/segment-{segment:016x}-0000000000000001.pages"
        ));
        assert_eq!(std::fs::metadata(path).unwrap().len(), 32_768);
    }
    let root_manifest =
        super::manifest_fixture::decode_routing_tree(&root, 2, format.declaration(), 128);
    assert_eq!(root_manifest.root_generation(), 2);
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
        assert_eq!(
            *segment,
            index as u64 / 4 + 1,
            "C5_PREDICATE:identity-placement-seam"
        );
        assert_eq!(
            *page,
            index as u64 / 2 + 1,
            "C5_PREDICATE:identity-placement-seam"
        );
        assert_eq!(
            *slot,
            index as u16 % 2 + 1,
            "C5_PREDICATE:identity-placement-seam"
        );
    }
    let store_identity = serving.store_identity();
    serving.close();

    let order = [14_usize, 0, 7, 3, 12, 1, 9, 5, 13, 2, 11, 6, 4, 10, 8];
    let request = order
        .iter()
        .map(|index| {
            let locator = ExternalPhysicalRecordLocator::new(
                store_identity,
                published.record_id(*index).unwrap(),
            );
            format!("{index}:{}", hex(&locator.encode()))
        })
        .collect::<Vec<_>>()
        .join(";");
    let output = super::child_process::run_child("segment_reader", &root, Some(&request));
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
