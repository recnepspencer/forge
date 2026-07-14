use crate::evidence_identity::worth_query_evidence_identity;
use crate::{WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag};

use super::finding::WorthQueryEvidenceReportAdoptionFinding;
use super::report::WorthQueryEvidenceReportAdoptionResidueRow;
use super::source_set::WorthQueryEvidenceReportAdoptionResidueClassification;

pub(crate) fn derive_adoption_residue_identity(
    source_label: &str,
    source_path: Option<&str>,
    symbol: &str,
    classification: WorthQueryEvidenceReportAdoptionResidueClassification,
    usage_count: usize,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerEvidenceReportAdoptionResidue)
        .field_shape(WorthQueryEvidenceTag::new("source_label"), source_label)
        .optional_shape(WorthQueryEvidenceTag::new("source_path"), source_path)
        .field_shape(WorthQueryEvidenceTag::new("symbol"), symbol)
        .field_shape(
            WorthQueryEvidenceTag::new("classification"),
            classification.as_str(),
        )
        .field_usize(WorthQueryEvidenceTag::new("usage_count"), usage_count)
        .seal()
}

pub(crate) fn derive_adoption_finding_identity(
    finding: &WorthQueryEvidenceReportAdoptionFinding,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerEvidenceReportAdoptionFinding)
        .field_shape(WorthQueryEvidenceTag::new("kind"), finding.kind().as_str())
        .field_shape(
            WorthQueryEvidenceTag::new("source_label"),
            finding.source_label(),
        )
        .optional_shape(
            WorthQueryEvidenceTag::new("source_path"),
            finding.source_path(),
        )
        .field_shape(WorthQueryEvidenceTag::new("symbol"), finding.symbol())
        .field_shape(
            WorthQueryEvidenceTag::new("syntax_class"),
            finding.syntax_class().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("classification"),
            finding.classification().as_str(),
        )
        .field_usize(WorthQueryEvidenceTag::new("line"), finding.line())
        .field_usize(WorthQueryEvidenceTag::new("column"), finding.column())
        .seal()
}

pub(crate) fn derive_adoption_report_identity(
    crate_name: &str,
    source_labels: &[String],
    residue_rows: &[WorthQueryEvidenceReportAdoptionResidueRow],
    finding_identities: &[WorthQueryEvidenceIdentity],
    parsed_item_count: usize,
    visited_site_count: usize,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerEvidenceReportAdoptionReport)
        .field_shape(WorthQueryEvidenceTag::new("crate_name"), crate_name)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("source_label"),
            source_labels.iter().map(String::as_str),
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("residue_row_identity"),
            residue_rows.iter().map(|row| row.row_identity()),
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("finding_identity"),
            finding_identities.iter(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("parsed_item_count"),
            parsed_item_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("visited_site_count"),
            visited_site_count,
        )
        .seal()
}
