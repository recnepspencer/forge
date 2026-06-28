use forge_foundational::{
    materialize_diagnostic_explanation_bundle, materialize_diagnostic_support_report,
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalDiagnosticAssemblyDebt,
    FoundationalDiagnosticCounterSnapshot, FoundationalDiagnosticDeliveryClass,
    FoundationalDiagnosticExplanationBundle, FoundationalDiagnosticExplanationInput,
    FoundationalDiagnosticGapClass, FoundationalDiagnosticGapClosurePosture,
    FoundationalDiagnosticGapTarget, FoundationalDiagnosticNamedGap,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticPartiality, FoundationalDiagnosticRow,
    FoundationalDiagnosticSubject, FoundationalDiagnosticSupportClaimStrength,
    FoundationalDiagnosticSupportInput, FoundationalDiagnosticSupportReport,
    FoundationalDiagnosticSurfaceAvailability, FoundationalProfileSet, FoundationalProfileSetInput,
    RetentionDeliveryProfile, SupportPostureProfile,
};

pub(crate) fn support_report(
    subject: FoundationalDiagnosticSubject,
    rows: &[FoundationalDiagnosticRow],
) -> FoundationalDiagnosticSupportReport {
    materialize_diagnostic_support_report(
        FoundationalDiagnosticSupportInput::new(
            subject,
            FoundationalDiagnosticOutcomeKind::Partial,
            rows.to_vec(),
            Vec::new(),
            Vec::new(),
            FoundationalDiagnosticSurfaceAvailability::retained_hot(),
            FoundationalDiagnosticSupportClaimStrength::DurableSupportReady,
            diagnostic_partiality(rows),
            diagnostic_counter_snapshot(rows),
            Vec::<FoundationalDiagnosticAssemblyDebt>::new(),
        ),
        diagnostic_profile(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .expect("recovery source diagnostics materialize as retained Foundational support")
}

pub(crate) fn explanation_bundle(
    subject: FoundationalDiagnosticSubject,
    rows: &[FoundationalDiagnosticRow],
) -> FoundationalDiagnosticExplanationBundle {
    materialize_diagnostic_explanation_bundle(
        FoundationalDiagnosticExplanationInput::new(
            subject,
            FoundationalDiagnosticOutcomeKind::Partial,
            rows.to_vec(),
            Vec::new(),
            Vec::new(),
            FoundationalDiagnosticSurfaceAvailability::retained_hot(),
            diagnostic_partiality(rows),
            diagnostic_counter_snapshot(rows),
            Vec::<FoundationalDiagnosticAssemblyDebt>::new(),
        ),
        diagnostic_profile(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .expect("recovery source diagnostics materialize as retained Foundational explanation")
}

fn diagnostic_profile() -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Standard,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
    })
    .expect("static recovery diagnostic profile is coherent")
}

fn diagnostic_partiality(rows: &[FoundationalDiagnosticRow]) -> FoundationalDiagnosticPartiality {
    let Some(gap_row) = rows.iter().find(|row| row.code().as_str() == "named-gap") else {
        return FoundationalDiagnosticPartiality::Complete;
    };
    FoundationalDiagnosticPartiality::PartialWithNamedGaps(vec![
        FoundationalDiagnosticNamedGap::new(
            FoundationalDiagnosticGapClass::ReplayEvidenceUnavailable,
            FoundationalDiagnosticGapTarget::Locator(gap_row.locator().clone()),
            FoundationalDiagnosticGapClosurePosture::Deferred,
        ),
    ])
}

fn diagnostic_counter_snapshot(
    rows: &[FoundationalDiagnosticRow],
) -> FoundationalDiagnosticCounterSnapshot {
    let retained = rows
        .iter()
        .filter(|row| {
            !matches!(
                row.outcome_kind(),
                FoundationalDiagnosticOutcomeKind::Unsupported
            )
        })
        .count() as u32;
    let redacted = rows
        .iter()
        .filter(|row| row.code().as_str() == "redacted-evidence")
        .count() as u32;
    FoundationalDiagnosticCounterSnapshot::new(retained, 0, redacted, 0, 0, 0)
}
