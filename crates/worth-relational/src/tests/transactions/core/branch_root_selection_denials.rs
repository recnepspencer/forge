use crate::branch::{RelationalBranchRoot, SelectedRelationalBranchState};
use crate::facade::history::BranchId;
use crate::facade::transactions::{
    CommitPreparationReason, SelectedBranchRootDenialReason, TransactionCommitError,
};
use crate::tests::support::{batch_create, create_entity_outcome, runtime_with_test_schema};

fn committed_child(runtime: &mut crate::runtime::RelationalRuntime) -> BranchId {
    create_entity_outcome(runtime, "root-selection-source");
    let source = BranchId("main".to_owned());
    let (_, basis) = runtime
        .observe_fork_source(&source)
        .expect("committed source has a fork basis");
    let child = BranchId("root-selection-child".to_owned());
    runtime
        .fork_branch(child.clone(), basis)
        .expect("fork installs a committed child root");
    child
}

#[test]
fn unavailable_committed_root_denies_before_planning() {
    let mut runtime = runtime_with_test_schema();
    let child = committed_child(&mut runtime);
    runtime
        .history
        .branch_cell_mut(&child)
        .expect("child remains registered")
        .clear_root_for_test();
    let options = runtime
        .owner_transaction_options_for_branch(&child)
        .expect("owner can issue the still-current binding");
    let symbols_before = runtime.services.symbols.clone();
    let symbol_table_before = runtime.config().identity.symbol_table.clone();
    let commit_count = runtime.history().immutable_commit_count();

    let mut transaction = runtime.begin_transaction(options);
    transaction.push_batch(batch_create("raw-key-denied-before-root-selection"));
    let error = transaction
        .commit()
        .expect_err("missing selected root must fail closed before normalization");
    assert!(matches!(
        error,
        TransactionCommitError::Preparation { error, .. }
            if error.reason()
                == CommitPreparationReason::SelectedBranchRoot(
                    SelectedBranchRootDenialReason::Unavailable,
                )
    ));
    assert_eq!(runtime.history().immutable_commit_count(), commit_count);
    assert_eq!(runtime.services.symbols, symbols_before);
    assert_eq!(runtime.config().identity.symbol_table, symbol_table_before);
}

#[test]
fn committed_root_reference_mismatch_denies_before_planning() {
    let mut runtime = runtime_with_test_schema();
    let child = committed_child(&mut runtime);
    runtime
        .history
        .branch_cell_mut(&child)
        .expect("child remains registered")
        .install_root(RelationalBranchRoot::empty());
    let options = runtime
        .owner_transaction_options_for_branch(&child)
        .expect("owner can issue the still-current binding");
    let commit_count = runtime.history().immutable_commit_count();

    let error = runtime
        .begin_transaction(options)
        .commit()
        .expect_err("mismatched selected root must fail closed");
    assert!(matches!(
        error,
        TransactionCommitError::Preparation { error, .. }
            if error.reason()
                == CommitPreparationReason::SelectedBranchRoot(
                    SelectedBranchRootDenialReason::ReferenceMismatch,
                )
    ));
    assert_eq!(runtime.history().immutable_commit_count(), commit_count);
}

#[test]
fn selected_state_shape_keeps_empty_and_committed_roots_distinct() {
    let mut runtime = runtime_with_test_schema();
    let empty = BranchId("root-selection-empty".to_owned());
    runtime.history.insert_branch_cell(
        crate::branch::RelationalBranchReferenceCell::empty(
            runtime.runtime_instance_id(),
            empty.clone(),
        )
        .expect("empty branch identity is valid"),
    );
    let empty_state = runtime
        .selected_branch_state(
            runtime
                .owner_transaction_options_for_branch(&empty)
                .expect("empty binding")
                .branch_binding(),
        )
        .expect("empty state is selectable");
    assert!(matches!(
        empty_state,
        SelectedRelationalBranchState::Empty(_)
    ));
    assert_eq!(empty_state.version_id().0, 0);
}
