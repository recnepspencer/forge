use super::{
    RecoveryPhysicsCounterExpectation, RecoveryPhysicsCrashLane, RecoveryPhysicsObserverKind,
    RecoveryPhysicsOracleKind, RecoveryPhysicsScenarioDrivers,
};
use forge_store_test_support::StorageBoundaryEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsScenarioDefinition {
    lane: RecoveryPhysicsCrashLane,
    seed: u64,
    backend_profile: &'static str,
    boundary_event: StorageBoundaryEvent,
    drivers: RecoveryPhysicsScenarioDrivers,
    observers: Vec<RecoveryPhysicsObserverKind>,
    oracles: Vec<RecoveryPhysicsOracleKind>,
    counter_expectations: Vec<RecoveryPhysicsCounterExpectation>,
}

impl RecoveryPhysicsScenarioDefinition {
    pub fn builder(lane: RecoveryPhysicsCrashLane) -> RecoveryPhysicsScenarioDefinitionBuilder {
        RecoveryPhysicsScenarioDefinitionBuilder::new(lane)
    }

    pub const fn lane(&self) -> RecoveryPhysicsCrashLane {
        self.lane
    }

    pub const fn seed(&self) -> u64 {
        self.seed
    }

    pub const fn backend_profile(&self) -> &'static str {
        self.backend_profile
    }

    pub const fn drivers(&self) -> &RecoveryPhysicsScenarioDrivers {
        &self.drivers
    }

    pub const fn boundary_event(&self) -> &StorageBoundaryEvent {
        &self.boundary_event
    }

    pub fn observers(&self) -> &[RecoveryPhysicsObserverKind] {
        &self.observers
    }

    pub fn oracles(&self) -> &[RecoveryPhysicsOracleKind] {
        &self.oracles
    }

    pub fn counter_expectations(&self) -> &[RecoveryPhysicsCounterExpectation] {
        &self.counter_expectations
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryPhysicsScenarioDefinitionDenial {
    MissingDrivers,
    MissingBoundaryEvent,
    MissingBackendProfile,
    MissingObserver(RecoveryPhysicsObserverKind),
    MissingOracle(RecoveryPhysicsOracleKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsScenarioDefinitionBuilder {
    lane: RecoveryPhysicsCrashLane,
    seed: u64,
    backend_profile: Option<&'static str>,
    boundary_event: Option<StorageBoundaryEvent>,
    drivers: Option<RecoveryPhysicsScenarioDrivers>,
    observers: Vec<RecoveryPhysicsObserverKind>,
    oracles: Vec<RecoveryPhysicsOracleKind>,
    counter_expectations: Vec<RecoveryPhysicsCounterExpectation>,
}

impl RecoveryPhysicsScenarioDefinitionBuilder {
    pub fn new(lane: RecoveryPhysicsCrashLane) -> Self {
        Self {
            lane,
            seed: 0,
            backend_profile: None,
            boundary_event: None,
            drivers: None,
            observers: Vec::new(),
            oracles: Vec::new(),
            counter_expectations: Vec::new(),
        }
    }

    pub const fn seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub const fn backend_profile(mut self, backend_profile: &'static str) -> Self {
        self.backend_profile = Some(backend_profile);
        self
    }

    pub fn drivers(mut self, drivers: RecoveryPhysicsScenarioDrivers) -> Self {
        self.drivers = Some(drivers);
        self
    }

    pub fn boundary_event(mut self, boundary_event: StorageBoundaryEvent) -> Self {
        self.boundary_event = Some(boundary_event);
        self
    }

    pub fn observer(mut self, observer: RecoveryPhysicsObserverKind) -> Self {
        if !self.observers.contains(&observer) {
            self.observers.push(observer);
        }
        self
    }

    pub fn oracle(mut self, oracle: RecoveryPhysicsOracleKind) -> Self {
        if !self.oracles.contains(&oracle) {
            self.oracles.push(oracle);
        }
        self
    }

    pub fn counter_expectation(mut self, expectation: RecoveryPhysicsCounterExpectation) -> Self {
        self.counter_expectations.push(expectation);
        self
    }

    pub fn define(
        self,
    ) -> Result<RecoveryPhysicsScenarioDefinition, RecoveryPhysicsScenarioDefinitionDenial> {
        for observer in RecoveryPhysicsObserverKind::REQUIRED {
            if !self.observers.contains(&observer) {
                return Err(RecoveryPhysicsScenarioDefinitionDenial::MissingObserver(
                    observer,
                ));
            }
        }
        for oracle in RecoveryPhysicsOracleKind::REQUIRED_SCENARIO_ORACLES {
            if !self.oracles.contains(&oracle) {
                return Err(RecoveryPhysicsScenarioDefinitionDenial::MissingOracle(
                    oracle,
                ));
            }
        }

        Ok(RecoveryPhysicsScenarioDefinition {
            lane: self.lane,
            seed: self.seed,
            backend_profile: self
                .backend_profile
                .ok_or(RecoveryPhysicsScenarioDefinitionDenial::MissingBackendProfile)?,
            boundary_event: self
                .boundary_event
                .ok_or(RecoveryPhysicsScenarioDefinitionDenial::MissingBoundaryEvent)?,
            drivers: self
                .drivers
                .ok_or(RecoveryPhysicsScenarioDefinitionDenial::MissingDrivers)?,
            observers: self.observers,
            oracles: self.oracles,
            counter_expectations: self.counter_expectations,
        })
    }
}
