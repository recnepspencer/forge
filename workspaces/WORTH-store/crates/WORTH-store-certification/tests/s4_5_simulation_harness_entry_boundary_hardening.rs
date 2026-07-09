#[path = "s4_closeout/fixture.rs"]
mod fixture;

use worth_store_physical_certification::{
    admit_s45_simulation_harness_entry, S45ExistingHarnessInventory, S45HarnessBoundaryDenial,
    S45HarnessSurfaceClassification, S45RegisteredHarnessSurface, S45RoadmapHarnessRequirement,
    S45RoadmapHarnessRequirementSet,
};

#[test]
fn s45_entry_identity_is_stable_across_reordered_closeout_evidence() {
    let direct_bundle = fixture::certify_complete_closeout();
    let reordered_bundle = fixture::certify_closeout_from_reordered_evidence();

    let direct_entry = admit_s45_simulation_harness_entry(
        &direct_bundle,
        S45RoadmapHarnessRequirementSet::roadmap2_required(),
        S45ExistingHarnessInventory::dedicated_workspace_baseline(),
    )
    .expect("direct closeout evidence admits");
    let reordered_entry = admit_s45_simulation_harness_entry(
        &reordered_bundle,
        S45RoadmapHarnessRequirementSet::from_requirements(
            scrambled_duplicate_requirements_from_spec(),
        ),
        S45ExistingHarnessInventory::from_registered_surfaces(scrambled_registered_surfaces()),
    )
    .expect("reordered closeout evidence admits");

    assert_eq!(direct_entry.identity(), reordered_entry.identity());
    assert_eq!(
        direct_entry.roadmap_requirements().requirements(),
        required_roadmap_requirements_from_spec()
    );
    assert_eq!(
        reordered_entry.roadmap_requirements().requirements(),
        required_roadmap_requirements_from_spec()
    );
}

#[test]
fn s45_entry_identity_changes_when_s4_recovered_outcome_changes() {
    let first_bundle = fixture::certify_complete_closeout();
    let second_bundle = fixture::certify_closeout_with_runtime_state_mismatch_artifacts();

    let first_entry = admit_s45_simulation_harness_entry(
        &first_bundle,
        S45RoadmapHarnessRequirementSet::roadmap2_required(),
        S45ExistingHarnessInventory::dedicated_workspace_baseline(),
    )
    .expect("first closeout admits");
    let second_entry = admit_s45_simulation_harness_entry(
        &second_bundle,
        S45RoadmapHarnessRequirementSet::roadmap2_required(),
        S45ExistingHarnessInventory::dedicated_workspace_baseline(),
    )
    .expect("second closeout admits");

    assert_ne!(first_entry.identity(), second_entry.identity());
    assert_ne!(
        first_entry.identity().recovered_root(),
        second_entry.identity().recovered_root()
    );
}

#[test]
fn s45_entry_rejects_each_missing_roadmap_requirement() {
    let bundle = fixture::certify_complete_closeout();

    assert_eq!(
        S45RoadmapHarnessRequirementSet::roadmap2_required().requirements(),
        required_roadmap_requirements_from_spec()
    );

    for missing_requirement in required_roadmap_requirements_from_spec() {
        let requirements = S45RoadmapHarnessRequirementSet::from_requirements(
            required_roadmap_requirements_from_spec()
                .iter()
                .copied()
                .filter(|requirement| requirement != missing_requirement)
                .collect(),
        );

        let denial = admit_s45_simulation_harness_entry(
            &bundle,
            requirements,
            S45ExistingHarnessInventory::dedicated_workspace_baseline(),
        )
        .expect_err("each missing Roadmap 2 requirement blocks entry");

        assert_eq!(
            denial,
            S45HarnessBoundaryDenial::MissingRoadmapHarnessRequirement(*missing_requirement)
        );
    }
}

#[test]
fn s45_inventory_classifies_every_registered_surface_exactly() {
    let inventory = S45ExistingHarnessInventory::dedicated_workspace_baseline();

    for (surface, classification) in registered_surface_classifications_from_spec() {
        assert_eq!(&surface.classification(), classification);
        assert!(inventory.surfaces().iter().any(|registered_surface| {
            registered_surface.path() == surface.path()
                && &registered_surface.classification() == classification
        }));
    }
}

