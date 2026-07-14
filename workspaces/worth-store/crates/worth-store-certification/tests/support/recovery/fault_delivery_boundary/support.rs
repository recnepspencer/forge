#[path = "../recovery_offline_verifier/runtime_recovery_fixture.rs"]
mod runtime_recovery_fixture;

use worth_store_physical_backend::ProductionStorageBoundarySeam;
use worth_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, AdmittedDriverContractSet,
    ExpectedFaultLocalization, FaultDeliveryDenial, ForbiddenShortcutSet,
    FreshRuntimeCrashRecoveryEvidence, ObservedFaultBoundary, PhysicalArtifactKind,
    PhysicalBoundarySeam, PhysicalBoundaryYieldpoint, PhysicalDriverKind, PhysicalFaultEvent,
    PhysicalFaultEventKind, PhysicalFaultFieldKind, PhysicalScenarioActor,
    PhysicalScenarioExpectation, PhysicalScenarioIntent, PhysicalScenarioSchedule,
    PhysicalSimulationCapabilitySet, PhysicalSimulationDriver, PhysicalSimulationProfile,
    PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily,
    ProductionStorageBoundaryDriver, SimulationEvidencePolicy, SimulationPlanningContext,
    SupportedObserverSet, SupportedOracleFamilySet,
};
use worth_store_recovery_physics::{
    FreshRuntimeRecoveryDriver, RecoveryOfflineVerifier, RecoveryProfileId,
    RecoveryRuntimeClassification, RuntimeRecoveryReport,
};
use worth_store_test_support::harness::test_authority::{
    io_pressure_fault_locus, observed_checksum_mismatch_boundary, observed_io_pressure_boundary,
    observed_torn_frame_boundary, page_generation_fault_locus, wal_frame_payload_fault_locus,
};
use worth_store_test_support::{
    admitted_developer_smoke_driver_contracts, deterministic_recovery_artifacts,
    NativeStoreAspectFixture,
};

#[derive(Clone)]
pub struct StorageFaultDeliveryCase {
    pub event_kind: PhysicalFaultEventKind,
    event: PhysicalFaultEvent,
    pub yieldpoint: PhysicalBoundaryYieldpoint,
    pub expected_seam: PhysicalBoundarySeam,
    pub expected_localization: ExpectedFaultLocalization,
    pub actual_boundary: ObservedFaultBoundary,
    pub artifact_kind: PhysicalArtifactKind,
    pub field_kind: PhysicalFaultFieldKind,
    pub requires_offset: bool,
}

pub fn deliver_storage_event(
    case: &StorageFaultDeliveryCase,
) -> Result<worth_store_physical_certification::FaultDeliveryReceipt, FaultDeliveryDenial> {
    let plan = lower_physical_simulation_plan(
        scenario_for_yieldpoint(case.yieldpoint.name()),
        context_for_yieldpoint(case.yieldpoint.clone()),
    )
    .unwrap();
    case.event.clone().deliver_through(
        plan.yieldpoint_binding(),
        case.yieldpoint.clone(),
        case.actual_boundary.clone(),
    )
}

