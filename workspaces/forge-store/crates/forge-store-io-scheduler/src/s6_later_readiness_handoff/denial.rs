use forge_store_contracts::S6LaterMilestoneDestination;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6LaterReadinessHandoffDenial {
    RawCounterSourceCannotMintHandoff {
        destination: S6LaterMilestoneDestination,
    },
    CertificationOnlyEvidenceCannotMintHandoff {
        destination: S6LaterMilestoneDestination,
    },
    MissingBackgroundPacingEvidence {
        destination: S6LaterMilestoneDestination,
    },
    BackgroundPacingEvidenceForWrongLane {
        expected: S6LaterMilestoneDestination,
        actual: S6LaterMilestoneDestination,
    },
    MissingSecureIoScope {
        destination: S6LaterMilestoneDestination,
    },
    SecurityScopeMismatch {
        destination: S6LaterMilestoneDestination,
    },
    SecureIoOperationNotFoundation {
        destination: S6LaterMilestoneDestination,
    },
    SecureIoPostureNotFoundation {
        destination: S6LaterMilestoneDestination,
    },
}

pub const fn reject_raw_s6_counters_as_later_readiness_handoff(
    destination: S6LaterMilestoneDestination,
) -> Result<(), S6LaterReadinessHandoffDenial> {
    Err(S6LaterReadinessHandoffDenial::RawCounterSourceCannotMintHandoff { destination })
}

pub const fn reject_certification_only_evidence_as_later_readiness_handoff(
    destination: S6LaterMilestoneDestination,
) -> Result<(), S6LaterReadinessHandoffDenial> {
    Err(S6LaterReadinessHandoffDenial::CertificationOnlyEvidenceCannotMintHandoff { destination })
}
