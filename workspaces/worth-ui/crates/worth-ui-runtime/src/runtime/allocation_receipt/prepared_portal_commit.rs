#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiPortalAllocationCommitBindDenial {
    LedgerBorrowUnavailable,
    LedgerPredecessorChanged {
        expected_truth_revision: u64,
        observed_truth_revision: u64,
        expected_transaction_generation: u64,
        observed_transaction_generation: u64,
    },
    BindingPredecessorChanged {
        expected_identity_digest: u64,
        observed_identity_digest: u64,
    },
}
