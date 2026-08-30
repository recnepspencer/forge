use crate::branch::{SignalBranchBasisObservationDenial, SignalBranchObservation};
use crate::state::SignalBranchId;

use super::SignalOwner;
use crate::branch::owner_services::{
    SignalBranchRegistryDenial, SignalOwnerOperationAdmission, SignalOwnerUnavailable,
};

impl<D, I, T> SignalOwner<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    #[allow(dead_code, reason = "Phase 4 basis operations consume this owner seam")]
    pub(in crate::branch::owner_services) fn observe_branch_exact(
        &self,
        admission: &SignalOwnerOperationAdmission,
        branch_id: SignalBranchId,
    ) -> Result<SignalBranchObservation, SignalBranchBasisObservationDenial> {
        self.registry
            .lookup(admission, branch_id)
            .map_err(|denial| map_basis_registry_denial(denial, branch_id))?
            .observe_exact(admission)
    }
}

fn map_basis_registry_denial(
    denial: SignalBranchRegistryDenial,
    branch_id: SignalBranchId,
) -> SignalBranchBasisObservationDenial {
    match denial {
        SignalBranchRegistryDenial::ForeignOwner | SignalBranchRegistryDenial::ExpiredAdmission => {
            SignalBranchBasisObservationDenial::OwnerUnavailable(SignalOwnerUnavailable)
        }
        SignalBranchRegistryDenial::UnknownBranch(_) => {
            SignalBranchBasisObservationDenial::UnknownBranch { branch_id }
        }
        SignalBranchRegistryDenial::RetirementInProgress(_) => {
            SignalBranchBasisObservationDenial::RetirementInProgress { branch_id }
        }
        SignalBranchRegistryDenial::TargetCellDenied(denial) => {
            crate::branch::owner_services::branch_execution_cell::basis::map_basis_cell_denial(
                denial, branch_id,
            )
        }
        SignalBranchRegistryDenial::DuplicateBranch(_)
        | SignalBranchRegistryDenial::LiveCapacityExhausted { .. }
        | SignalBranchRegistryDenial::ReservationCapacityExhausted { .. }
        | SignalBranchRegistryDenial::ExpiredRetirement(_)
        | SignalBranchRegistryDenial::OwnerMetadataOrdering => {
            SignalBranchBasisObservationDenial::OwnerInvariantViolation { branch_id }
        }
    }
}
