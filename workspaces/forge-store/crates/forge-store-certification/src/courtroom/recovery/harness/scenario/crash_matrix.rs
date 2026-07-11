use forge_store_test_support::{
    deterministic_s4_fresh_runtime_driver, FaultSchedulerDriver, FreshRuntimeRecoveryDriver,
    StorageBoundaryInterposerDriver,
};

use super::{
    RecoveryPhysicsCounterExpectation, RecoveryPhysicsCounterKind, RecoveryPhysicsCrashLane,
    RecoveryPhysicsObserverKind, RecoveryPhysicsOracleKind, RecoveryPhysicsScenarioDefinition,
    RecoveryPhysicsScenarioDefinitionDenial, RecoveryPhysicsScenarioDrivers,
    RecoveryPhysicsScenarioPlan, RecoveryPhysicsScenarioPlanDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsCrashMatrix {
    plans: Vec<RecoveryPhysicsScenarioPlan>,
}

impl RecoveryPhysicsCrashMatrix {
    pub fn roadmap_2_s4() -> RecoveryPhysicsCrashMatrixBuilder {
        RecoveryPhysicsCrashMatrixBuilder::default()
    }

    pub fn plans(&self) -> &[RecoveryPhysicsScenarioPlan] {
        &self.plans
    }

    pub fn plan_for_lane(
        &self,
        lane: RecoveryPhysicsCrashLane,
    ) -> Option<&RecoveryPhysicsScenarioPlan> {
        self.plans.iter().find(|plan| plan.lane() == lane)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsCrashMatrixBuilder {
    seed: u64,
    backend_profile: &'static str,
    fault_driver: FaultSchedulerDriver,
    storage_driver: StorageBoundaryInterposerDriver,
    runtime_driver: FreshRuntimeRecoveryDriver,
}

impl Default for RecoveryPhysicsCrashMatrixBuilder {
    fn default() -> Self {
        let backend_profile = "roadmap2-certification-boundary";
        Self {
            seed: 0x5346_000A,
            backend_profile,
            fault_driver: FaultSchedulerDriver::deterministic(0x5346_000A),
            storage_driver: StorageBoundaryInterposerDriver::production_like(backend_profile),
            runtime_driver: deterministic_s4_fresh_runtime_driver(),
        }
    }
}

impl RecoveryPhysicsCrashMatrixBuilder {
    pub const fn seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self.fault_driver = FaultSchedulerDriver::deterministic(seed);
        self
    }

    pub const fn backend_profile(mut self, backend_profile: &'static str) -> Self {
        self.backend_profile = backend_profile;
        self.storage_driver = StorageBoundaryInterposerDriver::production_like(backend_profile);
        self
    }

    pub fn recovery_driver(mut self, driver: FreshRuntimeRecoveryDriver) -> Self {
        self.runtime_driver = driver;
        self
    }

    pub fn lower(self) -> Result<RecoveryPhysicsCrashMatrix, RecoveryPhysicsCrashMatrixDenial> {
        let mut plans = Vec::with_capacity(RecoveryPhysicsCrashLane::REQUIRED_S4_LANES.len());
        for lane in RecoveryPhysicsCrashLane::REQUIRED_S4_LANES {
            let definition = self.definition_for_lane(lane)?;
            plans.push(RecoveryPhysicsScenarioPlan::lower(definition)?);
        }
        Ok(RecoveryPhysicsCrashMatrix { plans })
    }

    fn definition_for_lane(
        &self,
        lane: RecoveryPhysicsCrashLane,
    ) -> Result<RecoveryPhysicsScenarioDefinition, RecoveryPhysicsScenarioDefinitionDenial> {
        let scheduled_fault = self.fault_driver.schedule_fault(lane.crash_seam());
        let boundary_event = self
            .storage_driver
            .lower_boundary_event(scheduled_fault.seam(), scheduled_fault.ordinal());
        let drivers = RecoveryPhysicsScenarioDrivers::new(
            self.fault_driver,
            self.storage_driver.clone(),
            self.runtime_driver.clone(),
        );
        let mut builder = RecoveryPhysicsScenarioDefinition::builder(lane)
            .seed(self.seed)
            .backend_profile(self.backend_profile)
            .boundary_event(boundary_event)
            .drivers(drivers);
        for observer in RecoveryPhysicsObserverKind::REQUIRED {
            builder = builder.observer(observer);
        }
        for oracle in RecoveryPhysicsOracleKind::REQUIRED_SCENARIO_ORACLES {
            builder = builder.oracle(oracle);
        }
        for counter in RecoveryPhysicsCounterKind::REQUIRED_SCENARIO_COUNTERS {
            builder =
                builder.counter_expectation(RecoveryPhysicsCounterExpectation::exact(counter, 1));
        }
        builder.define()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryPhysicsCrashMatrixDenial {
    Definition(RecoveryPhysicsScenarioDefinitionDenial),
    Plan(RecoveryPhysicsScenarioPlanDenial),
}

impl From<RecoveryPhysicsScenarioDefinitionDenial> for RecoveryPhysicsCrashMatrixDenial {
    fn from(denial: RecoveryPhysicsScenarioDefinitionDenial) -> Self {
        Self::Definition(denial)
    }
}

impl From<RecoveryPhysicsScenarioPlanDenial> for RecoveryPhysicsCrashMatrixDenial {
    fn from(denial: RecoveryPhysicsScenarioPlanDenial) -> Self {
        Self::Plan(denial)
    }
}
