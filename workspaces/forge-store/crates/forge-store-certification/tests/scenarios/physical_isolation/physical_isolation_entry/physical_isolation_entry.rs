#[path = "../../../support/recovery/closeout/fixture.rs"]
mod closeout_fixture;
#[path = "../../../support/recovery/coverage_support/coverage_support.rs"]
mod coverage_support;

use forge_foundational::{
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceReceiptKind,
    FoundationalBoundaryEvidenceSourceBasisKind,
};
use forge_proof::{RecipeStageDxExt, RecipeStageKind};
use forge_store_physical_certification::{
    fixture_label_oracle_attempt,
    register_physical_isolation_certification_lane,
    reject_copied_simulation_harness_readiness_rows_as_physical_isolation_lane_registration,
    reject_generic_runner_as_physical_isolation_lane_registration,
    reject_harness_projection_as_physical_isolation_lane_registration,
    reject_loose_log_evidence_attempt,
    reject_raw_json_scenario_authority_attempt, reject_same_run_self_comparison_evidence_attempt,
    reject_terminal_json_evidence_attempt, reject_unresolved_simulation_plan_recipe,
    shortcut_denial_from_evidence_bundle_denial, shortcut_denial_from_fault_delivery_denial,
    shortcut_denial_from_oracle_denial, shortcut_denial_from_plan_denial,
    shortcut_denial_from_scenario_denial, shortcut_denial_from_terminal_projection_denial,
    shortcut_denial_from_transcript_denial, test_support_oracle_verdict_attempt, CoverageGapDenial,
    FaultDeliveryAttempt, ForbiddenShortcutKind, OracleFamilyKind,
    PhysicalCertificationEvidenceBundle, PhysicalDriverKind,
    PhysicalIsolationHarnessReadinessReceipt, PhysicalIsolationLaneRegistrationDenial,
    ShortcutRejectionBoundary, SimulationPlanDenial, SyntheticHarnessShortcutDenialReceipt,
    SyntheticHarnessShortcutRejectionReport, PhysicalIsolationCorrectnessNonClaimEvidence,
    PhysicalIsolationHarnessReadinessDenial,
};
use forge_store_physical_isolation::{
    admit_physical_isolation_entry, reject_copied_recovery_fields_as_physical_isolation_entry,
    reject_foundational_or_proof_projection_as_physical_isolation_entry,
    reject_json_authority_as_physical_isolation_entry,
    reject_live_runtime_state_as_physical_isolation_entry,
    reject_semantic_snapshot_as_physical_isolation_entry,
    reject_stale_recovery_readiness_as_physical_isolation_entry,
    reject_terminal_projection_as_physical_isolation_entry, PhysicalIsolationEntryCheckedOutcome,
    PhysicalIsolationEntryDenial, PhysicalIsolationEntryRequest,
};

#[test]
fn s5_entry_admits_only_typed_recovery_completion() {
    let completion = closeout_fixture::recovery_completion();

    let entry = admit_physical_isolation_entry(
        PhysicalIsolationEntryRequest::from_recovery_completion(&completion),
    )
    .unwrap();

    assert_eq!(entry.recovered_root(), completion.recovered_root());
    assert_eq!(
        entry.admitted_page_lsn_frontier(),
        completion.admitted_page_lsn_frontier()
    );
    assert_eq!(entry.replayed_frames(), completion.replayed_frames());
    assert_eq!(
        entry.identity().recovered_root(),
        completion.recovered_root()
    );
    assert_eq!(
        entry.root_epoch_basis(),
        entry.identity().root_epoch_basis()
    );
    assert!(!entry.is_store_physical_stability_authority());
    assert!(!entry.evidence().is_store_physical_stability_authority());
    assert_eq!(
        entry
            .evidence()
            .foundational()
            .executed_receipt()
            .receipt_kind(),
        FoundationalBoundaryEvidenceReceiptKind::Execution
    );
    assert_eq!(
        entry.evidence().foundational().freshness_posture(),
        FoundationalBoundaryEvidenceFreshnessPosture::ReconstructedFromReplay
    );
    assert_eq!(
        entry.evidence().foundational().source_basis().kind(),
        FoundationalBoundaryEvidenceSourceBasisKind::BoundaryArtifact
    );
    assert!(!entry
        .evidence()
        .foundational()
        .is_store_physical_stability_authority());
    assert_entry_proof_progression_is_store_authorized(&entry);
}

