use worth_store_recovery_physics::RecoveryMemoryAllocation;

pub(crate) fn with_recovery_memory_allocation<R>(
    run: impl FnOnce(RecoveryMemoryAllocation<'_>) -> R,
) -> R {
    worth_store_test_support::harness::recovery::memory_budget::with_recovery_memory_allocation(run)
}
