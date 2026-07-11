#[path = "s4_closeout/fixture.rs"]
mod fixture;

use forge_store_physical_certification::{
    admit_s45_simulation_harness_entry, reject_s45_copied_recovery_report,
    reject_s45_foundational_projection_authority, reject_s45_log_output,
    reject_s45_old_semantic_harness_label, reject_s45_s5_isolation_authority_attempt,
    reject_s45_same_run_self_comparison, reject_s45_terminal_projection,
    S45ExistingHarnessInventory, S45HarnessBoundaryDenial, S45HarnessNonClaim,
    S45RegisteredHarnessSurface, S45RoadmapHarnessRequirement, S45RoadmapHarnessRequirementSet,
};

#[test]
fn s45_entry_admits_executed_recovery_and_roadmap_requirements() {
    let recovery = fixture::executed_recovery_receipt();

    let entry = admit_s45_simulation_harness_entry(
        &recovery,
        S45RoadmapHarnessRequirementSet::roadmap2_required(),
        S45ExistingHarnessInventory::dedicated_workspace_baseline(),
    )
    .expect("executed recovery and roadmap harness evidence admit S.4.5 entry");

    assert!(entry.accepts_recovery_receipt_and_roadmap2_harness_evidence());
    assert!(entry
        .inventory()
        .contains_reusable_mechanics("forge-store-test-support::s4_recovery_physics"));
    assert!(entry
        .inventory()
        .contains_certification_meaning("forge-store-certification::s4_recovery_harness"));
    assert!(entry
        .non_claims()
        .contains(&S45HarnessNonClaim::NoS5PhysicalIsolationCorrectnessClaim));
    assert_eq!(entry.identity().recovered_root(), entry.recovered_root());
    assert_eq!(
        entry.identity().roadmap_requirements(),
        S45RoadmapHarnessRequirementSet::roadmap2_required().requirements()
    );
}