#[test]
fn independently_executed_recovery_has_same_entry_identity_and_root_epoch_basis() {
    let first_completion = closeout_fixture::recovery_completion();
    let second_completion = closeout_fixture::recovery_completion();
    let first = admit_recovery_completion_entry(&first_completion);
    let second = admit_recovery_completion_entry(&second_completion);

    assert_eq!(first.identity(), second.identity());
    assert_eq!(first.root_epoch_basis(), second.root_epoch_basis());
    assert_eq!(first.recovered_root(), second.recovered_root());
    assert_eq!(
        first.admitted_page_lsn_frontier(),
        second.admitted_page_lsn_frontier()
    );
}

fn admit_recovery_completion_entry(
    completion: &forge_store_recovery_physics::RecoveryCompletion,
) -> forge_store_physical_isolation::PhysicalIsolationEntryAdmission {
    admit_physical_isolation_entry(PhysicalIsolationEntryRequest::from_recovery_completion(
        completion,
    ))
    .unwrap()
}

fn assert_entry_proof_progression_is_store_authorized(
    entry: &forge_store_physical_isolation::PhysicalIsolationEntryAdmission,
) {
    let progression = entry.evidence().proof_progression();
    assert_eq!(
        progression.unresolved_recipe().stage(),
        RecipeStageKind::Unresolved
    );
    assert_eq!(
        progression.resolved_recipe().stage(),
        RecipeStageKind::Resolved
    );
    assert_eq!(
        progression.lowered_recipe().stage(),
        RecipeStageKind::Lowered
    );
    assert_eq!(
        progression.admitted_recipe().stage(),
        RecipeStageKind::Admitted
    );
    assert_eq!(
        progression
            .admitted_recipe()
            .strong_basis()
            .value()
            .root_epoch_basis(),
        entry.root_epoch_basis()
    );
    assert_eq!(
        progression
            .admitted_recipe()
            .strong_basis()
            .value()
            .identity(),
        entry.identity()
    );
    assert!(!entry
        .evidence()
        .proof_progression()
        .is_store_physical_stability_authority());
}

#[test]
fn s5_entry_rejects_copied_runtime_semantic_projection_and_json_authority() {
    assert_eq!(
        reject_copied_recovery_fields_as_physical_isolation_entry().unwrap_err(),
        PhysicalIsolationEntryDenial::CopiedRecoveryFields
    );
    assert_eq!(
        reject_live_runtime_state_as_physical_isolation_entry().unwrap_err(),
        PhysicalIsolationEntryDenial::LiveRuntimeState
    );
    assert_eq!(
        reject_terminal_projection_as_physical_isolation_entry().unwrap_err(),
        PhysicalIsolationEntryDenial::TerminalProjection
    );
    assert_eq!(
        reject_semantic_snapshot_as_physical_isolation_entry().unwrap_err(),
        PhysicalIsolationEntryDenial::SemanticSnapshot
    );
    assert_eq!(
        reject_json_authority_as_physical_isolation_entry().unwrap_err(),
        PhysicalIsolationEntryDenial::JsonAuthority
    );
    assert_eq!(
        reject_foundational_or_proof_projection_as_physical_isolation_entry().unwrap_err(),
        PhysicalIsolationEntryDenial::FoundationalOrProofProjection
    );
    assert_eq!(
        reject_stale_recovery_readiness_as_physical_isolation_entry(),
        PhysicalIsolationEntryCheckedOutcome::Stale(
            PhysicalIsolationEntryDenial::StaleRecoveryReadiness
        )
    );
    assert!(matches!(
        forge_store_physical_isolation::require_rebound_s4_recovery_readiness_for_physical_isolation_entry(),
        PhysicalIsolationEntryCheckedOutcome::RebindRequired(_)
    ));
}

#[test]
fn physical_isolation_lane_requires_entry_and_s45_harness_readiness() {
    let completion = closeout_fixture::recovery_completion();
    let entry = admit_physical_isolation_entry(
        PhysicalIsolationEntryRequest::from_recovery_completion(&completion),
    )
    .unwrap();
    let receipt = s45_harness_readiness_receipt();

    let registration =
        register_physical_isolation_certification_lane(&entry, receipt);

    assert_eq!(registration.entry_recovered_root(), entry.recovered_root());
    assert!(registration.does_not_claim_physical_isolation_correctness());
    assert!(registration
        .accepted_harness()
        .does_not_claim_physical_isolation_correctness());
}

