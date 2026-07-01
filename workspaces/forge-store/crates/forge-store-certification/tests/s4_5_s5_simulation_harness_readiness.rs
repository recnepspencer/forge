#[path = "s4_5_coverage_support.rs"]
mod coverage_support;

use std::collections::BTreeSet;

use forge_proof::{Recipe, Unresolved};
use forge_store_physical_certification::{
    accept_store_owned_s5_harness_readiness, fixture_label_oracle_attempt,
    reject_foundational_or_proof_projection_as_s5_harness_readiness,
    reject_future_slot_as_s5_harness_readiness, reject_generic_runner_as_s5_harness_readiness,
    reject_raw_json_scenario_authority_attempt, reject_same_run_self_comparison_evidence_attempt,
    reject_terminal_json_evidence_attempt, reject_unresolved_simulation_plan_recipe,
    shortcut_denial_from_evidence_bundle_denial, shortcut_denial_from_fault_delivery_denial,
    shortcut_denial_from_oracle_denial, shortcut_denial_from_plan_denial,
    shortcut_denial_from_scenario_denial, shortcut_denial_from_terminal_projection_denial,
    shortcut_denial_from_transcript_denial, test_support_oracle_verdict_attempt,
    CounterContractKind, CoverageGapDenial, FaultDeliveryAttempt, ForbiddenShortcutKind,
    OracleFamilyKind, PhysicalCertificationEvidenceBundle, PhysicalDriverKind,
    PhysicalScenarioActorRole, S5CounterContractReadiness, S5HarnessFutureExtensionReservation,
    S5HarnessFutureExtensionSlot, S5HarnessReadinessReceipt, S5InterleavingHarnessCapability,
    S5MaintenanceActorCapability, S5ProductionDriverCapability, S5RequiredYieldpoint,
    S5ReusableOracleReadiness, ShortcutRejectionBoundary, ShortcutRejectionObservationKind,
    SimulationPlanDenial, SyntheticHarnessShortcutDenialReceipt,
    SyntheticHarnessShortcutRejectionReport,
};
use forge_store_readiness::{S5CorrectnessNonClaimEvidence, S5SimulationHarnessReadinessDenial};

#[test]
fn s5_receives_store_owned_simulation_harness_readiness() {
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let matrix = coverage_support::complete_registry(&plan, &replay)
        .generate_matrix()
        .unwrap();
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();
    let shortcut_report = complete_shortcut_report();

    let receipt = S5HarnessReadinessReceipt::from_store_harness_evidence(
        &matrix,
        &evidence,
        &shortcut_report,
        S5CorrectnessNonClaimEvidence::shape_probe_only(),
    )
    .unwrap();

    assert_exact_set(receipt.interleaving(), expected_interleaving_capabilities());
    assert_exact_set(receipt.maintenance_actors(), expected_maintenance_actors());
    assert_exact_set(receipt.yieldpoints(), expected_yieldpoints());
    assert_exact_set(receipt.production_drivers(), expected_production_drivers());
    assert_exact_set(receipt.oracle_families(), expected_oracle_families());
    assert_exact_set(receipt.counter_contracts(), expected_counter_contracts());
    assert_eq!(
        receipt.transcript_digest(),
        evidence.primary().transcript_digest()
    );
    assert_eq!(receipt.shortcut_denial_count(), 9);

    let accepted = accept_store_owned_s5_harness_readiness(
        receipt,
        forge_store_physical_isolation::s5_simulation_harness_readiness_requirement(),
    );
    assert!(accepted.does_not_claim_s5_correctness());
}

