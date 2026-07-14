use crate::{
    close_physical_integrity_from_executed_evidence,
    courtroom::blobs::chunk_integrity_without_blob_lifecycle_tests::inspect_chunk_denial,
    courtroom::harness::test_support::integrity_handoff_test_support::intact_readiness,
    courtroom::harness::test_support::physical_integrity_closeout_line_cap_test_support::line_cap_composition_evidence,
    courtroom::harness::test_support::physical_integrity_closeout_test_support::{
        assert_synthetic_rejection, complete_physical_integrity_closeout_evidence,
        complete_physical_integrity_closeout_suite, copied_physical_substrate_synthetic_rejections,
        executed_boundary_denial_evidence, executed_localization_evidence,
        lane_plan_and_transcript, physical_integrity_harness, physical_integrity_readiness,
    },
    courtroom::physical_integrity::physical_integrity_closeout_bundle::PhysicalIntegrityCloseoutRequest,
    ExecutedCorruptionLocalizationEvidence, ExecutedIntegrityBoundaryDenialEvidence,
    IntegrityCloseoutDenialBoundary, IntegrityCloseoutEvidenceFamily,
    IntegrityHarnessExecutionEvidence, IntegrityHarnessTranscriptEvidence,
    PhysicalIntegrityAcceptanceSuite, PhysicalIntegrityCertificationBundle,
    PhysicalIntegrityCloseoutDenial, PhysicalIntegrityCloseoutSuite,
    PhysicalIntegrityCloseoutSuiteEvidence, PhysicalProofOracleKind, PhysicalScenarioDriverKind,
    PhysicalScenarioObserverKind, RoadmapLaneFamily, SyntheticCloseoutShortcutAttempt,
    SyntheticCloseoutShortcutInput, SyntheticCloseoutShortcutRejectionReport,
};
use forge_store_physical_integrity::{
    ChecksumAlgorithmClaim, ChecksumAlgorithmId, ChunkIntegrityDenialKind,
};

#[test]
fn physical_integrity_closeout_publishes_physical_integrity_report_and_recovery_handoff() {
    let s4_readiness = intact_readiness("new-closeout-handoff");
    let expected_identity = s4_readiness.payload().identity().clone();
    let expected_counters = s4_readiness.counters();
    let closeout = close_physical_integrity_from_executed_evidence(
        physical_integrity_readiness(),
        s4_readiness,
        executed_localization_evidence(),
        executed_boundary_denial_evidence(),
        line_cap_composition_evidence(),
    )
    .unwrap();

    assert!(closeout.report().proves_physical_integrity_closeout());
    assert!(closeout.report().reserves_recovery_physics());
    assert!(closeout.report().proves_no_raw_bytes_crossed());
    assert_eq!(closeout.report().acceptance_suite_count(), 6);
    assert_eq!(closeout.report().evidence_family_count(), 6);
    assert_eq!(
        closeout.report().harness_lane(),
        RoadmapLaneFamily::Integrity
    );
    assert_eq!(
        closeout.report().recovery_handoff_identity(),
        &expected_identity
    );
    assert_eq!(closeout.report().recovery_counters(), expected_counters);
    assert_eq!(closeout.report().suite_harnesses().len(), 6);
    assert!(closeout.report().suite_harnesses().iter().all(|harness| {
        harness.lane_family() == RoadmapLaneFamily::Integrity
            && !harness.driver_families().is_empty()
            && !harness.observer_families().is_empty()
            && !harness.oracle_families().is_empty()
    }));
    assert_closeout_harness_matrix(closeout.report().suite_harnesses());
    assert_eq!(closeout.suite().evidence().len(), 6);
    assert!(closeout.recovery_handoff().proves_no_raw_bytes_crossed());
    assert!(!closeout.recovery_handoff().claims_recovery());
}

#[test]
fn closeout_facade_runs_physical_integrity_harness_from_executed_evidence() {
    let s4_readiness = intact_readiness("facade");
    let closeout = close_physical_integrity_from_executed_evidence(
        physical_integrity_readiness(),
        s4_readiness,
        executed_localization_evidence(),
        executed_boundary_denial_evidence(),
        line_cap_composition_evidence(),
    )
    .unwrap();

    assert!(closeout.report().proves_physical_integrity_closeout());
    assert_closeout_harness_matrix(closeout.report().suite_harnesses());
}

