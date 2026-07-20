use crate::{
    courtroom::harness::test_support::bounded_memory_closeout_test_support::{
        background_bundle, foundational_receipt, foundational_receipt_with_protected_view,
        harness_evidence, harness_evidence_for_class, harness_evidence_without_acceptance_suite,
        physical_substrate_model_snapshot, pressure_bundles, synthetic_rejections,
    },
    BoundedMemoryCloseoutReport, BoundedMemoryOperationKind, BoundedMemoryResidencySuite,
    BoundedMemoryResidencySuiteDenial, BoundedOperationEnvelopeCounters,
    BoundedOperationEnvelopeReport, BufferPoolCertificationBundle,
    BufferPoolCertificationBundleDenial, LargeStorePressureClass, MemoryBoundaryDenialKind,
    RoadmapLaneFamily, S2AcceptanceSuiteKind, ScenarioDenialBoundary,
    SyntheticCloseoutShortcutAttempt,
};
use worth_store_contracts::DeniedBoundaryKind;

#[test]
fn bounded_memory_closeout_builds_the_integrity_algorithm_payload_without_claiming_readiness() {
    let payload =
        crate::courtroom::physical_integrity::readiness_handoff::model_payload_from_closeout(
            complete_closeout_report(),
            physical_substrate_model_snapshot(),
        )
        .unwrap();

    assert!(payload.protected_view_capability().is_concrete());
    assert!(payload.verifier_resident_envelope().is_bounded());
    assert!(payload.scrub_allocation_envelope().is_bounded());
    assert!(payload.inspection_lifetime_law().is_lease_scoped());
    assert!(payload.no_materialization_witness().forbids_whole_store());
    assert!(payload.no_materialization_witness().forbids_whole_object());
    assert_eq!(payload.denial_behavior().named_denial_count(), 6);
    assert!(payload
        .denial_behavior()
        .contains(DeniedBoundaryKind::ForgedViewAccess));
    assert!(payload
        .buffer_pool_authority_recap()
        .view_admission_authority_proven());
}

#[test]
fn closeout_rejects_missing_bounded_operation_or_denial() {
    let mut reports = operation_reports();
    reports.retain(|report| report.operation() != BoundedMemoryOperationKind::LargeRecordStreaming);
    let operation_denial = BoundedMemoryResidencySuite::admit(
        reports,
        &MemoryBoundaryDenialKind::ALL,
        harness_evidence(),
    )
    .unwrap_err();
    assert_eq!(
        operation_denial,
        BoundedMemoryResidencySuiteDenial::MissingOperation(
            BoundedMemoryOperationKind::LargeRecordStreaming
        )
    );

    let denial = BoundedMemoryResidencySuite::admit(
        operation_reports(),
        &MemoryBoundaryDenialKind::ALL[..5],
        harness_evidence(),
    )
    .unwrap_err();
    assert_eq!(
        denial,
        BoundedMemoryResidencySuiteDenial::MissingDenial(
            MemoryBoundaryDenialKind::ForgedViewAccess
        )
    );
}

#[test]
fn bundle_rejects_missing_large_store_pressure_class() {
    let mut pressure = pressure_bundles();
    pressure.retain(|bundle| bundle.pressure_class() != LargeStorePressureClass::StreamingPressure);
    let denial = BufferPoolCertificationBundle::admit(
        suite(),
        pressure,
        background_bundle(),
        foundational_receipt(),
        foundational_receipt_with_protected_view().1,
        synthetic_rejections(),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        BufferPoolCertificationBundleDenial::MissingPressureClass(
            LargeStorePressureClass::StreamingPressure
        )
    );
}

#[test]
fn bundle_rejects_harness_closeout_not_matching_pressure_bundles() {
    let suite = BoundedMemoryResidencySuite::admit(
        operation_reports(),
        &MemoryBoundaryDenialKind::ALL,
        harness_evidence_for_class(LargeStorePressureClass::BarelyOverBudget),
    )
    .unwrap();
    let (foundational, protected_view) = foundational_receipt_with_protected_view();
    let denial = BufferPoolCertificationBundle::admit(
        suite,
        pressure_bundles(),
        background_bundle(),
        foundational,
        protected_view,
        synthetic_rejections(),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        BufferPoolCertificationBundleDenial::MissingHarnessPressureClass(
            LargeStorePressureClass::ModeratelyOverBudget
        )
    );
}

#[test]
fn bundle_rejects_harness_closeout_missing_acceptance_suite_transcript() {
    let suite = BoundedMemoryResidencySuite::admit(
        operation_reports(),
        &MemoryBoundaryDenialKind::ALL,
        harness_evidence_without_acceptance_suite(S2AcceptanceSuiteKind::IntegrityReadinessHandoff),
    )
    .unwrap();
    let (foundational, protected_view) = foundational_receipt_with_protected_view();
    let denial = BufferPoolCertificationBundle::admit(
        suite,
        pressure_bundles(),
        background_bundle(),
        foundational,
        protected_view,
        synthetic_rejections(),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        BufferPoolCertificationBundleDenial::MissingHarnessAcceptanceSuite(
            S2AcceptanceSuiteKind::IntegrityReadinessHandoff
        )
    );
}

