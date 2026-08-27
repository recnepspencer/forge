use crate::history::data::{CanonicalCommitEnvelope, PositionedCanonicalCommit};
use crate::publication::patch::data::PatchStreamPosition;

/// One migration-owned recovery input after its wire vocabulary has been
/// decoded and schema-readmitted. Current recovery code consumes this generic
/// contract and never branches on a historical product version.
#[derive(Debug, Clone)]
pub(crate) struct ReadmittedCanonicalCommit {
    state: ReadmittedCanonicalCommitState,
}

#[derive(Debug, Clone)]
enum ReadmittedCanonicalCommitState {
    Exact(PositionedCanonicalCommit),
    RequiresReplayCompletion {
        position: PatchStreamPosition,
        canonical: CanonicalCommitEnvelope,
    },
}

impl ReadmittedCanonicalCommit {
    pub(crate) fn exact(positioned: PositionedCanonicalCommit) -> Self {
        Self {
            state: ReadmittedCanonicalCommitState::Exact(positioned),
        }
    }

    pub(in crate::durability::migration) fn requires_replay_completion(
        position: PatchStreamPosition,
        canonical: CanonicalCommitEnvelope,
    ) -> Self {
        Self {
            state: ReadmittedCanonicalCommitState::RequiresReplayCompletion {
                position,
                canonical,
            },
        }
    }

    pub(crate) fn readmit_current(
        position: PatchStreamPosition,
        canonical: CanonicalCommitEnvelope,
    ) -> Result<Self, String> {
        crate::runtime::readmit_positioned_canonical_commit(
            position,
            std::sync::Arc::new(canonical),
        )
        .map(Self::exact)
    }

    pub(crate) fn position(&self) -> PatchStreamPosition {
        match &self.state {
            ReadmittedCanonicalCommitState::Exact(commit) => commit.position(),
            ReadmittedCanonicalCommitState::RequiresReplayCompletion { position, .. } => *position,
        }
    }

    pub(crate) fn envelope(&self) -> &CanonicalCommitEnvelope {
        match &self.state {
            ReadmittedCanonicalCommitState::Exact(commit) => commit.envelope(),
            ReadmittedCanonicalCommitState::RequiresReplayCompletion { canonical, .. } => canonical,
        }
    }

    pub(crate) fn positioned(&self) -> Option<&PositionedCanonicalCommit> {
        match &self.state {
            ReadmittedCanonicalCommitState::Exact(commit) => Some(commit),
            ReadmittedCanonicalCommitState::RequiresReplayCompletion { .. } => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn envelope_mut_for_test(&mut self) -> &mut CanonicalCommitEnvelope {
        match &mut self.state {
            ReadmittedCanonicalCommitState::Exact(commit) => commit.envelope_mut_for_test(),
            ReadmittedCanonicalCommitState::RequiresReplayCompletion { canonical, .. } => canonical,
        }
    }

    pub(crate) fn needs_replay_completion(&self) -> bool {
        matches!(
            self.state,
            ReadmittedCanonicalCommitState::RequiresReplayCompletion { .. }
        )
    }

    pub(crate) fn complete(
        self,
        replayed: &CanonicalCommitEnvelope,
    ) -> Result<PositionedCanonicalCommit, String> {
        match self.state {
            ReadmittedCanonicalCommitState::Exact(commit) => Ok(commit),
            ReadmittedCanonicalCommitState::RequiresReplayCompletion {
                position,
                mut canonical,
            } => {
                canonical.install_record_allocations(replayed.record_allocations().to_vec());
                canonical.branch_cell_checkpoint = replayed.branch_cell_checkpoint.clone();
                crate::runtime::readmit_positioned_canonical_commit(
                    position,
                    std::sync::Arc::new(canonical),
                )
            }
        }
    }

    pub(crate) fn complete_metadata(
        self,
        checkpoint: crate::branch::RelationalBranchCellCheckpoint,
    ) -> Result<PositionedCanonicalCommit, String> {
        match self.state {
            ReadmittedCanonicalCommitState::Exact(commit) => Ok(commit),
            ReadmittedCanonicalCommitState::RequiresReplayCompletion {
                position,
                mut canonical,
            } => {
                canonical.branch_cell_checkpoint = Some(checkpoint);
                crate::runtime::readmit_positioned_canonical_commit(
                    position,
                    std::sync::Arc::new(canonical),
                )
            }
        }
    }
}