#[test]
fn readiness_shape_probe_lowers_and_executes_with_explicit_non_claim() {
    let scenario = coverage_support::scenario();
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle(&plan);

    assert_eq!(plan.scenario_identity(), scenario.identity());
    assert!(plan
        .actors()
        .contains_role(PhysicalScenarioActorRole::MaintenanceReclaimer));
    assert!(plan
        .drivers()
        .contains(PhysicalDriverKind::ProductionBoundaryYieldpoint));
    assert!(plan
        .drivers()
        .contains(PhysicalDriverKind::ShortcutRejectionBoundary));
    assert!(plan
        .oracle_families()
        .contains(OracleFamilyKind::S5ReadinessShape));
    assert!(plan
        .oracle_families()
        .contains(OracleFamilyKind::TranscriptReplayEvidence));
    assert!(plan
        .counter_contracts()
        .contains(CounterContractKind::BlockedReclaimAttempts));
    assert!(plan
        .counter_contracts()
        .contains(CounterContractKind::LatchWaits));
    assert!(plan
        .counter_contracts()
        .contains(CounterContractKind::EpochRetries));
    assert!(plan
        .counter_contracts()
        .contains(CounterContractKind::ProtectedReferences));
    assert!(plan
        .counter_contracts()
        .contains(CounterContractKind::PublicationSwaps));
    assert!(replay.schedule().replay_identity_matches_plan(&plan));
    assert!(replay
        .trace()
        .shortcut_rejections()
        .iter()
        .any(|observation| {
            observation.kind() == ShortcutRejectionObservationKind::PrivateMutationDenied
        }));
    assert!(replay.oracle_verdicts().iter().any(|verdict| {
        verdict.family() == OracleFamilyKind::S5ReadinessShape
            && verdict
                .non_claims()
                .contains(
                    &forge_store_physical_certification::PhysicalOracleNonClaim::
                        S5PhysicalIsolationCorrectness,
                )
    }));
}

#[test]
fn s5_handoff_denies_near_miss_store_owned_evidence() {
    assert_eq!(
        receipt_denial_for_developer_smoke_profile(),
        S5SimulationHarnessReadinessDenial::UnsupportedProfileMaturityEvidence
    );
    assert!(matches!(
        receipt_denial_for_matrix_evidence_identity_mismatch(),
        S5SimulationHarnessReadinessDenial::MissingDependency(_)
    ));
}

#[test]
fn s5_handoff_dependencies_deny_before_fake_receipts_exist() {
    assert_eq!(
        coverage_support::ci_plan_without_supported_driver(
            PhysicalDriverKind::ShortcutRejectionBoundary,
        )
        .unwrap_err(),
        SimulationPlanDenial::MissingPhysicalDriver(PhysicalDriverKind::ShortcutRejectionBoundary,)
    );
    assert_eq!(
        coverage_support::ci_plan_without_supported_oracle(
            OracleFamilyKind::ForbiddenShortcutRejection,
        )
        .unwrap_err(),
        SimulationPlanDenial::MissingOracleFamily(OracleFamilyKind::ForbiddenShortcutRejection)
    );
    assert_eq!(
        matrix_denial_for_missing_private_mutation_observation(),
        CoverageGapDenial::MissingMutationResult
    );
}

