use crate::declaration::UiDeclarationGraphHandoffDenial;
use crate::graph::{UiGraphInstantiationDenial, UiGraphMutationCommitDenial};
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiApplicationPreparationPhase {
    CandidateBasis,
    GraphHandoff,
    GraphAdmission,
    GraphCommit,
}

/// Phase-local denial from the single public application-preparation lane.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiApplicationPreparationDenial {
    CandidateSnapshotMismatch {
        candidate_snapshot_digest: u64,
        prepared_snapshot_digest: u64,
    },
    GraphHandoff(UiDeclarationGraphHandoffDenial),
    GraphAdmission(UiGraphInstantiationDenial),
    GraphCommit(UiGraphMutationCommitDenial),
}

impl WorthUiApplicationPreparationDenial {
    pub fn phase(&self) -> WorthUiApplicationPreparationPhase {
        match self {
            Self::CandidateSnapshotMismatch { .. } => {
                WorthUiApplicationPreparationPhase::CandidateBasis
            }
            Self::GraphHandoff(_) => WorthUiApplicationPreparationPhase::GraphHandoff,
            Self::GraphAdmission(_) => WorthUiApplicationPreparationPhase::GraphAdmission,
            Self::GraphCommit(_) => WorthUiApplicationPreparationPhase::GraphCommit,
        }
    }
}