pub fn storage_fault_delivery_cases() -> Vec<StorageFaultDeliveryCase> {
    vec![
        storage_case(
            PhysicalFaultEvent::torn_write(
                ProductionStorageBoundarySeam::WalAppendBeforeFlush,
                page_generation_fault_locus(),
            )
            .unwrap(),
            PhysicalFaultEventKind::TornWrite,
            ProductionStorageBoundarySeam::WalAppendBeforeFlush,
            ExpectedFaultLocalization::PhysicalIntegrityBoundary,
            observed_torn_frame_boundary(),
            PhysicalArtifactKind::PageImage,
            PhysicalFaultFieldKind::GenerationField,
        ),
        storage_case(
            PhysicalFaultEvent::dropped_flush(
                ProductionStorageBoundarySeam::WalFlush,
                wal_frame_payload_fault_locus(),
            )
            .unwrap(),
            PhysicalFaultEventKind::DroppedFlush,
            ProductionStorageBoundarySeam::WalFlush,
            ExpectedFaultLocalization::PreDecodeBoundary,
            observed_checksum_mismatch_boundary(),
            PhysicalArtifactKind::WalFrame,
            PhysicalFaultFieldKind::ChecksumProtectedPayload,
        ),
        storage_case(
            PhysicalFaultEvent::reordered_persistence(
                ProductionStorageBoundarySeam::RootSwap,
                wal_frame_payload_fault_locus(),
            )
            .unwrap(),
            PhysicalFaultEventKind::ReorderedPersistence,
            ProductionStorageBoundarySeam::RootSwap,
            ExpectedFaultLocalization::PreDecodeBoundary,
            observed_checksum_mismatch_boundary(),
            PhysicalArtifactKind::WalFrame,
            PhysicalFaultFieldKind::ChecksumProtectedPayload,
        ),
        storage_case(
            PhysicalFaultEvent::byte_corruption(
                ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
                wal_frame_payload_fault_locus(),
            )
            .unwrap(),
            PhysicalFaultEventKind::ByteCorruption,
            ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
            ExpectedFaultLocalization::PreDecodeBoundary,
            observed_checksum_mismatch_boundary(),
            PhysicalArtifactKind::WalFrame,
            PhysicalFaultFieldKind::ChecksumProtectedPayload,
        ),
        storage_case(
            PhysicalFaultEvent::stale_generation(
                ProductionStorageBoundarySeam::PagePin,
                page_generation_fault_locus(),
            )
            .unwrap(),
            PhysicalFaultEventKind::StaleGeneration,
            ProductionStorageBoundarySeam::PagePin,
            ExpectedFaultLocalization::PhysicalIntegrityBoundary,
            observed_torn_frame_boundary(),
            PhysicalArtifactKind::PageImage,
            PhysicalFaultFieldKind::GenerationField,
        ),
        storage_case(
            PhysicalFaultEvent::delayed_release(
                ProductionStorageBoundarySeam::LeasePublish,
                wal_frame_payload_fault_locus(),
            )
            .unwrap(),
            PhysicalFaultEventKind::DelayedRelease,
            ProductionStorageBoundarySeam::LeasePublish,
            ExpectedFaultLocalization::PreDecodeBoundary,
            observed_checksum_mismatch_boundary(),
            PhysicalArtifactKind::WalFrame,
            PhysicalFaultFieldKind::ChecksumProtectedPayload,
        ),
        storage_case(
            PhysicalFaultEvent::blocked_reclaim(
                ProductionStorageBoundarySeam::ReclaimEligibility,
                wal_frame_payload_fault_locus(),
            )
            .unwrap(),
            PhysicalFaultEventKind::BlockedReclaim,
            ProductionStorageBoundarySeam::ReclaimEligibility,
            ExpectedFaultLocalization::PreDecodeBoundary,
            observed_checksum_mismatch_boundary(),
            PhysicalArtifactKind::WalFrame,
            PhysicalFaultFieldKind::ChecksumProtectedPayload,
        ),
        io_stall_case(),
    ]
}

pub fn readiness_scenario() -> worth_store_physical_certification::CertifiedPhysicalScenario {
    readiness_scenario_scheduled_at("root-publication-before-observe")
}

pub fn readiness_scenario_scheduled_at(
    yieldpoint: &str,
) -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase6.fault.delivery")
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
    physical_scenario("store.physical.s45.phase6.crash.delivery")
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

pub fn fresh_runtime_crash_evidence() -> FreshRuntimeCrashRecoveryEvidence {
    let artifacts = deterministic_recovery_artifacts();
    let verifier = RecoveryOfflineVerifier::for_profile(
        "s4-format-v1",
        "strict-posix-fsync-dir-fsync",
        RecoveryProfileId::strict_offline_recovery_artifacts(),
    );
    let offline = verifier.verify_persisted_artifacts(&artifacts).unwrap();
    let (receipt, execution) =
        runtime_recovery_fixture::execute_reopened_recovery_fixture(&offline, &artifacts).unwrap();
    let evidence = verifier.verify_fresh_runtime_reopen(&artifacts).unwrap();
    let witness = FreshRuntimeRecoveryDriver::from_reopen_harness_evidence(evidence)
        .witness_from_reopened_execution(execution)
        .unwrap();
    let runtime_report = RuntimeRecoveryReport::from_verified_bounded_recovery(
        witness,
        &offline,
        RecoveryRuntimeClassification::Recovered,
        &receipt,
        Vec::new(),
    )
    .unwrap();

    FreshRuntimeCrashRecoveryEvidence::from_runtime_report(receipt, runtime_report).unwrap()
}

