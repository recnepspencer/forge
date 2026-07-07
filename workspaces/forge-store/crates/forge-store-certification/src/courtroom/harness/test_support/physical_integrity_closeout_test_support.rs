pub(crate) use super::physical_integrity_closeout_harness_test_support::{
    lane_plan_and_transcript, s3_harness,
};
use super::{
    bounded_memory_closeout_test_support::{
        background_bundle, foundational_receipt, foundational_receipt_with_protected_view,
        harness_evidence, operation_reports, pressure_bundles, s2_readiness,
        synthetic_rejections as bounded_memory_synthetic_rejections,
    },
    physical_container_integrity_test_support::{
        frame_start, inspect_page_denial, page_payload_with_record,
    },
    physical_integrity_closeout_harness_test_support::s3_synthetic_transcript,
    physical_integrity_closeout_line_cap_test_support::line_cap_composition_evidence,
    pre_decode_physical_admission_test_support::{
        crc32c, deny_checked_frame, stale_validation, with_pre_decode_admission,
    },
};
use crate::{
    chunk_integrity_without_blob_lifecycle_tests::inspect_chunk_denial,
    derived_index_damage_tests::{damaged_authority_denial, inspect_with_damaged_authority},
    scrub_execution_tests::resident_memory_over_budget_scrub_denial,
    wal_frame_integrity_tests::{inspect_denial, wal_payload},
    BoundedMemoryCloseoutReport, BoundedMemoryResidencySuite, BufferPoolCertificationBundle,
    PhysicalIntegrityCloseoutSuite, PhysicalIntegrityCloseoutSuiteEvidence, S2BoundaryDenialKind,
    S3AcceptanceSuiteKind, S3CloseoutHarnessExecutionEvidence, S3ExecutedBoundaryDenialEvidence,
    S3ExecutedCorruptionLocalizationEvidence, S3S4HandoffCloseoutEvidence,
    SyntheticCloseoutShortcutAttempt, SyntheticCloseoutShortcutInput,
    SyntheticCloseoutShortcutRejectionReport,
};
use forge_store_contracts::StableDigest;
use forge_store_physical_format::PhysicalFrameKind;
use forge_store_physical_integrity::{
    ChecksumAlgorithmClaim, ChecksumAlgorithmId, DeclaredPhysicalChecksum,
    ExecutedQuarantineFinding, IntegrityEntryDenial, IntegrityEntryDenialKind,
    PhysicalIntegrityAdmissionRequest, PhysicalQuarantineAuthority, PreDecodePhysicalDenial,
    PreDecodePhysicalDenialKind, QuarantineLifecyclePosture, QuarantineSealRequest,
};

pub(crate) fn complete_s3_closeout_suite(
    s4_readiness: &forge_store_recovery_physics::S4RecoveryPhysicsIntegrityReadiness,
) -> PhysicalIntegrityCloseoutSuite {
    PhysicalIntegrityCloseoutSuite::admit(complete_s3_closeout_evidence(s4_readiness)).unwrap()
}

pub(crate) fn complete_s3_closeout_evidence(
    s4_readiness: &forge_store_recovery_physics::S4RecoveryPhysicsIntegrityReadiness,
) -> Vec<PhysicalIntegrityCloseoutSuiteEvidence> {
    let localization = executed_localization_evidence();
    let denials = executed_boundary_denial_evidence();
    let synthetic_transcript = s3_synthetic_transcript();
    let synthetic_rejections = s3_synthetic_rejections(&synthetic_transcript);
    let synthetic_harness = s3_harness(
        S3AcceptanceSuiteKind::SyntheticShortcutRejection,
        S3CloseoutHarnessExecutionEvidence::synthetic_rejection(&synthetic_rejections),
    );
    let s4_handoff = S3S4HandoffCloseoutEvidence::from_readiness(s4_readiness);
    let line_cap = line_cap_composition_evidence();
    vec![
        PhysicalIntegrityCloseoutSuiteEvidence::corruption_localization(
            s3_harness(
                S3AcceptanceSuiteKind::CorruptionLocalization,
                S3CloseoutHarnessExecutionEvidence::corruption_localization(&localization),
            ),
            localization,
        ),
        PhysicalIntegrityCloseoutSuiteEvidence::boundary_denial(
            s3_harness(
                S3AcceptanceSuiteKind::BoundaryDenial,
                S3CloseoutHarnessExecutionEvidence::boundary_denial(&denials),
            ),
            denials,
        ),
        PhysicalIntegrityCloseoutSuiteEvidence::harness_transcript(s3_harness(
            S3AcceptanceSuiteKind::HarnessTranscript,
            S3CloseoutHarnessExecutionEvidence::harness_transcript(1),
        )),
        PhysicalIntegrityCloseoutSuiteEvidence::synthetic_rejection(
            synthetic_harness,
            synthetic_rejections,
        ),
        PhysicalIntegrityCloseoutSuiteEvidence::s4_handoff(
            s3_harness(
                S3AcceptanceSuiteKind::S4IntegrityHandoff,
                S3CloseoutHarnessExecutionEvidence::s4_handoff(&s4_handoff),
            ),
            s4_handoff,
        ),
        PhysicalIntegrityCloseoutSuiteEvidence::line_cap_composition(
            s3_harness(
                S3AcceptanceSuiteKind::LineCapComposition,
                S3CloseoutHarnessExecutionEvidence::line_cap_composition(&line_cap),
            ),
            line_cap,
        ),
    ]
}

