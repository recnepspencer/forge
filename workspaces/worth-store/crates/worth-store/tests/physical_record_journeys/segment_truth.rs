use worth_store::physical_runtime::{
    ManifestEntryCapacity, PageFillPercent, PhysicalRecordInitialization, PhysicalRecordOpen,
    PhysicalRecordPlacementPolicy, RecordAppendBatch, RecordAppendDenial, RecordAppendError,
    RecordByteLimit, RecordReadDenial, RecordReadLimits, RecordServingTerminalPosture,
    SegmentPageCount, StalePhysicalRecordPlacement,
};
use worth_store_physical_backend::MediaOperationRole;

use super::{configuration, media, success};

#[test]
fn segment_filename_and_header_disagreement_is_denied_before_record_decode() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, _, access) = configuration();
    let placement = PhysicalRecordPlacementPolicy::builder()
        .segment_pages(SegmentPageCount::new(1).unwrap())
        .page_fill(PageFillPercent::new(50).unwrap())
        .extent_threshold(RecordByteLimit::new(8_000).unwrap())
        .manifest_capacity(ManifestEntryCapacity::new(16).unwrap())
        .admit(format)
        .unwrap();
    let mut serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let payloads = [vec![1_u8; 4_000], vec![2_u8; 4_000], vec![3_u8; 4_000]];
    let published = serving
        .records_mut()
        .append_batch(
            RecordAppendBatch::try_from_iter(payloads.iter()).unwrap(),
            placement,
        )
        .unwrap();
    let first = published.record_id(0).unwrap();
    serving.close();

    let segments = root.join("families/records/segments");
    let wrong_header =
        std::fs::read(segments.join("segment-0000000000000002-0000000000000001.pages")).unwrap();
    std::fs::write(
        segments.join("segment-0000000000000001-0000000000000001.pages"),
        wrong_header,
    )
    .unwrap();

    let mut reopened =
        success(media(&root).open_record_store(PhysicalRecordOpen::new(format, access)));
    assert!(matches!(
        reopened.records().open(
            first,
            RecordReadLimits::new(RecordByteLimit::new(4_000).unwrap())
        ),
        Err(error)
            if error.denial() == RecordReadDenial::StalePlacement(
                StalePhysicalRecordPlacement::PageIdentity
            )
                && error.observation().generation_checks() == 3
                && error.observation().generation_rejections() == 1
    ));
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
    assert_eq!(
        reopened.abort().records().posture(),
        RecordServingTerminalPosture::InspectionRequired
    );
}

#[test]
fn dishonest_inline_tail_owner_is_denied_before_candidate_effects() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = configuration();
    let mut serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    serving
        .records_mut()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"published".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    serving.close();

    let manifest = root.join("families/records/roots/root-0000000000000002.manifest");
    let mut bytes = std::fs::read(&manifest).unwrap();
    bytes[344..352].copy_from_slice(&2_u64.to_le_bytes());
    let checksum = super::page_packing_oracle::independent_crc32c(&[&bytes[..36], &bytes[40..]]);
    bytes[36..40].copy_from_slice(&checksum.to_le_bytes());
    std::fs::write(&manifest, bytes).unwrap();
    assert_eq!(
        worth_store_offline_verifier::walk_current_durable_record_manifest(
            &root,
            format.declaration()
        ),
        Err(worth_store_offline_verifier::OfflineDurableManifestDenial::InvalidTreeShape)
    );

    let mut reopened =
        success(media(&root).open_record_store(PhysicalRecordOpen::new(format, access)));
    let before = reopened.media_counters();
    assert!(matches!(
        reopened.records_mut().append_batch(
            RecordAppendBatch::try_from_iter([b"candidate".as_slice()]).unwrap(),
            placement,
        ),
        Err(RecordAppendError::Denied(
            RecordAppendDenial::PublishedLayoutDamaged
        ))
    ));
    let after = reopened.media_counters();
    assert_eq!(
        after.attempts_for(MediaOperationRole::CreateNew),
        before.attempts_for(MediaOperationRole::CreateNew)
    );
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
    assert_eq!(
        reopened.abort().records().posture(),
        RecordServingTerminalPosture::InspectionRequired
    );
}

