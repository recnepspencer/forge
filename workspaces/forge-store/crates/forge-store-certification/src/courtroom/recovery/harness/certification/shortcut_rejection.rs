use crate::courtroom::recovery::harness::{
    RecoveryPhysicsCounterExpectation, RecoveryPhysicsCounterKind, RecoveryPhysicsOracleKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryPhysicsShortcutAttempt {
    LiveStateReuse,
    LogsOnlyProof,
    BackendResidueGuessing,
    DirectPrivateMutation,
    SameRunSelfComparison,
}

impl RecoveryPhysicsShortcutAttempt {
    pub const fn required_s4_denials() -> [Self; 5] {
        [
            Self::LiveStateReuse,
            Self::LogsOnlyProof,
            Self::BackendResidueGuessing,
            Self::DirectPrivateMutation,
            Self::SameRunSelfComparison,
        ]
    }

    pub const fn denial_boundary(self) -> RecoveryPhysicsShortcutDenialBoundary {
        match self {
            Self::LiveStateReuse => RecoveryPhysicsShortcutDenialBoundary::FreshRuntimeRecovery,
            Self::LogsOnlyProof => RecoveryPhysicsShortcutDenialBoundary::EvidenceBundle,
            Self::BackendResidueGuessing => {
                RecoveryPhysicsShortcutDenialBoundary::StorageBoundaryInterposer
            }
            Self::DirectPrivateMutation => RecoveryPhysicsShortcutDenialBoundary::PublicFacade,
            Self::SameRunSelfComparison => {
                RecoveryPhysicsShortcutDenialBoundary::IndependentFreshObserver
            }
        }
    }

    pub const fn denial_reason(self) -> RecoveryPhysicsShortcutDenialReason {
        match self {
            Self::LiveStateReuse => RecoveryPhysicsShortcutDenialReason::LiveStateCannotProveCrash,
            Self::LogsOnlyProof => RecoveryPhysicsShortcutDenialReason::LogsAreNotRecoveryEvidence,
            Self::BackendResidueGuessing => {
                RecoveryPhysicsShortcutDenialReason::BackendResidueIsNotSourceTruth
            }
            Self::DirectPrivateMutation => {
                RecoveryPhysicsShortcutDenialReason::PrivateMutationCannotEnterCertification
            }
            Self::SameRunSelfComparison => {
                RecoveryPhysicsShortcutDenialReason::SameRunComparisonCannotProveFreshRestart
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryPhysicsShortcutDenialBoundary {
    FreshRuntimeRecovery,
    EvidenceBundle,
    StorageBoundaryInterposer,
    PublicFacade,
    IndependentFreshObserver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryPhysicsShortcutDenialReason {
    LiveStateCannotProveCrash,
    LogsAreNotRecoveryEvidence,
    BackendResidueIsNotSourceTruth,
    PrivateMutationCannotEnterCertification,
    SameRunComparisonCannotProveFreshRestart,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsShortcutRejection {
    attempt: RecoveryPhysicsShortcutAttempt,
    boundary: RecoveryPhysicsShortcutDenialBoundary,
    reason: RecoveryPhysicsShortcutDenialReason,
    oracle: RecoveryPhysicsOracleKind,
    counter: RecoveryPhysicsCounterExpectation,
}

impl RecoveryPhysicsShortcutRejection {
    pub(crate) const fn denied(attempt: RecoveryPhysicsShortcutAttempt) -> Self {
        Self {
            attempt,
            boundary: attempt.denial_boundary(),
            reason: attempt.denial_reason(),
            oracle: RecoveryPhysicsOracleKind::RejectSyntheticShortcut,
            counter: RecoveryPhysicsCounterExpectation::exact(
                RecoveryPhysicsCounterKind::ShortcutDenials,
                1,
            ),
        }
    }

    pub const fn attempt(&self) -> RecoveryPhysicsShortcutAttempt {
        self.attempt
    }

    pub const fn boundary(&self) -> RecoveryPhysicsShortcutDenialBoundary {
        self.boundary
    }

    pub const fn reason(&self) -> RecoveryPhysicsShortcutDenialReason {
        self.reason
    }

    pub const fn oracle(&self) -> RecoveryPhysicsOracleKind {
        self.oracle
    }

    pub const fn counter(&self) -> RecoveryPhysicsCounterExpectation {
        self.counter
    }
}
