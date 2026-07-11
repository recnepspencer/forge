use std::collections::BTreeSet;

use crate::{
    PhysicalScenarioActor, PhysicalScenarioActorRole, PhysicalScenarioExpectationKind,
    PhysicalSimulationScenarioDefinition, PhysicalSimulationScenarioFamily,
};

use super::capabilities::{PhysicalSimulationCapability, PhysicalSimulationCapabilitySet};
use super::counter_contracts::{
    CounterContractKind, PhysicalCounterContract, RequiredCounterContractSet,
};
use super::stable_read_plan_requirements::physical_isolation_stable_read_plan_shape;
use super::SimulationPlanDenial;

mod blob_harness;
mod io_pressure;
mod physical_isolation;
mod replay_requirements;
mod shortcut_rejection;
pub(crate) use blob_harness::blob_harness_shape;
use io_pressure::io_pressure_shape;
use physical_isolation::{
    physical_isolation_checkpoint_publication_interlock_shape,
    physical_isolation_compaction_interlock_shape, physical_isolation_future_chunk_stability_shape,
    physical_isolation_readiness_drivers_for_yieldpoint, physical_isolation_readiness_shape,
    physical_isolation_readiness_with_shortcut_rejection_shape,
    physical_isolation_reclaim_reachability_shape, physical_isolation_restart_during_cutover_shape,
    physical_isolation_tier_movement_stability_shape,
};
use replay_requirements::{
    physical_isolation_checkpoint_publication_crash_replay_shape, s4_recovery_shape,
};
use shortcut_rejection::shortcut_rejection_shape;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalDriverKind {
    ProductionBoundaryYieldpoint,
    FreshRuntimeRecovery,
    MemoryPressureBoundary,
    IoPressureBoundary,
    OfflineVerifierBoundary,
    ShortcutRejectionBoundary,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObserverKind {
    IndependentPhysicalTrace,
    RecoveryOutcomeObserver,
    ShortcutRejectionObserver,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OracleFamilyKind {
    TranscriptReplayEvidence,
    PhysicalIsolationReadinessShape,
    PhysicalIsolationInterleaving,
    IoPressureSimulation,
    S4RecoveryDogfood,
    BlobHarnessEvidence,
    BlobHeavyQualification,
    ForbiddenShortcutRejection,
    FutureExtensionNonClaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FixtureClassKind {
    AspectNativeBoundaryFact,
    S4RecoveryArtifacts,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredPhysicalDriverSet {
    drivers: Vec<PhysicalDriverKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredObserverSet {
    observers: Vec<ObserverKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredOracleFamilySet {
    oracle_families: Vec<OracleFamilyKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredFixtureClassSet {
    fixture_classes: Vec<FixtureClassKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredActorSet {
    actors: Vec<PhysicalScenarioActor>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RequiredSimulationPlanShape {
    pub(crate) capabilities: PhysicalSimulationCapabilitySet,
    pub(crate) actors: RequiredActorSet,
    pub(crate) drivers: RequiredPhysicalDriverSet,
    pub(crate) observers: RequiredObserverSet,
    pub(crate) oracle_families: RequiredOracleFamilySet,
    pub(crate) counter_contracts: RequiredCounterContractSet,
    pub(crate) fixture_classes: RequiredFixtureClassSet,
}

impl RequiredSimulationPlanShape {
    pub(crate) fn from_scenario(
        definition: &PhysicalSimulationScenarioDefinition,
    ) -> Result<Self, SimulationPlanDenial> {
        let actors = RequiredActorSet::from_actors(definition.actors().iter().cloned());
        let actor_step_count = actors.len() as u64;
        let mut shape = match (definition.family(), definition.expectation().kind()) {
            (
                PhysicalSimulationScenarioFamily::S4RecoveryDogfood,
                PhysicalScenarioExpectationKind::S4RecoveryDogfood,
            ) => s4_recovery_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe,
                PhysicalScenarioExpectationKind::PhysicalIsolationReadinessShapeProbe,
            ) => physical_isolation_readiness_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe,
                PhysicalScenarioExpectationKind::PhysicalIsolationReadinessWithShortcutRejectionProbe,
            ) => physical_isolation_readiness_with_shortcut_rejection_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe,
                PhysicalScenarioExpectationKind::PhysicalIsolationCheckpointPublicationCrashReplay,
            ) => physical_isolation_checkpoint_publication_crash_replay_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::PhysicalIsolationStableReadPlanAdmission,
                PhysicalScenarioExpectationKind::StableReadPlanCounterContracts,
            )
            | (
                PhysicalSimulationScenarioFamily::PhysicalIsolationStableReadPlanAdmission,
                PhysicalScenarioExpectationKind::StableReadPlanTranscriptReplay,
            )
            | (
                PhysicalSimulationScenarioFamily::PhysicalIsolationStableReadPlanAdmission,
                PhysicalScenarioExpectationKind::StableReadPlanDenial,
            ) => physical_isolation_stable_read_plan_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock,
                PhysicalScenarioExpectationKind::PhysicalIsolationDenial,
            ) => physical_isolation_compaction_interlock_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock,
                PhysicalScenarioExpectationKind::PhysicalIsolationInterleaving,
            ) => physical_isolation_checkpoint_publication_interlock_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::PhysicalIsolationReclaimReachability,
                PhysicalScenarioExpectationKind::PhysicalIsolationInterleaving,
            )
            | (
                PhysicalSimulationScenarioFamily::PhysicalIsolationReclaimReachability,
                PhysicalScenarioExpectationKind::PhysicalIsolationDenial,
            ) => physical_isolation_reclaim_reachability_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::PhysicalIsolationTierMovementStability,
                PhysicalScenarioExpectationKind::PhysicalIsolationInterleaving,
            ) => physical_isolation_tier_movement_stability_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability,
                PhysicalScenarioExpectationKind::PhysicalIsolationInterleaving,
            ) => physical_isolation_future_chunk_stability_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::PhysicalIsolationRestartDuringCutover,
                PhysicalScenarioExpectationKind::PhysicalIsolationInterleaving,
            ) => physical_isolation_restart_during_cutover_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::IoPressureHarness,
                PhysicalScenarioExpectationKind::IoPressureSimulation,
            ) => io_pressure_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::BlobHarnessSeed,
                PhysicalScenarioExpectationKind::BlobHarnessSeed,
            ) => {
                let topology = definition
                    .expectation()
                    .blob_harness_topology()
                    .ok_or(SimulationPlanDenial::MissingBlobHarnessTopology)?;
                let metadata = definition
                    .expectation()
                    .blob_harness_metadata()
                    .ok_or(SimulationPlanDenial::MissingBlobHarnessTopology)?;
                blob_harness_shape(actor_step_count, topology, metadata)
            }
            (
                PhysicalSimulationScenarioFamily::ShortcutRejectionDogfood,
                PhysicalScenarioExpectationKind::ShortcutRejectionDogfood,
            ) => shortcut_rejection_shape(),
            (family, expectation) => {
                return Err(SimulationPlanDenial::UnsupportedScenarioShape {
                    family,
                    expectation,
                });
            }
        };
        if definition.family()
            == PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe
            && definition.expectation().kind()
                == PhysicalScenarioExpectationKind::PhysicalIsolationReadinessShapeProbe
        {
            shape.drivers = physical_isolation_readiness_drivers_for_yieldpoint(
                definition.schedule().production_boundary_yieldpoint(),
            );
        }
        shape.actors = actors;
        Ok(shape)
    }
}

