use crate::source_precedence::RecoverySourceReplayBasis;
use crate::{LogSequenceNumber, PageLsn, RecoveredPhysicalState};

use super::decoded_s4_recovery_record_set::DecodedS4RecoveryRecords;

pub(super) fn project_s4_recovered_physical_state(
    decoded: &DecodedS4RecoveryRecords<'_>,
) -> Option<RecoveredPhysicalState> {
    let checkpoint = decoded.checkpoint()?;
    let wal_frame = decoded.wal_frame()?;
    Some(RecoveredPhysicalState::from_projected_parts(
        format!(
            "s4-redo-root[{}:{}:{}:{}]",
            wal_frame.page_id,
            wal_frame.lsn,
            wal_frame.operation_digest,
            wal_frame.idempotence_digest
        ),
        Some(PageLsn::from_lsn(LogSequenceNumber::new(wal_frame.lsn))),
        RecoverySourceReplayBasis::empty(),
        format!(
            "CheckpointPlusWalTail:{}:{}",
            checkpoint.source_profile, checkpoint.source_candidate_count
        ),
        1,
        0,
    ))
}