#[test]
fn generic_runners_and_future_slots_cannot_satisfy_s5_readiness() {
    assert_eq!(
        reject_generic_runner_as_s5_harness_readiness().unwrap_err(),
        S5SimulationHarnessReadinessDenial::GenericRunnerCannotSatisfyReadiness
    );
    assert_eq!(
        reject_future_slot_as_s5_harness_readiness(S5HarnessFutureExtensionReservation::reserved(
            S5HarnessFutureExtensionSlot::BlobLifecycle,
        ),)
        .unwrap_err(),
        S5SimulationHarnessReadinessDenial::FutureBehaviorSlotCannotSatisfyReadiness
    );
    assert_eq!(
        reject_foundational_or_proof_projection_as_s5_harness_readiness().unwrap_err(),
        S5SimulationHarnessReadinessDenial::FoundationalOrProofProjectionCannotSatisfyReadiness
    );
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

fn receipt_denial_for_developer_smoke_profile() -> S5SimulationHarnessReadinessDenial {
    let plan = coverage_support::lowered_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let matrix = coverage_support::complete_registry(&plan, &replay)
        .generate_matrix()
        .unwrap();
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();
    S5HarnessReadinessReceipt::from_store_harness_evidence(
        &matrix,
        &evidence,
        &complete_shortcut_report(),
        S5CorrectnessNonClaimEvidence::shape_probe_only(),
    )
    .unwrap_err()
}

fn matrix_denial_for_missing_private_mutation_observation() -> CoverageGapDenial {
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle_without_mutation_denial(&plan);
    coverage_support::mutation_evidence_denial(&replay)
}

fn receipt_denial_for_matrix_evidence_identity_mismatch() -> S5SimulationHarnessReadinessDenial {
    let matrix_plan = coverage_support::lowered_ci_plan();
    let matrix_replay = coverage_support::replay_bundle(&matrix_plan);
    let matrix = coverage_support::complete_registry(&matrix_plan, &matrix_replay)
        .generate_matrix()
        .unwrap();
    let evidence_plan = coverage_support::shortcut_plan();
    let evidence = coverage_support::evidence_bundle(&evidence_plan);
    S5HarnessReadinessReceipt::from_store_harness_evidence(
        &matrix,
        &evidence,
        &complete_shortcut_report(),
        S5CorrectnessNonClaimEvidence::shape_probe_only(),
    )
    .unwrap_err()
}

fn expected_interleaving_capabilities() -> [S5InterleavingHarnessCapability; 10] {
    [
        S5InterleavingHarnessCapability::DeterministicReplaySchedule,
        S5InterleavingHarnessCapability::ProtectBeforeObserveShapeProbe,
        S5InterleavingHarnessCapability::RootKindSeparationShapeProbe,
        S5InterleavingHarnessCapability::TraversalAdmissionShapeProbe,
        S5InterleavingHarnessCapability::ByteGuardUsageShapeProbe,
        S5InterleavingHarnessCapability::NoHiddenLatchIoShapeProbe,
        S5InterleavingHarnessCapability::PublicationMemoryOrderingShapeProbe,
        S5InterleavingHarnessCapability::LeaseExpiryNonAuthorityShapeProbe,
        S5InterleavingHarnessCapability::FreeReuseGenerationFenceShapeProbe,
        S5InterleavingHarnessCapability::RestartDuringCutoverShapeProbe,
    ]
}

fn expected_maintenance_actors() -> [S5MaintenanceActorCapability; 2] {
    [
        S5MaintenanceActorCapability::ReclaimBarrierParticipant,
        S5MaintenanceActorCapability::RestartParticipant,
    ]
}

fn expected_yieldpoints() -> [S5RequiredYieldpoint; 6] {
    [
        S5RequiredYieldpoint::RootPublicationBeforeObserve,
        S5RequiredYieldpoint::RootSwapPublication,
        S5RequiredYieldpoint::ByteGuardAdmission,
        S5RequiredYieldpoint::ReclaimBarrier,
        S5RequiredYieldpoint::RestartDuringCutover,
        S5RequiredYieldpoint::ShortcutRejectionBoundary,
    ]
}

fn expected_production_drivers() -> [S5ProductionDriverCapability; 2] {
    [
        S5ProductionDriverCapability::ProductionBoundaryYieldpoint,
        S5ProductionDriverCapability::ShortcutRejectionBoundary,
    ]
}

fn expected_oracle_families() -> [S5ReusableOracleReadiness; 3] {
    [
        S5ReusableOracleReadiness::S5ReadinessShape,
        S5ReusableOracleReadiness::TranscriptReplayEvidence,
        S5ReusableOracleReadiness::ForbiddenShortcutRejection,
    ]
}

fn expected_counter_contracts() -> [S5CounterContractReadiness; 10] {
    [
        S5CounterContractReadiness::ActorStepExact,
        S5CounterContractReadiness::ReplayIdentityExact,
        S5CounterContractReadiness::ForbiddenShortcutExact,
        S5CounterContractReadiness::ProfileResourceEnvelope,
        S5CounterContractReadiness::LatchWaits,
        S5CounterContractReadiness::EpochRetries,
        S5CounterContractReadiness::ProtectedReferences,
        S5CounterContractReadiness::BlockedReclaimAttempts,
        S5CounterContractReadiness::PublicationSwaps,
        S5CounterContractReadiness::FutureS5SpecificCountersReserved,
    ]
}

fn assert_exact_set<T>(actual: &[T], expected: impl IntoIterator<Item = T>)
where
    T: Copy + Ord + std::fmt::Debug,
{
    let actual = actual.iter().copied().collect::<BTreeSet<_>>();
    let expected = expected.into_iter().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

#[test]
fn shortcut_report_still_names_required_shortcut_boundaries() {
    let report = complete_shortcut_report();
    assert!(report.all_required_shortcuts_denied());
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
        assert!(
            report
                .receipts()
                .iter()
                .any(|receipt| receipt.boundary() == boundary),
            "missing shortcut boundary {boundary:?}"
        );
    }
    assert!(report
        .receipts()
        .iter()
        .any(|receipt| receipt.shortcut() == ForbiddenShortcutKind::PrivateMutation));
}
