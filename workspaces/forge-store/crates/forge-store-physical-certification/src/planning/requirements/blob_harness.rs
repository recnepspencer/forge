use forge_store_blob_chunks::{BlobHarnessChunkTopology, BlobHarnessFailurePoint};

use crate::{
    BlobHarnessScenarioMetadata, CounterContractKind, PhysicalCounterContract,
    PhysicalScenarioActor, RequiredCounterContractSet,
};

use super::{
    baseline_capabilities, bounded_contract, FixtureClassKind, ObserverKind, OracleFamilyKind,
    PhysicalDriverKind, RequiredActorSet, RequiredFixtureClassSet, RequiredObserverSet,
    RequiredOracleFamilySet, RequiredPhysicalDriverSet, RequiredSimulationPlanShape,
};

pub(crate) fn blob_harness_shape(
    actor_step_count: u64,
    topology: BlobHarnessChunkTopology,
    metadata: BlobHarnessScenarioMetadata,
) -> RequiredSimulationPlanShape {
    RequiredSimulationPlanShape {
        capabilities: baseline_capabilities(),
        actors: RequiredActorSet::from_actors([PhysicalScenarioActor::recovery_driver(
            "blob-seed-replay",
        )]),
        drivers: blob_required_drivers(metadata.failure_point()),
        observers: RequiredObserverSet::from_observers([
            ObserverKind::IndependentPhysicalTrace,
            ObserverKind::ShortcutRejectionObserver,
        ]),
        oracle_families: RequiredOracleFamilySet::from_oracles([
            OracleFamilyKind::TranscriptReplayEvidence,
            OracleFamilyKind::BlobHarnessEvidence,
            OracleFamilyKind::BlobHeavyQualification,
            OracleFamilyKind::ForbiddenShortcutRejection,
        ]),
        counter_contracts: blob_counter_topology(actor_step_count, topology),
        fixture_classes: RequiredFixtureClassSet::from_fixture_classes([
            FixtureClassKind::AspectNativeBoundaryFact,
        ]),
    }
}

fn blob_required_drivers(failure_point: BlobHarnessFailurePoint) -> RequiredPhysicalDriverSet {
    let mut drivers = vec![
        PhysicalDriverKind::ProductionBoundaryYieldpoint,
        PhysicalDriverKind::ShortcutRejectionBoundary,
    ];
    match failure_point {
        BlobHarnessFailurePoint::NoFaultSeed => {
            drivers.push(PhysicalDriverKind::MemoryPressureBoundary);
        }
        BlobHarnessFailurePoint::AfterSessionCheckpoint => {
            drivers.push(PhysicalDriverKind::FreshRuntimeRecovery);
        }
        BlobHarnessFailurePoint::DuringTierMove => {
            drivers.push(PhysicalDriverKind::IoPressureBoundary);
        }
        BlobHarnessFailurePoint::DuringExport => {
            drivers.push(PhysicalDriverKind::OfflineVerifierBoundary);
        }
        BlobHarnessFailurePoint::AfterChunkWrite
        | BlobHarnessFailurePoint::AfterRootPublication
        | BlobHarnessFailurePoint::DuringReclaim => {}
    }
    RequiredPhysicalDriverSet::from_drivers(drivers)
}

#[cfg(test)]
mod tests {
    use forge_store_blob_chunks::{
        BlobHarnessAccessMode, BlobHarnessActorMix, BlobHarnessChunkSizeClass,
        BlobHarnessChunkTopology, BlobHarnessFailurePoint, BlobHarnessPlacementClass,
        BlobHarnessSecurityScopeClass, BlobHarnessSizeClass,
    };

    use crate::{BlobHarnessScenarioMetadata, PhysicalDriverKind};

    use super::{blob_harness_shape, blob_required_drivers};

    fn metadata(failure_point: BlobHarnessFailurePoint) -> BlobHarnessScenarioMetadata {
        BlobHarnessScenarioMetadata::new(
            BlobHarnessSizeClass::MemoryEnvelopeExceeding,
            BlobHarnessChunkSizeClass::Fixed64KiB,
            BlobHarnessPlacementClass::ExternalPlacementObserved,
            BlobHarnessSecurityScopeClass::ScopePreserving,
            BlobHarnessAccessMode::PartialReplication,
            failure_point,
            BlobHarnessActorMix::PlacementMovePartialReplication,
        )
    }

    #[test]
    fn blob_shape_requires_driver_for_each_failure_boundary() {
        assert!(blob_required_drivers(BlobHarnessFailurePoint::NoFaultSeed)
            .contains(PhysicalDriverKind::MemoryPressureBoundary));
        assert!(
            blob_required_drivers(BlobHarnessFailurePoint::AfterSessionCheckpoint)
                .contains(PhysicalDriverKind::FreshRuntimeRecovery)
        );
        assert!(
            blob_required_drivers(BlobHarnessFailurePoint::DuringTierMove)
                .contains(PhysicalDriverKind::IoPressureBoundary)
        );
        assert!(blob_required_drivers(BlobHarnessFailurePoint::DuringExport)
            .contains(PhysicalDriverKind::OfflineVerifierBoundary));
    }

    #[test]
    fn blob_shape_keeps_production_trace_driver_for_all_failure_points() {
        let topology = BlobHarnessChunkTopology::from_classes(
            BlobHarnessSizeClass::MemoryEnvelopeExceeding,
            BlobHarnessChunkSizeClass::Fixed64KiB,
        )
        .unwrap();
        let shape = blob_harness_shape(
            2,
            topology,
            metadata(BlobHarnessFailurePoint::DuringTierMove),
        );

        assert!(shape
            .drivers
            .contains(PhysicalDriverKind::ProductionBoundaryYieldpoint));
        assert!(shape
            .drivers
            .contains(PhysicalDriverKind::IoPressureBoundary));
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
        bounded_contract(CounterContractKind::AllocationBytes, topology.chunk_bytes()),
        bounded_contract(CounterContractKind::ResidentBytes, 512 * 1024 * 1024),
    ])
}
