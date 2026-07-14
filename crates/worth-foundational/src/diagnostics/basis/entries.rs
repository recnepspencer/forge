use crate::canonicalization::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalIntegerWidth,
};
use crate::diagnostics::materialization::{
    FoundationalDiagnosticAssemblyDebt, FoundationalDiagnosticCounterSnapshot,
    FoundationalDiagnosticNamedGap, FoundationalDiagnosticPartiality,
    FoundationalDiagnosticSupportClaimStrength, FoundationalDiagnosticSurfaceAvailability,
};
use crate::diagnostics::{
    FoundationalDiagnosticArtifactKind, FoundationalDiagnosticRow, FoundationalDiagnosticSubject,
};
use crate::profiles::FoundationalProfileSet;

use super::row_entries::append_row_entries;
use super::tokens::{
    admission_readiness_token, artifact_kind_token, assembly_debt_class_token, availability_token,
    certification_posture_token, compatibility_posture_token, delivery_class_token,
    diagnostic_richness_token, gap_class_token, gap_closure_posture_token, gap_target_fragment,
    partiality_token, retention_delivery_token, support_claim_strength_token,
    support_posture_token,
};

pub(super) fn diagnostic_bundle_entries(
    artifact_kind: FoundationalDiagnosticArtifactKind,
    subject: &FoundationalDiagnosticSubject,
    outcome_kind: crate::diagnostics::FoundationalDiagnosticOutcomeKind,
    profile: FoundationalProfileSet,
    delivery_class: crate::diagnostics::FoundationalDiagnosticDeliveryClass,
    availability: FoundationalDiagnosticSurfaceAvailability,
    partiality: &FoundationalDiagnosticPartiality,
    counters: FoundationalDiagnosticCounterSnapshot,
    assembly_debts: &[FoundationalDiagnosticAssemblyDebt],
    support_claim_strength: Option<FoundationalDiagnosticSupportClaimStrength>,
    rows: &[FoundationalDiagnosticRow],
) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        bundle_text_entry("bundle.artifact_kind", artifact_kind_token(artifact_kind)),
        bundle_text_entry("bundle.subject", &subject.canonical_key_fragment()),
        bundle_text_entry("bundle.outcome_kind", outcome_kind.canonical_name()),
        bundle_text_entry(
            "bundle.profile.diagnostic_richness",
            diagnostic_richness_token(profile.diagnostic_richness()),
        ),
        bundle_text_entry(
            "bundle.profile.support_posture",
            support_posture_token(profile.support_posture()),
        ),
        bundle_text_entry(
            "bundle.profile.compatibility_posture",
            compatibility_posture_token(profile.compatibility_posture()),
        ),
        bundle_text_entry(
            "bundle.profile.admission_readiness",
            admission_readiness_token(profile.admission_readiness()),
        ),
        bundle_text_entry(
            "bundle.profile.retention_delivery",
            retention_delivery_token(profile.retention_delivery()),
        ),
        bundle_text_entry(
            "bundle.profile.certification_posture",
            certification_posture_token(profile.certification_posture()),
        ),
        bundle_text_entry(
            "bundle.delivery_class",
            delivery_class_token(delivery_class),
        ),
        bundle_text_entry(
            "bundle.availability",
            availability_token(availability.availability()),
        ),
        bundle_bool_entry(
            "bundle.has_absence_cause",
            availability.absence_cause().is_some(),
        ),
        bundle_text_entry("bundle.partiality", partiality_token(partiality)),
        bundle_u64_entry("bundle.row_count", rows.len() as u64),
        bundle_u64_entry(
            "bundle.named_gap_count",
            partiality.named_gaps().len() as u64,
        ),
        bundle_u64_entry("bundle.debt_count", assembly_debts.len() as u64),
        bundle_u64_entry(
            "bundle.counter.retained_evidence",
            u64::from(counters.retained_evidence_count()),
        ),
        bundle_u64_entry(
            "bundle.counter.reconstructable_evidence",
            u64::from(counters.reconstructable_evidence_count()),
        ),
        bundle_u64_entry(
            "bundle.counter.redacted_evidence",
            u64::from(counters.redacted_evidence_count()),
        ),
        bundle_u64_entry(
            "bundle.counter.row_scan_fallback",
            u64::from(counters.row_scan_fallback_count()),
        ),
        bundle_u64_entry(
            "bundle.counter.whole_view_fallback",
            u64::from(counters.whole_view_fallback_count()),
        ),
        bundle_u64_entry(
            "bundle.counter.repeated_rediscovery",
            u64::from(counters.repeated_rediscovery_count()),
        ),
        bundle_bool_entry(
            "bundle.has_support_claim_strength",
            support_claim_strength.is_some(),
        ),
    ];
    if let Some(cause) = availability.absence_cause() {
        entries.push(bundle_text_entry(
            "bundle.absence_cause",
            cause.canonical_name(),
        ));
    }
    if let Some(strength) = support_claim_strength {
        entries.push(bundle_text_entry(
            "bundle.support_claim_strength",
            support_claim_strength_token(strength),
        ));
    }

    for (index, gap) in sorted_named_gaps(partiality.named_gaps())
        .iter()
        .enumerate()
    {
        append_named_gap_entries(&mut entries, index, gap);
    }
    for (index, debt) in sorted_debts(assembly_debts).iter().enumerate() {
        entries.push(generic_u64_entry(
            CanonicalBasisEntryKind::DiagnosticGap,
            &format!("bundle.debt.{index}.count"),
            u64::from(debt.count()),
        ));
        entries.push(generic_text_entry(
            CanonicalBasisEntryKind::DiagnosticGap,
            &format!("bundle.debt.{index}.class"),
            assembly_debt_class_token(debt.class()),
        ));
    }
    for (index, row) in rows.iter().enumerate() {
        append_row_entries(&mut entries, index, row);
    }

    entries
}

