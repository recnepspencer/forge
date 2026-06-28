use forge_proof::raw::{CanonicalVec, NonEmpty, TransitionOutcome, UniqueVec};

use super::super::denial::RecoveryEvidenceDenial;
use super::super::executed_evidence_source::RecoveryPhysicsEvidenceSource;
use super::checked_recipe::{
    checked_executed_replay, checked_recipe_outcome, CheckedExecutedRecoveryReplayOutcome,
    CheckedRecoveryRecipeOutcome,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecoveryProofProgressionStep {
    RecoveryEntry,
    LoweredRedoPlan,
    ExecutionReadyReplay,
    ExecutedReplay,
    StaleRestart,
    BoundaryReadmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecoveryProofSourceFamily {
    Checkpoint,
    Wal,
    RecoveredState,
    OfflineVerifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofProgressionRecoveryTrace {
    steps: NonEmpty<RecoveryProofProgressionStep>,
    outcome: TransitionOutcome<(), RecoveryEvidenceDenial>,
    checked_recipe_outcome: CheckedRecoveryRecipeOutcome,
}

impl ProofProgressionRecoveryTrace {
    pub fn from_source(source: &RecoveryPhysicsEvidenceSource) -> Self {
        let _authority = source.authority();
        Self {
            steps: NonEmpty::new(
                RecoveryProofProgressionStep::RecoveryEntry,
                vec![
                    RecoveryProofProgressionStep::LoweredRedoPlan,
                    RecoveryProofProgressionStep::ExecutionReadyReplay,
                    RecoveryProofProgressionStep::ExecutedReplay,
                    RecoveryProofProgressionStep::StaleRestart,
                    RecoveryProofProgressionStep::BoundaryReadmission,
                ],
            ),
            outcome: TransitionOutcome::success(()),
            checked_recipe_outcome: checked_recipe_outcome(),
        }
    }

    pub fn deny_empty_steps() -> RecoveryEvidenceDenial {
        RecoveryEvidenceDenial::EmptyProofCollection
    }

    pub fn admit_wal_replay_order(
        lsns: Vec<u64>,
    ) -> Result<CanonicalVec<u64>, RecoveryEvidenceDenial> {
        CanonicalVec::try_from_sorted(lsns)
            .map_err(|_| RecoveryEvidenceDenial::NonCanonicalWalReplayOrder)
    }

    pub fn admit_source_families(
        families: Vec<RecoveryProofSourceFamily>,
    ) -> Result<UniqueVec<RecoveryProofSourceFamily>, RecoveryEvidenceDenial> {
        UniqueVec::try_from_unique(families)
            .map_err(|_| RecoveryEvidenceDenial::DuplicateRecoverySourceFamily)
    }

    pub fn steps(&self) -> &NonEmpty<RecoveryProofProgressionStep> {
        &self.steps
    }

    pub const fn outcome(&self) -> &TransitionOutcome<(), RecoveryEvidenceDenial> {
        &self.outcome
    }

    pub const fn checked_recipe_admitted(&self) -> bool {
        matches!(self.checked_recipe_outcome, TransitionOutcome::Success(_))
    }

    pub const fn checked_recipe_outcome(&self) -> &CheckedRecoveryRecipeOutcome {
        &self.checked_recipe_outcome
    }

    pub fn checked_executed_replay(&self) -> CheckedExecutedRecoveryReplayOutcome {
        checked_executed_replay()
    }
}
