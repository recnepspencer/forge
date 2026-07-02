#[path = "s4_5_closeout_support.rs"]
mod closeout_support;
#[path = "s4_5_counter_strength/support.rs"]
mod counter_support;
#[path = "s4_5_coverage_support.rs"]
mod coverage_support;

use std::collections::BTreeSet;

use closeout_support::{
    acceptance_suite_receipts, alternate_s4_recovery_slice_evidence,
    complete_acceptance_suite_receipts, complete_executed_acceptance_suites,
    complete_shortcut_report, executed_acceptance_suites, s4_recovery_slice_evidence,
    s5_readiness_slice_evidence, shortcut_slice_evidence,
};

use forge_store_physical_certification::{
    ForbiddenShortcutKind, PhysicalSimulationHarnessCertificationBundle,
    PhysicalSimulationHarnessCloseoutDenial, PhysicalSimulationHarnessCloseoutSuite,
    S45AcceptanceEvidenceLane, S45AcceptanceSuiteName, S45AcceptanceSuiteReceiptSet,
    S45CloseoutCoverageReport, S45ExecutedAcceptanceSuiteEvidenceSet, S45HarnessDogfoodEvidence,
    ShortcutRejectionBoundary,
};
use forge_store_readiness::S5CorrectnessNonClaimEvidence;

#[test]
fn s45_closeout_dogfoods_public_authoring_and_publishes_s5_readiness() {
    let s4_recovery = s4_recovery_slice_evidence();
    let shortcut = shortcut_slice_evidence();
    let s5_probe = s5_readiness_slice_evidence();
    let dogfood_evidence = S45HarnessDogfoodEvidence::new(s4_recovery, shortcut, s5_probe);
    let coverage = S45CloseoutCoverageReport::from_dogfood_evidence(&dogfood_evidence);
    let suite_receipts = complete_acceptance_suite_receipts(&dogfood_evidence, &coverage);
    let shortcut_report = complete_shortcut_report();

    let bundle = PhysicalSimulationHarnessCertificationBundle::from_s45_public_authoring_slices(
        PhysicalSimulationHarnessCloseoutSuite::roadmap2_s45(),
        dogfood_evidence,
        suite_receipts,
        shortcut_report,
        S5CorrectnessNonClaimEvidence::shape_probe_only(),
    )
    .unwrap();
    let report = bundle.closeout_report();

    assert!(report
        .dogfood()
        .s4_recovery_slice()
        .used_public_authoring_api());
    assert!(report
        .dogfood()
        .shortcut_rejection_slice()
        .used_public_authoring_api());
    assert!(report
        .dogfood()
        .s5_readiness_shape_probe()
        .used_public_authoring_api());
    assert!(report.coverage().all_required_s45_lanes_are_satisfied());
    assert!(report.acceptance().all_required_s45_lanes_are_satisfied());
    assert!(report
        .acceptance()
        .suites()
        .iter()
        .all(|suite| suite.lanes().len() == 14));
    assert_acceptance_suite_sources_and_basis(report.acceptance());
    assert!(report.s5_readiness().does_not_claim_s5_correctness());
    assert_eq!(report.shortcut_denial_count(), 9);
    assert!(report
        .future_extension_slots()
        .all_reserved_without_future_behavior());
    assert_eq!(report.future_extension_slots().slots().len(), 5);
}

fn assert_acceptance_suite_sources_and_basis(
    acceptance: &forge_store_physical_certification::S45AcceptanceSuiteMap,
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
        S45AcceptanceSuiteName::required_s45()
            .into_iter()
            .collect::<BTreeSet<_>>()
    );
}

fn required_acceptance_lanes() -> [S45AcceptanceEvidenceLane; 14] {
    [
        S45AcceptanceEvidenceLane::Scenario,
        S45AcceptanceEvidenceLane::Plan,
        S45AcceptanceEvidenceLane::Schedule,
        S45AcceptanceEvidenceLane::Actors,
        S45AcceptanceEvidenceLane::Drivers,
        S45AcceptanceEvidenceLane::Observers,
        S45AcceptanceEvidenceLane::Oracles,
        S45AcceptanceEvidenceLane::Transcripts,
        S45AcceptanceEvidenceLane::Counters,
        S45AcceptanceEvidenceLane::Positive,
        S45AcceptanceEvidenceLane::Hostile,
        S45AcceptanceEvidenceLane::Shortcut,
        S45AcceptanceEvidenceLane::Replay,
        S45AcceptanceEvidenceLane::Mutation,
    ]
}

