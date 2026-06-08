use serde::Serialize;

use crate::logic::transaction::runtime::state::SignalBranchBasisDenial;
use crate::state::SignalBranchId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SignalMergeCompatibilityDenialKind {
    BranchBasisDenied,
    StaleBranchBasis,
    MissingScopedMergeProof,
    MissingStrategyWitness,
    CrossBasisMismatch,
    ReadmissionBasisMismatch,
    ScopedMergeProofMismatch,
    StrategyWitnessMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SignalMergeCompatibilityDenial {
    BranchBasisDenied(SignalBranchBasisDenial),
    StaleBranchBasis {
        branch_id: SignalBranchId,
        basis_digest: String,
    },
    MissingScopedMergeProof {
        branch_id: SignalBranchId,
    },
    MissingStrategyWitness {
        branch_id: SignalBranchId,
    },
    CrossBasisMismatch {
        expected_branch_id: SignalBranchId,
        observed_branch_id: SignalBranchId,
    },
    ReadmissionBasisMismatch {
        expected_branch_basis_digest: String,
        observed_branch_basis_digest: String,
    },
    ScopedMergeProofMismatch {
        expected_declaration_digest: String,
        observed_declaration_digest: String,
    },
    StrategyWitnessMismatch {
        expected_witness_digest: String,
        observed_witness_digest: String,
    },
}

impl SignalMergeCompatibilityDenial {
    pub fn kind(&self) -> SignalMergeCompatibilityDenialKind {
        match self {
            Self::BranchBasisDenied(_) => SignalMergeCompatibilityDenialKind::BranchBasisDenied,
            Self::StaleBranchBasis { .. } => SignalMergeCompatibilityDenialKind::StaleBranchBasis,
            Self::MissingScopedMergeProof { .. } => {
                SignalMergeCompatibilityDenialKind::MissingScopedMergeProof
            }
            Self::MissingStrategyWitness { .. } => {
                SignalMergeCompatibilityDenialKind::MissingStrategyWitness
            }
            Self::CrossBasisMismatch { .. } => {
                SignalMergeCompatibilityDenialKind::CrossBasisMismatch
            }
            Self::ReadmissionBasisMismatch { .. } => {
                SignalMergeCompatibilityDenialKind::ReadmissionBasisMismatch
            }
            Self::ScopedMergeProofMismatch { .. } => {
                SignalMergeCompatibilityDenialKind::ScopedMergeProofMismatch
            }
            Self::StrategyWitnessMismatch { .. } => {
                SignalMergeCompatibilityDenialKind::StrategyWitnessMismatch
            }
        }
    }
}