impl RequiredPhysicalDriverSet {
    pub(crate) fn from_drivers(drivers: impl IntoIterator<Item = PhysicalDriverKind>) -> Self {
        Self {
            drivers: sorted_unique(drivers),
        }
    }

    pub fn contains(&self, driver: PhysicalDriverKind) -> bool {
        self.drivers.contains(&driver)
    }

    pub fn iter(&self) -> impl Iterator<Item = PhysicalDriverKind> + '_ {
        self.drivers.iter().copied()
    }
}

impl RequiredObserverSet {
    pub(crate) fn from_observers(observers: impl IntoIterator<Item = ObserverKind>) -> Self {
        Self {
            observers: sorted_unique(observers),
        }
    }

    pub fn contains(&self, observer: ObserverKind) -> bool {
        self.observers.contains(&observer)
    }

    pub fn iter(&self) -> impl Iterator<Item = ObserverKind> + '_ {
        self.observers.iter().copied()
    }
}

impl RequiredOracleFamilySet {
    pub(crate) fn from_oracles(oracles: impl IntoIterator<Item = OracleFamilyKind>) -> Self {
        Self {
            oracle_families: sorted_unique(oracles),
        }
    }

    pub fn contains(&self, oracle_family: OracleFamilyKind) -> bool {
        self.oracle_families.contains(&oracle_family)
    }

