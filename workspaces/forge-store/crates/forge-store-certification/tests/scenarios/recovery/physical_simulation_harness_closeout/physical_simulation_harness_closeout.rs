#[path = "../../../s4_5_physical_simulation_harness_closeout/support.rs"]
mod closeout_support;
use forge_store_test_support::harness::recovery::counter_evidence as counter_support;
use forge_store_test_support::harness::recovery::coverage as coverage_support;

use std::collections::BTreeSet;

use closeout_support::{
    acceptance_suite_receipts, alternate_recovery_slice_evidence,
    complete_acceptance_suite_receipts, complete_executed_acceptance_suites,
    complete_shortcut_report, executed_acceptance_suites,
    physical_isolation_readiness_slice_evidence, recovery_slice_evidence, shortcut_slice_evidence,
};

use forge_store_physical_certification::{
    ExecutedSimulationHarnessAcceptanceSuiteEvidenceSet, ForbiddenShortcutKind,
    PhysicalIsolationCorrectnessNonClaimEvidence, PhysicalSimulationHarnessCertificationBundle,
    PhysicalSimulationHarnessCloseoutDenial, PhysicalSimulationHarnessCloseoutSuite,
    ShortcutRejectionBoundary, SimulationHarnessAcceptanceEvidenceLane,
    SimulationHarnessAcceptanceSuiteName, SimulationHarnessAcceptanceSuiteReceiptSet,
    SimulationHarnessCloseoutCoverageReport, SimulationHarnessDogfoodEvidence,
};

#[test]
fn simulation_harness_closeout_dogfoods_public_authoring_and_publishes_physical_isolation_readiness(
) {
    let recovery = recovery_slice_evidence();
    let shortcut = shortcut_slice_evidence();
    let s5_probe = physical_isolation_readiness_slice_evidence();
    let dogfood_evidence = SimulationHarnessDogfoodEvidence::new(recovery, shortcut, s5_probe);
    let coverage =
        SimulationHarnessCloseoutCoverageReport::from_dogfood_evidence(&dogfood_evidence);
    let suite_receipts = complete_acceptance_suite_receipts(&dogfood_evidence, &coverage);
    let shortcut_report = complete_shortcut_report();

    let bundle = PhysicalSimulationHarnessCertificationBundle::from_simulation_harness_public_authoring_slices(
        PhysicalSimulationHarnessCloseoutSuite::simulation_admission(),
        dogfood_evidence,
        suite_receipts,
        shortcut_report,
        PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
    )
    .unwrap();
    let report = bundle.closeout_report();

    assert!(report
        .dogfood()
        .recovery_slice()
        .used_public_authoring_api());
    assert!(report
        .dogfood()
        .shortcut_rejection_slice()
        .used_public_authoring_api());
    assert!(report
        .dogfood()
        .physical_isolation_readiness_shape_probe()
        .used_public_authoring_api());
    assert!(report
        .coverage()
        .all_required_simulation_harness_lanes_are_satisfied());
    assert!(report
        .acceptance()
        .all_required_simulation_harness_lanes_are_satisfied());
    assert!(report
        .acceptance()
        .suites()
        .iter()
        .all(|suite| suite.lanes().len() == 14));
    assert_acceptance_suite_sources_and_basis(report.acceptance());
    assert!(report
        .physical_isolation_readiness()
        .does_not_claim_physical_isolation_correctness());
    assert_eq!(report.shortcut_denial_count(), 9);
    assert!(report
        .future_extension_slots()
        .all_reserved_without_future_behavior());
    assert_eq!(report.future_extension_slots().slots().len(), 5);
}

fn assert_acceptance_suite_sources_and_basis(
    acceptance: &forge_store_physical_certification::SimulationHarnessAcceptanceSuiteMap,
) {
    let mut suites = BTreeSet::new();
    let mut execution_basis = BTreeSet::new();
    for suite in acceptance.suites() {
        assert_eq!(suite.source().suite(), suite.suite());
        assert_ne!(suite.execution_basis_digest(), &[0_u8; 32]);
        assert!(execution_basis.insert(*suite.execution_basis_digest()));
        for lane in required_acceptance_lanes() {
            assert!(suite.contains(lane));
        }
        suites.insert(suite.suite());
    }
    assert_eq!(
        suites,
        SimulationHarnessAcceptanceSuiteName::required_simulation_harness()
            .into_iter()
            .collect::<BTreeSet<_>>()
    );
}

