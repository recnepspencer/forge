use crate::evidence_identity::worth_query_evidence_identity;

use super::finding::WorthQueryBoundaryAuditFinding;
use super::registry_coverage::WorthQueryBoundaryAuditCoverageRow;
use crate::{WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag};

const SOURCE_IDENTITY_CHUNK_SIZE: usize = 1_024;

pub(crate) fn derive_boundary_audit_coverage_identity(
    coverage_rows: &[WorthQueryBoundaryAuditCoverageRow],
) -> WorthQueryEvidenceIdentity {
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

    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerBoundaryAuditCoverage)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("coverage_row"),
            row_descriptors.iter().map(String::as_str),
        )
        .seal()
}

pub(crate) fn derive_boundary_audit_finding_identity(
    finding: &WorthQueryBoundaryAuditFinding,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerBoundaryAuditFinding)
        .field_shape(WorthQueryEvidenceTag::new("kind"), finding.kind().as_str())
        .field_shape(WorthQueryEvidenceTag::new("seam"), finding.seam_key())
        .field_shape(
            WorthQueryEvidenceTag::new("source_label"),
            finding.source_label(),
        )
        .optional_shape(
            WorthQueryEvidenceTag::new("source_path"),
            finding.site().source_path(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("syntax_class"),
            finding.syntax_class().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("mechanism"),
            finding.mechanism().as_str(),
        )
        .field_usize(WorthQueryEvidenceTag::new("line"), finding.line())
        .field_usize(WorthQueryEvidenceTag::new("column"), finding.column())
        .seal()
}

pub(crate) fn derive_boundary_audit_report_identity(
    crate_name: &str,
    source_labels: &[&str],
    source_paths: &[Option<String>],
    coverage_identity: &WorthQueryEvidenceIdentity,
    finding_identities: &[WorthQueryEvidenceIdentity],
    parsed_item_count: usize,
    visited_call_count: usize,
) -> WorthQueryEvidenceIdentity {
    let source_chunk_identities = source_labels
        .chunks(SOURCE_IDENTITY_CHUNK_SIZE)
        .zip(source_paths.chunks(SOURCE_IDENTITY_CHUNK_SIZE))
        .map(|(labels, paths)| derive_boundary_audit_source_chunk_identity(labels, paths))
        .collect::<Vec<_>>();
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerBoundaryAuditReport)
        .field_shape(WorthQueryEvidenceTag::new("crate_name"), crate_name)
        .field_usize(
            WorthQueryEvidenceTag::new("source_count"),
            source_labels.len(),
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("source_chunk_identity"),
            source_chunk_identities.iter(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("coverage_identity"),
            coverage_identity,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("parsed_item_count"),
            parsed_item_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("visited_call_count"),
            visited_call_count,
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("finding_identity"),
            finding_identities.iter(),
        )
        .seal()
}

fn derive_boundary_audit_source_chunk_identity(
    source_labels: &[&str],
    source_paths: &[Option<String>],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerBoundaryAuditSourceInventory)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("source_label"),
            source_labels.iter().copied(),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("source_path"),
            source_paths
                .iter()
                .map(|path| path.as_deref().unwrap_or("")),
        )
        .seal()
}
