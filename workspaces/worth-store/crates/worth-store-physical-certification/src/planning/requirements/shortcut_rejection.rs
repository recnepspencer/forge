use crate::{
    CounterContractKind, FixtureClassKind, ObserverKind, OracleFamilyKind, PhysicalCounterContract,
    PhysicalDriverKind,
};

use super::{
    baseline_capabilities, positive_contract, RequiredActorSet, RequiredFixtureClassSet,
    RequiredObserverSet, RequiredOracleFamilySet, RequiredPhysicalDriverSet,
    RequiredSimulationPlanShape,
};
use crate::RequiredCounterContractSet;

pub(super) fn shortcut_rejection_shape() -> RequiredSimulationPlanShape {
    RequiredSimulationPlanShape {
        capabilities: baseline_capabilities(),
        actors: RequiredActorSet::from_actors([]),
        drivers: RequiredPhysicalDriverSet::from_drivers([
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
            PhysicalDriverKind::ShortcutRejectionBoundary,
        ]),
        observers: RequiredObserverSet::from_observers([ObserverKind::ShortcutRejectionObserver]),
        oracle_families: RequiredOracleFamilySet::from_oracles([
            OracleFamilyKind::TranscriptReplayEvidence,
            OracleFamilyKind::ForbiddenShortcutRejection,
        ]),
        counter_contracts: RequiredCounterContractSet::from_contracts([
            PhysicalCounterContract::exact(CounterContractKind::ForbiddenShortcutExact, 0),
            PhysicalCounterContract::exact(CounterContractKind::ReplayIdentityExact, 1),
            positive_contract(CounterContractKind::BlockedReclaimAttempts),
        ]),
        fixture_classes: RequiredFixtureClassSet::from_fixture_classes([
            FixtureClassKind::AspectNativeBoundaryFact,
        ]),
    }
}