#[test]
fn unsorted_duplicate_and_cross_root_entries_fail_before_membership() {
    let parent = tempfile::tempdir().unwrap();
    for corruption in [
        RoutingCorruption::Unsorted,
        RoutingCorruption::Duplicate,
        RoutingCorruption::CrossRoot,
    ] {
        exercise_routing_corruption(parent.path(), corruption);
    }
}

#[derive(Clone, Copy, Debug)]
enum RoutingCorruption {
    Unsorted,
    Duplicate,
    CrossRoot,
}

fn exercise_routing_corruption(parent: &std::path::Path, corruption: RoutingCorruption) {
    let root = parent.join(format!("{corruption:?}"));
    let (format, placement, access) = configuration();
    let mut serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let published = serving
        .records_mut()
        .append_batch(
            RecordAppendBatch::try_from_iter([
                b"record-1".as_slice(),
                b"record-2".as_slice(),
                b"record-3".as_slice(),
                b"record-4".as_slice(),
            ])
            .unwrap(),
            placement,
        )
        .unwrap();
    let record = published.record_id(0).unwrap();
    serving.close();

    let block_path =
        root.join("families/records/roots/root-0000000000000002-block-0000000000000001.manifest");
    let mut block = std::fs::read(&block_path).unwrap();
    match corruption {
        RoutingCorruption::Unsorted => {
            let second = block[168..256].to_vec();
            let third = block[256..344].to_vec();
            block[168..256].copy_from_slice(&third);
            block[256..344].copy_from_slice(&second);
        }
        RoutingCorruption::Duplicate => {
            let second = block[168..256].to_vec();
            block[256..344].copy_from_slice(&second);
        }
        RoutingCorruption::CrossRoot => {
            let tree_identity = u64::from_le_bytes(block[40..48].try_into().unwrap());
            block[40..48].copy_from_slice(&(tree_identity + 1).to_le_bytes());
        }
    }
    reseal_frame(&mut block);
    std::fs::write(&block_path, &block).unwrap();

    let root_path = root.join("families/records/roots/root-0000000000000002.manifest");
    let mut root_bytes = std::fs::read(&root_path).unwrap();
    let block_checksum = super::page_packing_oracle::independent_crc32c(&[&block]);
    root_bytes[108..112].copy_from_slice(&block_checksum.to_le_bytes());
    reseal_frame(&mut root_bytes);
    std::fs::write(&root_path, root_bytes).unwrap();

    let expected_offline = match corruption {
        RoutingCorruption::Unsorted | RoutingCorruption::Duplicate => {
            worth_store_offline_verifier::OfflineDurableManifestDenial::InvalidTreeShape
        }
        RoutingCorruption::CrossRoot => {
            worth_store_offline_verifier::OfflineDurableManifestDenial::ReferenceMismatch
        }
    };
    assert_eq!(
        worth_store_offline_verifier::walk_current_durable_record_manifest(
            &root,
            format.declaration()
        ),
        Err(expected_offline)
    );
    let reopened = success(media(&root).open_record_store(PhysicalRecordOpen::new(format, access)));
    let error = match reopened.records().open(
        record,
        RecordReadLimits::new(RecordByteLimit::new(8).unwrap()),
    ) {
        Ok(_) => panic!("{corruption:?} routing block must not admit a read session"),
        Err(error) => error,
    };
    assert_eq!(error.denial(), RecordReadDenial::ArtifactDamaged);
    assert_eq!(error.observation().manifest_blocks(), 1);
    assert_eq!(error.observation().manifest_bytes(), block.len() as u64);
    assert_eq!(error.observation().touched_pages(), 0);
    assert_eq!(error.observation().touched_extents(), 0);
    reopened.abort();
}

fn reseal_frame(bytes: &mut [u8]) {
    let checksum = super::page_packing_oracle::independent_crc32c(&[&bytes[..36], &bytes[40..]]);
    bytes[36..40].copy_from_slice(&checksum.to_le_bytes());
}
