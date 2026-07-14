use super::BlobRecoveryRecordCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobRecoveryRecordDenialKind {
    ChunkBytesWithoutIntegrityAdmission,
    IntegrityWithoutCheckpointFrontier,
    CheckpointFrontierWithoutRootCandidate,
    RootCandidateWithoutPublication,
    PublicationWithoutClosedResumeSession,
    PublicationWithoutManifestAgreement,
    MissingWalSource,
    MissingCheckpointSource,
    MissingManifestSource,
    BackendResidueRejected,
    WalRecordKindMismatch,
    ManifestRowMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRecoveryRecordDenial {
    kind: BlobRecoveryRecordDenialKind,
    counters: BlobRecoveryRecordCounterSnapshot,
}

impl BlobRecoveryRecordDenial {
    pub(crate) const fn start(kind: BlobRecoveryRecordDenialKind) -> Self {
        Self {
            kind,
            counters: BlobRecoveryRecordCounterSnapshot::start().with_denial(),
        }
    }

    pub const fn kind(&self) -> BlobRecoveryRecordDenialKind {
        self.kind
    }

    pub const fn counters(&self) -> BlobRecoveryRecordCounterSnapshot {
        self.counters
    }
}
