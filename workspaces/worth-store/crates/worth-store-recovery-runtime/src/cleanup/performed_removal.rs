/// Runtime-held view of one Store-minted performed cleanup removal.
///
/// The Store occurrence remains the authority. This wrapper is intentionally
/// construction-private and is populated only by Phase 7 execution.
pub struct PerformedRecoveryCleanupRemoval {
    pub(crate) performed: worth_store::physical_runtime::PerformedRecoveryPhysicalEffect<
        worth_store::physical_runtime::RecoveryCleanupRemovalAction,
    >,
}

impl PerformedRecoveryCleanupRemoval {
    pub(crate) const fn new(
        performed: worth_store::physical_runtime::PerformedRecoveryPhysicalEffect<
            worth_store::physical_runtime::RecoveryCleanupRemovalAction,
        >,
    ) -> Self {
        Self { performed }
    }

    pub fn occurrence(&self) -> &worth_store::physical_runtime::RecoveryCleanupRemovalOccurrence {
        match self.performed.occurrence() {
            worth_store::physical_runtime::RecoveryPhysicalEffectOccurrence::CleanupRemoval(
                occurrence,
            ) => occurrence,
            _ => unreachable!("cleanup action carries cleanup occurrence"),
        }
    }
}
