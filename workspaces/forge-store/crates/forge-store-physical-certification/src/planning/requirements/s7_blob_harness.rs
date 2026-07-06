use forge_store_blob_chunks::BlobHarnessChunkTopology;

use crate::{
    CounterContractKind, PhysicalCounterContract, PhysicalScenarioActor, RequiredCounterContractSet,
};

use super::{
    baseline_capabilities, bounded_contract, FixtureClassKind, ObserverKind, OracleFamilyKind,
    PhysicalDriverKind, RequiredActorSet, RequiredFixtureClassSet, RequiredObserverSet,
    RequiredOracleFamilySet, RequiredPhysicalDriverSet, RequiredSimulationPlanShape,
};

pub(crate) fn s7_blob_harness_shape(
    actor_step_count: u64,
    topology: BlobHarnessChunkTopology,
) -> RequiredSimulationPlanShape {
    RequiredSimulationPlanShape {
        capabilities: baseline_capabilities(),
        actors: RequiredActorSet::from_actors([PhysicalScenarioActor::recovery_driver(
            "blob-seed-replay",
        )]),
        drivers: RequiredPhysicalDriverSet::from_drivers([
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
            PhysicalDriverKind::MemoryPressureBoundary,
            PhysicalDriverKind::ShortcutRejectionBoundary,
        ]),
        observers: RequiredObserverSet::from_observers([
            ObserverKind::IndependentPhysicalTrace,
            ObserverKind::ShortcutRejectionObserver,
        ]),
        oracle_families: RequiredOracleFamilySet::from_oracles([
            OracleFamilyKind::TranscriptReplayEvidence,
            OracleFamilyKind::ForbiddenShortcutRejection,
        ]),
        counter_contracts: blob_counter_topology(actor_step_count, topology),
        fixture_classes: RequiredFixtureClassSet::from_fixture_classes([
            FixtureClassKind::AspectNativeBoundaryFact,
        ]),
    }
}

fn blob_counter_topology(
    actor_step_count: u64,
    topology: BlobHarnessChunkTopology,
) -> RequiredCounterContractSet {
    RequiredCounterContractSet::from_contracts([
        PhysicalCounterContract::exact(CounterContractKind::ActorStepExact, actor_step_count),
        PhysicalCounterContract::exact(CounterContractKind::ReplayIdentityExact, 1),
        PhysicalCounterContract::exact(CounterContractKind::ForbiddenShortcutExact, 0),
        PhysicalCounterContract::exact(
            CounterContractKind::BlobChunkCountExact,
            topology.chunk_count(),
        ),
        PhysicalCounterContract::exact(
            CounterContractKind::BlobLogicalBytesExact,
            topology.logical_bytes(),
        ),
        PhysicalCounterContract::profile_scoped(CounterContractKind::ProfileResourceEnvelope),
        bounded_contract(CounterContractKind::AllocationBytes, 1024 * 1024),
        bounded_contract(CounterContractKind::ResidentBytes, 512 * 1024 * 1024),
    ])
}