#[test]
fn physical_isolation_lane_denies_near_miss_s45_receipts_before_registration() {
    assert_eq!(
        receipt_denial_for_developer_smoke_profile(),
        PhysicalIsolationHarnessReadinessDenial::UnsupportedProfileMaturityEvidence
    );
    assert!(matches!(
        receipt_denial_for_matrix_evidence_identity_mismatch(),
        PhysicalIsolationHarnessReadinessDenial::MissingDependency(_)
    ));
    assert_eq!(
        missing_private_mutation_observation_denial(),
        CoverageGapDenial::MissingMutationResult
    );
    assert_eq!(
        coverage_support::ci_plan_without_supported_driver(
            PhysicalDriverKind::ShortcutRejectionBoundary,
        )
        .unwrap_err(),
        SimulationPlanDenial::MissingPhysicalDriver(PhysicalDriverKind::ShortcutRejectionBoundary)
    );
    assert_eq!(
        coverage_support::ci_plan_without_supported_oracle(
            OracleFamilyKind::ForbiddenShortcutRejection,
        )
        .unwrap_err(),
        SimulationPlanDenial::MissingOracleFamily(OracleFamilyKind::ForbiddenShortcutRejection)
    );
}

#[test]
fn physical_isolation_lane_rejects_copied_rows_and_runner_shortcuts() {
    assert_eq!(
        reject_copied_simulation_harness_readiness_rows_as_physical_isolation_lane_registration()
            .unwrap_err(),
        PhysicalIsolationLaneRegistrationDenial::CopiedS45ReadinessRows
    );
    assert_eq!(
        reject_generic_runner_as_physical_isolation_lane_registration().unwrap_err(),
        PhysicalIsolationLaneRegistrationDenial::GenericRunner
    );
    assert_eq!(
        reject_harness_projection_as_physical_isolation_lane_registration().unwrap_err(),
        PhysicalIsolationLaneRegistrationDenial::HarnessProjection
    );
}

fn s45_harness_readiness_receipt() -> PhysicalIsolationHarnessReadinessReceipt {
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let matrix = coverage_support::complete_registry(&plan, &replay)
        .generate_matrix()
        .unwrap();
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();
    PhysicalIsolationHarnessReadinessReceipt::from_store_harness_evidence(
        &matrix,
        &evidence,
        &complete_shortcut_report(),
        PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
    )
    .unwrap()
}

fn receipt_denial_for_developer_smoke_profile() -> PhysicalIsolationHarnessReadinessDenial {
    let plan = coverage_support::lowered_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let matrix = coverage_support::complete_registry(&plan, &replay)
        .generate_matrix()
        .unwrap();
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();
    PhysicalIsolationHarnessReadinessReceipt::from_store_harness_evidence(
        &matrix,
        &evidence,
        &complete_shortcut_report(),
        PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
    )
    .unwrap_err()
}

fn receipt_denial_for_matrix_evidence_identity_mismatch() -> PhysicalIsolationHarnessReadinessDenial
{
    let matrix_plan = coverage_support::lowered_ci_plan();
    let matrix_replay = coverage_support::replay_bundle(&matrix_plan);
    let matrix = coverage_support::complete_registry(&matrix_plan, &matrix_replay)
        .generate_matrix()
        .unwrap();
    let evidence_plan = coverage_support::shortcut_plan();
    let evidence = coverage_support::evidence_bundle(&evidence_plan);
    PhysicalIsolationHarnessReadinessReceipt::from_store_harness_evidence(
        &matrix,
        &evidence,
        &complete_shortcut_report(),
        PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
    )
    .unwrap_err()
}

fn missing_private_mutation_observation_denial() -> CoverageGapDenial {
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle_without_mutation_denial(&plan);
    coverage_support::mutation_evidence_denial(&replay)
}

fn complete_shortcut_report() -> SyntheticHarnessShortcutRejectionReport {
    SyntheticHarnessShortcutRejectionReport::from_denied_shortcuts(
        complete_shortcut_denial_receipts(),
    )
    .unwrap()
}

fn complete_shortcut_denial_receipts() -> Vec<SyntheticHarnessShortcutDenialReceipt> {
    vec![
        shortcut_denial_from_evidence_bundle_denial(
            reject_loose_log_evidence_attempt().unwrap_err(),
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
            reject_unresolved_simulation_plan_recipe(forge_proof::Recipe::new(
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
fn shortcut_report_names_required_s5_entry_boundaries() {
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
