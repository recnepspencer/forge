use std::collections::BTreeSet;

use crate::{
    PhysicalScenarioActor, PhysicalScenarioActorRole, PhysicalScenarioExpectationKind,
    PhysicalSimulationScenarioDefinition, PhysicalSimulationScenarioFamily,
};

use super::capabilities::{PhysicalSimulationCapability, PhysicalSimulationCapabilitySet};
use super::counter_contracts::{
    CounterContractKind, PhysicalCounterContract, RequiredCounterContractSet,
};
use super::stable_read_plan_requirements::s5_stable_read_plan_shape;
use super::SimulationPlanDenial;

mod replay_requirements;
mod s5_interleaving_families;
mod s6_io_pressure;
mod shortcut_rejection;
use replay_requirements::{s4_recovery_shape, s5_checkpoint_publication_crash_replay_shape};
use s5_interleaving_families::{
    s5_checkpoint_publication_interlock_shape, s5_compaction_interlock_shape,
    s5_future_chunk_stability_shape, s5_reclaim_reachability_shape,
    s5_restart_during_cutover_shape, s5_tier_movement_stability_shape,
};
use s6_io_pressure::s6_io_pressure_shape;
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
    S5ReadinessShape,
    S5PhysicalIsolationInterleaving,
    S6IoPressureSimulation,
    S4RecoveryDogfood,
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
                PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe,
                PhysicalScenarioExpectationKind::S5ReadinessShapeProbe,
            ) => s5_readiness_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe,
                PhysicalScenarioExpectationKind::S5ReadinessWithShortcutRejectionProbe,
            ) => s5_readiness_with_shortcut_rejection_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe,
                PhysicalScenarioExpectationKind::S5CheckpointPublicationCrashReplay,
            ) => s5_checkpoint_publication_crash_replay_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::S5StableReadPlanAdmission,
                PhysicalScenarioExpectationKind::StableReadPlanCounterContracts,
            )
            | (
                PhysicalSimulationScenarioFamily::S5StableReadPlanAdmission,
                PhysicalScenarioExpectationKind::StableReadPlanTranscriptReplay,
            )
            | (
                PhysicalSimulationScenarioFamily::S5StableReadPlanAdmission,
                PhysicalScenarioExpectationKind::StableReadPlanDenial,
            ) => s5_stable_read_plan_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::S5CompactionInterlock,
                PhysicalScenarioExpectationKind::S5PhysicalIsolationDenial,
            ) => s5_compaction_interlock_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::S5CheckpointPublicationInterlock,
                PhysicalScenarioExpectationKind::S5PhysicalIsolationInterleaving,
            ) => s5_checkpoint_publication_interlock_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::S5ReclaimReachability,
                PhysicalScenarioExpectationKind::S5PhysicalIsolationInterleaving,
            )
            | (
                PhysicalSimulationScenarioFamily::S5ReclaimReachability,
                PhysicalScenarioExpectationKind::S5PhysicalIsolationDenial,
            ) => s5_reclaim_reachability_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::S5TierMovementStability,
                PhysicalScenarioExpectationKind::S5PhysicalIsolationInterleaving,
            ) => s5_tier_movement_stability_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::S5FutureChunkStability,
                PhysicalScenarioExpectationKind::S5PhysicalIsolationInterleaving,
            ) => s5_future_chunk_stability_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::S5RestartDuringCutover,
                PhysicalScenarioExpectationKind::S5PhysicalIsolationInterleaving,
            ) => s5_restart_during_cutover_shape(actor_step_count),
            (
                PhysicalSimulationScenarioFamily::S6IoPressureHarness,
                PhysicalScenarioExpectationKind::S6IoPressureSimulation,
            ) => s6_io_pressure_shape(actor_step_count),
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
        if definition.family() == PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe
            && definition.expectation().kind()
                == PhysicalScenarioExpectationKind::S5ReadinessShapeProbe
        {
            shape.drivers = s5_readiness_drivers_for_yieldpoint(
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

fn s5_readiness_shape(actor_step_count: u64) -> RequiredSimulationPlanShape {
    RequiredSimulationPlanShape {
        capabilities: baseline_capabilities(),
        actors: RequiredActorSet::from_actors([PhysicalScenarioActor::recovery_driver("recovery")]),
        drivers: RequiredPhysicalDriverSet::from_drivers([
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
        ]),
        observers: RequiredObserverSet::from_observers([ObserverKind::IndependentPhysicalTrace]),
        oracle_families: RequiredOracleFamilySet::from_oracles([
            OracleFamilyKind::TranscriptReplayEvidence,
            OracleFamilyKind::S5ReadinessShape,
        ]),
        counter_contracts: RequiredCounterContractSet::from_contracts([
            PhysicalCounterContract::exact(CounterContractKind::ActorStepExact, actor_step_count),
            PhysicalCounterContract::exact(CounterContractKind::ReplayIdentityExact, 1),
            PhysicalCounterContract::exact(CounterContractKind::PublicationSwaps, 1),
            PhysicalCounterContract::profile_scoped(CounterContractKind::ProfileResourceEnvelope),
            bounded_contract(CounterContractKind::AllocationBytes, 64 * 1024),
            bounded_contract(CounterContractKind::PagePins, 8),
            bounded_contract(CounterContractKind::IoQueueDepth, 4),
            monotonic_contract(CounterContractKind::LatchWaits),
            monotonic_contract(CounterContractKind::EpochRetries),
            positive_contract(CounterContractKind::ProtectedReferences),
            positive_contract(CounterContractKind::CompactionCandidateRanges),
            positive_contract(CounterContractKind::CopiedPages),
            monotonic_contract(CounterContractKind::Retries),
        ]),
        fixture_classes: RequiredFixtureClassSet::from_fixture_classes([
            FixtureClassKind::AspectNativeBoundaryFact,
        ]),
    }
}

fn s5_readiness_with_shortcut_rejection_shape(
    actor_step_count: u64,
) -> RequiredSimulationPlanShape {
    RequiredSimulationPlanShape {
        capabilities: baseline_capabilities(),
        actors: RequiredActorSet::from_actors([]),
        drivers: RequiredPhysicalDriverSet::from_drivers([
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
            PhysicalDriverKind::ShortcutRejectionBoundary,
        ]),
        observers: RequiredObserverSet::from_observers([
            ObserverKind::IndependentPhysicalTrace,
            ObserverKind::ShortcutRejectionObserver,
        ]),
        oracle_families: RequiredOracleFamilySet::from_oracles([
            OracleFamilyKind::TranscriptReplayEvidence,
            OracleFamilyKind::S5ReadinessShape,
            OracleFamilyKind::ForbiddenShortcutRejection,
        ]),
        counter_contracts: RequiredCounterContractSet::from_contracts([
            PhysicalCounterContract::exact(CounterContractKind::ActorStepExact, actor_step_count),
            PhysicalCounterContract::exact(CounterContractKind::ReplayIdentityExact, 1),
            PhysicalCounterContract::exact(CounterContractKind::ForbiddenShortcutExact, 0),
            PhysicalCounterContract::exact(CounterContractKind::PublicationSwaps, 1),
            PhysicalCounterContract::profile_scoped(CounterContractKind::ProfileResourceEnvelope),
            bounded_contract(CounterContractKind::AllocationBytes, 64 * 1024),
            bounded_contract(CounterContractKind::PagePins, 8),
            bounded_contract(CounterContractKind::IoQueueDepth, 4),
            monotonic_contract(CounterContractKind::LatchWaits),
            monotonic_contract(CounterContractKind::EpochRetries),
            positive_contract(CounterContractKind::ProtectedReferences),
            positive_contract(CounterContractKind::BlockedReclaimAttempts),
            positive_contract(CounterContractKind::CompactionCandidateRanges),
            positive_contract(CounterContractKind::CopiedPages),
        ]),
        fixture_classes: RequiredFixtureClassSet::from_fixture_classes([
            FixtureClassKind::AspectNativeBoundaryFact,
        ]),
    }
}

fn s5_readiness_drivers_for_yieldpoint(yieldpoint: &str) -> RequiredPhysicalDriverSet {
    match yieldpoint {
        "io-pressure-boundary" => {
            RequiredPhysicalDriverSet::from_drivers([PhysicalDriverKind::IoPressureBoundary])
        }
        _ => RequiredPhysicalDriverSet::from_drivers([
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
        ]),
    }
}

fn baseline_capabilities() -> PhysicalSimulationCapabilitySet {
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

fn bounded_contract(kind: CounterContractKind, maximum: u64) -> PhysicalCounterContract {
    PhysicalCounterContract::bounded(kind, maximum)
        .expect("static bounded counter contract is valid")
}

fn positive_contract(kind: CounterContractKind) -> PhysicalCounterContract {
    PhysicalCounterContract::positive(kind).expect("static positive counter contract is valid")
}

fn monotonic_contract(kind: CounterContractKind) -> PhysicalCounterContract {
    PhysicalCounterContract::monotonic(kind).expect("static monotonic counter contract is valid")
}

fn sorted_unique<T: Ord>(values: impl IntoIterator<Item = T>) -> Vec<T> {
    values
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
