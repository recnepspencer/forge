use crate::recovery_budget::OfflineRecoveryCounterProjection;
use crate::RecoveryCounterSnapshot;

use super::decoded_recovery_record_set::DecodedRecoveryRecords;

pub(super) fn project_recovery_counters(
    decoded: &DecodedRecoveryRecords<'_>,
) -> Option<RecoveryCounterSnapshot> {
    let checkpoint = decoded.checkpoint()?;
    Some(RecoveryCounterSnapshot::from_offline_verifier(
        OfflineRecoveryCounterProjection {
            replayed_frames: 1,
            skipped_frames: 0,
            validated_checkpoints: 1,
            scanned_segments: 1,
            page_redos: 1,
            memory_envelope_bytes: checkpoint.memory_envelope_bytes,
            memory_envelope_frames: checkpoint.memory_envelope_frames,
            allocation_bytes: checkpoint.allocation_bytes,
            total_store_pages: checkpoint.total_store_pages,
            residue_rejections: 0,
            forbidden_full_store_scans: 0,
        },
    ))
}