#[test]
fn closeout_suite_rejects_missing_suite_and_missing_boundary() {
    let s4_readiness = intact_readiness("missing-suite");
    let mut evidence = complete_physical_integrity_closeout_evidence(&s4_readiness);
    evidence.retain(|row| {
        row.acceptance_suite() != PhysicalIntegrityAcceptanceSuite::RecoveryIntegrityHandoff
    });
    let denial = PhysicalIntegrityCloseoutSuite::admit(evidence).unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::MissingAcceptanceSuite(
            PhysicalIntegrityAcceptanceSuite::RecoveryIntegrityHandoff
        )
    );

    let mut evidence = complete_physical_integrity_closeout_evidence(&s4_readiness);
    let mut denials = executed_boundary_denial_evidence();
    denials.retain(|row| row.boundary() != IntegrityCloseoutDenialBoundary::DigestAsChecksum);
    evidence[1] = PhysicalIntegrityCloseoutSuiteEvidence::boundary_denial(
        physical_integrity_harness(
            PhysicalIntegrityAcceptanceSuite::BoundaryDenial,
            IntegrityHarnessExecutionEvidence::boundary_denial(&denials),
        ),
        denials,
    );
    let denial = PhysicalIntegrityCloseoutSuite::admit(evidence).unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::MissingBoundaryDenial(
            IntegrityCloseoutDenialBoundary::DigestAsChecksum
        )
    );
}

#[test]
fn closeout_suite_rejects_harness_copied_across_suite_rows() {
    let s4_readiness = intact_readiness("copied-harness-suite");
    let mut evidence = complete_physical_integrity_closeout_evidence(&s4_readiness);
    let wrong_harness = evidence
        .iter()
        .find(|row| {
            row.acceptance_suite() == PhysicalIntegrityAcceptanceSuite::CorruptionLocalization
        })
        .unwrap()
        .harness()
        .clone();
    let denials = executed_boundary_denial_evidence();
    evidence[1] = PhysicalIntegrityCloseoutSuiteEvidence::boundary_denial(wrong_harness, denials);

    let denial = PhysicalIntegrityCloseoutSuite::admit(evidence).unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::MismatchedHarnessSuite(
            PhysicalIntegrityAcceptanceSuite::BoundaryDenial
        )
    );
}

#[test]
fn closeout_rejects_mismatched_executed_artifacts_instead_of_labels() {
    let wrong_checksum_denial =
        ChecksumAlgorithmId::admit_claim(ChecksumAlgorithmClaim::checksum_as_authenticity_claim())
            .unwrap_err();
    let denial = ExecutedIntegrityBoundaryDenialEvidence::from_digest_as_checksum_denial(
        wrong_checksum_denial,
    )
    .unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::UnexecutedBoundaryDenial(
            IntegrityCloseoutDenialBoundary::DigestAsChecksum
        )
    );

    let chunk_denial = inspect_chunk_denial("payload-damage", b"DATA", 1024);
    assert_eq!(
        chunk_denial.kind(),
        ChunkIntegrityDenialKind::ChunkPayloadDamage
    );
    let denial = ExecutedCorruptionLocalizationEvidence::from_extent_damage_denial(&chunk_denial)
        .unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::UnexecutedCorruptionLocalization(
            crate::CorruptionLocalizationBoundary::ExtentDamage
        )
    );
}

#[test]
fn closeout_rejects_synthetic_reports_copied_from_another_transcript() {
    let s4_readiness = intact_readiness("copied-synthetic");
    let mut evidence = complete_physical_integrity_closeout_evidence(&s4_readiness);
    let copied = copied_physical_substrate_synthetic_rejections();
    evidence[3] = PhysicalIntegrityCloseoutSuiteEvidence::synthetic_rejection(
        physical_integrity_harness(
            PhysicalIntegrityAcceptanceSuite::SyntheticShortcutRejection,
            IntegrityHarnessExecutionEvidence::synthetic_rejection(&copied),
        ),
        copied,
    );

    let denial = PhysicalIntegrityCloseoutSuite::admit(evidence).unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::SyntheticRejectionTranscriptMismatch(
            SyntheticCloseoutShortcutAttempt::LogsOnlyProof
        )
    );
}

#[test]
fn physical_integrity_synthetic_closeout_rejects_expected_errors_buffers_and_fixture_labels() {
    let s4_readiness = intact_readiness("synthetic-closeout");
    let suite = complete_physical_integrity_closeout_suite(&s4_readiness);
    let synthetic = suite
        .evidence_for(PhysicalIntegrityAcceptanceSuite::SyntheticShortcutRejection)
        .unwrap();

    assert_synthetic_rejection(
        synthetic.synthetic_rejections(),
        SyntheticCloseoutShortcutAttempt::ExpectedErrorsOnly,
    );
    assert_synthetic_rejection(
        synthetic.synthetic_rejections(),
        SyntheticCloseoutShortcutAttempt::InMemoryOnlyBuffers,
    );
    assert_synthetic_rejection(
        synthetic.synthetic_rejections(),
        SyntheticCloseoutShortcutAttempt::FixtureLabelsOnly,
    );
}