fn append_named_gap_entries(
    entries: &mut Vec<CanonicalBasisEntry>,
    index: usize,
    gap: &FoundationalDiagnosticNamedGap,
) {
    entries.push(generic_text_entry(
        CanonicalBasisEntryKind::DiagnosticGap,
        &format!("bundle.gap.{index}.class"),
        gap_class_token(gap.gap_class()),
    ));
    entries.push(generic_text_entry(
        CanonicalBasisEntryKind::DiagnosticGap,
        &format!("bundle.gap.{index}.closure_posture"),
        gap_closure_posture_token(gap.closure_posture()),
    ));
    let (target_kind, target_value) = gap_target_fragment(gap.target());
    entries.push(generic_text_entry(
        CanonicalBasisEntryKind::DiagnosticGap,
        &format!("bundle.gap.{index}.target_kind"),
        target_kind,
    ));
    entries.push(generic_text_entry(
        CanonicalBasisEntryKind::DiagnosticGap,
        &format!("bundle.gap.{index}.target"),
        &target_value,
    ));
}

fn sorted_named_gaps(
    gaps: &[FoundationalDiagnosticNamedGap],
) -> Vec<FoundationalDiagnosticNamedGap> {
    let mut sorted = gaps.to_vec();
    sorted.sort_by_key(|gap| {
        let (_, target_fragment) = gap_target_fragment(gap.target());
        format!(
            "{}|{}|{}",
            gap_class_token(gap.gap_class()),
            target_fragment,
            gap_closure_posture_token(gap.closure_posture())
        )
    });
    sorted
}

fn sorted_debts(
    debts: &[FoundationalDiagnosticAssemblyDebt],
) -> Vec<FoundationalDiagnosticAssemblyDebt> {
    let mut sorted = debts.to_vec();
    sorted.sort_by_key(|debt| {
        format!(
            "{}|{:010}",
            assembly_debt_class_token(debt.class()),
            debt.count()
        )
    });
    sorted
}

fn bundle_text_entry(locus: &str, value: &str) -> CanonicalBasisEntry {
    generic_text_entry(CanonicalBasisEntryKind::DiagnosticBundle, locus, value)
}

fn bundle_u64_entry(locus: &str, value: u64) -> CanonicalBasisEntry {
    generic_u64_entry(CanonicalBasisEntryKind::DiagnosticBundle, locus, value)
}

fn bundle_bool_entry(locus: &str, value: bool) -> CanonicalBasisEntry {
    generic_bool_entry(CanonicalBasisEntryKind::DiagnosticBundle, locus, value)
}

pub(super) fn generic_text_entry(
    kind: CanonicalBasisEntryKind,
    locus: &str,
    value: &str,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Diagnostic,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        kind,
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}

pub(super) fn generic_u64_entry(
    kind: CanonicalBasisEntryKind,
    locus: &str,
    value: u64,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Diagnostic,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        kind,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: u128::from(value),
        },
    )
}

pub(super) fn generic_bool_entry(
    kind: CanonicalBasisEntryKind,
    locus: &str,
    value: bool,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Diagnostic,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        kind,
        CanonicalBasisValue::Bool(value),
    )
}
