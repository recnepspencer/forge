use forge_store_test_support::harness::recovery::coverage as coverage_support;

use forge_proof::{Recipe, Unresolved};
use forge_store_physical_certification::{
    fixture_label_oracle_attempt, log_only_oracle_attempt,
    reject_foundational_materialization_as_store_authority,
    reject_raw_json_scenario_authority_attempt, reject_same_run_self_comparison_evidence_attempt,
    reject_terminal_json_evidence_attempt, reject_unresolved_simulation_plan_recipe,
    shortcut_denial_from_evidence_bundle_denial, shortcut_denial_from_fault_delivery_denial,
    shortcut_denial_from_harness_boundary_denial, shortcut_denial_from_oracle_denial,
    shortcut_denial_from_plan_denial, shortcut_denial_from_scenario_denial,
    shortcut_denial_from_terminal_projection_denial, shortcut_denial_from_transcript_denial,
    test_support_oracle_verdict_attempt, FaultDeliveryAttempt, ForbiddenShortcutKind,
    OracleFamilyKind, PhysicalCertificationEvidenceBundle, PhysicalDriverKind,
    PhysicalEvidenceBundleDenial, PhysicalProofOracleKind, PhysicalProofOracleVerdictKind,
    PhysicalScenarioActorRole, PhysicalScenarioIntent, PhysicalSimulationScenarioFamily,
    ShortcutRejectionBoundary, ShortcutRejectionObservationKind, SimulationHarnessBoundaryDenial,
    SyntheticHarnessShortcutDenialReceipt, SyntheticHarnessShortcutRejectionDenial,
    SyntheticHarnessShortcutRejectionReport,
};
use forge_store_test_support::developer_smoke_replay_seed;

#[test]
fn forbidden_shortcut_report_requires_all_store_owned_denials() {
    let report = SyntheticHarnessShortcutRejectionReport::from_denied_shortcuts(
        complete_shortcut_denial_receipts(),
    )
    .unwrap()
    .require_all_certification_shortcuts_denied()
    .unwrap();

    assert!(report.all_required_shortcuts_denied());
    assert!(report.all_required_boundaries_denied());
    assert_eq!(report.receipts().len(), 9);
}

#[test]
fn partial_forbidden_shortcut_report_is_denied() {
    let mut receipts = complete_shortcut_denial_receipts();
    receipts.retain(|receipt| receipt.shortcut() != ForbiddenShortcutKind::PrivateMutation);
    let denial =
        SyntheticHarnessShortcutRejectionReport::from_denied_shortcuts(receipts).unwrap_err();

    assert_eq!(
        denial,
        SyntheticHarnessShortcutRejectionDenial::MissingRequiredShortcut(
            ForbiddenShortcutKind::PrivateMutation
        )
    );
}

#[test]
fn same_shortcut_kind_from_wrong_boundary_cannot_satisfy_required_boundary() {
    let mut receipts = complete_shortcut_denial_receipts();
    receipts
        .retain(|receipt| receipt.boundary() != ShortcutRejectionBoundary::TranscriptCopiedFields);
    receipts.push(
        shortcut_denial_from_harness_boundary_denial(
            SimulationHarnessBoundaryDenial::CopiedS4ReportCannotAdmitEntry,
        )
        .unwrap(),
    );

    let denial =
        SyntheticHarnessShortcutRejectionReport::from_denied_shortcuts(receipts).unwrap_err();

    assert_eq!(
        denial,
        SyntheticHarnessShortcutRejectionDenial::MissingRequiredBoundary(
            ShortcutRejectionBoundary::TranscriptCopiedFields
        )
    );
}

#[test]
fn foundational_materialization_remains_non_store_authority() {
    let denial = reject_foundational_materialization_as_store_authority().unwrap_err();

    assert_eq!(
        denial,
        PhysicalEvidenceBundleDenial::FoundationalMaterializationIsNotStoreAuthority
    );
    assert!(shortcut_denial_from_evidence_bundle_denial(denial).is_none());
}

