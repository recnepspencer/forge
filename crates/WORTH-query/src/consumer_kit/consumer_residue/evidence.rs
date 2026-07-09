use crate::evidence_identity::worth_query_evidence_identity;
use crate::{WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag};

use super::finding::WorthQueryConsumerResidueFinding;

pub(crate) fn derive_consumer_residue_finding_identity(
    finding: &WorthQueryConsumerResidueFinding,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerResidueFinding)
        .field_shape(
            WorthQueryEvidenceTag::new("source_label"),
            finding.source_label(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("source_path"),
            finding.source_path(),
        )
        .field_usize(WorthQueryEvidenceTag::new("line"), finding.line())
        .field_usize(WorthQueryEvidenceTag::new("column"), finding.column())
        .field_shape(
            WorthQueryEvidenceTag::new("residue_class"),
            finding.residue_class().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("detection_key"),
            finding.detection_key(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("replacement_lane"),
            finding.replacement_lane(),
        )
        .seal()
}

pub(crate) fn derive_consumer_residue_report_identity(
    consumer_name: &str,
    audited_roots: &[String],
    inventory_digest: &str,
    scanned_file_count: usize,
    parsed_item_count: usize,
    visited_node_count: usize,
    finding_identities: &[WorthQueryEvidenceIdentity],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerResidueReport)
        .field_shape(WorthQueryEvidenceTag::new("consumer_name"), consumer_name)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("audited_root"),
            audited_roots.iter().map(String::as_str),
        )
        .field_value(
            WorthQueryEvidenceTag::new("source_inventory_digest"),
            inventory_digest,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("scanned_file_count"),
            scanned_file_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("parsed_item_count"),
            parsed_item_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("visited_node_count"),
            visited_node_count,
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("finding_identity"),
            finding_identities.iter(),
        )
        .seal()
}

pub(crate) fn derive_consumer_residue_source_inventory_identity(
    consumer_name: &str,
    audited_roots: &[String],
    audited_source_paths: &[String],
    skipped_non_rust_file_count: usize,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerResidueReport)
        .field_shape(WorthQueryEvidenceTag::new("role"), "source-inventory")
        .field_shape(WorthQueryEvidenceTag::new("consumer_name"), consumer_name)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("audited_root"),
            audited_roots.iter().map(String::as_str),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("audited_source_path"),
            audited_source_paths.iter().map(String::as_str),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("skipped_non_rust_file_count"),
            skipped_non_rust_file_count,
        )
        .seal()
}

pub(crate) fn derive_consumer_residue_certification_case_identity(
    case_id: &str,
    checked_source_count: usize,
    checked_class_count: usize,
    finding_count: usize,
    satisfied: bool,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerResidueReport)
        .field_shape(WorthQueryEvidenceTag::new("role"), "certification-case")
        .field_shape(WorthQueryEvidenceTag::new("case_id"), case_id)
        .field_usize(
            WorthQueryEvidenceTag::new("checked_source_count"),
            checked_source_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("checked_class_count"),
            checked_class_count,
        )
        .field_usize(WorthQueryEvidenceTag::new("finding_count"), finding_count)
        .field_bool(WorthQueryEvidenceTag::new("satisfied"), satisfied)
        .seal()
}
