use crate::state::SignalBranchHandle;
use worth_foundational::FoundationalBranchReferenceMismatchAxis;

use crate::data::error::SignalError;
use crate::state::SignalBranchId;

use super::SignalBranchIdentityConstructionDenial;
use super::{
    AdmittedSignalBranchBasis, SignalBranchRetentionAcquisitionDenial, SignalOwnerUnavailable,
};

#[derive(Debug)]
pub enum SignalBranchForkOperationDenial {
    OwnerUnavailable(SignalOwnerUnavailable),
    OperationCapacityExhausted {
        maximum_in_flight_operations: usize,
    },
    OwnerReentry,
    CancelledNoMovement,
    LiveBranchCapacityExhausted {
        maximum_live_branches: usize,
    },
    ReservationCapacityExhausted {
        maximum_reservations: usize,
    },
    NameAlreadyReserved,
    NameAlreadyInstalled,
    UnknownBranch {
        branch_id: SignalBranchId,
    },
    RetirementInProgress {
        branch_id: SignalBranchId,
    },
    RetiredBranch {
        branch_id: SignalBranchId,
    },
    QuarantinedBranch {
        branch_id: SignalBranchId,
    },
    OwnerCellMisuse {
        branch_id: SignalBranchId,
    },
    BasisMismatch {
        axes: Vec<FoundationalBranchReferenceMismatchAxis>,
    },
    RetentionUnavailable {
        denial: SignalBranchRetentionAcquisitionDenial,
    },
    InvalidIdentity {
        denial: SignalBranchIdentityConstructionDenial,
    },
    BranchIdentityExhausted,
    OwnerDeniedNoMovement {
        error: SignalError,
    },
}

/// Owner-issued result of a canonical Signal branch fork.
#[derive(Debug, Clone)]
pub struct SignalBranchForkOutcome {
    created_branch: SignalBranchHandle,
    created_basis: AdmittedSignalBranchBasis,
}

impl SignalBranchForkOutcome {
    pub(crate) fn owner_issued(
        created_branch: SignalBranchHandle,
        created_basis: AdmittedSignalBranchBasis,
    ) -> Self {
        Self {
            created_branch,
            created_basis,
        }
    }

    pub fn created_branch(&self) -> &SignalBranchHandle {
        &self.created_branch
    }

    pub fn created_basis(&self) -> &AdmittedSignalBranchBasis {
        &self.created_basis
    }

    pub fn into_parts(self) -> (SignalBranchHandle, AdmittedSignalBranchBasis) {
        (self.created_branch, self.created_basis)
    }
}
