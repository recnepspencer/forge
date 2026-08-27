use crate::branch::{
    RelationalBranchBasisDenial, RelationalBranchBasisMismatchAxis, RelationalBranchIdentityDenial,
};
use crate::history::data::BranchId;
use crate::mvcc::RelationalTransactionValidationInput;
use crate::runtime::RelationalRuntime;

pub(super) fn replay_transaction_context(
    runtime: &RelationalRuntime,
    target: &BranchId,
    merge_parents: &[BranchId],
) -> Result<RelationalTransactionValidationInput, RelationalBranchBasisDenial> {
    let target_identity = runtime
        .branch_identity(target)
        .map_err(identity_to_basis_denial)?;
    let context = runtime.transaction_validation_input_for(&target_identity)?;
    let parent_bases = merge_parents
        .iter()
        .map(|branch| {
            let identity = runtime
                .branch_identity(branch)
                .map_err(identity_to_basis_denial)?;
            runtime.observe_branch(&identity).map(|(_, basis)| basis)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(context.with_merge_parent_bases(parent_bases))
}

fn identity_to_basis_denial(denial: RelationalBranchIdentityDenial) -> RelationalBranchBasisDenial {
    match denial {
        RelationalBranchIdentityDenial::ForeignRuntime {
            expected_runtime_instance_id,
            actual_runtime_instance_id,
        } => RelationalBranchBasisDenial::ForeignRuntime {
            expected_runtime_instance_id,
            actual_runtime_instance_id,
        },
        RelationalBranchIdentityDenial::UnknownBranch(branch) => {
            RelationalBranchBasisDenial::UnknownBranch(branch)
        }
        RelationalBranchIdentityDenial::IdentityMismatch => {
            RelationalBranchBasisDenial::MixedAxis(RelationalBranchBasisMismatchAxis::Branch)
        }
    }
}
