use crate::evidence_identity::forge_query_evidence_identity;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

use super::finding::ForgeQueryEvidenceReportAdoptionFinding;
use super::report::ForgeQueryEvidenceReportAdoptionResidueRow;
use super::source_set::ForgeQueryEvidenceReportAdoptionResidueClassification;

pub(crate) fn derive_adoption_residue_identity(
    source_label: &str,
    source_path: Option<&str>,
    symbol: &str,
    classification: ForgeQueryEvidenceReportAdoptionResidueClassification,
    usage_count: usize,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerEvidenceReportAdoptionResidue)
        .field_shape(ForgeQueryEvidenceTag::new("source_label"), source_label)
        .optional_shape(ForgeQueryEvidenceTag::new("source_path"), source_path)
        .field_shape(ForgeQueryEvidenceTag::new("symbol"), symbol)
        .field_shape(
            ForgeQueryEvidenceTag::new("classification"),
            classification.as_str(),
        )
        .field_usize(ForgeQueryEvidenceTag::new("usage_count"), usage_count)
        .seal()
}

pub(crate) fn derive_adoption_finding_identity(
    finding: &ForgeQueryEvidenceReportAdoptionFinding,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerEvidenceReportAdoptionFinding)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), finding.kind().as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("source_label"),
            finding.source_label(),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("source_path"),
            finding.source_path(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("symbol"), finding.symbol())
        .field_shape(
            ForgeQueryEvidenceTag::new("syntax_class"),
            finding.syntax_class().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("classification"),
            finding.classification().as_str(),
        )
        .field_usize(ForgeQueryEvidenceTag::new("line"), finding.line())
        .field_usize(ForgeQueryEvidenceTag::new("column"), finding.column())
        .seal()
}

pub(crate) fn derive_adoption_report_identity(
    crate_name: &str,
    source_labels: &[String],
    residue_rows: &[ForgeQueryEvidenceReportAdoptionResidueRow],
    finding_identities: &[ForgeQueryEvidenceIdentity],
    parsed_item_count: usize,
    visited_site_count: usize,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerEvidenceReportAdoptionReport)
        .field_shape(ForgeQueryEvidenceTag::new("crate_name"), crate_name)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("source_label"),
            source_labels.iter().map(String::as_str),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("residue_row_identity"),
            residue_rows.iter().map(|row| row.row_identity()),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("finding_identity"),
            finding_identities.iter(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("parsed_item_count"),
            parsed_item_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("visited_site_count"),
            visited_site_count,
        )
        .seal()
}
