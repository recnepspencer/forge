use forge_store_test_support::{
    FaultSchedulerDriver, FreshRuntimeRecoveryDriver, StorageBoundaryInterposerDriver,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsScenarioDrivers {
    fault_scheduler: FaultSchedulerDriver,
    storage_interposer: StorageBoundaryInterposerDriver,
    fresh_runtime: FreshRuntimeRecoveryDriver,
}

impl RecoveryPhysicsScenarioDrivers {
    pub const fn new(
        fault_scheduler: FaultSchedulerDriver,
        storage_interposer: StorageBoundaryInterposerDriver,
        fresh_runtime: FreshRuntimeRecoveryDriver,
    ) -> Self {
        Self {
            fault_scheduler,
            storage_interposer,
            fresh_runtime,
        }
    }

    pub const fn fault_scheduler(&self) -> FaultSchedulerDriver {
        self.fault_scheduler
    }

    pub const fn storage_interposer(&self) -> &StorageBoundaryInterposerDriver {
        &self.storage_interposer
    }

    pub const fn fresh_runtime(&self) -> &FreshRuntimeRecoveryDriver {
        &self.fresh_runtime
    }
}
