use super::{RelationalBranchBasisDenial, RelationalBranchIdentity};
use crate::runtime::RelationalRuntime;

pub(crate) fn require_local_branch_identity(
    runtime: &RelationalRuntime,
    identity: &RelationalBranchIdentity,
) -> Result<(), RelationalBranchBasisDenial> {
    if identity.runtime_instance_id() != runtime.runtime_instance_id() {
        return Err(RelationalBranchBasisDenial::ForeignRuntime {
            expected_runtime_instance_id: runtime.runtime_instance_id(),
            actual_runtime_instance_id: identity.runtime_instance_id(),
        });
    }
    let current = runtime
        .history
        .branch_cell(identity.branch_id())
        .ok_or_else(|| RelationalBranchBasisDenial::UnknownBranch(identity.branch_id().clone()))?;
    if current.identity() != identity {
        return Err(RelationalBranchBasisDenial::MixedAxis(
            super::RelationalBranchBasisMismatchAxis::Branch,
        ));
    }
    Ok(())
}

pub(crate) fn identity_denial(
    denial: super::RelationalBranchIdentityDenial,
) -> RelationalBranchBasisDenial {
    match denial {
        super::RelationalBranchIdentityDenial::ForeignRuntime {
            expected_runtime_instance_id,
            actual_runtime_instance_id,
        } => RelationalBranchBasisDenial::ForeignRuntime {
            expected_runtime_instance_id,
            actual_runtime_instance_id,
        },
        super::RelationalBranchIdentityDenial::UnknownBranch(branch) => {
            RelationalBranchBasisDenial::UnknownBranch(branch)
        }
        super::RelationalBranchIdentityDenial::IdentityMismatch => {
            RelationalBranchBasisDenial::MixedAxis(super::RelationalBranchBasisMismatchAxis::Branch)
        }
    }
}
