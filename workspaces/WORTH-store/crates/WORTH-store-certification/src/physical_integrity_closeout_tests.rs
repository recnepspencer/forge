use crate::{
    chunk_integrity_without_blob_lifecycle_tests::inspect_chunk_denial,
    close_s3_physical_integrity_from_executed_evidence,
    physical_integrity_closeout_bundle::PhysicalIntegrityCloseoutRequest,
    courtroom::harness::test_support::physical_integrity_closeout_line_cap_test_support::line_cap_composition_evidence,
    courtroom::harness::test_support::physical_integrity_closeout_test_support::{
        assert_synthetic_rejection, complete_s3_closeout_evidence, complete_s3_closeout_suite,
        copied_s2_synthetic_rejections, executed_boundary_denial_evidence,
        executed_localization_evidence, lane_plan_and_transcript, s3_harness, s3_readiness,
    },
    courtroom::harness::test_support::s4_integrity_handoff_test_support::intact_readiness,
    PhysicalIntegrityCertificationBundle, PhysicalIntegrityCloseoutDenial,
    PhysicalIntegrityCloseoutSuite, PhysicalIntegrityCloseoutSuiteEvidence,
    PhysicalProofOracleKind, PhysicalScenarioDriverKind, PhysicalScenarioObserverKind,
    RoadmapLaneFamily, S3AcceptanceSuiteKind, S3CloseoutDenialBoundary, S3CloseoutEvidenceFamily,
    S3CloseoutHarnessExecutionEvidence, S3ExecutedBoundaryDenialEvidence,
    S3ExecutedCorruptionLocalizationEvidence, S3HarnessTranscriptEvidence,
    SyntheticCloseoutShortcutAttempt, SyntheticCloseoutShortcutInput,
    SyntheticCloseoutShortcutRejectionReport,
};
use worth_store_physical_integrity::{
    ChecksumAlgorithmClaim, ChecksumAlgorithmId, ChunkIntegrityDenialKind,
};

#[test]
fn physical_integrity_closeout_publishes_s3_report_and_s4_handoff() {
    let s4_readiness = intact_readiness("s3-closeout-handoff");
    let expected_identity = s4_readiness.payload().identity().clone();
    let expected_counters = s4_readiness.counters();
    let closeout = close_s3_physical_integrity_from_executed_evidence(
        s3_readiness(),
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
    assert_eq!(closeout.report().s4_handoff_identity(), &expected_identity);
    assert_eq!(closeout.report().s4_counters(), expected_counters);
    assert_eq!(closeout.report().suite_harnesses().len(), 6);
    assert!(closeout.report().suite_harnesses().iter().all(|harness| {
        harness.lane_family() == RoadmapLaneFamily::Integrity
            && !harness.driver_families().is_empty()
            && !harness.observer_families().is_empty()
            && !harness.oracle_families().is_empty()
    }));
    assert_closeout_harness_matrix(closeout.report().suite_harnesses());
    assert_eq!(closeout.suite().evidence().len(), 6);
    assert!(closeout.s4_handoff().proves_no_raw_bytes_crossed());
    assert!(!closeout.s4_handoff().claims_recovery());
}

#[test]
fn closeout_facade_runs_s3_harness_from_executed_evidence() {
    let s4_readiness = intact_readiness("facade");
    let closeout = close_s3_physical_integrity_from_executed_evidence(
        s3_readiness(),
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
    let mut evidence = complete_s3_closeout_evidence(&s4_readiness);
    evidence.retain(|row| row.acceptance_suite() != S3AcceptanceSuiteKind::S4IntegrityHandoff);
    let denial = PhysicalIntegrityCloseoutSuite::admit(evidence).unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::MissingAcceptanceSuite(
            S3AcceptanceSuiteKind::S4IntegrityHandoff
        )
    );

    let mut evidence = complete_s3_closeout_evidence(&s4_readiness);
    let mut denials = executed_boundary_denial_evidence();
    denials.retain(|row| row.boundary() != S3CloseoutDenialBoundary::DigestAsChecksum);
    evidence[1] = PhysicalIntegrityCloseoutSuiteEvidence::boundary_denial(
        s3_harness(
            S3AcceptanceSuiteKind::BoundaryDenial,
            S3CloseoutHarnessExecutionEvidence::boundary_denial(&denials),
        ),
        denials,
    );
    let denial = PhysicalIntegrityCloseoutSuite::admit(evidence).unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::MissingBoundaryDenial(
            S3CloseoutDenialBoundary::DigestAsChecksum
        )
    );
}

#[test]
fn closeout_suite_rejects_harness_copied_across_suite_rows() {
    let s4_readiness = intact_readiness("copied-harness-suite");
    let mut evidence = complete_s3_closeout_evidence(&s4_readiness);
    let wrong_harness = evidence
        .iter()
        .find(|row| row.acceptance_suite() == S3AcceptanceSuiteKind::CorruptionLocalization)
        .unwrap()
        .harness()
        .clone();
    let denials = executed_boundary_denial_evidence();
    evidence[1] = PhysicalIntegrityCloseoutSuiteEvidence::boundary_denial(wrong_harness, denials);

    let denial = PhysicalIntegrityCloseoutSuite::admit(evidence).unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::MismatchedHarnessSuite(
            S3AcceptanceSuiteKind::BoundaryDenial
        )
    );
}

