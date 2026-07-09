use worth_store_physical_backend::{BackendDurabilityProfileId, ProductionStorageBoundarySeam};
use worth_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, AdmittedDriverContractSet,
    CrashRuntimeIsolationDriver, DriverAdmissionDenial, ForbiddenShortcutSet, IoPressureDriver,
    MemoryPressureDriver, OfflineVerifierDriver, PhysicalBoundarySeam, PhysicalBoundaryYieldpoint,
    PhysicalDriverKind, PhysicalScenarioActor, PhysicalScenarioExpectation, PhysicalScenarioIntent,
    PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet, PhysicalSimulationProfile,
    PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily,
    ProductionStorageBoundaryDriver, SimulationEvidencePolicy, SimulationPlanDenial,
    SimulationPlanningContext, SupportedObserverSet, SupportedOracleFamilySet,
    SupportedPhysicalDriverSet,
};
use worth_store_test_support::{
    admitted_developer_smoke_driver_contracts, fake_in_memory_only_driver_attempt,
    private_mutation_driver_attempt_fixture, sleep_based_scheduling_driver_attempt,
    test_support_verdict_driver_attempt_fixture, NativeStoreAspectFixture,
};

#[test]
fn forbidden_driver_shortcuts_deny_at_driver_admission() {
    assert_eq!(
        private_mutation_driver_attempt_fixture().unwrap_err(),
        DriverAdmissionDenial::PrivateMutationDriverDenied
    );
    assert_eq!(
        fake_in_memory_only_driver_attempt().unwrap_err(),
        DriverAdmissionDenial::FakeInMemoryOnlyDriverDenied
    );
    assert_eq!(
        sleep_based_scheduling_driver_attempt().unwrap_err(),
        DriverAdmissionDenial::SleepBasedSchedulingDenied
    );
    assert_eq!(
        test_support_verdict_driver_attempt_fixture().unwrap_err(),
        DriverAdmissionDenial::TestSupportVerdictDriverDenied
    );
}

#[test]
fn driver_without_declared_yieldpoint_denies_before_planning() {
    let denial = ProductionStorageBoundaryDriver::for_backend_profile(
        BackendDurabilityProfileId::PosixFileFsyncDirFsync,
    )
    .admit()
    .unwrap_err();

    assert_eq!(
        denial,
        DriverAdmissionDenial::NoYieldpointsDeclared(
            PhysicalDriverKind::ProductionBoundaryYieldpoint
        )
    );
}

#[test]
fn schedule_with_no_admitted_yieldpoint_binding_denies_before_execution() {
    let denial = lower_physical_simulation_plan(unbound_schedule_scenario(), complete_context())
        .expect_err("unbound schedule yieldpoint must deny");

    assert_eq!(
        denial,
        SimulationPlanDenial::UnboundYieldpointSchedule("wal-append-before-flush".to_owned())
    );
}

#[test]
fn admitted_non_required_driver_yieldpoint_cannot_satisfy_scenario_schedule() {
    let denial =
        lower_physical_simulation_plan(non_required_driver_schedule_scenario(), complete_context())
            .expect_err("non-required admitted driver yieldpoint must deny");

    assert_eq!(
        denial,
        SimulationPlanDenial::UnboundYieldpointSchedule("memory-pressure-boundary".to_owned())
    );
}

#[test]
fn admitted_driver_cannot_bind_registered_seam_it_did_not_declare() {
    let missing = ProductionStorageBoundarySeam::CrashSeam;
    let driver = production_driver_without_registered_seam(missing);
    let context = complete_context()
        .with_driver_contracts(AdmittedDriverContractSet::from_drivers([driver]).unwrap());

    let denial = lower_physical_simulation_plan(
        scenario_scheduled_at(missing.token(), "store.physical.s5.missing.registered.seam"),
        context,
    )
    .expect_err("registered but undeclared yieldpoint must deny before execution");

    assert_eq!(
        denial,
        SimulationPlanDenial::UnboundYieldpointSchedule(missing.token().to_owned())
    );
}

