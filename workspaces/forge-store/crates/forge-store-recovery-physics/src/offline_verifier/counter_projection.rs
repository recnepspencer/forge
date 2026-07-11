use crate::RecoveryCounterSnapshot;

use super::decoded_recovery_record_set::DecodedRecoveryRecords;

pub(super) fn project_recovery_counters(
    decoded: &DecodedRecoveryRecords<'_>,
) -> Option<RecoveryCounterSnapshot> {
    let checkpoint = decoded.checkpoint()?;
    Some(RecoveryCounterSnapshot::from_offline_verifier(
        1,
        0,
        1,
        1,
        1,
        checkpoint.memory_envelope_bytes,
        checkpoint.memory_envelope_frames,
        checkpoint.allocation_bytes,
        checkpoint.total_store_pages,
        0,
        0,
    ))
}
