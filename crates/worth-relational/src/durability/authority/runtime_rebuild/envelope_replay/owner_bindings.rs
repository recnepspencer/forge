use crate::durability::data::{DurabilityError, RecoveryFailureClass};
use crate::history::data::BranchId;
use crate::runtime::RelationalRuntime;
use crate::transactions::data::TransactionOptions;

pub(super) fn owner_options_for_branch(
    restored: &RelationalRuntime,
    branch: &BranchId,
) -> Result<TransactionOptions, DurabilityError> {
    let identity = restored.branch_identity(branch).map_err(|denial| {
        DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            format!("recovered branch cannot issue transaction binding: {denial:?}"),
        )
    })?;
    restored
        .transaction_options_for(&identity)
        .map_err(|denial| {
            DurabilityError::new(
                RecoveryFailureClass::ReplayFailure,
                format!("recovered branch binding was denied: {denial:?}"),
            )
        })
}

pub(super) fn owner_merge_parent_bindings(
    restored: &RelationalRuntime,
    branches: &[BranchId],
) -> Result<Vec<crate::branch::RelationalLegacyBranchBinding>, DurabilityError> {
    branches
        .iter()
        .map(|branch| {
            let identity = restored.branch_identity(branch).map_err(|denial| {
                DurabilityError::new(
                    RecoveryFailureClass::ReplayFailure,
                    format!("recovered merge parent identity was denied: {denial:?}"),
                )
            })?;
            restored
                .transaction_options_for(&identity)
                .map(|options| options.branch_binding().clone())
                .map_err(|denial| {
                    DurabilityError::new(
                        RecoveryFailureClass::ReplayFailure,
                        format!("recovered merge parent binding was denied: {denial:?}"),
                    )
                })
        })
        .collect()
}
