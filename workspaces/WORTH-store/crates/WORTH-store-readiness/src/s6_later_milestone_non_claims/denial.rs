use super::S6LaterMilestoneDestination;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6LaterMilestoneHandoffDenial {
    WrongDestination {
        expected: S6LaterMilestoneDestination,
        actual: S6LaterMilestoneDestination,
    },
    RawCounterSourceCannotMintHandoff {
        destination: S6LaterMilestoneDestination,
    },
    CertificationOnlyEvidenceCannotMintHandoff {
        destination: S6LaterMilestoneDestination,
    },
    MissingNonClaim {
        destination: S6LaterMilestoneDestination,
    },
}
