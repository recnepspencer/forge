use crate::PhysicalScenarioActor;

use super::super::capabilities::{PhysicalSimulationCapability, PhysicalSimulationCapabilitySet};
use super::super::counter_contracts::{
    CounterContractKind, PhysicalCounterContract, RequiredCounterContractSet,
};
use super::{
    FixtureClassKind, ObserverKind, OracleFamilyKind, PhysicalDriverKind, RequiredActorSet,
    RequiredFixtureClassSet, RequiredObserverSet, RequiredOracleFamilySet,
    RequiredPhysicalDriverSet, RequiredSimulationPlanShape,
};

pub(crate) fn s5_compaction_interlock_shape(actor_step_count: u64) -> RequiredSimulationPlanShape {
    RequiredSimulationPlanShape {
        capabilities: baseline_capabilities(),
        actors: RequiredActorSet::from_actors([
            PhysicalScenarioActor::foreground_reader("reader"),
            PhysicalScenarioActor::compaction_driver("compactor"),
            PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
        ]),
        drivers: RequiredPhysicalDriverSet::from_drivers([
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
            PhysicalDriverKind::MemoryPressureBoundary,
        ]),
        observers: RequiredObserverSet::from_observers([ObserverKind::IndependentPhysicalTrace]),
        oracle_families: s5_oracle_families(),
        counter_contracts: compaction_counter_contracts(actor_step_count),
        fixture_classes: s5_fixture_classes(),
    }
}

pub(crate) fn s5_checkpoint_publication_interlock_shape(
    actor_step_count: u64,
) -> RequiredSimulationPlanShape {
    RequiredSimulationPlanShape {
        capabilities: baseline_capabilities(),
        actors: RequiredActorSet::from_actors([
            PhysicalScenarioActor::foreground_reader("reader"),
            PhysicalScenarioActor::checkpoint_driver("checkpoint"),
            PhysicalScenarioActor::recovery_driver("recovery"),
        ]),
        drivers: RequiredPhysicalDriverSet::from_drivers([
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
            PhysicalDriverKind::FreshRuntimeRecovery,
        ]),
        observers: RequiredObserverSet::from_observers([ObserverKind::IndependentPhysicalTrace]),
        oracle_families: s5_oracle_families(),
        counter_contracts: checkpoint_counter_contracts(actor_step_count),
        fixture_classes: s5_fixture_classes(),
    }
}

pub(crate) fn s5_reclaim_reachability_shape(actor_step_count: u64) -> RequiredSimulationPlanShape {
    RequiredSimulationPlanShape {
        capabilities: baseline_capabilities(),
        actors: RequiredActorSet::from_actors([
            PhysicalScenarioActor::foreground_reader("reader"),
            PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
        ]),
        drivers: RequiredPhysicalDriverSet::from_drivers([
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
        ]),
        observers: RequiredObserverSet::from_observers([ObserverKind::IndependentPhysicalTrace]),
        oracle_families: s5_oracle_families(),
        counter_contracts: reclaim_counter_contracts(actor_step_count),
        fixture_classes: s5_fixture_classes(),
    }
}

pub(crate) fn s5_tier_movement_stability_shape(
    actor_step_count: u64,
) -> RequiredSimulationPlanShape {
    RequiredSimulationPlanShape {
        capabilities: baseline_capabilities(),
        actors: RequiredActorSet::from_actors([
            PhysicalScenarioActor::foreground_reader("reader"),
            PhysicalScenarioActor::maintenance_reclaimer("tier-movement"),
        ]),
        drivers: RequiredPhysicalDriverSet::from_drivers([
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
        ]),
        observers: RequiredObserverSet::from_observers([ObserverKind::IndependentPhysicalTrace]),
        oracle_families: s5_oracle_families(),
        counter_contracts: stability_counter_contracts(actor_step_count),
        fixture_classes: s5_fixture_classes(),
    }
}

pub(crate) fn s5_future_chunk_stability_shape(
    actor_step_count: u64,
) -> RequiredSimulationPlanShape {
    RequiredSimulationPlanShape {
        capabilities: baseline_capabilities(),
        actors: RequiredActorSet::from_actors([
            PhysicalScenarioActor::foreground_reader("reader"),
            PhysicalScenarioActor::future_extension_slot("future-chunk"),
        ]),
        drivers: RequiredPhysicalDriverSet::from_drivers([
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
        ]),
        observers: RequiredObserverSet::from_observers([ObserverKind::IndependentPhysicalTrace]),
        oracle_families: s5_oracle_families(),
        counter_contracts: stability_counter_contracts(actor_step_count),
        fixture_classes: s5_fixture_classes(),
    }
}

