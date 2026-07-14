pub(super) fn admit_replan_budget(
    selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
    durable_resize: bool,
) -> Result<(), super::UiAllocationReplanTransactionCommitDenial> {
    let budget = selection.policy().budget();
    let attempted = u16::try_from(selection.ordered_neighborhoods().len()).unwrap_or(u16::MAX);
    if attempted > budget.max_committed_receipts() {
        return Err(
            super::UiAllocationReplanTransactionCommitDenial::CommitBudgetExceeded {
                attempted,
                maximum: budget.max_committed_receipts(),
            },
        );
    }
    if durable_resize && budget.max_durable_mutations() < 1 {
        return Err(
            super::UiAllocationReplanTransactionCommitDenial::DurableMutationBudgetExceeded {
                attempted: 1,
                maximum: budget.max_durable_mutations(),
            },
        );
    }
    Ok(())
}