#[test]
fn physical_integrity_harness_closeout_rejects_non_integrity_lane_transcripts() {
    let (plan, transcript) = lane_plan_and_transcript(RoadmapLaneFamily::BufferPool);
    let denial = IntegrityHarnessTranscriptEvidence::from_suite_plan_and_transcript(
        PhysicalIntegrityAcceptanceSuite::HarnessTranscript,
        &plan,
        &transcript,
        IntegrityHarnessExecutionEvidence::harness_transcript(1),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::WrongHarnessLane(
            PhysicalIntegrityAcceptanceSuite::HarnessTranscript
        )
    );
}

#[test]
fn physical_integrity_synthetic_closeout_does_not_count_mismatched_transcript_as_rejection() {
    let (_, input_transcript) = lane_plan_and_transcript(RoadmapLaneFamily::BufferPool);
    let (_, request_transcript) = lane_plan_and_transcript(RoadmapLaneFamily::Integrity);
    let input = SyntheticCloseoutShortcutInput::from_transcript(
        SyntheticCloseoutShortcutAttempt::FixtureLabelsOnly,
        &input_transcript,
    );

    let admitted = SyntheticCloseoutShortcutRejectionReport::attempt_shortcut_certification(
        input,
        &request_transcript,
    );

    assert!(admitted.is_ok());
}

#[test]
fn closeout_rejects_recovery_suite_evidence_copied_from_another_handoff() {
    let request_handoff = intact_readiness("request-handoff");
    let copied_handoff = intact_readiness("copied-handoff");
    let suite = complete_physical_integrity_closeout_suite(&copied_handoff);

    let denial =
        PhysicalIntegrityCertificationBundle::close(PhysicalIntegrityCloseoutRequest::new(
            suite,
            physical_integrity_readiness(),
            request_handoff,
        ))
        .unwrap_err();

    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::RecoveryHandoffEvidenceMismatch
    );
}

fn assert_closeout_harness_matrix(harnesses: &[crate::IntegrityCloseoutHarnessSummary]) {
    let expected = [
        (
            PhysicalIntegrityAcceptanceSuite::CorruptionLocalization,
            IntegrityCloseoutEvidenceFamily::CorruptionLocalization,
            PhysicalScenarioDriverKind::ByteFlipInjection,
            PhysicalScenarioObserverKind::DamageClassification,
            PhysicalProofOracleKind::DamageLocalizesToPhysicalBoundary,
        ),
        (
            PhysicalIntegrityAcceptanceSuite::BoundaryDenial,
            IntegrityCloseoutEvidenceFamily::BoundaryDenial,
            PhysicalScenarioDriverKind::IntegrityBoundaryDenialProbe,
            PhysicalScenarioObserverKind::PreDecodeIntegrityAdmission,
            PhysicalProofOracleKind::DamagedBytesDenyBeforeLogicalDecode,
        ),
        (
            PhysicalIntegrityAcceptanceSuite::HarnessTranscript,
            IntegrityCloseoutEvidenceFamily::HarnessTranscript,
            PhysicalScenarioDriverKind::PersistedFileDevice,
            PhysicalScenarioObserverKind::EvidenceExport,
            PhysicalProofOracleKind::TranscriptPreservesEvidence,
        ),
        (
            PhysicalIntegrityAcceptanceSuite::SyntheticShortcutRejection,
            IntegrityCloseoutEvidenceFamily::SyntheticShortcutRejection,
            PhysicalScenarioDriverKind::SyntheticShortcutAttempt,
            PhysicalScenarioObserverKind::MaterializationShortcut,
            PhysicalProofOracleKind::SyntheticShortcutRejected,
        ),
        (
            PhysicalIntegrityAcceptanceSuite::RecoveryIntegrityHandoff,
            IntegrityCloseoutEvidenceFamily::RecoveryIntegrityHandoff,
            PhysicalScenarioDriverKind::RecoveryIntegrityHandoffProbe,
            PhysicalScenarioObserverKind::RecoveryIntegrityHandoff,
            PhysicalProofOracleKind::RecoveryHandoffContainsOnlyIntegrityEvidence,
        ),
        (
            PhysicalIntegrityAcceptanceSuite::LineCapComposition,
            IntegrityCloseoutEvidenceFamily::LineCapComposition,
            PhysicalScenarioDriverKind::IntegrityCompositionDiscovery,
            PhysicalScenarioObserverKind::IntegrityComposition,
            PhysicalProofOracleKind::IntegrityCompositionChecked,
        ),
    ];
    for (suite, family, driver, observer, oracle) in expected {
        assert!(harnesses.iter().any(|harness| {
            harness.acceptance_suite() == suite
                && harness.evidence_family() == family
                && harness.lane_family() == RoadmapLaneFamily::Integrity
                && harness.driver_families().contains(&driver)
                && harness.observer_families().contains(&observer)
                && harness.oracle_families().contains(&oracle)
        }));
    }
}
