use crate::branch::{
    SignalBranchAdmissionLease, SignalBranchRetentionAcquisitionDenial, SignalBranchRetentionLease,
    SignalBranchRetentionReleaseOutcome,
};
use crate::state::SignalBranchId;

use super::catalog::BranchManager;

impl<D, I, T> BranchManager<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn acquire_admitted_retention(
        &self,
        branch_id: SignalBranchId,
    ) -> Result<SignalBranchAdmissionLease, SignalBranchRetentionAcquisitionDenial> {
        self.retention.acquire_admitted(branch_id)
    }

    pub fn acquire_retention(
        &self,
        runtime_instance_id: String,
        branch_id: SignalBranchId,
    ) -> Result<SignalBranchRetentionLease, SignalBranchRetentionAcquisitionDenial> {
        self.retention
            .acquire_external(runtime_instance_id, branch_id)
    }

    pub fn release_retention(
        &self,
        runtime_instance_id: &str,
        lease: SignalBranchRetentionLease,
    ) -> SignalBranchRetentionReleaseOutcome {
        self.retention.release_external(runtime_instance_id, lease)
    }

    pub fn branch_admitted_retention_count(&self, branch_id: SignalBranchId) -> u32 {
        self.retention.admitted_count(branch_id)
    }

    pub fn branch_retention_count(&self, branch_id: SignalBranchId) -> u32 {
        self.retention.external_count(branch_id)
    }
}
