use worth_store_test_support::harness::recovery::closeout as fixture;

use worth_store_physical_certification::{
    admit_simulation_harness_entry, ExistingSimulationHarnessInventory,
    RegisteredSimulationHarnessSurface, SimulationHarnessBoundaryDenial,
    SimulationHarnessRoadmapRequirement, SimulationHarnessRoadmapRequirementSet,
    SimulationHarnessSurfaceClassification,
};

#[test]
fn simulation_harness_entry_identity_is_stable_across_independent_recovery_execution() {
    let direct_recovery = fixture::executed_recovery_receipt();
    let repeated_recovery = fixture::executed_recovery_receipt();

    let direct_entry = admit_simulation_harness_entry(
        &direct_recovery,
        SimulationHarnessRoadmapRequirementSet::certification_required(),
        ExistingSimulationHarnessInventory::dedicated_workspace_baseline(),
    )
    .expect("direct closeout evidence admits");
    let repeated_entry = admit_simulation_harness_entry(
        &repeated_recovery,
        SimulationHarnessRoadmapRequirementSet::from_requirements(
            scrambled_duplicate_requirements_from_spec(),
        ),
        ExistingSimulationHarnessInventory::from_registered_surfaces(
            scrambled_registered_surfaces(),
        ),
    )
    .expect("repeated recovery evidence admits");

    assert_eq!(direct_entry.identity(), repeated_entry.identity());
    assert_eq!(
        direct_entry.roadmap_requirements().requirements(),
        required_roadmap_requirements_from_spec()
    );
    assert_eq!(
        repeated_entry.roadmap_requirements().requirements(),
        required_roadmap_requirements_from_spec()
    );
}

#[test]
fn simulation_harness_entry_identity_changes_when_recovery_recovered_outcome_changes() {
    let first_recovery = fixture::executed_recovery_receipt();
    let second_recovery =
        fixture::executed_recovery_receipt_with_operation_digest("alternate-operation");

    let first_entry = admit_simulation_harness_entry(
        &first_recovery,
        SimulationHarnessRoadmapRequirementSet::certification_required(),
        ExistingSimulationHarnessInventory::dedicated_workspace_baseline(),
    )
    .expect("first closeout admits");
    let second_entry = admit_simulation_harness_entry(
        &second_recovery,
        SimulationHarnessRoadmapRequirementSet::certification_required(),
        ExistingSimulationHarnessInventory::dedicated_workspace_baseline(),
    )
    .expect("second closeout admits");

    assert_ne!(first_entry.identity(), second_entry.identity());
    assert_ne!(
        first_entry.identity().recovered_root(),
        second_entry.identity().recovered_root()
    );
}

#[test]
fn simulation_harness_entry_rejects_each_missing_roadmap_requirement() {
    let recovery = fixture::executed_recovery_receipt();

    assert_eq!(
        SimulationHarnessRoadmapRequirementSet::certification_required().requirements(),
        required_roadmap_requirements_from_spec()
    );

    for missing_requirement in required_roadmap_requirements_from_spec() {
        let requirements = SimulationHarnessRoadmapRequirementSet::from_requirements(
            required_roadmap_requirements_from_spec()
                .iter()
                .copied()
                .filter(|requirement| requirement != missing_requirement)
                .collect(),
        );

        let denial = admit_simulation_harness_entry(
            &recovery,
            requirements,
            ExistingSimulationHarnessInventory::dedicated_workspace_baseline(),
        )
        .expect_err("each missing Roadmap 2 requirement blocks entry");

        assert_eq!(
            denial,
            SimulationHarnessBoundaryDenial::MissingRoadmapHarnessRequirement(*missing_requirement)
        );
    }
}

#[test]
fn simulation_harness_inventory_classifies_every_registered_surface_exactly() {
    let inventory = ExistingSimulationHarnessInventory::dedicated_workspace_baseline();

    for (surface, classification) in registered_surface_classifications_from_spec() {
        assert_eq!(&surface.classification(), classification);
        assert!(inventory.surfaces().iter().any(|registered_surface| {
            registered_surface.path() == surface.path()
                && &registered_surface.classification() == classification
        }));
    }
}