#[test]
fn closeout_rejects_mismatched_executed_artifacts_instead_of_labels() {
    let wrong_checksum_denial =
        ChecksumAlgorithmId::admit_claim(ChecksumAlgorithmClaim::checksum_as_authenticity_claim())
            .unwrap_err();
    let denial =
        S3ExecutedBoundaryDenialEvidence::from_digest_as_checksum_denial(wrong_checksum_denial)
            .unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::UnexecutedBoundaryDenial(
            S3CloseoutDenialBoundary::DigestAsChecksum
        )
    );

    let chunk_denial = inspect_chunk_denial("payload-damage", b"DATA", 1024);
    assert_eq!(
        chunk_denial.kind(),
        ChunkIntegrityDenialKind::ChunkPayloadDamage
    );
    let denial = S3ExecutedCorruptionLocalizationEvidence::from_extent_damage_denial(&chunk_denial)
        .unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::UnexecutedCorruptionLocalization(
            crate::S3CorruptionLocalizationBoundary::ExtentDamage
        )
    );
}

#[test]
fn closeout_rejects_synthetic_reports_copied_from_another_transcript() {
    let s4_readiness = intact_readiness("copied-synthetic");
    let mut evidence = complete_s3_closeout_evidence(&s4_readiness);
    let copied = copied_s2_synthetic_rejections();
    evidence[3] = PhysicalIntegrityCloseoutSuiteEvidence::synthetic_rejection(
        s3_harness(
            S3AcceptanceSuiteKind::SyntheticShortcutRejection,
            S3CloseoutHarnessExecutionEvidence::synthetic_rejection(&copied),
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
fn s3_synthetic_closeout_rejects_expected_errors_buffers_and_fixture_labels() {
    let s4_readiness = intact_readiness("synthetic-closeout");
    let suite = complete_s3_closeout_suite(&s4_readiness);
    let synthetic = suite
        .evidence_for(S3AcceptanceSuiteKind::SyntheticShortcutRejection)
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
fn s3_harness_closeout_rejects_non_integrity_lane_transcripts() {
    let (plan, transcript) = lane_plan_and_transcript(RoadmapLaneFamily::BufferPool);
    let denial = S3HarnessTranscriptEvidence::from_suite_plan_and_transcript(
        S3AcceptanceSuiteKind::HarnessTranscript,
        &plan,
        &transcript,
        S3CloseoutHarnessExecutionEvidence::harness_transcript(1),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::WrongHarnessLane(S3AcceptanceSuiteKind::HarnessTranscript)
    );
}

#[test]
fn s3_synthetic_closeout_does_not_count_mismatched_transcript_as_rejection() {
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
fn closeout_rejects_s4_suite_evidence_copied_from_another_handoff() {
    let request_handoff = intact_readiness("request-handoff");
    let copied_handoff = intact_readiness("copied-handoff");
    let suite = complete_s3_closeout_suite(&copied_handoff);

    let denial = PhysicalIntegrityCertificationBundle::close(
        PhysicalIntegrityCloseoutRequest::new(suite, s3_readiness(), request_handoff),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::S4HandoffEvidenceMismatch
    );
}

fn assert_closeout_harness_matrix(harnesses: &[crate::S3CloseoutSuiteHarnessSummary]) {
    let expected = [
        (
            S3AcceptanceSuiteKind::CorruptionLocalization,
            S3CloseoutEvidenceFamily::CorruptionLocalization,
            PhysicalScenarioDriverKind::S3ByteFlipInjection,
            PhysicalScenarioObserverKind::S3DamageClassification,
            PhysicalProofOracleKind::S3DamageLocalizesToPhysicalBoundary,
        ),
        (
            S3AcceptanceSuiteKind::BoundaryDenial,
            S3CloseoutEvidenceFamily::BoundaryDenial,
            PhysicalScenarioDriverKind::S3BoundaryDenialProbe,
            PhysicalScenarioObserverKind::S3PreDecodeAdmission,
            PhysicalProofOracleKind::S3DamagedBytesDenyBeforeLogicalDecode,
        ),
        (
            S3AcceptanceSuiteKind::HarnessTranscript,
            S3CloseoutEvidenceFamily::HarnessTranscript,
            PhysicalScenarioDriverKind::PersistedFileDevice,
            PhysicalScenarioObserverKind::EvidenceExport,
            PhysicalProofOracleKind::TranscriptPreservesEvidence,
        ),
        (
            S3AcceptanceSuiteKind::SyntheticShortcutRejection,
            S3CloseoutEvidenceFamily::SyntheticShortcutRejection,
            PhysicalScenarioDriverKind::S3SyntheticShortcutAttempt,
            PhysicalScenarioObserverKind::MaterializationShortcut,
            PhysicalProofOracleKind::S3SyntheticShortcutRejected,
        ),
        (
            S3AcceptanceSuiteKind::S4IntegrityHandoff,
            S3CloseoutEvidenceFamily::S4IntegrityHandoff,
            PhysicalScenarioDriverKind::S3RecoveryHandoffProbe,
            PhysicalScenarioObserverKind::S3RecoveryHandoff,
            PhysicalProofOracleKind::S3RecoveryHandoffContainsOnlyIntegrityEvidence,
        ),
        (
            S3AcceptanceSuiteKind::LineCapComposition,
            S3CloseoutEvidenceFamily::LineCapComposition,
            PhysicalScenarioDriverKind::S3LineCapDiscovery,
            PhysicalScenarioObserverKind::S3LineCapComposition,
            PhysicalProofOracleKind::S3LineCapCompositionChecked,
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
