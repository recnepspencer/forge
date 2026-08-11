use super::RecoveryFreshReopenOccurrence;

impl RecoveryFreshReopenOccurrence {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        session: [u8; 16],
        plan: [u8; 32],
        generation: u64,
        selector: worth_store_physical_backend::CompletedScheduledRecoveryReopenRead,
        root: worth_store_physical_backend::CompletedScheduledRecoveryReopenRead,
        selector_work: crate::physical_runtime::PhysicalWorkIdentity,
        root_work: crate::physical_runtime::PhysicalWorkIdentity,
        selector_signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
        root_signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
    ) -> Self {
        Self {
            session,
            plan,
            generation,
            selector,
            root,
            selector_work,
            root_work,
            selector_signal,
            root_signal,
        }
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }
    pub const fn session(&self) -> [u8; 16] {
        self.session
    }
    pub const fn plan(&self) -> [u8; 32] {
        self.plan
    }
    pub const fn selector(
        &self,
    ) -> &worth_store_physical_backend::CompletedScheduledRecoveryReopenRead {
        &self.selector
    }
    pub const fn root(
        &self,
    ) -> &worth_store_physical_backend::CompletedScheduledRecoveryReopenRead {
        &self.root
    }
}
