use super::{BranchId, RelationalRuntime, TransactionOptions};
use crate::facade::commit_strategies::CommitStrategiesAuthorityFacade;
use crate::transactions::RelationalTransaction;

/// Begin a main-branch transaction after obtaining its owner-issued binding.
///
/// Keeping the two borrows inside this helper preserves the same production
/// admission path while allowing fixtures to hold the runtime mutably for the
/// returned transaction.
pub(crate) fn test_owner_begin_transaction_for_main(
    runtime: &mut RelationalRuntime,
) -> RelationalTransaction<'_> {
    let options = test_owner_transaction_options_for_main(runtime);
    runtime.begin_transaction(options)
}

/// Begin a registered branch transaction after obtaining its owner-issued
/// binding.
pub(crate) fn test_owner_begin_transaction_for_branch(
    runtime: &mut RelationalRuntime,
    branch_id: BranchId,
) -> RelationalTransaction<'_> {
    let options = test_owner_transaction_options_for_branch(runtime, branch_id);
    runtime.begin_transaction(options)
}

/// Begin an owner-issued transaction with exact owner-issued merge parents.
pub(crate) fn test_owner_begin_merge_transaction(
    runtime: &mut RelationalRuntime,
    target_branch: BranchId,
    parent_branches: impl IntoIterator<Item = BranchId>,
) -> RelationalTransaction<'_> {
    let options = test_owner_merge_transaction_options(runtime, target_branch, parent_branches);
    runtime.begin_transaction(options)
}

/// Obtain an owner-issued strategy target and the mutable strategy authority
/// without overlapping the immutable binding lookup with the authority borrow.
pub(crate) fn test_owner_strategy_authority(
    runtime: &mut RelationalRuntime,
    target_branch: Option<BranchId>,
) -> (TransactionOptions, CommitStrategiesAuthorityFacade<'_>) {
    let options = target_branch
        .map(|branch| test_owner_transaction_options_for_branch(&*runtime, branch))
        .unwrap_or_else(|| test_owner_transaction_options_for_main(&*runtime));
    let authority = runtime.commit_strategies_authority();
    (options, authority)
}

/// Obtain transaction options only through the runtime's owner identity door.
/// Test fixtures must exercise the same binding path as production callers.
pub(crate) fn test_owner_transaction_options_for_main(
    runtime: &RelationalRuntime,
) -> TransactionOptions {
    let identity = runtime.main_branch_identity();
    runtime
        .transaction_options_for(&identity)
        .expect("configured main branch remains owner-admissible")
}

/// Obtain transaction options for an already registered owner branch.
pub(crate) fn test_owner_transaction_options_for_branch(
    runtime: &RelationalRuntime,
    branch_id: BranchId,
) -> TransactionOptions {
    let identity = runtime
        .branch_identity(&branch_id)
        .unwrap_or_else(|error| panic!("test branch must be owner-registered: {error:?}"));
    runtime
        .transaction_options_for(&identity)
        .expect("registered test branch remains owner-admissible")
}

/// Attach exact owner-issued parent bindings to an owner-issued target.
pub(crate) fn test_owner_merge_transaction_options(
    runtime: &RelationalRuntime,
    target_branch: BranchId,
    parent_branches: impl IntoIterator<Item = BranchId>,
) -> TransactionOptions {
    let target = test_owner_transaction_options_for_branch(runtime, target_branch);
    let parents = parent_branches
        .into_iter()
        .map(|branch_id| {
            test_owner_transaction_options_for_branch(runtime, branch_id)
                .branch_binding()
                .clone()
        })
        .collect();
    target.with_merge_parent_bindings(parents)
}
