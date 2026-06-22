use crate::evidence_identity::forge_query_evidence_identity;

use super::finding::ForgeQueryBoundaryAuditFinding;
use super::registry_coverage::ForgeQueryBoundaryAuditCoverageRow;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

pub(crate) fn derive_boundary_audit_coverage_identity(
    coverage_rows: &[ForgeQueryBoundaryAuditCoverageRow],
) -> ForgeQueryEvidenceIdentity {
    let row_descriptors = coverage_rows
        .iter()
        .map(|row| {
            format!(
                "{}:{}:{}",
                row.seam_key(),
                row.mechanism().as_str(),
                row.audit_required()
            )
        })
        .collect::<Vec<_>>();

    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerBoundaryAuditCoverage)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("coverage_row"),
            row_descriptors.iter().map(String::as_str),
        )
        .seal()
}

pub(crate) fn derive_boundary_audit_finding_identity(
    finding: &ForgeQueryBoundaryAuditFinding,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerBoundaryAuditFinding)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), finding.kind().as_str())
        .field_shape(ForgeQueryEvidenceTag::new("seam"), finding.seam_key())
        .field_shape(
            ForgeQueryEvidenceTag::new("source_label"),
            finding.source_label(),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("source_path"),
            finding.site().source_path(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("syntax_class"),
            finding.syntax_class().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("mechanism"),
            finding.mechanism().as_str(),
        )
        .field_usize(ForgeQueryEvidenceTag::new("line"), finding.line())
        .field_usize(ForgeQueryEvidenceTag::new("column"), finding.column())
        .seal()
}

pub(crate) fn derive_boundary_audit_report_identity(
    crate_name: &str,
    source_labels: &[&str],
    source_paths: &[Option<String>],
    coverage_identity: &ForgeQueryEvidenceIdentity,
    finding_identities: &[ForgeQueryEvidenceIdentity],
    parsed_item_count: usize,
    visited_call_count: usize,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerBoundaryAuditReport)
        .field_shape(ForgeQueryEvidenceTag::new("crate_name"), crate_name)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("source_label"),
            source_labels.iter().copied(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("source_path"),
            source_paths
                .iter()
                .map(|path| path.as_deref().unwrap_or("")),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("coverage_identity"),
            coverage_identity,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("parsed_item_count"),
            parsed_item_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("visited_call_count"),
            visited_call_count,
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("finding_identity"),
            finding_identities.iter(),
        )
        .seal()
}
