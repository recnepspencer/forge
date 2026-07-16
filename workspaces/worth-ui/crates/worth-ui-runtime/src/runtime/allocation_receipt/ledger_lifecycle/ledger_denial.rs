pub(super) fn denied(
    denial: super::UiAllocationReplanTransactionCommitDenial,
) -> super::UiAllocationReplanTransactionOutcome {
    super::UiAllocationReplanTransactionOutcome::Denied(denial)
}

pub(super) fn retain_denial<T>(
    state: &mut super::ledger_state::UiAllocationReceiptLedgerState,
    transaction: &super::UiAllocationReplanTransaction,
    denial: super::UiAllocationReplanTransactionCommitDenial,
) -> T
where
    T: From<super::UiAllocationReplanTransactionOutcome>,
{
    let bucket = state
        .denied_transactions
        .entry(transaction.idempotency_key())
        .or_default();
    if let Some((_, retained)) = bucket
        .iter_mut()
        .find(|(item, _)| item.same_idempotency_basis(transaction))
    {
        *retained = denial.clone();
    } else {
        bucket.push((transaction.clone(), denial.clone()));
    }
    super::UiAllocationReplanTransactionOutcome::Denied(denial).into()
}
