use forge_store_io_scheduler::foreground_reservation::ForegroundIoLaneKind;
use forge_store_io_scheduler::BackgroundIoPressureClass;
use forge_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};

use crate::s6_io_pressure_test_support::{
    fault_phase_for, replay_bundle_for, s6_oracle_denial_without_pressure_observation,
};
use crate::{
    all_s6_fault_evidence_classes, all_s6_io_pressure_fault_kinds, CoverageRowDimension,
    CoverageSurfaceKind, OracleDenial, PhysicalFaultEvidenceClass, PhysicalSimulationProfile,
    S6BackendSafetyQualificationDenial, S6HarnessSecureIoPosture, S6IoPressureFaultKind,
    S6IoPressureHarnessEvidence, S6IoPressureHarnessScenario, S6PressureEvidenceMaturity,
};

#[test]
fn replay_bundle_admits_s6_pressure_evidence_and_exact_coverage_rows() {
    let scenario = S6IoPressureHarnessScenario::deterministic_read_under_repair_pressure();
    let replay = replay_bundle_for(scenario.clone(), PhysicalSimulationProfile::DeveloperSmoke);

    let evidence = S6IoPressureHarnessEvidence::from_replay_bundle(scenario, &replay).unwrap();
    let coverage = evidence.executed_replay_coverage_rows();

    assert_ne!(evidence.replay_identity(), &[0; 32]);
    assert_eq!(coverage.replay_identity(), evidence.replay_identity());
    assert_eq!(
        evidence.replay_profile(),
        PhysicalSimulationProfile::DeveloperSmoke
    );
    assert_eq!(coverage.rows().len(), 6);
    assert_eq!(
        coverage.iter().map(|row| row.surface()).collect::<Vec<_>>(),
        vec![
            CoverageSurfaceKind::Scenario,
            CoverageSurfaceKind::Actor,
            CoverageSurfaceKind::Driver,
            CoverageSurfaceKind::Counter,
            CoverageSurfaceKind::Oracle,
            CoverageSurfaceKind::Transcript,
        ]
    );
    for row in coverage.iter() {
        assert_eq!(row.source_identity(), evidence.replay_identity());
        assert_eq!(row.dimensions(), expected_phase10_dimensions());
    }
}

#[test]
fn backend_safety_qualification_distinguishes_all_fault_evidence_classes() {
    let denials = [
        (
            PhysicalFaultEvidenceClass::Simulated,
            S6BackendSafetyQualificationDenial::SimulatedBackendSuccess,
        ),
        (
            PhysicalFaultEvidenceClass::InjectedProductionBoundary,
            S6BackendSafetyQualificationDenial::InjectedBoundaryOnly,
        ),
        (
            PhysicalFaultEvidenceClass::BackendEmulated,
            S6BackendSafetyQualificationDenial::BackendEmulatedOnly,
        ),
        (
            PhysicalFaultEvidenceClass::ObservedHost,
            S6BackendSafetyQualificationDenial::ObservedHostOnly,
        ),
    ];
    for (fault_class, expected_denial) in denials {
        let evidence = evidence_for_fault_class(fault_class);

        assert_eq!(
            evidence.require_real_backend_safety().unwrap_err(),
            expected_denial
        );
    }
    for fault_class in [
        PhysicalFaultEvidenceClass::CertifiedBackend,
        PhysicalFaultEvidenceClass::ExternallyGuaranteed,
    ] {
        let evidence = evidence_for_fault_class(fault_class);
        let qualification = evidence.require_real_backend_safety().unwrap();

        assert_eq!(
            qualification.backend_profile(),
            BackendTargetProfile::PosixFileFsyncDirSync
        );
        assert_eq!(qualification.evidence_class(), fault_class);
    }
}

#[test]
fn weak_backend_evidence_class_cannot_qualify_real_backend_safety() {
    for backend_evidence_class in [
        CapabilityEvidenceClass::DeclaredByConfig,
        CapabilityEvidenceClass::ObservedByProbe,
        CapabilityEvidenceClass::ExternallyGuaranteed,
        CapabilityEvidenceClass::UnverifiableAssumption,
    ] {
        let scenario = S6IoPressureHarnessScenario::deterministic_read_under_repair_pressure()
            .with_fault_evidence_class(PhysicalFaultEvidenceClass::CertifiedBackend)
            .with_backend_evidence_class(backend_evidence_class);
        let replay = replay_bundle_for(scenario.clone(), PhysicalSimulationProfile::DeveloperSmoke);
        let evidence = S6IoPressureHarnessEvidence::from_replay_bundle(scenario, &replay).unwrap();

        assert_eq!(
            evidence.require_real_backend_safety().unwrap_err(),
            S6BackendSafetyQualificationDenial::BackendEvidenceClassTooWeak {
                required: CapabilityEvidenceClass::CertifiedBackendProfile,
                actual: backend_evidence_class,
            }
        );
    }
}

#[test]
fn all_pressure_faults_require_io_pressure_fault_delivery() {
    for fault_kind in all_s6_io_pressure_fault_kinds() {
        let scenario = S6IoPressureHarnessScenario::deterministic_read_under_repair_pressure()
            .with_fault_kind(fault_kind);
        let replay = replay_bundle_for(scenario.clone(), PhysicalSimulationProfile::DeveloperSmoke);
        let evidence =
            S6IoPressureHarnessEvidence::from_replay_bundle(scenario.clone(), &replay).unwrap();

        assert_eq!(evidence.fault_phase(), fault_phase_for(fault_kind));
        assert!(
            evidence
                .executed_replay_coverage_rows()
                .iter()
                .all(|row| row
                    .has_dimension(&CoverageRowDimension::S6IoPressureFaultKind(fault_kind)))
        );
    }
}

