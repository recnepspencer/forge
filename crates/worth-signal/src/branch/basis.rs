use super::authority::{mint_signal_branch_authority, SignalBranchBasisAuthority};
use super::reference::SignalBranchObservation;
use crate::state::SignalBranchId;

/// Owner-issued Signal observation token. Construction is private to the
/// Signal branch owner; callers cannot mint one from a descriptor.
#[derive(Debug)]
pub struct AdmittedSignalBranchBasis {
    observation: SignalBranchObservation,
    _authority: SignalBranchBasisAuthority,
    owner_branch_id: Option<SignalBranchId>,
}

impl Clone for AdmittedSignalBranchBasis {
    fn clone(&self) -> Self {
        Self {
            observation: self.observation.clone(),
            _authority: mint_signal_branch_authority(),
            owner_branch_id: self.owner_branch_id,
        }
    }
}

impl AdmittedSignalBranchBasis {
    pub fn observation(&self) -> &SignalBranchObservation {
        &self.observation
    }

    pub(crate) fn owner_branch_id(&self) -> Option<SignalBranchId> {
        self.owner_branch_id
    }
}

pub fn admit_signal_branch_observation(
    observation: SignalBranchObservation,
    authority: SignalBranchBasisAuthority,
) -> AdmittedSignalBranchBasis {
    AdmittedSignalBranchBasis {
        observation,
        _authority: authority,
        owner_branch_id: None,
    }
}

pub(crate) fn admit_runtime_signal_branch_observation(
    observation: SignalBranchObservation,
    branch_id: SignalBranchId,
) -> AdmittedSignalBranchBasis {
    AdmittedSignalBranchBasis {
        observation,
        _authority: mint_signal_branch_authority(),
        owner_branch_id: Some(branch_id),
    }
}
