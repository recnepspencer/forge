use forge_store_physical_backend::BackendDurabilityProfileId;
use forge_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, CheckpointActor, CompactionActor,
    DriverBoundaryKind, DriverEvidenceSurface, DriverFaultClass, ForbiddenShortcutSet,
    ForegroundReadActor, ForegroundWriteActor, IoPressureDriver, MemoryPressureDriver,
    OfflineVerifierActor, OfflineVerifierDriver, PhysicalBoundaryYieldpoint, PhysicalDriverKind,
    PhysicalScenarioActor, PhysicalScenarioActorRole, PhysicalScenarioExpectation,
    PhysicalScenarioIntent, PhysicalScenarioSchedule, PhysicalSimulationActor,
    PhysicalSimulationActorAdmissionDenial, PhysicalSimulationCapabilitySet,
    PhysicalSimulationProfile, PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily,
    ProductionStorageBoundaryDriver, ReclaimActor, RecoveryActor, ScrubActor,
    SimulationEvidencePolicy, SimulationPlanningContext, SupportedObserverSet,
    SupportedOracleFamilySet,
};
use forge_store_test_support::{
    admitted_developer_smoke_driver_contracts, NativeStoreAspectFixture,
};

#[test]
fn admitted_production_driver_contract_binds_scenario_yieldpoint_before_execution() {
    let plan = lower_physical_simulation_plan(physical_isolation_scenario(), complete_context()).unwrap();

    assert!(plan
        .driver_contracts()
        .contains_driver(PhysicalDriverKind::ProductionBoundaryYieldpoint));
    assert!(!plan
        .driver_contracts()
        .contains_driver(PhysicalDriverKind::FreshRuntimeRecovery));
    assert!(!plan
        .driver_contracts()
        .contains_driver(PhysicalDriverKind::ShortcutRejectionBoundary));
    assert!(!plan
        .driver_contracts()
        .contains_driver(PhysicalDriverKind::MemoryPressureBoundary));
    assert!(!plan
        .driver_contracts()
        .contains_driver(PhysicalDriverKind::IoPressureBoundary));
    assert!(!plan
        .driver_contracts()
        .contains_driver(PhysicalDriverKind::OfflineVerifierBoundary));
    assert_eq!(
        plan.yieldpoint_binding().scheduled_yieldpoint(),
        "root-publication-before-observe"
    );
    assert_eq!(
        plan.yieldpoint_binding().declared_yieldpoint().name(),
        "root-publication-before-observe"
    );
    assert!(plan
        .driver_contracts()
        .binds_yieldpoint("root-publication-before-observe"));
}

#[test]
fn production_storage_driver_declares_profile_faults_and_evidence_surfaces() {
    let driver = ProductionStorageBoundaryDriver::for_backend_profile(
        BackendDurabilityProfileId::PosixFileFsyncDirFsync,
    )
    .declare_yieldpoint(PhysicalBoundaryYieldpoint::root_publication_before_observe())
    .admit()
    .unwrap();

    assert_eq!(
        driver.backend_profile(),
        Some(BackendDurabilityProfileId::PosixFileFsyncDirFsync)
    );
    assert_eq!(
        driver.profile().driver(),
        PhysicalDriverKind::ProductionBoundaryYieldpoint
    );
    assert!(driver
        .profile()
        .supported_faults()
        .contains(&DriverFaultClass::NoFault));
    assert!(driver
        .profile()
        .unsupported_faults()
        .contains(&DriverFaultClass::FutureExtensionSlot));
    assert!(driver
        .profile()
        .evidence_surfaces()
        .contains(&DriverEvidenceSurface::ProductionBoundaryTrace));
}

