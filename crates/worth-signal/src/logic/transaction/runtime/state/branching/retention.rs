use crate::branch::{
    AdmittedSignalBranchBasis, SignalBranchRetentionAcquisitionDenial, SignalBranchRetentionLease,
    SignalBranchRetentionReleaseOutcome,
};

use super::super::runtime_state::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn retain_signal_component_basis(
        &mut self,
        basis: &AdmittedSignalBranchBasis,
    ) -> Result<SignalBranchRetentionLease, SignalBranchRetentionAcquisitionDenial> {
        let branch_id = basis.owner_branch_id();
        let runtime_instance_id = self.branches.owner_runtime_instance_id().to_string();
        let Some(target) = basis.observation().target().as_basis() else {
            return Err(SignalBranchRetentionAcquisitionDenial::ForeignBasis);
        };
        if target.graph_instance_id() != runtime_instance_id {
            return Err(SignalBranchRetentionAcquisitionDenial::ForeignBasis);
        }
        if self.branches.branch_handle(branch_id).is_none() {
            return if self.branches.branch_retirement_receipt(branch_id).is_some() {
                Err(SignalBranchRetentionAcquisitionDenial::RetiredBranch { branch_id })
            } else {
                Err(SignalBranchRetentionAcquisitionDenial::UnknownBranch { branch_id })
            };
        }
        let live = self
            .signal_branch_observation(
                &self
                    .branches
                    .branch_handle(branch_id)
                    .expect("checked live Signal branch must remain catalogued"),
            )
            .map_err(|_| SignalBranchRetentionAcquisitionDenial::UnknownBranch { branch_id })?;
        if let Err(mismatch) = basis.observation().compare(&live) {
            return Err(SignalBranchRetentionAcquisitionDenial::StaleBasis {
                axes: mismatch.axes().to_vec(),
            });
        }
        self.branches
            .acquire_retention(runtime_instance_id, branch_id)
    }

    pub fn release_signal_component_basis(
        &mut self,
        lease: SignalBranchRetentionLease,
    ) -> SignalBranchRetentionReleaseOutcome {
        let runtime_instance_id = self.branches.owner_runtime_instance_id().to_string();
        self.branches
            .release_retention(runtime_instance_id.as_str(), lease)
    }
}
