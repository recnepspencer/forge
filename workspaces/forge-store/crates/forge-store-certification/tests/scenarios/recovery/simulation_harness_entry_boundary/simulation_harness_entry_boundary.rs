#[path = "../../../support/recovery/closeout/fixture.rs"]
mod fixture;

use forge_store_physical_certification::{
    admit_simulation_harness_entry, reject_simulation_harness_copied_recovery_report,
    reject_simulation_harness_foundational_projection_authority,
    reject_simulation_harness_log_output, reject_simulation_harness_old_semantic_harness_label,
    reject_simulation_harness_physical_isolation_authority_attempt,
    reject_simulation_harness_same_run_self_comparison,
    reject_simulation_harness_terminal_projection,
    ExistingSimulationHarnessInventory, RegisteredSimulationHarnessSurface,
    SimulationHarnessBoundaryDenial, SimulationHarnessNonClaim,
    SimulationHarnessRoadmapRequirement, SimulationHarnessRoadmapRequirementSet,
};

#[test]
fn simulation_harness_entry_admits_executed_recovery_and_roadmap_requirements() {
    let recovery = fixture::executed_recovery_receipt();

    let entry = admit_simulation_harness_entry(
        &recovery,
        SimulationHarnessRoadmapRequirementSet::certification_required(),
        ExistingSimulationHarnessInventory::dedicated_workspace_baseline(),
    )
    .expect("executed recovery and roadmap harness evidence admit S.4.5 entry");

    assert!(entry.accepts_recovery_receipt_and_harness_evidence());
    assert!(entry
        .inventory()
        .contains_reusable_mechanics("forge-store-test-support::s4_recovery_physics"));
    assert!(entry
        .inventory()
        .contains_certification_meaning("forge-store-certification::s4_recovery_harness"));
    assert!(entry
        .non_claims()
        .contains(&SimulationHarnessNonClaim::NoPhysicalIsolationCorrectnessClaim));
    assert_eq!(entry.identity().recovered_root(), entry.recovered_root());
    assert_eq!(
        entry.identity().roadmap_requirements(),
        SimulationHarnessRoadmapRequirementSet::certification_required().requirements()
    );
}

#[test]
fn simulation_harness_entry_identity_is_stable_across_independent_recovery_execution() {
    let first_recovery = fixture::executed_recovery_receipt();
    let second_recovery = fixture::executed_recovery_receipt();
    let mut scrambled_requirements =
        SimulationHarnessRoadmapRequirementSet::certification_required()
            .requirements()
            .to_vec();
    scrambled_requirements.reverse();
    scrambled_requirements.push(SimulationHarnessRoadmapRequirement::GoldenPathAuthoringApi);

    let first_entry = admit_simulation_harness_entry(
        &first_recovery,
        SimulationHarnessRoadmapRequirementSet::certification_required(),
        ExistingSimulationHarnessInventory::dedicated_workspace_baseline(),
    )
    .expect("first independently executed recovery admits");
    let second_entry = admit_simulation_harness_entry(
        &second_recovery,
        SimulationHarnessRoadmapRequirementSet::from_requirements(scrambled_requirements),
        ExistingSimulationHarnessInventory::from_registered_surfaces(vec![
            RegisteredSimulationHarnessSurface::ObsoleteSemanticHarness,
            RegisteredSimulationHarnessSurface::CertificationS4RecoveryHarness,
            RegisteredSimulationHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
            RegisteredSimulationHarnessSurface::TestSupportTerminalProjectionJsonFixtures,
            RegisteredSimulationHarnessSurface::TestSupportNativeAspectFixtures,
            RegisteredSimulationHarnessSurface::TestSupportS4RecoveryPhysics,
        ]),
    )
    .expect("second independently executed recovery admits");

    assert_eq!(first_entry.identity(), second_entry.identity());
    assert_eq!(
        first_entry.roadmap_requirements(),
        second_entry.roadmap_requirements()
    );
}

#[test]
fn simulation_harness_entry_rejects_missing_roadmap_requirement() {
    let recovery = fixture::executed_recovery_receipt();
    let incomplete_requirements = SimulationHarnessRoadmapRequirementSet::from_requirements(
        SimulationHarnessRoadmapRequirementSet::certification_required()
            .requirements()
            .iter()
            .copied()
            .filter(|requirement| {
                *requirement != SimulationHarnessRoadmapRequirement::DeterministicScheduler
            })
            .collect(),
    );

    let denial = admit_simulation_harness_entry(
        &recovery,
        incomplete_requirements,
        ExistingSimulationHarnessInventory::dedicated_workspace_baseline(),
    )
    .expect_err("missing Roadmap 2 harness requirement cannot admit S.4.5 entry");

    assert_eq!(
        denial,
        SimulationHarnessBoundaryDenial::MissingRoadmapHarnessRequirement(
            SimulationHarnessRoadmapRequirement::DeterministicScheduler
        )
    );
}