#[test]
fn pressure_and_verifier_drivers_declare_first_class_contract_profiles() {
    let memory_driver = MemoryPressureDriver::deterministic_pressure_boundary()
        .declare_yieldpoint(PhysicalBoundaryYieldpoint::memory_pressure_boundary())
        .admit()
        .unwrap();
    let io_driver = IoPressureDriver::deterministic_queue_boundary()
        .declare_yieldpoint(PhysicalBoundaryYieldpoint::io_pressure_boundary())
        .admit()
        .unwrap();
    let verifier_driver = OfflineVerifierDriver::layout_walk_boundary()
        .declare_yieldpoint(
            PhysicalBoundaryYieldpoint::offline_verifier_layout_walk_before_runtime_recovery(),
        )
        .admit()
        .unwrap();

    assert_eq!(
        memory_driver.profile().boundary(),
        DriverBoundaryKind::MemoryPressure
    );
    assert_eq!(
        memory_driver.profile().driver(),
        PhysicalDriverKind::MemoryPressureBoundary
    );
    assert!(memory_driver
        .profile()
        .supported_faults()
        .contains(&DriverFaultClass::MemoryPressure));
    assert!(memory_driver
        .profile()
        .evidence_surfaces()
        .contains(&DriverEvidenceSurface::MemoryPressureEnvelope));

    assert_eq!(
        io_driver.profile().boundary(),
        DriverBoundaryKind::IoPressure
    );
    assert_eq!(
        io_driver.profile().driver(),
        PhysicalDriverKind::IoPressureBoundary
    );
    assert!(io_driver
        .profile()
        .supported_faults()
        .contains(&DriverFaultClass::IoPressure));
    assert!(io_driver
        .profile()
        .evidence_surfaces()
        .contains(&DriverEvidenceSurface::IoPressureEnvelope));

    assert_eq!(
        verifier_driver.profile().boundary(),
        DriverBoundaryKind::OfflineVerifier
    );
    assert_eq!(
        verifier_driver.profile().driver(),
        PhysicalDriverKind::OfflineVerifierBoundary
    );
    assert!(verifier_driver
        .profile()
        .supported_faults()
        .contains(&DriverFaultClass::Corruption));
    assert!(verifier_driver
        .profile()
        .evidence_surfaces()
        .contains(&DriverEvidenceSurface::OfflineVerifierTrace));
}

#[test]
fn equivalent_driver_contracts_preserve_plan_identity() {
    let first = lower_physical_simulation_plan(physical_isolation_scenario(), complete_context()).unwrap();
    let second = lower_physical_simulation_plan(physical_isolation_scenario(), complete_context()).unwrap();

    assert_eq!(first.identity(), second.identity());
    assert_eq!(first.yieldpoint_binding(), second.yieldpoint_binding());
}

#[test]
fn actor_contracts_admit_concrete_production_facing_roles() {
    let actors = [
        ForegroundReadActor::admit("reader").unwrap().actor().role(),
        ForegroundWriteActor::admit("writer")
            .unwrap()
            .actor()
            .role(),
        CheckpointActor::admit("checkpoint").unwrap().actor().role(),
        CompactionActor::admit("compaction").unwrap().actor().role(),
        RecoveryActor::admit("recovery").unwrap().actor().role(),
        ReclaimActor::admit("reclaim").unwrap().actor().role(),
        ScrubActor::admit("scrub").unwrap().actor().role(),
        OfflineVerifierActor::admit("verifier")
            .unwrap()
            .actor()
            .role(),
    ];

    assert_eq!(
        actors,
        [
            PhysicalScenarioActorRole::ForegroundReader,
            PhysicalScenarioActorRole::ForegroundWriter,
            PhysicalScenarioActorRole::CheckpointDriver,
            PhysicalScenarioActorRole::CompactionDriver,
            PhysicalScenarioActorRole::RecoveryDriver,
            PhysicalScenarioActorRole::MaintenanceReclaimer,
            PhysicalScenarioActorRole::ScrubDriver,
            PhysicalScenarioActorRole::OfflineVerifier,
        ]
    );
    assert_eq!(
        PhysicalSimulationActor::future_extension_slot("future").unwrap_err(),
        PhysicalSimulationActorAdmissionDenial::FutureExtensionActorCannotExecute
    );
}

fn complete_context() -> SimulationPlanningContext {
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

fn physical_isolation_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s5.driver.contract")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("driver", 7)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .unwrap()
}
