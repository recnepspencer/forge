use serde::Serialize;

use crate::diagnostics::replay::ReplayEventKind;
use crate::logic::transaction::runtime::state::merge::SignalMergeCompatibilityDenial;
use crate::state::SignalBranchId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SignalMergeSupportInspectionAbsenceKind {
    CompatibilityDenied,
    MissingCompatibilityWitness,
    ReplayDetailUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SignalMergeSupportInspectionAbsence {
    CompatibilityDenied(SignalMergeCompatibilityDenial),
    MissingCompatibilityWitness {
        branch_id: SignalBranchId,
    },
    ReplayDetailUnavailable {
        branch_id: SignalBranchId,
        replay_kind: ReplayEventKind,
    },
}

impl SignalMergeSupportInspectionAbsence {
    pub fn kind(&self) -> SignalMergeSupportInspectionAbsenceKind {
        match self {
            Self::CompatibilityDenied(_) => {
                SignalMergeSupportInspectionAbsenceKind::CompatibilityDenied
            }
            Self::MissingCompatibilityWitness { .. } => {
                SignalMergeSupportInspectionAbsenceKind::MissingCompatibilityWitness
            }
            Self::ReplayDetailUnavailable { .. } => {
                SignalMergeSupportInspectionAbsenceKind::ReplayDetailUnavailable
            }
        }
    }
}
