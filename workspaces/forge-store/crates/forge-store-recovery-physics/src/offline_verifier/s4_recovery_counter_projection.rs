use crate::RecoveryCounterSnapshot;

use super::decoded_s4_recovery_record_set::DecodedS4RecoveryRecords;

pub(super) fn project_s4_recovery_counters(
    decoded: &DecodedS4RecoveryRecords<'_>,
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