    pub fn iter(&self) -> impl Iterator<Item = OracleFamilyKind> + '_ {
        self.oracle_families.iter().copied()
    }
}

impl RequiredFixtureClassSet {
    pub(crate) fn from_fixture_classes(
        fixture_classes: impl IntoIterator<Item = FixtureClassKind>,
    ) -> Self {
        Self {
            fixture_classes: sorted_unique(fixture_classes),
        }
    }

    pub fn contains(&self, fixture_class: FixtureClassKind) -> bool {
        self.fixture_classes.contains(&fixture_class)
    }

    pub fn iter(&self) -> impl Iterator<Item = FixtureClassKind> + '_ {
        self.fixture_classes.iter().copied()
    }
}

impl RequiredActorSet {
    pub(crate) fn from_actors(actors: impl IntoIterator<Item = PhysicalScenarioActor>) -> Self {
        Self {
            actors: sorted_unique(actors),
        }
    }

    pub fn contains_role(&self, role: PhysicalScenarioActorRole) -> bool {
        self.actors.iter().any(|actor| actor.role() == role)
    }

    pub fn contains_actor_id(&self, id: &str) -> bool {
        self.actors.iter().any(|actor| actor.id() == id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &PhysicalScenarioActor> {
        self.actors.iter()
    }

    pub fn len(&self) -> usize {
        self.actors.len()
    }
}

pub(super) fn baseline_capabilities() -> PhysicalSimulationCapabilitySet {
    PhysicalSimulationCapabilitySet::from_capabilities([
        PhysicalSimulationCapability::ProductionBoundaryDriver,
        PhysicalSimulationCapability::IndependentObserver,
        PhysicalSimulationCapability::CertificationOracleFamily,
        PhysicalSimulationCapability::CounterContracts,
        PhysicalSimulationCapability::FixtureClassAdmission,
        PhysicalSimulationCapability::EvidencePolicy,
        PhysicalSimulationCapability::ForbiddenShortcutDenial,
    ])
}

pub(super) fn bounded_contract(kind: CounterContractKind, maximum: u64) -> PhysicalCounterContract {
    PhysicalCounterContract::bounded(kind, maximum)
        .expect("static bounded counter contract is valid")
}

pub(super) fn positive_contract(kind: CounterContractKind) -> PhysicalCounterContract {
    PhysicalCounterContract::positive(kind).expect("static positive counter contract is valid")
}

pub(super) fn monotonic_contract(kind: CounterContractKind) -> PhysicalCounterContract {
    PhysicalCounterContract::monotonic(kind).expect("static monotonic counter contract is valid")
}

fn sorted_unique<T: Ord>(values: impl IntoIterator<Item = T>) -> Vec<T> {
    values
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