#[test]
fn s45_entry_identity_is_stable_across_independent_recovery_execution() {
    let first_recovery = fixture::executed_recovery_receipt();
    let second_recovery = fixture::executed_recovery_receipt();
    let mut scrambled_requirements = S45RoadmapHarnessRequirementSet::roadmap2_required()
        .requirements()
        .to_vec();
    scrambled_requirements.reverse();
    scrambled_requirements.push(S45RoadmapHarnessRequirement::GoldenPathAuthoringApi);

    let first_entry = admit_s45_simulation_harness_entry(
        &first_recovery,
        S45RoadmapHarnessRequirementSet::roadmap2_required(),
        S45ExistingHarnessInventory::dedicated_workspace_baseline(),
    )
    .expect("first independently executed recovery admits");
    let second_entry = admit_s45_simulation_harness_entry(
        &second_recovery,
        S45RoadmapHarnessRequirementSet::from_requirements(scrambled_requirements),
        S45ExistingHarnessInventory::from_registered_surfaces(vec![
            S45RegisteredHarnessSurface::ObsoleteSemanticHarness,
            S45RegisteredHarnessSurface::CertificationS4RecoveryHarness,
            S45RegisteredHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
            S45RegisteredHarnessSurface::TestSupportTerminalProjectionJsonFixtures,
            S45RegisteredHarnessSurface::TestSupportNativeAspectFixtures,
            S45RegisteredHarnessSurface::TestSupportS4RecoveryPhysics,
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
fn s45_entry_rejects_missing_roadmap_requirement() {
    let recovery = fixture::executed_recovery_receipt();
    let incomplete_requirements = S45RoadmapHarnessRequirementSet::from_requirements(
        S45RoadmapHarnessRequirementSet::roadmap2_required()
            .requirements()
            .iter()
            .copied()
            .filter(|requirement| {
                *requirement != S45RoadmapHarnessRequirement::DeterministicScheduler
            })
            .collect(),
    );

    let denial = admit_s45_simulation_harness_entry(
        &recovery,
        incomplete_requirements,
        S45ExistingHarnessInventory::dedicated_workspace_baseline(),
    )
    .expect_err("missing Roadmap 2 harness requirement cannot admit S.4.5 entry");

    assert_eq!(
        denial,
        S45HarnessBoundaryDenial::MissingRoadmapHarnessRequirement(
            S45RoadmapHarnessRequirement::DeterministicScheduler
        )
    );
}

#[test]
fn s45_entry_rejects_shortcut_surrogates() {
    assert_eq!(
        reject_s45_copied_recovery_report("copied report"),
        S45HarnessBoundaryDenial::CopiedS4ReportCannotAdmitEntry
    );
    assert_eq!(
        reject_s45_log_output("log line"),
        S45HarnessBoundaryDenial::LogOutputCannotAdmitEntry
    );
    assert_eq!(
        reject_s45_old_semantic_harness_label("legacy semantic label"),
        S45HarnessBoundaryDenial::OldSemanticHarnessContextCannotAdmitEntry
    );
    assert_eq!(
        reject_s45_same_run_self_comparison(),
        S45HarnessBoundaryDenial::SameRunSelfComparisonCannotAdmitEntry
    );
    assert_eq!(
        reject_s45_terminal_projection("{\"terminal\":\"projection\"}"),
        S45HarnessBoundaryDenial::TerminalProjectionCannotAdmitEntry
    );
    assert_eq!(
        reject_s45_s5_isolation_authority_attempt(),
        S45HarnessBoundaryDenial::S5IsolationAuthorityCannotBeMintedByHarnessEntry
    );
    assert_eq!(
        reject_s45_foundational_projection_authority(),
        S45HarnessBoundaryDenial::FoundationalProjectionCannotReplaceStoreAuthority
    );
}

#[test]
fn s45_inventory_classifies_existing_surfaces() {
    let inventory = S45ExistingHarnessInventory::dedicated_workspace_baseline();

    assert!(
        inventory.contains_reusable_mechanics("forge-store-test-support::native_aspect_fixtures")
    );
    assert!(
        inventory.contains_certification_meaning("forge-store-certification::s4_recovery_harness")
    );
    assert!(inventory.contains_obsolete_semantic_context("crates/forge-store/src/tests/harness"));
}

#[test]
fn s45_inventory_registered_test_support_surfaces_stay_mechanics() {
    let inventory = S45ExistingHarnessInventory::dedicated_workspace_baseline();

    assert!(inventory.contains_reusable_mechanics(
        S45RegisteredHarnessSurface::TestSupportS4RecoveryPhysics.path()
    ));
    assert!(inventory.contains_reusable_mechanics(
        S45RegisteredHarnessSurface::TestSupportNativeAspectFixtures.path()
    ));
    assert!(inventory.contains_milestone_local_mechanics(
        S45RegisteredHarnessSurface::TestSupportTerminalProjectionJsonFixtures.path()
    ));
    assert!(inventory.contains_milestone_local_mechanics(
        S45RegisteredHarnessSurface::TestSupportHostileReadmissionJsonFixtures.path()
    ));
}

#[test]
fn s45_inventory_denies_missing_registered_baseline_surface() {
    let recovery = fixture::executed_recovery_receipt();
    let inventory = S45ExistingHarnessInventory::from_registered_surfaces(vec![
        S45RegisteredHarnessSurface::TestSupportS4RecoveryPhysics,
        S45RegisteredHarnessSurface::TestSupportNativeAspectFixtures,
        S45RegisteredHarnessSurface::TestSupportTerminalProjectionJsonFixtures,
        S45RegisteredHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
        S45RegisteredHarnessSurface::ObsoleteSemanticHarness,
    ]);

    let denial = admit_s45_simulation_harness_entry(
        &recovery,
        S45RoadmapHarnessRequirementSet::roadmap2_required(),
        inventory,
    )
    .expect_err("missing registered certification surface cannot admit");

    assert_eq!(
        denial,
        S45HarnessBoundaryDenial::MissingCertificationMeaningInventory
    );
}

#[test]
fn s45_inventory_denies_missing_milestone_local_surface() {
    let recovery = fixture::executed_recovery_receipt();
    let inventory = S45ExistingHarnessInventory::from_registered_surfaces(vec![
        S45RegisteredHarnessSurface::TestSupportS4RecoveryPhysics,
        S45RegisteredHarnessSurface::TestSupportNativeAspectFixtures,
        S45RegisteredHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
        S45RegisteredHarnessSurface::CertificationS4RecoveryHarness,
        S45RegisteredHarnessSurface::ObsoleteSemanticHarness,
    ]);

    let denial = admit_s45_simulation_harness_entry(
        &recovery,
        S45RoadmapHarnessRequirementSet::roadmap2_required(),
        inventory,
    )
    .expect_err("missing registered milestone-local surface cannot admit");

    assert_eq!(
        denial,
        S45HarnessBoundaryDenial::MissingMilestoneLocalMechanicsInventory
    );
}

#[test]
fn s45_inventory_denies_legacy_harness_as_authority() {
    let recovery = fixture::executed_recovery_receipt();
    let inventory = S45ExistingHarnessInventory::from_registered_surfaces(vec![
        S45RegisteredHarnessSurface::TestSupportS4RecoveryPhysics,
        S45RegisteredHarnessSurface::TestSupportNativeAspectFixtures,
        S45RegisteredHarnessSurface::TestSupportTerminalProjectionJsonFixtures,
        S45RegisteredHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
        S45RegisteredHarnessSurface::CertificationS4RecoveryHarness,
    ]);

    let denial = admit_s45_simulation_harness_entry(
        &recovery,
        S45RoadmapHarnessRequirementSet::roadmap2_required(),
        inventory,
    )
    .expect_err("legacy semantic harness context cannot be promoted to authority");

    assert_eq!(
        denial,
        S45HarnessBoundaryDenial::MissingObsoleteSemanticContextInventory
    );
}
