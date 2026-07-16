#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SharedFrontierAction {
    DurabilityAdmitted,
    RecoveryPrecedencePreserved,
    LiveLeaseAcquired,
    LeaseReleased,
    CompactionCutover,
    Crash,
    Reopen,
    QuarantineSealed,
    QuarantineVerificationStarted,
    QuarantineReadmitted,
    ReclaimDeferred,
    ReclaimReleased,
    GenerationReused,
    CheckpointPublicationRequested,
    ImportAdmissionPending,
    ReplicationAdmissionPending,
    ExternalDurabilityAdmitted,
    ExternalPublicationRequested,
    ReplicationDivergenceDetected,
}

impl SharedFrontierAction {
    pub const fn all() -> [Self; 19] {
        [
            Self::DurabilityAdmitted,
            Self::RecoveryPrecedencePreserved,
            Self::LiveLeaseAcquired,
            Self::LeaseReleased,
            Self::CompactionCutover,
            Self::Crash,
            Self::Reopen,
            Self::QuarantineSealed,
            Self::QuarantineVerificationStarted,
            Self::QuarantineReadmitted,
            Self::ReclaimDeferred,
            Self::ReclaimReleased,
            Self::GenerationReused,
            Self::CheckpointPublicationRequested,
            Self::ImportAdmissionPending,
            Self::ReplicationAdmissionPending,
            Self::ExternalDurabilityAdmitted,
            Self::ExternalPublicationRequested,
            Self::ReplicationDivergenceDetected,
        ]
    }
}