#[test]
fn simulation_harness_inventory_denies_missing_reusable_mechanics_surface() {
    let recovery = fixture::executed_recovery_receipt();
    let inventory = ExistingSimulationHarnessInventory::from_registered_surfaces(vec![
        RegisteredSimulationHarnessSurface::TestSupportTerminalProjectionJsonFixtures,
        RegisteredSimulationHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
        RegisteredSimulationHarnessSurface::CertificationS4RecoveryHarness,
        RegisteredSimulationHarnessSurface::ObsoleteSemanticHarness,
    ]);

    let denial = admit_simulation_harness_entry(
        &recovery,
        SimulationHarnessRoadmapRequirementSet::certification_required(),
        inventory,
    )
    .expect_err("missing registered reusable mechanics surface cannot admit");

    assert_eq!(
        denial,
        SimulationHarnessBoundaryDenial::MissingReusableMechanicsInventory
    );
}

fn required_roadmap_requirements_from_spec() -> &'static [SimulationHarnessRoadmapRequirement] {
    &[
        SimulationHarnessRoadmapRequirement::GoldenPathAuthoringApi,
        SimulationHarnessRoadmapRequirement::AspectNativeScenarioDefinitions,
        SimulationHarnessRoadmapRequirement::DeterministicScheduler,
        SimulationHarnessRoadmapRequirement::NamedProductionBoundaryYieldpoints,
        SimulationHarnessRoadmapRequirement::ProductionFacingDriverContracts,
        SimulationHarnessRoadmapRequirement::ActorFaultCrashVocabulary,
        SimulationHarnessRoadmapRequirement::ObserverOracleSeparation,
        SimulationHarnessRoadmapRequirement::CertificationOwnedOracleFamilies,
        SimulationHarnessRoadmapRequirement::CounterStrengthContracts,
        SimulationHarnessRoadmapRequirement::ProductionBackedFixtureManifests,
        SimulationHarnessRoadmapRequirement::ReplayableTranscriptsAndEvidence,
        SimulationHarnessRoadmapRequirement::GeneratedCoverageMatrix,
        SimulationHarnessRoadmapRequirement::HarnessMaturityLadder,
        SimulationHarnessRoadmapRequirement::ForbiddenShortcutRejection,
        SimulationHarnessRoadmapRequirement::RecoveryDogfoodSlice,
        SimulationHarnessRoadmapRequirement::PhysicalIsolationReadinessShapeProbeNonClaim,
        SimulationHarnessRoadmapRequirement::FutureExtensionSlotContainment,
        SimulationHarnessRoadmapRequirement::MutationStyleHarnessValidation,
    ]
}

fn scrambled_duplicate_requirements_from_spec() -> Vec<SimulationHarnessRoadmapRequirement> {
    let mut requirements = required_roadmap_requirements_from_spec().to_vec();
    requirements.reverse();
    requirements.push(SimulationHarnessRoadmapRequirement::GoldenPathAuthoringApi);
    requirements
}

fn scrambled_registered_surfaces() -> Vec<RegisteredSimulationHarnessSurface> {
    vec![
        RegisteredSimulationHarnessSurface::ObsoleteSemanticHarness,
        RegisteredSimulationHarnessSurface::CertificationS4RecoveryHarness,
        RegisteredSimulationHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
        RegisteredSimulationHarnessSurface::TestSupportTerminalProjectionJsonFixtures,
        RegisteredSimulationHarnessSurface::TestSupportNativeAspectFixtures,
        RegisteredSimulationHarnessSurface::TestSupportS4RecoveryPhysics,
    ]
}

fn registered_surface_classifications_from_spec() -> &'static [(
    RegisteredSimulationHarnessSurface,
    SimulationHarnessSurfaceClassification,
)] {
    &[
        (
            RegisteredSimulationHarnessSurface::TestSupportS4RecoveryPhysics,
            SimulationHarnessSurfaceClassification::ReusableMechanics,
        ),
        (
            RegisteredSimulationHarnessSurface::TestSupportNativeAspectFixtures,
            SimulationHarnessSurfaceClassification::ReusableMechanics,
        ),
        (
            RegisteredSimulationHarnessSurface::TestSupportTerminalProjectionJsonFixtures,
            SimulationHarnessSurfaceClassification::MilestoneLocalMechanics,
        ),
        (
            RegisteredSimulationHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
            SimulationHarnessSurfaceClassification::MilestoneLocalMechanics,
        ),
        (
            RegisteredSimulationHarnessSurface::CertificationS4RecoveryHarness,
            SimulationHarnessSurfaceClassification::CertificationMeaning,
        ),
        (
            RegisteredSimulationHarnessSurface::ObsoleteSemanticHarness,
            SimulationHarnessSurfaceClassification::ObsoleteSemanticContext,
        ),
    ]
}