#[test]
fn legitimate_native_shortcut_scenario_to_evidence_lane_still_closes() {
    let scenario = coverage_support::shortcut_scenario();
    let definition = scenario.definition();
    assert_eq!(
        definition.family(),
        PhysicalSimulationScenarioFamily::ShortcutRejectionDogfood
    );
    assert_eq!(
        definition.intent(),
        PhysicalScenarioIntent::ForbiddenShortcutRejectionShape
    );
    assert_eq!(
        definition.schedule().production_boundary_yieldpoint(),
        "shortcut-rejection-boundary"
    );
    assert!(definition
        .actors()
        .iter()
        .any(|actor| actor.role() == PhysicalScenarioActorRole::ShortcutRejectionProbe));

    let plan = coverage_support::shortcut_plan();
    assert_eq!(plan.scenario_identity(), scenario.identity());
    assert!(plan
        .drivers()
        .contains(PhysicalDriverKind::ShortcutRejectionBoundary));
    assert!(plan
        .oracle_families()
        .contains(OracleFamilyKind::TranscriptReplayEvidence));
    assert!(plan
        .oracle_families()
        .contains(OracleFamilyKind::ForbiddenShortcutRejection));

    let replay = coverage_support::replay_bundle(&plan);
    assert_eq!(replay.plan().identity(), plan.identity());
    assert!(replay.schedule().replay_identity_matches_plan(&plan));
    assert_eq!(replay.schedule().seed(), developer_smoke_replay_seed());
    assert_eq!(replay.counter_receipt().plan_identity(), plan.identity());
    assert!(replay
        .trace()
        .shortcut_rejections()
        .iter()
        .any(|observation| {
            observation.kind() == ShortcutRejectionObservationKind::PrivateMutationDenied
        }));
    assert!(replay.oracle_verdicts().iter().any(|verdict| {
        verdict.family() == OracleFamilyKind::ForbiddenShortcutRejection
            && verdict.oracle() == PhysicalProofOracleKind::NoPrivateMutation
            && verdict.kind() == PhysicalProofOracleVerdictKind::Satisfied
    }));

    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();
    let primary = evidence.primary();

    assert_eq!(
        primary.scenario_digest(),
        scenario.identity().digest_bytes()
    );
    assert_eq!(primary.plan_digest(), plan.identity().digest_bytes());
    assert_eq!(
        primary.transcript_digest(),
        evidence.replay().transcript_identity().digest_bytes()
    );
    assert_eq!(primary.oracle_verdict_count(), 2);
    assert!(primary.counter_row_count() > 0);
    assert!(evidence.failure_digest().is_none());
}

fn complete_shortcut_denial_receipts() -> Vec<SyntheticHarnessShortcutDenialReceipt> {
    vec![
        shortcut_denial_from_evidence_bundle_denial(
            forge_store_physical_certification::reject_loose_log_evidence_attempt().unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_scenario_denial(
            reject_raw_json_scenario_authority_attempt(r#"{"scenario":"fake"}"#).unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_terminal_projection_denial(
            reject_terminal_json_evidence_attempt().unwrap_err(),
        ),
        shortcut_denial_from_evidence_bundle_denial(
            reject_same_run_self_comparison_evidence_attempt().unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_fault_delivery_denial(
            FaultDeliveryAttempt::private_mutation()
                .admit()
                .unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_oracle_denial(fixture_label_oracle_attempt().unwrap_err()).unwrap(),
        shortcut_denial_from_transcript_denial(
            forge_store_physical_certification::reject_copied_transcript_fields().unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_plan_denial(
            reject_unresolved_simulation_plan_recipe(Recipe::<Unresolved, _>::new(
                coverage_support::shortcut_plan(),
            ))
            .unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_oracle_denial(test_support_oracle_verdict_attempt().unwrap_err())
            .unwrap(),
    ]
}

#[test]
fn harness_boundary_denials_can_feed_shortcut_receipts_without_owning_evidence() {
    let copied = shortcut_denial_from_harness_boundary_denial(
        SimulationHarnessBoundaryDenial::CopiedS4ReportCannotAdmitEntry,
    )
    .unwrap();
    let log = shortcut_denial_from_oracle_denial(log_only_oracle_attempt().unwrap_err()).unwrap();

    assert_eq!(
        copied.shortcut(),
        ForbiddenShortcutKind::CopiedDigestAuthority
    );
    assert_eq!(log.shortcut(), ForbiddenShortcutKind::LogsAsProof);
}
