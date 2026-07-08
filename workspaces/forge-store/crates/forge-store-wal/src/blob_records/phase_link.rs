use crate::{BlobWalRecordKind, DurablePublicationPhase};

/// Maps blob WAL record kinds to durable-publication lifecycle phases.
///
/// WAL records carry evidence; publication law remains in blob-chunks.
pub const fn durable_phase_for_record_kind(kind: BlobWalRecordKind) -> DurablePublicationPhase {
    match kind {
        BlobWalRecordKind::ChunkAppend | BlobWalRecordKind::RootCandidate => {
            DurablePublicationPhase::Prepared
        }
        BlobWalRecordKind::GenerationPublication | BlobWalRecordKind::SessionCheckpoint => {
            DurablePublicationPhase::Logged
        }
        BlobWalRecordKind::SessionCloseout => DurablePublicationPhase::Acknowledged,
    }
}

pub const fn record_kind_admits_recovery_replay(kind: BlobWalRecordKind) -> bool {
    matches!(
        kind,
        BlobWalRecordKind::GenerationPublication | BlobWalRecordKind::SessionCloseout
    )
}
