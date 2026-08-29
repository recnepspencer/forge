use super::{BranchId, RelationalRuntime, RelationalTransactionValidationInput};
use crate::facade::commit_strategies::CommitStrategiesAuthorityFacade;
use crate::mvcc::BranchBoundRelationalTransaction;

/// Observe the configured main identity explicitly and return its owner basis.
pub(crate) fn test_owner_main_basis(
    runtime: &RelationalRuntime,
) -> Result<crate::branch::AdmittedRelationalBranchBasis, crate::branch::RelationalBranchBasisDenial>
{
    let identity = runtime.main_branch_identity();
    runtime.admit_branch_basis(&identity)
}

/// Begin a main-branch transaction after observing its admitted basis.
///
/// Keeping the two borrows inside this helper preserves the same production
/// admission path while allowing fixtures to hold the runtime mutably for the
/// returned transaction.
pub(crate) fn test_owner_begin_transaction_for_main(
    runtime: &RelationalRuntime,
) -> BranchBoundRelationalTransaction {
    let identity = runtime.main_branch_identity();
    let (_, basis) = runtime
        .observe_branch(&identity)
        .expect("configured main branch remains owner-admissible");
    runtime
        .begin_branch_transaction(&basis, crate::mvcc::RelationalTransactionIntent::ordinary())
        .expect("main basis belongs to its issuing runtime")
}

/// Begin a registered branch transaction after observing its admitted basis.
pub(crate) fn test_owner_begin_transaction_for_branch(
    runtime: &RelationalRuntime,
    branch_id: BranchId,
) -> BranchBoundRelationalTransaction {
    let identity = runtime
        .branch_identity(&branch_id)
        .unwrap_or_else(|error| panic!("test branch must be owner-registered: {error:?}"));
    let (_, basis) = runtime
        .observe_branch(&identity)
        .expect("registered test branch remains owner-admissible");
    runtime
        .begin_branch_transaction(&basis, crate::mvcc::RelationalTransactionIntent::ordinary())
        .expect("branch basis belongs to its issuing runtime")
}

/// Begin an owner-issued transaction with exact owner-issued merge parents.
pub(crate) fn test_owner_begin_merge_transaction(
    runtime: &RelationalRuntime,
    target_branch: BranchId,
    parent_branches: impl IntoIterator<Item = BranchId>,
) -> BranchBoundRelationalTransaction {
    let target_identity = runtime
        .branch_identity(&target_branch)
        .unwrap_or_else(|error| panic!("merge target must be owner-registered: {error:?}"));
    let (_, target_basis) = runtime
        .observe_branch(&target_identity)
        .expect("merge target remains owner-admissible");
    let parent_bases = parent_branches
        .into_iter()
        .map(|branch_id| {
            let identity = runtime
                .branch_identity(&branch_id)
                .unwrap_or_else(|error| panic!("merge parent must be owner-registered: {error:?}"));
            runtime
                .observe_branch(&identity)
                .map(|(_, basis)| basis)
                .expect("merge parent remains owner-admissible")
        })
        .collect();
    let mut transaction = runtime
        .begin_branch_transaction(
            &target_basis,
            crate::mvcc::RelationalTransactionIntent::ordinary(),
        )
        .expect("merge target basis belongs to its issuing runtime");
    transaction.merge_parent_bases = parent_bases;
    transaction
}

/// Obtain an owner-issued strategy target and the mutable strategy authority
/// without overlapping the immutable binding lookup with the authority borrow.
pub(crate) fn test_owner_strategy_authority(
    runtime: &RelationalRuntime,
    target_branch: Option<BranchId>,
) -> (
    RelationalTransactionValidationInput,
    CommitStrategiesAuthorityFacade,
) {
    let options = target_branch
        .map(|branch| test_owner_transaction_validation_input_for_branch(&*runtime, branch))
        .unwrap_or_else(|| test_owner_transaction_validation_input_for_main(&*runtime));
    let authority = runtime.commit_strategies_authority();
    (options, authority)
}

/// Obtain owner-private validation input through the runtime's identity door.
pub(crate) fn test_owner_transaction_validation_input_for_main(
    runtime: &RelationalRuntime,
) -> RelationalTransactionValidationInput {
    let identity = runtime.main_branch_identity();
    runtime
        .transaction_validation_input_for(&identity)
        .expect("configured main branch remains owner-admissible")
}

/// Obtain owner-private validation input for an already registered branch.
pub(crate) fn test_owner_transaction_validation_input_for_branch(
    runtime: &RelationalRuntime,
    branch_id: BranchId,
) -> RelationalTransactionValidationInput {
    let identity = runtime
        .branch_identity(&branch_id)
        .unwrap_or_else(|error| panic!("test branch must be owner-registered: {error:?}"));
    runtime
        .transaction_validation_input_for(&identity)
        .expect("registered test branch remains owner-admissible")
}