pub fn developer_smoke_production_trace(
) -> worth_store_physical_certification::ProductionBoundaryDriverTrace {
    admitted_developer_smoke_driver_contracts()
        .unwrap()
        .iter()
        .find(|driver| driver.kind() == PhysicalDriverKind::ProductionBoundaryYieldpoint)
        .and_then(|driver| driver.production_boundary_trace())
        .unwrap()
}

pub fn alternate_production_trace(
) -> worth_store_physical_certification::ProductionBoundaryDriverTrace {
    production_driver_with_profile(
        worth_store_physical_backend::BackendDurabilityProfileId::WindowsFlushFileBuffers,
    )
    .production_boundary_trace()
    .unwrap()
}

pub fn production_driver_with_all_seams() -> PhysicalSimulationDriver {
    production_driver_with_profile(
        worth_store_physical_backend::BackendDurabilityProfileId::PosixFileFsyncDirFsync,
    )
}

fn storage_case(
    event: PhysicalFaultEvent,
    event_kind: PhysicalFaultEventKind,
    seam: ProductionStorageBoundarySeam,
    expected_localization: ExpectedFaultLocalization,
    actual_boundary: ObservedFaultBoundary,
    artifact_kind: PhysicalArtifactKind,
    field_kind: PhysicalFaultFieldKind,
) -> StorageFaultDeliveryCase {
    StorageFaultDeliveryCase {
        event_kind,
        event,
        yieldpoint: PhysicalBoundaryYieldpoint::production_storage(seam),
        expected_seam: PhysicalBoundarySeam::ProductionStorage(seam),
        expected_localization,
        actual_boundary,
        artifact_kind,
        field_kind,
        requires_offset: true,
    }
}

fn io_stall_case() -> StorageFaultDeliveryCase {
    StorageFaultDeliveryCase {
        event_kind: PhysicalFaultEventKind::IoStall,
        event: PhysicalFaultEvent::io_stall(io_pressure_fault_locus()).unwrap(),
        yieldpoint: PhysicalBoundaryYieldpoint::io_pressure_boundary(),
        expected_seam: PhysicalBoundarySeam::IoPressure,
        expected_localization: ExpectedFaultLocalization::ProductionDriverBoundary,
        actual_boundary: observed_io_pressure_boundary(),
        artifact_kind: PhysicalArtifactKind::PageImage,
        field_kind: PhysicalFaultFieldKind::SlotState,
        requires_offset: true,
    }
}

fn scenario_for_yieldpoint(
    yieldpoint: &str,
) -> worth_store_physical_certification::CertifiedPhysicalScenario {
    if yieldpoint == "io-pressure-boundary" {
        io_pressure_scenario()
    } else {
        readiness_scenario_scheduled_at(yieldpoint)
    }
}

fn io_pressure_scenario() -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase6.io-stall.delivery")
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

fn context_for_yieldpoint(yieldpoint: PhysicalBoundaryYieldpoint) -> SimulationPlanningContext {
    let context = complete_context();
    if yieldpoint.seam() == PhysicalBoundarySeam::IoPressure {
        return context.with_driver_contracts(
            AdmittedDriverContractSet::from_drivers(
                admitted_developer_smoke_driver_contracts()
                    .unwrap()
                    .iter()
                    .filter(|driver| {
                        driver.kind() != PhysicalDriverKind::ProductionBoundaryYieldpoint
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

fn production_driver_with_profile(
    profile: worth_store_physical_backend::BackendDurabilityProfileId,
) -> PhysicalSimulationDriver {
    let mut driver = ProductionStorageBoundaryDriver::for_backend_profile(profile);
    for seam in ProductionStorageBoundarySeam::registered_backend_operation_seams() {
        driver = driver.declare_yieldpoint(PhysicalBoundaryYieldpoint::production_storage(*seam));
    }
    driver.admit().unwrap()
}