#[test]
fn simulation_harness_entry_rejects_shortcut_surrogates() {
    assert_eq!(
        reject_simulation_harness_copied_recovery_report("copied report"),
        SimulationHarnessBoundaryDenial::CopiedS4ReportCannotAdmitEntry
    );
    assert_eq!(
        reject_simulation_harness_log_output("log line"),
        SimulationHarnessBoundaryDenial::LogOutputCannotAdmitEntry
    );
    assert_eq!(
        reject_simulation_harness_old_semantic_harness_label("legacy semantic label"),
        SimulationHarnessBoundaryDenial::OldSemanticHarnessContextCannotAdmitEntry
    );
    assert_eq!(
        reject_simulation_harness_same_run_self_comparison(),
        SimulationHarnessBoundaryDenial::SameRunSelfComparisonCannotAdmitEntry
    );
    assert_eq!(
        reject_simulation_harness_terminal_projection("{\"terminal\":\"projection\"}"),
        SimulationHarnessBoundaryDenial::TerminalProjectionCannotAdmitEntry
    );
    assert_eq!(
        reject_simulation_harness_physical_isolation_authority_attempt(),
        SimulationHarnessBoundaryDenial::PhysicalIsolationAuthorityCannotBeMintedByHarnessEntry
    );
    assert_eq!(
        reject_simulation_harness_foundational_projection_authority(),
        SimulationHarnessBoundaryDenial::FoundationalProjectionCannotReplaceStoreAuthority
    );
}

#[test]
fn simulation_harness_inventory_classifies_existing_surfaces() {
    let inventory = ExistingSimulationHarnessInventory::dedicated_workspace_baseline();

    assert!(
        inventory.contains_reusable_mechanics("forge-store-test-support::native_aspect_fixtures")
    );
    assert!(
        inventory.contains_certification_meaning("forge-store-certification::s4_recovery_harness")
    );
    assert!(inventory.contains_obsolete_semantic_context("crates/forge-store/src/tests/harness"));
}

#[test]
fn simulation_harness_inventory_registered_test_support_surfaces_stay_mechanics() {
    let inventory = ExistingSimulationHarnessInventory::dedicated_workspace_baseline();

    assert!(inventory.contains_reusable_mechanics(
        RegisteredSimulationHarnessSurface::TestSupportS4RecoveryPhysics.path()
    ));
    assert!(inventory.contains_reusable_mechanics(
        RegisteredSimulationHarnessSurface::TestSupportNativeAspectFixtures.path()
    ));
    assert!(inventory.contains_milestone_local_mechanics(
        RegisteredSimulationHarnessSurface::TestSupportTerminalProjectionJsonFixtures.path()
    ));
    assert!(inventory.contains_milestone_local_mechanics(
        RegisteredSimulationHarnessSurface::TestSupportHostileReadmissionJsonFixtures.path()
    ));
}

#[test]
fn simulation_harness_inventory_denies_missing_registered_baseline_surface() {
    let recovery = fixture::executed_recovery_receipt();
    let inventory = ExistingSimulationHarnessInventory::from_registered_surfaces(vec![
        RegisteredSimulationHarnessSurface::TestSupportS4RecoveryPhysics,
        RegisteredSimulationHarnessSurface::TestSupportNativeAspectFixtures,
        RegisteredSimulationHarnessSurface::TestSupportTerminalProjectionJsonFixtures,
        RegisteredSimulationHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
        RegisteredSimulationHarnessSurface::ObsoleteSemanticHarness,
    ]);

    let denial = admit_simulation_harness_entry(
        &recovery,
        SimulationHarnessRoadmapRequirementSet::certification_required(),
        inventory,
    )
    .expect_err("missing registered certification surface cannot admit");

    assert_eq!(
        denial,
        SimulationHarnessBoundaryDenial::MissingCertificationMeaningInventory
    );
}

#[test]
fn simulation_harness_inventory_denies_missing_milestone_local_surface() {
    let recovery = fixture::executed_recovery_receipt();
    let inventory = ExistingSimulationHarnessInventory::from_registered_surfaces(vec![
        RegisteredSimulationHarnessSurface::TestSupportS4RecoveryPhysics,
        RegisteredSimulationHarnessSurface::TestSupportNativeAspectFixtures,
        RegisteredSimulationHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
        RegisteredSimulationHarnessSurface::CertificationS4RecoveryHarness,
        RegisteredSimulationHarnessSurface::ObsoleteSemanticHarness,
    ]);

    let denial = admit_simulation_harness_entry(
        &recovery,
        SimulationHarnessRoadmapRequirementSet::certification_required(),
        inventory,
    )
    .expect_err("missing registered milestone-local surface cannot admit");

    assert_eq!(
        denial,
        SimulationHarnessBoundaryDenial::MissingMilestoneLocalMechanicsInventory
    );
}

#[test]
fn simulation_harness_inventory_denies_legacy_harness_as_authority() {
    let recovery = fixture::executed_recovery_receipt();
    let inventory = ExistingSimulationHarnessInventory::from_registered_surfaces(vec![
        RegisteredSimulationHarnessSurface::TestSupportS4RecoveryPhysics,
        RegisteredSimulationHarnessSurface::TestSupportNativeAspectFixtures,
        RegisteredSimulationHarnessSurface::TestSupportTerminalProjectionJsonFixtures,
        RegisteredSimulationHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
        RegisteredSimulationHarnessSurface::CertificationS4RecoveryHarness,
    ]);

    let denial = admit_simulation_harness_entry(
        &recovery,
        SimulationHarnessRoadmapRequirementSet::certification_required(),
        inventory,
    )
    .expect_err("legacy semantic harness context cannot be promoted to authority");

    assert_eq!(
        denial,
        SimulationHarnessBoundaryDenial::MissingObsoleteSemanticContextInventory
    );
}