#[test]
fn driver_missing_required_relevant_yieldpoint_denies() {
    let denial = ProductionStorageBoundaryDriver::for_backend_profile(
        BackendDurabilityProfileId::PosixFileFsyncDirFsync,
    )
    .declare_yieldpoint(PhysicalBoundaryYieldpoint::wal_append_before_flush())
    .admit()
    .unwrap_err();

    assert_eq!(
        denial,
        DriverAdmissionDenial::MissingRelevantYieldpoint {
            driver: PhysicalDriverKind::ProductionBoundaryYieldpoint,
            yieldpoint: "root-publication-before-observe",
        }
    );
}

#[test]
fn production_driver_rejects_yieldpoints_owned_by_other_capabilities() {
    let denial = ProductionStorageBoundaryDriver::for_backend_profile(
        BackendDurabilityProfileId::PosixFileFsyncDirFsync,
    )
    .declare_yieldpoint(PhysicalBoundaryYieldpoint::root_publication_before_observe())
    .declare_yieldpoint(PhysicalBoundaryYieldpoint::fresh_runtime_replay_open())
    .admit()
    .unwrap_err();

    assert_eq!(
        denial,
        DriverAdmissionDenial::IrrelevantYieldpointForDriver {
            driver: PhysicalDriverKind::ProductionBoundaryYieldpoint,
            seam: PhysicalBoundarySeam::FreshRuntimeRecovery,
        }
    );
}

#[test]
fn crash_driver_rejects_production_storage_yieldpoints() {
    let denial = CrashRuntimeIsolationDriver::fresh_runtime_recovery()
        .declare_yieldpoint(PhysicalBoundaryYieldpoint::fresh_runtime_replay_open())
        .declare_yieldpoint(PhysicalBoundaryYieldpoint::root_publication_before_observe())
        .admit()
        .unwrap_err();

    assert_eq!(
        denial,
        DriverAdmissionDenial::IrrelevantYieldpointForDriver {
            driver: PhysicalDriverKind::FreshRuntimeRecovery,
            seam: PhysicalBoundarySeam::ProductionStorage(
                ProductionStorageBoundarySeam::RootPublicationBeforeObserve
            ),
        }
    );
}

#[test]
fn pressure_and_verifier_drivers_reject_foreign_yieldpoint_seams() {
    let memory_denial = MemoryPressureDriver::deterministic_pressure_boundary()
        .declare_yieldpoint(PhysicalBoundaryYieldpoint::memory_pressure_boundary())
        .declare_yieldpoint(PhysicalBoundaryYieldpoint::io_pressure_boundary())
        .admit()
        .unwrap_err();
    let io_denial = IoPressureDriver::deterministic_queue_boundary()
        .declare_yieldpoint(PhysicalBoundaryYieldpoint::io_pressure_boundary())
        .declare_yieldpoint(
            PhysicalBoundaryYieldpoint::offline_verifier_layout_walk_before_runtime_recovery(),
        )
        .admit()
        .unwrap_err();
    let verifier_denial = OfflineVerifierDriver::layout_walk_boundary()
        .declare_yieldpoint(
            PhysicalBoundaryYieldpoint::offline_verifier_layout_walk_before_runtime_recovery(),
        )
        .declare_yieldpoint(PhysicalBoundaryYieldpoint::memory_pressure_boundary())
        .admit()
        .unwrap_err();

    assert_eq!(
        memory_denial,
        DriverAdmissionDenial::IrrelevantYieldpointForDriver {
            driver: PhysicalDriverKind::MemoryPressureBoundary,
            seam: PhysicalBoundarySeam::IoPressure,
        }
    );
    assert_eq!(
        io_denial,
        DriverAdmissionDenial::IrrelevantYieldpointForDriver {
            driver: PhysicalDriverKind::IoPressureBoundary,
            seam: PhysicalBoundarySeam::OfflineVerifier(
                worth_store_physical_certification::OfflineVerifierBoundarySeam::LayoutWalkBeforeRuntimeRecovery
            ),
        }
    );
    assert_eq!(
        verifier_denial,
        DriverAdmissionDenial::IrrelevantYieldpointForDriver {
            driver: PhysicalDriverKind::OfflineVerifierBoundary,
            seam: PhysicalBoundarySeam::MemoryPressure,
        }
    );
}