pub(crate) fn s5_restart_during_cutover_shape(
    actor_step_count: u64,
) -> RequiredSimulationPlanShape {
    RequiredSimulationPlanShape {
        capabilities: baseline_capabilities(),
        actors: RequiredActorSet::from_actors([
            PhysicalScenarioActor::foreground_reader("reader"),
            PhysicalScenarioActor::foreground_writer("writer"),
            PhysicalScenarioActor::recovery_driver("recovery"),
        ]),
        drivers: RequiredPhysicalDriverSet::from_drivers([
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
            PhysicalDriverKind::FreshRuntimeRecovery,
        ]),
        observers: RequiredObserverSet::from_observers([ObserverKind::IndependentPhysicalTrace]),
        oracle_families: s5_oracle_families(),
        counter_contracts: restart_counter_contracts(actor_step_count),
        fixture_classes: s5_fixture_classes(),
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

fn s5_oracle_families() -> RequiredOracleFamilySet {
    RequiredOracleFamilySet::from_oracles([
        OracleFamilyKind::TranscriptReplayEvidence,
        OracleFamilyKind::S5PhysicalIsolationInterleaving,
    ])
}

fn compaction_counter_contracts(actor_step_count: u64) -> RequiredCounterContractSet {
    let mut contracts = base_s5_counter_contracts(actor_step_count);
    contracts.extend([
        positive_contract(CounterContractKind::BlockedReclaimAttempts),
        positive_contract(CounterContractKind::CompactionCandidateRanges),
        positive_contract(CounterContractKind::CopiedPages),
    ]);
    RequiredCounterContractSet::from_contracts(contracts)
}

fn checkpoint_counter_contracts(actor_step_count: u64) -> RequiredCounterContractSet {
    let mut contracts = base_s5_counter_contracts(actor_step_count);
    contracts.push(PhysicalCounterContract::exact(
        CounterContractKind::PublicationSwaps,
        1,
    ));
    RequiredCounterContractSet::from_contracts(contracts)
}

fn reclaim_counter_contracts(actor_step_count: u64) -> RequiredCounterContractSet {
    let mut contracts = base_s5_counter_contracts(actor_step_count);
    contracts.extend([
        positive_contract(CounterContractKind::BlockedReclaimAttempts),
        positive_contract(CounterContractKind::CompactionCandidateRanges),
    ]);
    RequiredCounterContractSet::from_contracts(contracts)
}

fn stability_counter_contracts(actor_step_count: u64) -> RequiredCounterContractSet {
    let mut contracts = base_s5_counter_contracts(actor_step_count);
    contracts.push(monotonic_contract(
        CounterContractKind::FutureS5SpecificCounters,
    ));
    RequiredCounterContractSet::from_contracts(contracts)
}

fn restart_counter_contracts(actor_step_count: u64) -> RequiredCounterContractSet {
    let mut contracts = checkpoint_counter_contracts(actor_step_count)
        .iter()
        .copied()
        .collect::<Vec<_>>();
    contracts.push(monotonic_contract(CounterContractKind::Retries));
    RequiredCounterContractSet::from_contracts(contracts)
}

fn base_s5_counter_contracts(actor_step_count: u64) -> Vec<PhysicalCounterContract> {
    vec![
        PhysicalCounterContract::exact(CounterContractKind::ActorStepExact, actor_step_count),
        PhysicalCounterContract::exact(CounterContractKind::ReplayIdentityExact, 1),
        PhysicalCounterContract::profile_scoped(CounterContractKind::ProfileResourceEnvelope),
        bounded_contract(CounterContractKind::AllocationBytes, 64 * 1024),
        bounded_contract(CounterContractKind::ResidentBytes, 64 * 1024),
        bounded_contract(CounterContractKind::PagePins, 8),
        monotonic_contract(CounterContractKind::LatchWaits),
        monotonic_contract(CounterContractKind::EpochRetries),
        positive_contract(CounterContractKind::ProtectedReferences),
    ]
}

fn s5_fixture_classes() -> RequiredFixtureClassSet {
    RequiredFixtureClassSet::from_fixture_classes([FixtureClassKind::AspectNativeBoundaryFact])
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