#[test]
fn harness_closeout_names_families_per_acceptance_suite_transcript() {
    let harness = harness_evidence();

    assert_eq!(
        harness.transcript_families() as usize,
        harness.suite_transcripts().len()
    );

    for suite in S2AcceptanceSuiteKind::ALL {
        let transcript = harness.transcript_for_acceptance_suite(suite).unwrap();
        assert_eq!(transcript.acceptance_suite(), suite);
        assert_eq!(transcript.lane_family(), RoadmapLaneFamily::BufferPool);
        assert!(transcript.pressure_class().is_some());
        assert!(transcript.names_required_families());
        assert!(!transcript.driver_families().is_empty());
        assert!(!transcript.observer_families().is_empty());
        assert!(!transcript.oracle_families().is_empty());
    }
}

#[test]
fn synthetic_shortcut_rejections_retain_named_boundaries() {
    let reports = synthetic_rejections();

    assert_synthetic_rejection_boundary(
        &reports,
        SyntheticCloseoutShortcutAttempt::LogsOnlyProof,
        ScenarioDenialBoundary::WholeStoreMaterialization,
    );
    assert_synthetic_rejection_boundary(
        &reports,
        SyntheticCloseoutShortcutAttempt::SameRunSelfComparison,
        ScenarioDenialBoundary::BypassedLoweredPlan,
    );
    assert_synthetic_rejection_boundary(
        &reports,
        SyntheticCloseoutShortcutAttempt::SmallFixtureOnly,
        ScenarioDenialBoundary::BypassedObserverTrace,
    );
    assert_synthetic_rejection_boundary(
        &reports,
        SyntheticCloseoutShortcutAttempt::TestSupportOwnedOracleMeaning,
        ScenarioDenialBoundary::TestSupportOwnedMeaning,
    );
}

#[test]
fn bundle_requires_synthetic_shortcut_rejections_before_closeout() {
    let mut incomplete_rejections = synthetic_rejections();
    incomplete_rejections.retain(|report| {
        report.rejected_attempt() != SyntheticCloseoutShortcutAttempt::TestSupportOwnedOracleMeaning
    });
    let denial = BufferPoolCertificationBundle::admit(
        suite(),
        pressure_bundles(),
        background_bundle(),
        foundational_receipt(),
        foundational_receipt_with_protected_view().1,
        incomplete_rejections,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        BufferPoolCertificationBundleDenial::MissingSyntheticRejection(
            SyntheticCloseoutShortcutAttempt::TestSupportOwnedOracleMeaning
        )
    );
}

#[test]
fn bundle_rejects_symbolic_operation_envelope_counters() {
    let mut reports = operation_reports();
    for report in &mut reports {
        if report.operation() == BoundedMemoryOperationKind::AdmittedRead {
            *report = BoundedOperationEnvelopeReport::from_counters(
                BoundedMemoryOperationKind::AdmittedRead,
                BoundedOperationEnvelopeCounters::exact(8192, 1, 0, 64, 64, 0),
            )
            .unwrap();
        }
    }
    let suite = BoundedMemoryResidencySuite::admit(
        reports,
        &MemoryBoundaryDenialKind::ALL,
        harness_evidence(),
    )
    .unwrap();
    let (foundational, protected_view) = foundational_receipt_with_protected_view();
    let denial = BufferPoolCertificationBundle::admit(
        suite,
        pressure_bundles(),
        background_bundle(),
        foundational,
        protected_view,
        synthetic_rejections(),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        BufferPoolCertificationBundleDenial::OperationEnvelopeMismatch(
            BoundedMemoryOperationKind::AdmittedRead
        )
    );
}

fn complete_closeout_report() -> BoundedMemoryCloseoutReport {
    let (foundational, protected_view) = foundational_receipt_with_protected_view();
    BoundedMemoryCloseoutReport::close(
        BufferPoolCertificationBundle::admit(
            suite(),
            pressure_bundles(),
            background_bundle(),
            foundational,
            protected_view,
            synthetic_rejections(),
        )
        .unwrap(),
    )
    .unwrap()
}

fn suite() -> BoundedMemoryResidencySuite {
    BoundedMemoryResidencySuite::admit(
        operation_reports(),
        &MemoryBoundaryDenialKind::ALL,
        harness_evidence(),
    )
    .unwrap()
}

fn operation_reports() -> Vec<crate::BoundedOperationEnvelopeReport> {
    let background = background_bundle();
    crate::courtroom::harness::test_support::bounded_memory_closeout_test_support::operation_reports(
        &foundational_receipt(),
        &background,
    )
}

fn assert_synthetic_rejection_boundary(
    reports: &[crate::SyntheticCloseoutShortcutRejectionReport],
    attempt: SyntheticCloseoutShortcutAttempt,
    boundary: ScenarioDenialBoundary,
) {
    assert!(reports.iter().any(|report| {
        report.rejected_attempt() == attempt && report.rejected_boundary() == boundary
    }));
}