#[test]
fn s45_inventory_denies_missing_reusable_mechanics_surface() {
    let bundle = fixture::certify_complete_closeout();
    let inventory = S45ExistingHarnessInventory::from_registered_surfaces(vec![
        S45RegisteredHarnessSurface::TestSupportTerminalProjectionJsonFixtures,
        S45RegisteredHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
        S45RegisteredHarnessSurface::CertificationS4RecoveryHarness,
        S45RegisteredHarnessSurface::ObsoleteSemanticHarness,
    ]);

    let denial = admit_s45_simulation_harness_entry(
        &bundle,
        S45RoadmapHarnessRequirementSet::roadmap2_required(),
        inventory,
    )
    .expect_err("missing registered reusable mechanics surface cannot admit");

    assert_eq!(
        denial,
        S45HarnessBoundaryDenial::MissingReusableMechanicsInventory
    );
}

fn required_roadmap_requirements_from_spec() -> &'static [S45RoadmapHarnessRequirement] {
    &[
        S45RoadmapHarnessRequirement::GoldenPathAuthoringApi,
        S45RoadmapHarnessRequirement::AspectNativeScenarioDefinitions,
        S45RoadmapHarnessRequirement::DeterministicScheduler,
        S45RoadmapHarnessRequirement::NamedProductionBoundaryYieldpoints,
        S45RoadmapHarnessRequirement::ProductionFacingDriverContracts,
        S45RoadmapHarnessRequirement::ActorFaultCrashVocabulary,
        S45RoadmapHarnessRequirement::ObserverOracleSeparation,
        S45RoadmapHarnessRequirement::CertificationOwnedOracleFamilies,
        S45RoadmapHarnessRequirement::CounterStrengthContracts,
        S45RoadmapHarnessRequirement::ProductionBackedFixtureManifests,
        S45RoadmapHarnessRequirement::ReplayableTranscriptsAndEvidence,
        S45RoadmapHarnessRequirement::GeneratedCoverageMatrix,
        S45RoadmapHarnessRequirement::HarnessMaturityLadder,
        S45RoadmapHarnessRequirement::ForbiddenShortcutRejection,
        S45RoadmapHarnessRequirement::S4RecoveryDogfoodSlice,
        S45RoadmapHarnessRequirement::S5ReadinessShapeProbeNonClaim,
        S45RoadmapHarnessRequirement::FutureExtensionSlotContainment,
        S45RoadmapHarnessRequirement::MutationStyleHarnessValidation,
    ]
}

fn scrambled_duplicate_requirements_from_spec() -> Vec<S45RoadmapHarnessRequirement> {
    let mut requirements = required_roadmap_requirements_from_spec().to_vec();
    requirements.reverse();
    requirements.push(S45RoadmapHarnessRequirement::GoldenPathAuthoringApi);
    requirements
}

fn scrambled_registered_surfaces() -> Vec<S45RegisteredHarnessSurface> {
    vec![
        S45RegisteredHarnessSurface::ObsoleteSemanticHarness,
        S45RegisteredHarnessSurface::CertificationS4RecoveryHarness,
        S45RegisteredHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
        S45RegisteredHarnessSurface::TestSupportTerminalProjectionJsonFixtures,
        S45RegisteredHarnessSurface::TestSupportNativeAspectFixtures,
        S45RegisteredHarnessSurface::TestSupportS4RecoveryPhysics,
    ]
}

fn registered_surface_classifications_from_spec(
) -> &'static [(S45RegisteredHarnessSurface, S45HarnessSurfaceClassification)] {
    &[
        (
            S45RegisteredHarnessSurface::TestSupportS4RecoveryPhysics,
            S45HarnessSurfaceClassification::ReusableMechanics,
        ),
        (
            S45RegisteredHarnessSurface::TestSupportNativeAspectFixtures,
            S45HarnessSurfaceClassification::ReusableMechanics,
        ),
        (
            S45RegisteredHarnessSurface::TestSupportTerminalProjectionJsonFixtures,
            S45HarnessSurfaceClassification::MilestoneLocalMechanics,
        ),
        (
            S45RegisteredHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
            S45HarnessSurfaceClassification::MilestoneLocalMechanics,
        ),
        (
            S45RegisteredHarnessSurface::CertificationS4RecoveryHarness,
            S45HarnessSurfaceClassification::CertificationMeaning,
        ),
        (
            S45RegisteredHarnessSurface::ObsoleteSemanticHarness,
            S45HarnessSurfaceClassification::ObsoleteSemanticContext,
        ),
    ]
}