#[test]
fn missing_pressure_observation_denies_s6_oracle_and_evidence() {
    let scenario = S6IoPressureHarnessScenario::deterministic_read_under_repair_pressure();

    let denial = s6_oracle_denial_without_pressure_observation(scenario);

    assert_eq!(denial, OracleDenial::MissingS6IoPressureObservation);
}

#[test]
fn deterministic_and_large_profiles_preserve_s6_evidence_topology() {
    let deterministic = S6IoPressureHarnessScenario::deterministic_read_under_repair_pressure();
    let large = deterministic
        .clone()
        .with_foreground_lane(ForegroundIoLaneKind::CommitCriticalWalWrite)
        .with_background_pressure(BackgroundIoPressureClass::CheckpointFlush)
        .with_fault_kind(S6IoPressureFaultKind::DelayedSync);

    let deterministic_replay = replay_bundle_for(
        deterministic.clone(),
        PhysicalSimulationProfile::DeveloperSmoke,
    );
    let large_replay = replay_bundle_for(large.clone(), PhysicalSimulationProfile::CiCertification);
    let deterministic_evidence =
        S6IoPressureHarnessEvidence::from_replay_bundle(deterministic, &deterministic_replay)
            .unwrap();
    let large_evidence =
        S6IoPressureHarnessEvidence::from_replay_bundle(large.clone(), &large_replay).unwrap();

    assert_eq!(
        deterministic_replay.plan().scenario_family(),
        large_replay.plan().scenario_family()
    );
    assert_eq!(
        deterministic_replay.plan().actors(),
        large_replay.plan().actors()
    );
    assert_eq!(
        deterministic_replay.plan().drivers(),
        large_replay.plan().drivers()
    );
    assert_eq!(
        deterministic_replay.plan().oracle_families(),
        large_replay.plan().oracle_families()
    );
    assert_eq!(
        deterministic_replay.plan().counter_contracts(),
        large_replay.plan().counter_contracts()
    );
    assert_eq!(
        deterministic_replay
            .schedule()
            .actor_steps()
            .iter()
            .map(|step| step.actor_role())
            .collect::<Vec<_>>(),
        large_replay
            .schedule()
            .actor_steps()
            .iter()
            .map(|step| step.actor_role())
            .collect::<Vec<_>>()
    );
    for row in large_evidence.executed_replay_coverage_rows().iter() {
        assert!(
            row.has_dimension(&CoverageRowDimension::ResourceEnvelopeProfile(
                PhysicalSimulationProfile::CiCertification
            ))
        );
        assert!(row.has_dimension(&CoverageRowDimension::S6ForegroundLane(
            ForegroundIoLaneKind::CommitCriticalWalWrite
        )));
        assert!(
            row.has_dimension(&CoverageRowDimension::S6BackgroundPressure(
                BackgroundIoPressureClass::CheckpointFlush
            ))
        );
        assert!(
            row.has_dimension(&CoverageRowDimension::S6IoPressureFaultKind(
                S6IoPressureFaultKind::DelayedSync
            ))
        );
    }
    assert_ne!(
        deterministic_evidence
            .executed_replay_coverage_rows()
            .rows(),
        large_evidence.executed_replay_coverage_rows().rows()
    );
}

#[test]
fn all_fault_evidence_classes_are_explicit() {
    assert_eq!(all_s6_fault_evidence_classes().len(), 6);
    assert!(all_s6_fault_evidence_classes().contains(&PhysicalFaultEvidenceClass::BackendEmulated));
}

fn evidence_for_fault_class(
    fault_class: PhysicalFaultEvidenceClass,
) -> S6IoPressureHarnessEvidence {
    let scenario = S6IoPressureHarnessScenario::deterministic_read_under_repair_pressure()
        .with_fault_evidence_class(fault_class);
    let replay = replay_bundle_for(scenario.clone(), PhysicalSimulationProfile::DeveloperSmoke);
    S6IoPressureHarnessEvidence::from_replay_bundle(scenario, &replay).unwrap()
}

fn expected_phase10_dimensions() -> &'static [CoverageRowDimension] {
    &[
        CoverageRowDimension::ResourceEnvelopeProfile(PhysicalSimulationProfile::DeveloperSmoke),
        CoverageRowDimension::BackgroundInterference(crate::PhysicalDriverKind::IoPressureBoundary),
        CoverageRowDimension::FaultPhase(
            crate::PhysicalScenarioFaultKind::S6BackendLatencyInjection,
        ),
        CoverageRowDimension::S6BackendTarget(BackendTargetProfile::PosixFileFsyncDirSync),
        CoverageRowDimension::S6ForegroundLane(ForegroundIoLaneKind::PointRead),
        CoverageRowDimension::S6BackgroundPressure(BackgroundIoPressureClass::RepairScan),
        CoverageRowDimension::S6SecureIoPosture(S6HarnessSecureIoPosture::ScopePreserving),
        CoverageRowDimension::S6IoPressureFaultKind(S6IoPressureFaultKind::BackendLatencyInjection),
        CoverageRowDimension::S6FaultEvidenceClass(
            PhysicalFaultEvidenceClass::InjectedProductionBoundary,
        ),
        CoverageRowDimension::S6EvidenceMaturity(
            S6PressureEvidenceMaturity::ProductionBoundaryInjected,
        ),
    ]
}
