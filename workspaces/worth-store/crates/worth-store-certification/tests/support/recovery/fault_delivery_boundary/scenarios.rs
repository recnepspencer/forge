use super::drivers::production_driver_with_all_seams;
use worth_store_physical_certification::{
    AdmittedDriverContractSet, ForbiddenShortcutSet, PhysicalBoundaryYieldpoint,
    PhysicalScenarioActor, PhysicalScenarioExpectation, PhysicalScenarioIntent,
    PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet, PhysicalSimulationProfile,
    PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily, SimulationEvidencePolicy,
    SimulationPlanningContext, SupportedObserverSet, SupportedOracleFamilySet,
};
use worth_store_test_support::{
    admitted_developer_smoke_driver_contracts, NativeStoreAspectFixture,
};

pub fn readiness_scenario() -> worth_store_physical_certification::CertifiedPhysicalScenario {
    readiness_scenario_scheduled_at("root-publication-before-observe")
}

pub fn readiness_scenario_scheduled_at(
    yieldpoint: &str,
) -> worth_store_physical_certification::CertifiedPhysicalScenario {
    worth_store_physical_certification::physical_scenario(
        "store.physical.s45.phase6.fault.delivery",
    )
    .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
    .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
    .fixture(
        NativeStoreAspectFixture::segment_header("fault-delivery", 6)
            .boundary_fact()
            .clone(),
    )
    .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
    .actor(PhysicalScenarioActor::foreground_reader("reader"))
    .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
        yieldpoint,
    ))
    .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
    .certify_definition()
    .unwrap()
}

pub fn recovery_scenario() -> worth_store_physical_certification::CertifiedPhysicalScenario {
    worth_store_physical_certification::physical_scenario(
        "store.physical.s45.phase6.crash.delivery",
    )
    .family(PhysicalSimulationScenarioFamily::RecoveryDogfood)
    .intent(PhysicalScenarioIntent::RecoveryReplayDogfood)
    .fixture(
        NativeStoreAspectFixture::segment_header("fault-delivery", 6)
            .boundary_fact()
            .clone(),
    )
    .actor(PhysicalScenarioActor::recovery_driver("recovery"))
    .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
        "fresh-runtime-replay-open",
    ))
    .expectation(PhysicalScenarioExpectation::recovery_dogfood())
    .certify_definition()
    .unwrap()
}

pub fn complete_context() -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(PhysicalSimulationProfile::DeveloperSmoke)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(
            PhysicalSimulationCapabilitySet::physical_isolation_readiness_shape_probe(),
        )
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::physical_certification_baseline())
}

pub fn scenario_for_yieldpoint(
    yieldpoint: &str,
) -> worth_store_physical_certification::CertifiedPhysicalScenario {
    if yieldpoint == "io-pressure-boundary" {
        io_pressure_scenario()
    } else {
        readiness_scenario_scheduled_at(yieldpoint)
    }
}

fn io_pressure_scenario() -> worth_store_physical_certification::CertifiedPhysicalScenario {
    worth_store_physical_certification::physical_scenario(
        "store.physical.s45.phase6.io-stall.delivery",
    )
    .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
    .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
    .fixture(
        NativeStoreAspectFixture::segment_header("fault-delivery", 6)
            .boundary_fact()
            .clone(),
    )
    .actor(PhysicalScenarioActor::foreground_reader("reader"))
    .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
        "io-pressure-boundary",
    ))
    .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
    .certify_definition()
    .unwrap()
}

pub fn context_for_yieldpoint(yieldpoint: PhysicalBoundaryYieldpoint) -> SimulationPlanningContext {
    let context = complete_context();
    if yieldpoint.seam() == worth_store_physical_certification::PhysicalBoundarySeam::IoPressure {
        return context.with_driver_contracts(
            AdmittedDriverContractSet::from_drivers(
                admitted_developer_smoke_driver_contracts()
                    .unwrap()
                    .iter()
                    .filter(|driver| {
                        driver.kind()
                            != worth_store_physical_certification::PhysicalDriverKind::ProductionBoundaryYieldpoint
                    })
                    .cloned(),
            )
            .unwrap(),
        );
    }
    context.with_driver_contracts(
        AdmittedDriverContractSet::from_drivers([production_driver_with_all_seams()]).unwrap(),
    )
}