fn required_acceptance_lanes() -> [SimulationHarnessAcceptanceEvidenceLane; 14] {
    [
        SimulationHarnessAcceptanceEvidenceLane::Scenario,
        SimulationHarnessAcceptanceEvidenceLane::Plan,
        SimulationHarnessAcceptanceEvidenceLane::Schedule,
        SimulationHarnessAcceptanceEvidenceLane::Actors,
        SimulationHarnessAcceptanceEvidenceLane::Drivers,
        SimulationHarnessAcceptanceEvidenceLane::Observers,
        SimulationHarnessAcceptanceEvidenceLane::Oracles,
        SimulationHarnessAcceptanceEvidenceLane::Transcripts,
        SimulationHarnessAcceptanceEvidenceLane::Counters,
        SimulationHarnessAcceptanceEvidenceLane::Positive,
        SimulationHarnessAcceptanceEvidenceLane::Hostile,
        SimulationHarnessAcceptanceEvidenceLane::Shortcut,
        SimulationHarnessAcceptanceEvidenceLane::Replay,
        SimulationHarnessAcceptanceEvidenceLane::Mutation,
    ]
}

#[test]
fn closeout_rejects_missing_named_acceptance_suite_receipt() {
    let dogfood_evidence = SimulationHarnessDogfoodEvidence::new(
        recovery_slice_evidence(),
        shortcut_slice_evidence(),
        physical_isolation_readiness_slice_evidence(),
    );
    let coverage =
        SimulationHarnessCloseoutCoverageReport::from_dogfood_evidence(&dogfood_evidence);
    let mut receipts = acceptance_suite_receipts(&dogfood_evidence, &coverage);
    receipts.retain(|receipt| {
        receipt.suite() != SimulationHarnessAcceptanceSuiteName::FaultDeliveryBoundary
    });

    let denial = SimulationHarnessAcceptanceSuiteReceiptSet::from_receipts(receipts).unwrap_err();

    assert_eq!(
        denial,
        PhysicalSimulationHarnessCloseoutDenial::MissingAcceptanceSuiteReceipt {
            suite: SimulationHarnessAcceptanceSuiteName::FaultDeliveryBoundary,
        }
    );
}

#[test]
fn acceptance_receipts_are_issued_by_closeout_suite_authority() {
    let dogfood_evidence = SimulationHarnessDogfoodEvidence::new(
        recovery_slice_evidence(),
        shortcut_slice_evidence(),
        physical_isolation_readiness_slice_evidence(),
    );
    let coverage =
        SimulationHarnessCloseoutCoverageReport::from_dogfood_evidence(&dogfood_evidence);
    let receipts = PhysicalSimulationHarnessCloseoutSuite::simulation_admission()
        .execute_required_acceptance_suites(complete_executed_acceptance_suites(
            &dogfood_evidence,
            &coverage,
        ))
        .unwrap();

    assert!(receipts.receipts().iter().any(|receipt| receipt.suite()
        == SimulationHarnessAcceptanceSuiteName::PhysicalIsolationHarnessReadiness));
}

#[test]
fn closeout_suite_requires_each_named_executed_suite_proof() {
    let dogfood_evidence = SimulationHarnessDogfoodEvidence::new(
        recovery_slice_evidence(),
        shortcut_slice_evidence(),
        physical_isolation_readiness_slice_evidence(),
    );
    let coverage =
        SimulationHarnessCloseoutCoverageReport::from_dogfood_evidence(&dogfood_evidence);
    let mut executed = executed_acceptance_suites(&dogfood_evidence, &coverage);
    executed
        .retain(|suite| suite.suite() != SimulationHarnessAcceptanceSuiteName::GeneratedCoverage);

    let denial =
        ExecutedSimulationHarnessAcceptanceSuiteEvidenceSet::from_executed_suites(executed)
            .unwrap_err();

    assert_eq!(
        denial,
        PhysicalSimulationHarnessCloseoutDenial::MissingAcceptanceSuiteExecution {
            suite: SimulationHarnessAcceptanceSuiteName::GeneratedCoverage,
        }
    );
}

