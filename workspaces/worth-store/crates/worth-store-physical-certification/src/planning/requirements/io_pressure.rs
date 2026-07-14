use crate::{
    CounterContractKind, FixtureClassKind, ObserverKind, OracleFamilyKind, PhysicalCounterContract,
    PhysicalDriverKind, PhysicalScenarioActor, PhysicalSimulationCapability,
};

use super::{
    RequiredActorSet, RequiredFixtureClassSet, RequiredObserverSet, RequiredOracleFamilySet,
    RequiredPhysicalDriverSet, RequiredSimulationPlanShape,
};
use crate::{PhysicalSimulationCapabilitySet, RequiredCounterContractSet};

pub(super) fn io_pressure_shape(actor_step_count: u64) -> RequiredSimulationPlanShape {
    RequiredSimulationPlanShape {
        capabilities: PhysicalSimulationCapabilitySet::from_capabilities([
            PhysicalSimulationCapability::ProductionBoundaryDriver,
            PhysicalSimulationCapability::IndependentObserver,
            PhysicalSimulationCapability::CertificationOracleFamily,
            PhysicalSimulationCapability::CounterContracts,
            PhysicalSimulationCapability::FixtureClassAdmission,
            PhysicalSimulationCapability::EvidencePolicy,
            PhysicalSimulationCapability::ForbiddenShortcutDenial,
            PhysicalSimulationCapability::ProfileSupport,
        ]),
        actors: RequiredActorSet::from_actors([
            PhysicalScenarioActor::foreground_reader("foreground-reader"),
            PhysicalScenarioActor::scrub_driver("repair-scan"),
        ]),
        drivers: RequiredPhysicalDriverSet::from_drivers([
            PhysicalDriverKind::IoPressureBoundary,
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
        ]),
        observers: RequiredObserverSet::from_observers([ObserverKind::IndependentPhysicalTrace]),
        oracle_families: RequiredOracleFamilySet::from_oracles([
            OracleFamilyKind::TranscriptReplayEvidence,
            OracleFamilyKind::IoPressureSimulation,
        ]),
        counter_contracts: RequiredCounterContractSet::from_contracts([
            PhysicalCounterContract::exact(CounterContractKind::ActorStepExact, actor_step_count),
            PhysicalCounterContract::exact(CounterContractKind::ReplayIdentityExact, 1),
            PhysicalCounterContract::profile_scoped(CounterContractKind::ProfileResourceEnvelope),
            PhysicalCounterContract::bounded(CounterContractKind::IoQueueDepth, 64)
                .expect("static S.6 queue-depth counter contract is bounded"),
            PhysicalCounterContract::positive(CounterContractKind::IoInterferenceEvents)
                .expect("static S.6 interference contract is positive"),
            PhysicalCounterContract::bounded(CounterContractKind::AllocationBytes, 128 * 1024)
                .expect("static S.6 allocation contract is bounded"),
        ]),
        fixture_classes: RequiredFixtureClassSet::from_fixture_classes([
            FixtureClassKind::AspectNativeBoundaryFact,
        ]),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        CounterContractKind, OracleFamilyKind, PhysicalDriverKind, PhysicalScenarioActorRole,
    };

    use super::io_pressure_shape;

    #[test]
    fn io_pressure_shape_uses_existing_simulation_harness_harness_surfaces() {
        let shape = io_pressure_shape(2);

        assert!(shape
            .drivers
            .contains(PhysicalDriverKind::IoPressureBoundary));
        assert!(shape
            .drivers
            .contains(PhysicalDriverKind::ProductionBoundaryYieldpoint));
        assert!(shape
            .oracle_families
            .contains(OracleFamilyKind::IoPressureSimulation));
        assert!(shape
            .counter_contracts
            .contains(CounterContractKind::IoQueueDepth));
        assert!(shape
            .counter_contracts
            .contains(CounterContractKind::IoInterferenceEvents));
        assert!(shape
            .actors
            .contains_role(PhysicalScenarioActorRole::ForegroundReader));
        assert!(shape
            .actors
            .contains_role(PhysicalScenarioActorRole::ScrubDriver));
    }
}
