use super::{inspect_complete_wal_segment, WalSegmentArtifactIdentity};
use crate::{
    plan_wal_frame_append, LogSequenceNumber, WalAppendFrontier, WalLsnRange, WalSegmentGeneration,
    WalSegmentId,
};

fn identity() -> WalSegmentArtifactIdentity {
    WalSegmentArtifactIdentity::new(
        WalSegmentId::new(7).unwrap(),
        WalSegmentGeneration::new(3).unwrap(),
    )
}

fn two_frames() -> Vec<u8> {
    let first_range =
        WalLsnRange::new(LogSequenceNumber::new(10), LogSequenceNumber::new(11)).unwrap();
    let first = plan_wal_frame_append(
        WalAppendFrontier::empty(identity().segment(), identity().generation()),
        first_range,
        "first",
        b"payload-a",
    )
    .unwrap();
    let second_range =
        WalLsnRange::new(LogSequenceNumber::new(11), LogSequenceNumber::new(13)).unwrap();
    let second = plan_wal_frame_append(
        first.resulting_frontier(),
        second_range,
        "second",
        b"payload-b",
    )
    .unwrap();
    [
        first.frame().encoded_frame(),
        second.frame().encoded_frame(),
    ]
    .concat()
}

#[test]
fn canonical_name_and_complete_segment_reconstruct_exact_identity_and_range() {
    let identity = identity();
    assert_eq!(
        WalSegmentArtifactIdentity::parse(&identity.file_name()),
        Some(identity)
    );
    assert!(WalSegmentArtifactIdentity::parse("segment-07-generation-3.wal").is_none());

    let bytes = two_frames();
    let inspection = inspect_complete_wal_segment(identity, &bytes).unwrap();
    assert_eq!(inspection.identity(), identity);
    assert_eq!(inspection.frame_count(), 2);
    assert_eq!(inspection.byte_count(), bytes.len() as u64);
    assert_eq!(inspection.lsn_range().start().get(), 10);
    assert_eq!(inspection.lsn_range().end_exclusive().get(), 13);
}

#[test]
fn incomplete_or_identity_substituted_segment_is_rejected() {
    let bytes = two_frames();
    assert!(inspect_complete_wal_segment(identity(), &bytes[..bytes.len() - 1]).is_err());
    let substituted =
        WalSegmentArtifactIdentity::new(WalSegmentId::new(8).unwrap(), identity().generation());
    assert!(inspect_complete_wal_segment(substituted, &bytes).is_err());
}
