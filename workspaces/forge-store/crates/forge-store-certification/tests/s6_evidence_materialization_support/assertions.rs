use forge_store_certification::{
    S6CertificationEvidenceAdoptionReceipt, S6CertificationMaterializationDenial,
    S6CounterStrengthFamily, S6MaterializedCertificationEvidenceBundle,
    S6MaterializedCounterStrength, S6PostAdmissionViolationCause, S6PostAdmissionViolationFamily,
};
use forge_store_readiness::{
    S6ReadinessCertificationCounterFamily, S6ReadinessCertificationCounterStrength,
    S6ReadinessResidualDebtEvidenceKind,
};

pub fn assert_counter_strength_matrix(
    bundle: &S6MaterializedCertificationEvidenceBundle,
    expected: &[(
        S6CounterStrengthFamily,
        S6MaterializedCounterStrength,
        usize,
    )],
) {
    let actual: Vec<_> = bundle
        .counter_strengths()
        .iter()
        .map(|row| (row.family(), row.strength(), row.observed_rows()))
        .collect();
    assert_eq!(actual, expected);
}

pub fn assert_readiness_counter_strength_matrix(
    receipt: &S6CertificationEvidenceAdoptionReceipt,
    expected: &[(
        S6ReadinessCertificationCounterFamily,
        S6ReadinessCertificationCounterStrength,
        usize,
    )],
) {
    let actual: Vec<_> = receipt
        .counter_strengths()
        .iter()
        .map(|row| (row.family(), row.strength(), row.observed_rows()))
        .collect();
    assert_eq!(actual, expected);
}

pub fn assert_fixture_counter_strength_matrix(bundle: &S6MaterializedCertificationEvidenceBundle) {
    use S6CounterStrengthFamily as Family;
    use S6MaterializedCounterStrength as Strength;
    assert_counter_strength_matrix(
        bundle,
        &[
            (
                Family::ForegroundReservation,
                Strength::CertificationOnly,
                2,
            ),
            (Family::BackgroundPacing, Strength::CertificationOnly, 2),
            (Family::QueueExecution, Strength::CertificationOnly, 13),
            (Family::FlushDurability, Strength::Exact, 1),
            (Family::LatencyInterference, Strength::Unavailable, 0),
            (
                Family::LaterReadinessHandoff,
                Strength::CertificationOnly,
                5,
            ),
            (Family::SecureIoPreservation, Strength::Exact, 2),
            (Family::AccessPolicy, Strength::Exact, 2),
            (Family::PostAdmissionViolation, Strength::Derived, 2),
            (Family::QualificationMatrix, Strength::CertificationOnly, 7),
        ],
    );
}

pub fn assert_readiness_fixture_counter_strength_matrix(
    receipt: &S6CertificationEvidenceAdoptionReceipt,
) {
    use S6ReadinessCertificationCounterFamily as Family;
    use S6ReadinessCertificationCounterStrength as Strength;
    assert_readiness_counter_strength_matrix(
        receipt,
        &[
            (
                Family::ForegroundReservation,
                Strength::CertificationOnly,
                2,
            ),
            (Family::BackgroundPacing, Strength::CertificationOnly, 2),
            (Family::QueueExecution, Strength::CertificationOnly, 13),
            (Family::FlushDurability, Strength::Exact, 1),
            (Family::LatencyInterference, Strength::Unavailable, 0),
            (
                Family::LaterReadinessHandoff,
                Strength::CertificationOnly,
                5,
            ),
            (Family::SecureIoPreservation, Strength::Exact, 2),
            (Family::AccessPolicy, Strength::Exact, 2),
            (Family::PostAdmissionViolation, Strength::Derived, 2),
            (Family::QualificationMatrix, Strength::CertificationOnly, 7),
        ],
    );
}

pub fn assert_readiness_residual_debt_matrix(
    receipt: &S6CertificationEvidenceAdoptionReceipt,
    expected: &[(S6ReadinessResidualDebtEvidenceKind, usize)],
) {
    let actual: Vec<_> = receipt
        .residual_debt_rows()
        .iter()
        .map(|row| (row.kind(), row.observed_claims()))
        .collect();
    assert_eq!(actual, expected);
}

pub fn assert_performance_receipts_are_exact_for_fixture(
    bundle: &S6MaterializedCertificationEvidenceBundle,
) {
    assert_receipt_rows(
        bundle.performance().runtime_execution_receipt(),
        &[
            ("store.s6.background.denied", 0),
            ("store.s6.background.yield", 0),
            ("store.s6.flush.rows", 1),
            ("store.s6.foreground.wait", 2),
            ("store.s6.queue.denied", 0),
            ("store.s6.queue.peak_depth", 1),
            ("store.s6.queue.submitted", 4100),
            ("store.s6.queue.violations", 0),
        ],
    );
    assert_receipt_rows(
        bundle.performance().access_policy_receipt(),
        &[
            ("store.s6.access_policy.buffered_admissions", 1),
            ("store.s6.access_policy.denials", 0),
            ("store.s6.access_policy.direct_io_admissions", 0),
            ("store.s6.access_policy.mixed_mode_admissions", 0),
            ("store.s6.access_policy.mmap_admissions", 1),
            ("store.s6.access_policy.rows", 2),
            ("store.s6.access_policy.security_scope_preservations", 2),
            ("store.s6.access_policy.violations", 1),
        ],
    );
    assert_receipt_rows(
        bundle.performance().qualification_receipt(),
        &[
            ("store.s6.harness.real_backend_safety", 1),
            ("store.s6.qualification.certified_support", 1),
            ("store.s6.qualification.rows", 7),
        ],
    );
}

pub fn assert_violation_row(
    bundle: &S6MaterializedCertificationEvidenceBundle,
    family: S6PostAdmissionViolationFamily,
    cause: S6PostAdmissionViolationCause,
    observed_violations: u64,
    strength: S6MaterializedCounterStrength,
) {
    let row = bundle
        .post_admission_violations()
        .iter()
        .find(|row| row.family() == family)
        .expect("violation family should be materialized");
    assert_eq!(row.cause(), cause);
    assert_eq!(row.observed_violations(), observed_violations);
    assert_eq!(row.counter_strength(), strength);
}

pub fn assert_source_denial(
    result: Result<
        forge_store_certification::StoreOwnedS6CertificationMaterializationSources,
        S6CertificationMaterializationDenial,
    >,
    expected: S6CertificationMaterializationDenial,
) {
    assert_eq!(
        result.expect_err("near-miss source binding should be denied"),
        expected
    );
}

fn assert_receipt_rows(
    receipt: &forge_foundational::FoundationalCounterBackedPerformanceReceipt<
        forge_foundational::FoundationalAuthoritativePerformanceClaim,
    >,
    expected: &[(&str, u64)],
) {
    let actual: Vec<_> = receipt
        .counter_rows()
        .iter()
        .map(|row| (row.name().as_str(), row.observed_count()))
        .collect();
    assert_eq!(actual, expected);
}
