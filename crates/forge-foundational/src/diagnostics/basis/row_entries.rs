use crate::canonicalization::{CanonicalBasisEntry, CanonicalBasisEntryKind};
use crate::diagnostics::rows::FoundationalDiagnosticSupportEvidencePosture;
use crate::diagnostics::FoundationalDiagnosticRow;

use super::entries::{generic_bool_entry, generic_text_entry};
use super::tokens::{
    breach_class_token, denial_class_token, evidence_posture_token, locality_claim_token,
    severity_token, widened_fallout_token,
};

pub(super) fn append_row_entries(
    entries: &mut Vec<CanonicalBasisEntry>,
    index: usize,
    row: &FoundationalDiagnosticRow,
) {
    let prefix = format!("bundle.row.{index}");
    entries.push(generic_text_entry(
        CanonicalBasisEntryKind::DiagnosticRow,
        &format!("{prefix}.family"),
        row.family().canonical_name(),
    ));
    entries.push(generic_text_entry(
        CanonicalBasisEntryKind::DiagnosticRow,
        &format!("{prefix}.code"),
        row.code().as_str(),
    ));
    entries.push(generic_text_entry(
        CanonicalBasisEntryKind::DiagnosticRow,
        &format!("{prefix}.scope"),
        row.scope().as_str(),
    ));
    entries.push(generic_text_entry(
        CanonicalBasisEntryKind::DiagnosticRow,
        &format!("{prefix}.subject"),
        &row.subject().canonical_key_fragment(),
    ));
    entries.push(generic_text_entry(
        CanonicalBasisEntryKind::DiagnosticRow,
        &format!("{prefix}.locator"),
        &row.locator().canonical_key_fragment(),
    ));
    entries.push(generic_text_entry(
        CanonicalBasisEntryKind::DiagnosticRow,
        &format!("{prefix}.outcome_kind"),
        row.outcome_kind().canonical_name(),
    ));
    for (label_index, label) in row.semantic_labels().labels().iter().enumerate() {
        entries.push(generic_text_entry(
            CanonicalBasisEntryKind::DiagnosticRow,
            &format!("{prefix}.label.{label_index}"),
            label.as_str(),
        ));
    }

    match row {
        FoundationalDiagnosticRow::Decision(value) => {
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.severity"),
                severity_token(value.severity()),
            ));
            entries.push(generic_bool_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.has_denial_class"),
                value.denial_class().is_some(),
            ));
            if let Some(class) = value.denial_class() {
                entries.push(generic_text_entry(
                    CanonicalBasisEntryKind::DiagnosticRow,
                    &format!("{prefix}.denial_class"),
                    denial_class_token(class),
                ));
            }
            append_locality_entries(
                entries,
                &prefix,
                value.locality_claim(),
                value.widened_fallout_posture(),
            );
        }
        FoundationalDiagnosticRow::Failure(value) => {
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.severity"),
                severity_token(value.severity()),
            ));
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.breach_class"),
                breach_class_token(value.breach_class()),
            ));
            append_locality_entries(
                entries,
                &prefix,
                value.locality_claim(),
                value.widened_fallout_posture(),
            );
        }
        FoundationalDiagnosticRow::Comparison(value) => {
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.severity"),
                severity_token(value.severity()),
            ));
            entries.push(generic_bool_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.has_mismatch_locator"),
                value.mismatch_locator().is_some(),
            ));
            if let Some(locator) = value.mismatch_locator() {
                entries.push(generic_text_entry(
                    CanonicalBasisEntryKind::DiagnosticRow,
                    &format!("{prefix}.mismatch_locator"),
                    &locator.canonical_key_fragment(),
                ));
            }
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.evidence_posture"),
                evidence_posture_token(value.evidence_posture()),
            ));
        }
        FoundationalDiagnosticRow::Support(value) => {
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.severity"),
                severity_token(value.severity()),
            ));
            append_locality_entries(
                entries,
                &prefix,
                value.locality_claim(),
                value.widened_fallout_posture(),
            );
            append_support_evidence_posture_entries(entries, &prefix, value.evidence_posture());
        }
        FoundationalDiagnosticRow::ProvenanceReady(value) => {
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.severity"),
                severity_token(value.severity()),
            ));
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.evidence_origin_locator"),
                &value.evidence_origin_locator().canonical_key_fragment(),
            ));
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.evidence_posture"),
                evidence_posture_token(value.evidence_posture()),
            ));
        }
    }
}

fn append_locality_entries(
    entries: &mut Vec<CanonicalBasisEntry>,
    prefix: &str,
    locality: crate::diagnostics::FoundationalDiagnosticLocalityClaim,
    widened: crate::diagnostics::FoundationalDiagnosticWidenedFalloutPosture,
) {
    entries.push(generic_text_entry(
        CanonicalBasisEntryKind::DiagnosticRow,
        &format!("{prefix}.locality_claim"),
        locality_claim_token(locality),
    ));
    entries.push(generic_text_entry(
        CanonicalBasisEntryKind::DiagnosticRow,
        &format!("{prefix}.widened_fallout"),
        widened_fallout_token(widened),
    ));
}

fn append_support_evidence_posture_entries(
    entries: &mut Vec<CanonicalBasisEntry>,
    prefix: &str,
    posture: &FoundationalDiagnosticSupportEvidencePosture,
) {
    match posture {
        FoundationalDiagnosticSupportEvidencePosture::Present(value) => {
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.support_posture"),
                "present",
            ));
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.support_evidence_posture"),
                evidence_posture_token(*value),
            ));
        }
        FoundationalDiagnosticSupportEvidencePosture::Absent(value) => {
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.support_posture"),
                "absent",
            ));
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.support_absence_cause"),
                value.canonical_name(),
            ));
        }
        FoundationalDiagnosticSupportEvidencePosture::OmittedConstructionBug(value) => {
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.support_posture"),
                "omitted-construction-bug",
            ));
            entries.push(generic_text_entry(
                CanonicalBasisEntryKind::DiagnosticRow,
                &format!("{prefix}.support_breach_class"),
                breach_class_token(*value),
            ));
        }
    }
}
