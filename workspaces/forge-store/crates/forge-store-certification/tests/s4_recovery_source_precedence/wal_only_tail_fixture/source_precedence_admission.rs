use forge_store_recovery_physics::{
    WalLsnRange, WalOnlyTailProof, WalSegmentGeneration, WalSegmentId,
};
use forge_store_wal::{admit_replay_cursor, WalSegmentScanRecord, WalTopologyScan};

use super::wal_tail_evidence::vetted_wal_frame;

pub(crate) fn wal_only_tail_proof(range: WalLsnRange) -> WalOnlyTailProof {
    let cursor = admit_replay_cursor(
        WalTopologyScan::from_segment_scan([WalSegmentScanRecord::current(
            WalSegmentId::new(99).unwrap(),
            WalSegmentGeneration::new(1).unwrap(),
            range,
        )]),
        WalSegmentGeneration::new(1).unwrap(),
    )
    .unwrap();
    WalOnlyTailProof::from_vetted_wal_frame(&vetted_wal_frame(range), &cursor).unwrap()
}
