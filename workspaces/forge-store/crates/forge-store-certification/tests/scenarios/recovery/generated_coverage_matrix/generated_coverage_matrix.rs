#[path = "../../../support/recovery/coverage_support/coverage_support.rs"]
mod coverage_support;

use forge_store_physical_certification::{
    CounterContractKind, CoverageRowDimension, CoverageSurfaceKind, FixtureClassKind,
    GeneratedCoverageMatrix, HarnessCoverageStage, HarnessMaturityLevel, HarnessSubsystem,
    MutationValidationPosture, ObserverKind, OracleFamilyKind, PhysicalDriverKind,
    PhysicalIsolationHarnessMaturityDependencyEvidence, PhysicalIsolationReadinessDependencySet,
    PhysicalIsolationCorrectnessNonClaimEvidence, PhysicalIsolationHarnessMaturityDependency,
    PhysicalProofOracleKind, PhysicalScenarioActorRole, PhysicalScenarioFaultKind,
    PhysicalSimulationProfile,
};

#[test]
fn coverage_matrix_is_generated_from_registered_execution_surfaces() {
    let plan = coverage_support::lowered_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let matrix = coverage_support::complete_registry(&plan, &replay)
        .generate_matrix()
        .unwrap();

    assert_eq!(matrix.sequence(), HarnessCoverageStage::SimulationAdmission);
    assert_surface(&matrix, CoverageSurfaceKind::Scenario);
    assert_surface(&matrix, CoverageSurfaceKind::Plan);
    assert_surface(&matrix, CoverageSurfaceKind::YieldpointSchedule);
    assert_surface(&matrix, CoverageSurfaceKind::Actor);
    assert_surface(&matrix, CoverageSurfaceKind::Driver);
    assert_surface(&matrix, CoverageSurfaceKind::Oracle);
    assert_surface(&matrix, CoverageSurfaceKind::Counter);
    assert_surface(&matrix, CoverageSurfaceKind::Transcript);
    assert_surface(&matrix, CoverageSurfaceKind::MutationResult);
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::Plan,
        CoverageRowDimension::ArtifactClass(FixtureClassKind::AspectNativeBoundaryFact),
    );
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::Actor,
        CoverageRowDimension::ActorRole(PhysicalScenarioActorRole::ForegroundReader),
    );
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::Actor,
        CoverageRowDimension::ActorRole(PhysicalScenarioActorRole::MaintenanceReclaimer),
    );
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::YieldpointSchedule,
        CoverageRowDimension::ProductionBoundaryYieldpoint(
            "root-publication-before-observe".to_string(),
        ),
    );
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::Scenario,
        CoverageRowDimension::FaultPhase(PhysicalScenarioFaultKind::NoFault),
    );
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::Plan,
        CoverageRowDimension::ResourceEnvelopeProfile(PhysicalSimulationProfile::DeveloperSmoke),
    );
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::Driver,
        CoverageRowDimension::BackgroundInterference(
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
        ),
    );
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::Oracle,
        CoverageRowDimension::AuthorityFamily(OracleFamilyKind::PhysicalIsolationReadinessShape),
    );
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::Oracle,
        CoverageRowDimension::AuthorityFamily(OracleFamilyKind::TranscriptReplayEvidence),
    );
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::Oracle,
        CoverageRowDimension::Oracle(PhysicalProofOracleKind::CounterContract),
    );
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::Oracle,
        CoverageRowDimension::Oracle(PhysicalProofOracleKind::TranscriptReplay),
    );
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::Transcript,
        CoverageRowDimension::OfflineVerifier(ObserverKind::ShortcutRejectionObserver),
    );
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::Counter,
        CoverageRowDimension::CounterContract(CounterContractKind::ReplayIdentityExact),
    );
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::Transcript,
        CoverageRowDimension::TranscriptOutput,
    );
    assert_row_dimension(
        &matrix,
        CoverageSurfaceKind::MutationResult,
        CoverageRowDimension::MutationValidationPosture(
            MutationValidationPosture::ExpectedFailureObserved,
        ),
    );
}

