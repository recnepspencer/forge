use crate::PhysicalScenarioActor;

use super::capabilities::{PhysicalSimulationCapability, PhysicalSimulationCapabilitySet};
use super::counter_contracts::{
    CounterContractKind, PhysicalCounterContract, RequiredCounterContractSet,
};
use super::requirements::{
    FixtureClassKind, ObserverKind, OracleFamilyKind, PhysicalDriverKind, RequiredActorSet,
    RequiredFixtureClassSet, RequiredObserverSet, RequiredOracleFamilySet,
    RequiredPhysicalDriverSet, RequiredSimulationPlanShape,
};

pub(crate) fn physical_isolation_stable_read_plan_shape(
    actor_step_count: u64,
) -> RequiredSimulationPlanShape {
    RequiredSimulationPlanShape {
        capabilities: baseline_capabilities(),
        actors: RequiredActorSet::from_actors([
            PhysicalScenarioActor::foreground_reader("reader"),
            PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
        ]),
        drivers: RequiredPhysicalDriverSet::from_drivers([
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
            PhysicalDriverKind::MemoryPressureBoundary,
        ]),
        observers: RequiredObserverSet::from_observers([ObserverKind::IndependentPhysicalTrace]),
        oracle_families: RequiredOracleFamilySet::from_oracles([
            OracleFamilyKind::TranscriptReplayEvidence,
            OracleFamilyKind::PhysicalIsolationReadinessShape,
        ]),
        counter_contracts: RequiredCounterContractSet::from_contracts([
            PhysicalCounterContract::exact(CounterContractKind::ActorStepExact, actor_step_count),
            PhysicalCounterContract::exact(CounterContractKind::ReplayIdentityExact, 1),
            PhysicalCounterContract::profile_scoped(CounterContractKind::ProfileResourceEnvelope),
            bounded_contract(CounterContractKind::AllocationBytes, 64 * 1024),
            bounded_contract(CounterContractKind::ResidentBytes, 64 * 1024),
            bounded_contract(CounterContractKind::PagePins, 8),
            monotonic_contract(CounterContractKind::LatchWaits),
            monotonic_contract(CounterContractKind::EpochRetries),
            positive_contract(CounterContractKind::ProtectedReferences),
            monotonic_contract(CounterContractKind::Retries),
            positive_contract(CounterContractKind::BlockedReclaimAttempts),
        ]),
        fixture_classes: RequiredFixtureClassSet::from_fixture_classes([
            FixtureClassKind::AspectNativeBoundaryFact,
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