#[test]
fn closeout_rejects_missing_named_acceptance_suite_receipt() {
    let dogfood_evidence = S45HarnessDogfoodEvidence::new(
        s4_recovery_slice_evidence(),
        shortcut_slice_evidence(),
        s5_readiness_slice_evidence(),
    );
    let coverage = S45CloseoutCoverageReport::from_dogfood_evidence(&dogfood_evidence);
    let mut receipts = acceptance_suite_receipts(&dogfood_evidence, &coverage);
    receipts.retain(|receipt| receipt.suite() != S45AcceptanceSuiteName::FaultDeliveryBoundary);

    let denial = S45AcceptanceSuiteReceiptSet::from_receipts(receipts).unwrap_err();

    assert_eq!(
        denial,
        PhysicalSimulationHarnessCloseoutDenial::MissingAcceptanceSuiteReceipt {
            suite: S45AcceptanceSuiteName::FaultDeliveryBoundary,
        }
    );
}

#[test]
fn acceptance_receipts_are_issued_by_closeout_suite_authority() {
    let dogfood_evidence = S45HarnessDogfoodEvidence::new(
        s4_recovery_slice_evidence(),
        shortcut_slice_evidence(),
        s5_readiness_slice_evidence(),
    );
    let coverage = S45CloseoutCoverageReport::from_dogfood_evidence(&dogfood_evidence);
    let receipts = PhysicalSimulationHarnessCloseoutSuite::roadmap2_s45()
        .execute_required_acceptance_suites(complete_executed_acceptance_suites(
            &dogfood_evidence,
            &coverage,
        ))
        .unwrap();

    assert!(receipts
        .receipts()
        .iter()
        .any(|receipt| receipt.suite() == S45AcceptanceSuiteName::S5SimulationHarnessReadiness));
}

#[test]
fn closeout_suite_requires_each_named_executed_suite_proof() {
    let dogfood_evidence = S45HarnessDogfoodEvidence::new(
        s4_recovery_slice_evidence(),
        shortcut_slice_evidence(),
        s5_readiness_slice_evidence(),
    );
    let coverage = S45CloseoutCoverageReport::from_dogfood_evidence(&dogfood_evidence);
    let mut executed = executed_acceptance_suites(&dogfood_evidence, &coverage);
    executed.retain(|suite| suite.suite() != S45AcceptanceSuiteName::GeneratedCoverage);

    let denial = S45ExecutedAcceptanceSuiteEvidenceSet::from_executed_suites(executed).unwrap_err();

    assert_eq!(
        denial,
        PhysicalSimulationHarnessCloseoutDenial::MissingAcceptanceSuiteExecution {
            suite: S45AcceptanceSuiteName::GeneratedCoverage,
        }
    );
}

#[test]
fn closeout_suite_rejects_replayed_executed_suite_proof() {
    let dogfood_evidence = S45HarnessDogfoodEvidence::new(
        s4_recovery_slice_evidence(),
        shortcut_slice_evidence(),
        s5_readiness_slice_evidence(),
    );
    let coverage = S45CloseoutCoverageReport::from_dogfood_evidence(&dogfood_evidence);
    let mut executed = executed_acceptance_suites(&dogfood_evidence, &coverage);
    executed.push(executed[0].clone());

    let denial = S45ExecutedAcceptanceSuiteEvidenceSet::from_executed_suites(executed).unwrap_err();

    assert_eq!(
        denial,
        PhysicalSimulationHarnessCloseoutDenial::DuplicateAcceptanceSuiteExecution {
            suite: S45AcceptanceSuiteName::EntryBoundary,
        }
    );
}

#[test]
fn closeout_rejects_acceptance_receipts_from_different_dogfood_evidence() {
    let original_dogfood = S45HarnessDogfoodEvidence::new(
        s4_recovery_slice_evidence(),
        shortcut_slice_evidence(),
        s5_readiness_slice_evidence(),
    );
    let original_coverage = S45CloseoutCoverageReport::from_dogfood_evidence(&original_dogfood);
    let stale_receipts = complete_acceptance_suite_receipts(&original_dogfood, &original_coverage);
    let current_dogfood = S45HarnessDogfoodEvidence::new(
        alternate_s4_recovery_slice_evidence(),
        shortcut_slice_evidence(),
        s5_readiness_slice_evidence(),
    );

    let denial = PhysicalSimulationHarnessCertificationBundle::from_s45_public_authoring_slices(
        PhysicalSimulationHarnessCloseoutSuite::roadmap2_s45(),
        current_dogfood,
        stale_receipts,
        complete_shortcut_report(),
        S5CorrectnessNonClaimEvidence::shape_probe_only(),
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
            s45_reserved_future_slots();

    for slot in inventory.slots() {
        assert!(!slot.implements_future_behavior());
        assert!(!slot.can_satisfy_s5_readiness());
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
