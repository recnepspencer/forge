use super::{certify_basis_lifecycle_performance_slopes, BasisLifecycleSlopeFamily};

#[test]
fn performance_slope_report_has_one_row_per_required_output() {
    let report = certify_basis_lifecycle_performance_slopes();

    for family in [
        BasisLifecycleSlopeFamily::Normalization,
        BasisLifecycleSlopeFamily::Eligibility,
        BasisLifecycleSlopeFamily::LowerRuntimeBinding,
        BasisLifecycleSlopeFamily::ScopedUse,
        BasisLifecycleSlopeFamily::ReceiptEmission,
        BasisLifecycleSlopeFamily::EnvelopeMaterialization,
        BasisLifecycleSlopeFamily::SupportLookup,
    ] {
        let row = report
            .rows()
            .iter()
            .find(|row| row.family() == family)
            .unwrap_or_else(|| panic!("missing slope family {}", family.as_str()));
        assert_eq!(row.operation_lane(), "observation");
        assert!(!row.counter_digest().is_empty());
        assert!(!row.bounded_by().is_empty());
        assert_eq!(
            report.digest_for_output(family.output_name()),
            Some(row.slope_digest())
        );
    }

    assert_eq!(report.rows().len(), 7);
    assert!(!report.report_digest().is_empty());
}

#[test]
fn performance_slope_digests_are_stage_specific() {
    let report = certify_basis_lifecycle_performance_slopes();
    let mut digests = report
        .rows()
        .iter()
        .map(|row| row.slope_digest())
        .collect::<Vec<_>>();
    digests.sort_unstable();
    digests.dedup();

    assert_eq!(digests.len(), report.rows().len());
}

#[test]
fn performance_slope_rows_expose_exact_counter_claims() {
    let report = certify_basis_lifecycle_performance_slopes();

    let normalization = row_for(&report, BasisLifecycleSlopeFamily::Normalization);
    assert_eq!(normalization.counters().raw_intent_width(), 1);
    assert_eq!(normalization.counters().normalized_family_count(), 1);
    assert_eq!(normalization.counters().source_path_count(), 1);

    let eligibility = row_for(&report, BasisLifecycleSlopeFamily::Eligibility);
    assert_eq!(eligibility.counters().eligibility_rows_consulted(), 1);
    assert_eq!(
        eligibility.counters().lower_runtime_evidence_check_count(),
        0
    );

    let lower_runtime = row_for(&report, BasisLifecycleSlopeFamily::LowerRuntimeBinding);
    assert_eq!(
        lower_runtime
            .counters()
            .lower_runtime_binding_attempt_count(),
        1
    );
    assert_eq!(
        lower_runtime
            .counters()
            .lower_runtime_readmission_check_count(),
        1
    );
    assert_eq!(lower_runtime.counters().retained_evidence_lookup_width(), 1);
    assert_eq!(
        lower_runtime
            .counters()
            .lower_runtime_mismatch_denial_count(),
        0
    );

    let scoped = row_for(&report, BasisLifecycleSlopeFamily::ScopedUse);
    assert_eq!(scoped.counters().scoped_capability_construction_count(), 1);

    let receipt = row_for(&report, BasisLifecycleSlopeFamily::ReceiptEmission);
    assert_eq!(receipt.counters().basis_receipt_emission_count(), 1);
    assert_eq!(receipt.counters().retained_evidence_lookup_width(), 1);

    let envelope = row_for(&report, BasisLifecycleSlopeFamily::EnvelopeMaterialization);
    assert_eq!(
        envelope.counters().basis_envelope_materialization_count(),
        1
    );

    let support = row_for(&report, BasisLifecycleSlopeFamily::SupportLookup);
    assert_eq!(support.counters().basis_support_lookup_count(), 1);
    assert_eq!(support.counters().basis_support_lookup_width(), 1);
}

fn row_for(
    report: &super::BasisLifecyclePerformanceSlopeReport,
    family: BasisLifecycleSlopeFamily,
) -> &super::BasisLifecycleSlopeDigest {
    report
        .rows()
        .iter()
        .find(|row| row.family() == family)
        .unwrap_or_else(|| panic!("missing slope family {}", family.as_str()))
}
