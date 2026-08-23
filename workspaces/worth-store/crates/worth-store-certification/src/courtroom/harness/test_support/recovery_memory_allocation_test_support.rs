use worth_store::physical_runtime::RecoveryPhysicalAllocation;

pub(crate) fn with_recovery_memory_allocation<R>(
    run: impl FnOnce(RecoveryPhysicalAllocation<'_>) -> R,
) -> R {
    worth_store_test_support::harness::recovery::memory_budget::with_recovery_memory_allocation(run)
}