pub(crate) fn executed_boundary_denial_evidence() -> Vec<S3ExecutedBoundaryDenialEvidence> {
    let forged_checksum = inspect_denial(wal_payload("crc32c", 4, "checksum-fail", b"DATA"));
    let digest = StableDigest::new("phase15-closeout-digest").unwrap();
    let digest_denial = ChecksumAlgorithmId::admit_claim(
        ChecksumAlgorithmClaim::artifact_digest_substitution(&digest),
    )
    .unwrap_err();
    let authenticity_denial =
        ChecksumAlgorithmId::admit_claim(ChecksumAlgorithmClaim::checksum_as_authenticity_claim())
            .unwrap_err();
    let raw_entry =
        IntegrityEntryDenial::new(IntegrityEntryDenialKind::MissingProtectedPhysicalByteView);
    let copied_quarantine = copied_quarantine_denial();
    let scrub = resident_memory_over_budget_scrub_denial();

    vec![
        S3ExecutedBoundaryDenialEvidence::from_forged_checksum_denial(&forged_checksum).unwrap(),
        S3ExecutedBoundaryDenialEvidence::from_digest_as_checksum_denial(digest_denial).unwrap(),
        S3ExecutedBoundaryDenialEvidence::from_checksum_authenticity_denial(authenticity_denial)
            .unwrap(),
        S3ExecutedBoundaryDenialEvidence::from_raw_byte_entry_denial(raw_entry).unwrap(),
        S3ExecutedBoundaryDenialEvidence::from_copied_quarantine_record_denial(copied_quarantine)
            .unwrap(),
        S3ExecutedBoundaryDenialEvidence::from_over_budget_scrub_plan_denial(scrub).unwrap(),
    ]
}

pub(crate) fn s3_readiness() -> forge_store_readiness::S3PhysicalIntegrityReadiness {
    complete_bounded_memory_closeout()
        .publish_s3_physical_integrity_readiness(s2_readiness())
        .unwrap()
}

pub(crate) fn copied_s2_synthetic_rejections() -> Vec<SyntheticCloseoutShortcutRejectionReport> {
    bounded_memory_synthetic_rejections()
}

pub(crate) fn assert_synthetic_rejection(
    reports: &[crate::SyntheticCloseoutShortcutRejectionReport],
    attempt: SyntheticCloseoutShortcutAttempt,
) {
    assert!(reports.iter().any(|report| {
        report.rejected_attempt() == attempt
            && report.rejected_boundary() == attempt.required_boundary()
    }));
}