#[test]
fn supported_driver_labels_do_not_authorize_new_driver_contracts() {
    let supported = SupportedPhysicalDriverSet::all_for_developer_smoke();
    let admitted = AdmittedDriverContractSet::empty();

    assert!(supported.contains(PhysicalDriverKind::MemoryPressureBoundary));
    assert!(!admitted.contains_driver(PhysicalDriverKind::MemoryPressureBoundary));
}

#[test]
fn duplicate_driver_contracts_deny_instead_of_silently_collapsing() {
    let first = admitted_production_driver();
    let second = admitted_production_driver();
    let denial = AdmittedDriverContractSet::from_drivers([first, second]).unwrap_err();

    assert_eq!(
        denial,
        DriverAdmissionDenial::DuplicateDriverKind(
            PhysicalDriverKind::ProductionBoundaryYieldpoint
        )
    );
}

#[test]
fn loose_supported_driver_set_cannot_replace_admitted_contracts() {
    let denial = lower_physical_simulation_plan(
        bound_schedule_scenario(),
        complete_context()
            .with_driver_contracts(
                worth_store_physical_certification::AdmittedDriverContractSet::empty(),
            )
            .with_supported_drivers(SupportedPhysicalDriverSet::all_for_developer_smoke()),
    )
    .expect_err("loose supported-driver labels must not satisfy driver authority");

    assert_eq!(
        denial,
        SimulationPlanDenial::MissingPhysicalDriver(
            PhysicalDriverKind::ProductionBoundaryYieldpoint
        )
    );
}

fn complete_context() -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(PhysicalSimulationProfile::DeveloperSmoke)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(PhysicalSimulationCapabilitySet::s5_readiness_shape_probe())
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::roadmap2_baseline())
}

fn bound_schedule_scenario() -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s5.bound.driver")
        .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("driver", 8)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_s5_readiness_shape())
        .certify_definition()
        .unwrap()
}

fn admitted_production_driver() -> worth_store_physical_certification::PhysicalSimulationDriver {
    ProductionStorageBoundaryDriver::for_backend_profile(
        BackendDurabilityProfileId::PosixFileFsyncDirFsync,
    )
    .declare_yieldpoint(PhysicalBoundaryYieldpoint::root_publication_before_observe())
    .admit()
    .unwrap()
}

fn unbound_schedule_scenario() -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s5.unbound.driver")
        .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("driver", 8)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "wal-append-before-flush",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_s5_readiness_shape())
        .certify_definition()
        .unwrap()
}

fn non_required_driver_schedule_scenario(
) -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s5.non.required.driver.schedule")
        .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("driver", 8)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "memory-pressure-boundary",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_s5_readiness_shape())
        .certify_definition()
        .unwrap()
}

fn production_driver_without_registered_seam(
    missing: ProductionStorageBoundarySeam,
) -> worth_store_physical_certification::PhysicalSimulationDriver {
    let mut driver = ProductionStorageBoundaryDriver::for_backend_profile(
        BackendDurabilityProfileId::PosixFileFsyncDirFsync,
    );
    for seam in ProductionStorageBoundarySeam::phase4_registered_seams() {
        if *seam != missing {
            driver =
                driver.declare_yieldpoint(PhysicalBoundaryYieldpoint::production_storage(*seam));
        }
    }
    driver.admit().unwrap()
}

fn scenario_scheduled_at(
    yieldpoint: &str,
    name: &str,
) -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario(name)
        .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("driver", 9)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            yieldpoint,
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_s5_readiness_shape())
        .certify_definition()
        .unwrap()
}
