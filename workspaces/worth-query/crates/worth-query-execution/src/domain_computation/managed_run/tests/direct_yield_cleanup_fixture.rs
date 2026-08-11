pub(in crate::domain_computation::managed_run) fn complete_direct_yield_cleanup(
    yielded: crate::domain_computation::WorthQueryYieldedDirectRun,
) -> crate::domain_computation::WorthQueryDirectYieldCleanupInspection {
    match yielded.cleanup() {
        crate::domain_computation::WorthQueryDirectYieldCleanupOutcome::Complete(receipt) => {
            receipt.inspection().clone()
        }
        crate::domain_computation::WorthQueryDirectYieldCleanupOutcome::RecoveryRequired(_) => {
            panic!("direct provider checkpoint unexpectedly required cleanup recovery")
        }
    }
}
