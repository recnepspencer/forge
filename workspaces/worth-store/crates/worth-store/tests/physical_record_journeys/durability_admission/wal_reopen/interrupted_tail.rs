use std::{fs::OpenOptions, io::Write};

use worth_store::physical_runtime::{PhysicalRecordOpen, PhysicalWalOpenFailure};
use worth_store_wal::{
    inspect_complete_wal_segment, plan_wal_frame_append, LogSequenceNumber, WalAppendFrontier,
    WalArtifactStoreDenial, WalLsnRange, WalSegmentArtifactIdentity, WalSegmentGeneration,
    WalSegmentId,
};

use super::{
    build_three_segment_inventory_with_limit, build_three_segment_inventory_with_policy,
    configuration, durability_with_wal_policy, initialize_empty_store, media, reopen_failure,
    segment_path, success, wal_policy, SEGMENT_BYTES,
};

#[test]
fn interrupted_final_frame_is_truncated_to_its_verified_prefix_before_reopen() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    let reopen_bytes = SEGMENT_BYTES + 37;
    build_three_segment_inventory_with_policy(&store_root, reopen_bytes, 3);
    let active = segment_path(&store_root, 3, 1);
    let complete = std::fs::read(&active).unwrap();
    let identity = WalSegmentArtifactIdentity::new(
        WalSegmentId::new(3).unwrap(),
        WalSegmentGeneration::new(1).unwrap(),
    );
    let inspection = inspect_complete_wal_segment(identity, &complete).unwrap();
    let lsn_start = inspection.lsn_range().end_exclusive();
    let lsn_end = LogSequenceNumber::new(lsn_start.get() + 1);
    let planned = plan_wal_frame_append(
        WalAppendFrontier::observed(
            identity.segment(),
            identity.generation(),
            complete.len() as u64,
            lsn_start,
        ),
        WalLsnRange::new(lsn_start, lsn_end).unwrap(),
        "interrupted-next-frame",
        b"not-yet-committed",
    )
    .unwrap();
    OpenOptions::new()
        .append(true)
        .open(&active)
        .unwrap()
        .write_all(&planned.frame().encoded_frame()[..37])
        .unwrap();

    let media_owner = media(&store_root);
    let durability = durability_with_wal_policy(&media_owner, wal_policy(reopen_bytes, 3));
    let (format, _, access) = configuration();
    success(media_owner.open_record_store(PhysicalRecordOpen::new(format, access, durability)))
        .close();

    if std::fs::metadata(&active).unwrap().len() != complete.len() as u64 {
        panic!("MUTANT_PREDICATE:c7-interrupted-active-tail-cleanup-omitted");
    }
    let second_media = media(&store_root);
    let second_policy = durability_with_wal_policy(&second_media, wal_policy(reopen_bytes, 3));
    success(second_media.open_record_store(PhysicalRecordOpen::new(format, access, second_policy)))
        .close();
}

#[test]
fn partial_first_frame_in_the_exact_active_successor_is_removed_before_reopen() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    build_three_segment_inventory_with_limit(&store_root, 4);
    let source = std::fs::read(segment_path(&store_root, 3, 1)).unwrap();
    let partial = segment_path(&store_root, 4, 1);
    std::fs::write(&partial, &source[..37]).unwrap();

    let media_owner = media(&store_root);
    let durability = durability_with_wal_policy(&media_owner, wal_policy(SEGMENT_BYTES, 4));
    let (format, _, access) = configuration();
    let serving = match media_owner
        .open_record_store(PhysicalRecordOpen::new(format, access, durability))
        .into_raw()
    {
        worth_proof::TransitionOutcome::Success(serving) => serving,
        _ => panic!("MUTANT_PREDICATE:c7-interrupted-successor-prefix-cleanup-omitted"),
    };
    serving.close();
    if partial.exists() {
        panic!("MUTANT_PREDICATE:c7-interrupted-successor-prefix-cleanup-omitted");
    }
}

#[test]
fn partial_first_frame_without_a_verified_predecessor_is_rejected_without_cleanup() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    initialize_empty_store(&store_root);
    let source_parent = tempfile::tempdir().unwrap();
    let source_root = source_parent.path().join("store");
    build_three_segment_inventory_with_limit(&source_root, 4);
    let source = std::fs::read(segment_path(&source_root, 1, 1)).unwrap();
    let partial = segment_path(&store_root, 1, 1);
    std::fs::write(&partial, &source[..37]).unwrap();

    assert_eq!(
        reopen_failure(&store_root, wal_policy(SEGMENT_BYTES, 4)),
        PhysicalWalOpenFailure::SegmentInspection(WalArtifactStoreDenial::InvalidFrame),
    );
    assert_eq!(std::fs::read(&partial).unwrap(), &source[..37]);
}