pub(crate) fn executed_localization_evidence() -> Vec<S3ExecutedCorruptionLocalizationEvidence> {
    let byte_flip = deny_checked_frame(
        b"byte-flip-source",
        b"byte-flip-sourcf",
        PreDecodePhysicalDenialKind::ChecksumMismatch,
    );
    let mut torn = page_payload_with_record(b"torn-frame");
    let start = frame_start(&torn);
    torn[start + 5] = torn[start + 5].wrapping_add(4);
    let torn = inspect_page_denial(&torn);
    let stale = stale_generation_denial();
    let manifest = damaged_authority_denial();
    let index = inspect_with_damaged_authority();
    let wal = inspect_denial(wal_payload("crc32c", 4, "checksum-fail", b"DATA"));
    let extent = inspect_chunk_denial("extent-boundary-damage", b"DATA", 1024);
    let chunk = inspect_chunk_denial("payload-damage", b"DATA", 1024);

    vec![
        S3ExecutedCorruptionLocalizationEvidence::from_pre_decode_byte_flip(&byte_flip).unwrap(),
        S3ExecutedCorruptionLocalizationEvidence::from_torn_frame_denial(&torn).unwrap(),
        S3ExecutedCorruptionLocalizationEvidence::from_pre_decode_stale_generation(&stale).unwrap(),
        S3ExecutedCorruptionLocalizationEvidence::from_manifest_denial(&manifest).unwrap(),
        S3ExecutedCorruptionLocalizationEvidence::from_index_page_denial(&index).unwrap(),
        S3ExecutedCorruptionLocalizationEvidence::from_wal_frame_denial(&wal).unwrap(),
        S3ExecutedCorruptionLocalizationEvidence::from_extent_damage_denial(&extent).unwrap(),
        S3ExecutedCorruptionLocalizationEvidence::from_chunk_damage_denial(&chunk).unwrap(),
    ]
}

fn stale_generation_denial() -> PreDecodePhysicalDenial {
    let mut denial = None;
    with_pre_decode_admission(b"stale-generation", |admission, _, witness| {
        denial = Some(
            admission
                .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                    stale_validation(),
                    witness,
                    PhysicalFrameKind::RecordFrame,
                    DeclaredPhysicalChecksum::new(crc32c(b"stale-generation")),
                ))
                .unwrap_err(),
        );
    });
    denial.unwrap()
}

fn copied_quarantine_denial() -> forge_store_physical_integrity::QuarantineSealDenial {
    let index_denial = inspect_with_damaged_authority();
    let finding = ExecutedQuarantineFinding::from_index_page_denial(&index_denial).unwrap();
    PhysicalQuarantineAuthority::seal(
        QuarantineSealRequest::from_executed_finding(finding)
            .with_initial_posture(QuarantineLifecyclePosture::SupersededByRecovery),
    )
    .unwrap_err()
}

fn complete_bounded_memory_closeout() -> BoundedMemoryCloseoutReport {
    let (foundational, protected_view) = foundational_receipt_with_protected_view();
    BoundedMemoryCloseoutReport::close(
        BufferPoolCertificationBundle::admit(
            bounded_memory_suite(),
            pressure_bundles(),
            background_bundle(),
            foundational,
            protected_view,
            bounded_memory_synthetic_rejections(),
        )
        .unwrap(),
    )
    .unwrap()
}

fn bounded_memory_suite() -> BoundedMemoryResidencySuite {
    let background = background_bundle();
    BoundedMemoryResidencySuite::admit(
        operation_reports(&foundational_receipt(), &background),
        &S2BoundaryDenialKind::ALL,
        harness_evidence(),
    )
    .unwrap()
}

fn s3_synthetic_rejections(
    transcript: &crate::PhysicalStoryTranscript,
) -> Vec<SyntheticCloseoutShortcutRejectionReport> {
    [
        SyntheticCloseoutShortcutAttempt::LogsOnlyProof,
        SyntheticCloseoutShortcutAttempt::SameRunSelfComparison,
        SyntheticCloseoutShortcutAttempt::ExpectedErrorsOnly,
        SyntheticCloseoutShortcutAttempt::InMemoryOnlyBuffers,
        SyntheticCloseoutShortcutAttempt::SmallFixtureOnly,
        SyntheticCloseoutShortcutAttempt::FixtureLabelsOnly,
        SyntheticCloseoutShortcutAttempt::TestSupportOwnedOracleMeaning,
    ]
    .into_iter()
    .map(|attempt| {
        let input = SyntheticCloseoutShortcutInput::from_transcript(attempt, transcript);
        let denial = SyntheticCloseoutShortcutRejectionReport::attempt_shortcut_certification(
            input, transcript,
        )
        .unwrap_err();
        SyntheticCloseoutShortcutRejectionReport::from_failed_shortcut_attempt(denial)
    })
    .collect()
}
