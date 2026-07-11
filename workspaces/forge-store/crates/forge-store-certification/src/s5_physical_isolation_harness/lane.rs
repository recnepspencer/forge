use forge_store_physical_certification::{
    physical_scenario, CertifiedPhysicalScenario, PhysicalScenarioActor,
    PhysicalScenarioExpectation, PhysicalScenarioFault, PhysicalScenarioFaultKind,
    PhysicalScenarioIntent, PhysicalSimulationScenarioFamily,
};
use forge_store_test_support::{
    physical_isolation_boundary_fact, physical_isolation_boundary_yieldpoint,
};

#[derive(Debug, Clone)]
pub struct S5PhysicalIsolationHarnessLane {
    name: &'static str,
    scenario: CertifiedPhysicalScenario,
    expected_fault: PhysicalScenarioFaultKind,
}

pub fn s5_physical_isolation_lanes() -> Vec<S5PhysicalIsolationHarnessLane> {
    vec![
        lane(
            "compaction-interlock",
            PhysicalSimulationScenarioFamily::S5CompactionInterlock,
            PhysicalScenarioIntent::S5CompactionEarlyReclaimMutant,
            PhysicalScenarioFault::early_reclaim(),
            PhysicalScenarioExpectation::s5_physical_isolation_denial(),
            [
                PhysicalScenarioActor::foreground_reader("reader"),
                PhysicalScenarioActor::compaction_driver("compactor"),
            ],
        ),
        lane(
            "checkpoint-publication",
            PhysicalSimulationScenarioFamily::S5CheckpointPublicationInterlock,
            PhysicalScenarioIntent::S5CheckpointPublicationInterlock,
            PhysicalScenarioFault::mixed_root_read(),
            PhysicalScenarioExpectation::s5_physical_isolation_interleaving(),
            [
                PhysicalScenarioActor::foreground_reader("reader"),
                PhysicalScenarioActor::checkpoint_driver("checkpoint"),
            ],
        ),
        lane(
            "reclaim-reachability",
            PhysicalSimulationScenarioFamily::S5ReclaimReachability,
            PhysicalScenarioIntent::S5ReclaimReachabilityBarrier,
            PhysicalScenarioFault::early_reclaim(),
            PhysicalScenarioExpectation::s5_physical_isolation_denial(),
            [
                PhysicalScenarioActor::foreground_reader("reader"),
                PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
            ],
        ),
        lane(
            "tier-movement",
            PhysicalSimulationScenarioFamily::S5TierMovementStability,
            PhysicalScenarioIntent::S5TierMovementStabilityOnly,
            PhysicalScenarioFault::no_fault(),
            PhysicalScenarioExpectation::s5_physical_isolation_interleaving(),
            [
                PhysicalScenarioActor::foreground_reader("reader"),
                PhysicalScenarioActor::maintenance_reclaimer("tier-movement"),
            ],
        ),
        lane(
            "future-chunk-stability",
            PhysicalSimulationScenarioFamily::S5FutureChunkStability,
            PhysicalScenarioIntent::S5FutureChunkStabilityOnly,
            PhysicalScenarioFault::no_fault(),
            PhysicalScenarioExpectation::s5_physical_isolation_interleaving(),
            [
                PhysicalScenarioActor::foreground_reader("reader"),
                PhysicalScenarioActor::future_extension_slot("future-chunk"),
            ],
        ),
        lane(
            "restart-during-cutover",
            PhysicalSimulationScenarioFamily::S5RestartDuringCutover,
            PhysicalScenarioIntent::S5RestartDuringCutover,
            PhysicalScenarioFault::stale_epoch_reuse(),
            PhysicalScenarioExpectation::s5_physical_isolation_interleaving(),
            [
                PhysicalScenarioActor::foreground_reader("reader"),
                PhysicalScenarioActor::foreground_writer("writer"),
            ],
        ),
    ]
}

impl S5PhysicalIsolationHarnessLane {
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
) -> S5PhysicalIsolationHarnessLane {
    let expected_fault = fault.kind();
    let mut builder = physical_scenario(format!("store.physical.s5.interleaving.{name}"))
        .family(family)
        .intent(intent)
        .fixture(physical_isolation_boundary_fact(name, 11));
    for actor in actors {
        builder = builder.actor(actor);
    }
    S5PhysicalIsolationHarnessLane {
        name,
        expected_fault,
        scenario: builder
            .fault(fault)
            .schedule(physical_isolation_boundary_yieldpoint())
            .expectation(expectation)
            .certify_definition()
            .unwrap(),
    }
}