#[test]
fn generated_maturity_maps_physical_isolation_ci_dependencies_without_physical_isolation_correctness_claim() {
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let matrix = coverage_support::complete_registry(&plan, &replay)
        .generate_matrix()
        .unwrap();
    let expected_dependencies = expected_dependency_evidence(&matrix);
    let maturity = matrix
        .derive_maturity()
        .require_subsystem_level(
            PhysicalIsolationReadinessDependencySet::required_for_ci(),
            HarnessMaturityLevel::CiCertifiable,
        )
        .unwrap();

    assert_eq!(
        maturity.level_for(HarnessSubsystem::ReplayableTranscripts),
        Some(HarnessMaturityLevel::CiCertifiable)
    );
    let dependency_evidence = maturity
        .physical_isolation_readiness_dependency_evidence()
        .unwrap();
    assert_eq!(
        dependency_pairs(&dependency_evidence),
        expected_dependencies,
        "S5 dependency evidence must point to exact generated coverage rows"
    );

    let readiness = maturity
        .admit_physical_isolation_simulation_harness_readiness(
            PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
        )
        .unwrap();

    assert_eq!(
        dependency_pairs(readiness.dependencies()),
        expected_dependencies,
        "readiness must preserve generated dependency evidence"
    );
    assert_eq!(
        readiness.non_claim(),
        PhysicalIsolationCorrectnessNonClaimEvidence::ShapeProbeOnly
    );
    assert!(readiness.does_not_claim_physical_isolation_correctness());
}

fn expected_dependency_evidence(
    matrix: &GeneratedCoverageMatrix,
) -> Vec<(PhysicalIsolationHarnessMaturityDependency, [u8; 32])> {
    PhysicalIsolationHarnessMaturityDependency::required_for_ci()
        .into_iter()
        .map(|dependency| {
            let row = matrix
                .rows()
                .iter()
                .find(|row| row.surface() == surface_for_dependency(dependency))
                .unwrap();
            (dependency, *row.source_identity())
        })
        .collect()
}

fn dependency_pairs(
    evidence: &[PhysicalIsolationHarnessMaturityDependencyEvidence],
) -> Vec<(PhysicalIsolationHarnessMaturityDependency, [u8; 32])> {
    evidence
        .iter()
        .map(|evidence| (evidence.dependency(), *evidence.coverage_row_digest()))
        .collect()
}

fn surface_for_dependency(
    dependency: PhysicalIsolationHarnessMaturityDependency,
) -> CoverageSurfaceKind {
    match dependency {
        PhysicalIsolationHarnessMaturityDependency::ScenarioDefinitions => {
            CoverageSurfaceKind::Scenario
        }
        PhysicalIsolationHarnessMaturityDependency::DeterministicScheduler => {
            CoverageSurfaceKind::YieldpointSchedule
        }
        PhysicalIsolationHarnessMaturityDependency::ActorModel => CoverageSurfaceKind::Actor,
        PhysicalIsolationHarnessMaturityDependency::ProductionDriverContracts => {
            CoverageSurfaceKind::Driver
        }
        PhysicalIsolationHarnessMaturityDependency::CertificationOracleFamilies => {
            CoverageSurfaceKind::Oracle
        }
        PhysicalIsolationHarnessMaturityDependency::CounterStrengthContracts => {
            CoverageSurfaceKind::Counter
        }
        PhysicalIsolationHarnessMaturityDependency::ReplayableTranscripts => {
            CoverageSurfaceKind::Transcript
        }
        PhysicalIsolationHarnessMaturityDependency::MutationValidation => {
            CoverageSurfaceKind::MutationResult
        }
    }
}

fn assert_row_dimension(
    matrix: &GeneratedCoverageMatrix,
    surface: CoverageSurfaceKind,
    dimension: CoverageRowDimension,
) {
    assert!(
        matrix
            .rows()
            .iter()
            .any(|row| row.surface() == surface && row.has_dimension(&dimension)),
        "missing generated coverage dimension {dimension:?} on {surface:?}"
    );
}

fn assert_surface(matrix: &GeneratedCoverageMatrix, surface: CoverageSurfaceKind) {
    assert!(
        matrix.rows().iter().any(|row| row.surface() == surface),
        "missing generated coverage surface {surface:?}"
    );
}
