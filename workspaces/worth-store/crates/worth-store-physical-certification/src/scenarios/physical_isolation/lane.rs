use super::fixture::physical_isolation_boundary_fact;
use crate::{
    physical_scenario, CertifiedPhysicalScenario, PhysicalScenarioActor,
    PhysicalScenarioExpectation, PhysicalScenarioFault, PhysicalScenarioFaultKind,
    PhysicalScenarioIntent, PhysicalSimulationScenarioFamily,
};

#[derive(Debug, Clone)]
pub struct PhysicalIsolationHarnessLane {
    name: &'static str,
    scenario: CertifiedPhysicalScenario,
    expected_fault: PhysicalScenarioFaultKind,
}

pub fn physical_isolation_lanes() -> Vec<PhysicalIsolationHarnessLane> {
    vec![
        lane(
            "compaction-interlock",
            PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock,
            PhysicalScenarioIntent::PhysicalIsolationCompactionEarlyReclaimMutant,
            PhysicalScenarioFault::early_reclaim(),
            PhysicalScenarioExpectation::physical_isolation_denial(),
            [
                PhysicalScenarioActor::foreground_reader("reader"),
                PhysicalScenarioActor::compaction_driver("compactor"),
            ],
        ),
        lane(
            "checkpoint-publication",
            PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock,
            PhysicalScenarioIntent::PhysicalIsolationCheckpointPublicationInterlock,
            PhysicalScenarioFault::mixed_root_read(),
            PhysicalScenarioExpectation::physical_isolation_interleaving(),
            [
                PhysicalScenarioActor::foreground_reader("reader"),
                PhysicalScenarioActor::checkpoint_driver("checkpoint"),
            ],
        ),
        lane(
            "reclaim-reachability",
            PhysicalSimulationScenarioFamily::PhysicalIsolationReclaimReachability,
            PhysicalScenarioIntent::PhysicalIsolationReclaimReachabilityBarrier,
            PhysicalScenarioFault::early_reclaim(),
            PhysicalScenarioExpectation::physical_isolation_denial(),
            [
                PhysicalScenarioActor::foreground_reader("reader"),
                PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
            ],
        ),
        lane(
            "tier-movement",
            PhysicalSimulationScenarioFamily::PhysicalIsolationTierMovementStability,
            PhysicalScenarioIntent::PhysicalIsolationTierMovementStabilityOnly,
            PhysicalScenarioFault::no_fault(),
            PhysicalScenarioExpectation::physical_isolation_interleaving(),
            [
                PhysicalScenarioActor::foreground_reader("reader"),
                PhysicalScenarioActor::maintenance_reclaimer("tier-movement"),
            ],
        ),
        lane(
            "future-chunk-stability",
            PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability,
            PhysicalScenarioIntent::PhysicalIsolationFutureChunkStabilityOnly,
            PhysicalScenarioFault::no_fault(),
            PhysicalScenarioExpectation::physical_isolation_interleaving(),
            [
                PhysicalScenarioActor::foreground_reader("reader"),
                PhysicalScenarioActor::future_extension_slot("future-chunk"),
            ],
        ),
        lane(
            "restart-during-cutover",
            PhysicalSimulationScenarioFamily::PhysicalIsolationRestartDuringCutover,
            PhysicalScenarioIntent::PhysicalIsolationRestartDuringCutover,
            PhysicalScenarioFault::stale_epoch_reuse(),
            PhysicalScenarioExpectation::physical_isolation_interleaving(),
            [
                PhysicalScenarioActor::foreground_reader("reader"),
                PhysicalScenarioActor::foreground_writer("writer"),
            ],
        ),
    ]
}

impl PhysicalIsolationHarnessLane {
    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn scenario(&self) -> &CertifiedPhysicalScenario {
        &self.scenario
    }

    pub const fn expected_fault(&self) -> PhysicalScenarioFaultKind {
        self.expected_fault
    }
}

fn lane<const N: usize>(
    name: &'static str,
    family: PhysicalSimulationScenarioFamily,
    intent: PhysicalScenarioIntent,
    fault: PhysicalScenarioFault,
    expectation: PhysicalScenarioExpectation,
    actors: [PhysicalScenarioActor; N],
) -> PhysicalIsolationHarnessLane {
    let expected_fault = fault.kind();
    let mut builder = physical_scenario(format!("store.physical.s5.interleaving.{name}"))
        .family(family)
        .intent(intent)
        .fixture(physical_isolation_boundary_fact(name, 11));
    for actor in actors {
        builder = builder.actor(actor);
    }
    PhysicalIsolationHarnessLane {
        name,
        expected_fault,
        scenario: builder
            .fault(fault)
            .schedule(crate::PhysicalScenarioSchedule::named_boundary_yieldpoint(
                "root-publication-before-observe",
            ))
            .expectation(expectation)
            .certify_definition()
            .unwrap(),
    }
}