#[test]
fn closeout_suite_rejects_replayed_executed_suite_proof() {
    let dogfood_evidence = SimulationHarnessDogfoodEvidence::new(
        recovery_slice_evidence(),
        shortcut_slice_evidence(),
        physical_isolation_readiness_slice_evidence(),
    );
    let coverage =
        SimulationHarnessCloseoutCoverageReport::from_dogfood_evidence(&dogfood_evidence);
    let mut executed = executed_acceptance_suites(&dogfood_evidence, &coverage);
    executed.push(executed[0].clone());

    let denial =
        ExecutedSimulationHarnessAcceptanceSuiteEvidenceSet::from_executed_suites(executed)
            .unwrap_err();

    assert_eq!(
        denial,
        PhysicalSimulationHarnessCloseoutDenial::DuplicateAcceptanceSuiteExecution {
            suite: SimulationHarnessAcceptanceSuiteName::EntryBoundary,
        }
    );
}

#[test]
fn closeout_rejects_acceptance_receipts_from_different_dogfood_evidence() {
    let original_dogfood = SimulationHarnessDogfoodEvidence::new(
        recovery_slice_evidence(),
        shortcut_slice_evidence(),
        physical_isolation_readiness_slice_evidence(),
    );
    let original_coverage =
        SimulationHarnessCloseoutCoverageReport::from_dogfood_evidence(&original_dogfood);
    let stale_receipts = complete_acceptance_suite_receipts(&original_dogfood, &original_coverage);
    let current_dogfood = SimulationHarnessDogfoodEvidence::new(
        alternate_recovery_slice_evidence(),
        shortcut_slice_evidence(),
        physical_isolation_readiness_slice_evidence(),
    );

    let denial = PhysicalSimulationHarnessCertificationBundle::from_simulation_harness_public_authoring_slices(
        PhysicalSimulationHarnessCloseoutSuite::simulation_admission(),
        current_dogfood,
        stale_receipts,
        complete_shortcut_report(),
        PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
    )
    .unwrap_err();

    assert!(matches!(
        denial,
        PhysicalSimulationHarnessCloseoutDenial::StaleAcceptanceSuiteReceipt { .. }
    ));
}

#[test]
fn future_extension_slots_are_visible_without_future_behavior_or_readiness() {
    let inventory =
        forge_store_physical_certification::FutureHarnessExtensionSlotInventory::
            simulation_harness_reserved_future_slots();

    for slot in inventory.slots() {
        assert!(!slot.implements_future_behavior());
        assert!(!slot.can_satisfy_physical_isolation_readiness());
    }
    assert!(inventory.all_reserved_without_future_behavior());
}

#[test]
fn shortcut_report_in_closeout_names_all_forbidden_boundaries() {
    let report = complete_shortcut_report();
    for boundary in [
        ShortcutRejectionBoundary::EvidenceLooseLog,
        ShortcutRejectionBoundary::ScenarioJsonAuthority,
        ShortcutRejectionBoundary::EvidenceTerminalProjection,
        ShortcutRejectionBoundary::EvidenceSameRunSelfComparison,
        ShortcutRejectionBoundary::FaultDeliveryPrivateMutation,
        ShortcutRejectionBoundary::OracleFixtureLabel,
        ShortcutRejectionBoundary::TranscriptCopiedFields,
        ShortcutRejectionBoundary::PlanProofProgressionSkipped,
        ShortcutRejectionBoundary::OracleTestSupportVerdict,
    ] {
        assert!(report
            .receipts()
            .iter()
            .any(|receipt| receipt.boundary() == boundary));
    }
    assert!(report
        .receipts()
        .iter()
        .any(|receipt| receipt.shortcut() == ForbiddenShortcutKind::PrivateMutation));
}
