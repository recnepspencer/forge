use crate::evidence_identity::forge_query_evidence_identity;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

use super::finding::ForgeQueryConsumerResidueFinding;

pub(crate) fn derive_consumer_residue_finding_identity(
    finding: &ForgeQueryConsumerResidueFinding,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerResidueFinding)
        .field_shape(
            ForgeQueryEvidenceTag::new("source_label"),
            finding.source_label(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_path"),
            finding.source_path(),
        )
        .field_usize(ForgeQueryEvidenceTag::new("line"), finding.line())
        .field_usize(ForgeQueryEvidenceTag::new("column"), finding.column())
        .field_shape(
            ForgeQueryEvidenceTag::new("residue_class"),
            finding.residue_class().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("detection_key"),
            finding.detection_key(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("replacement_lane"),
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
    finding_identities: &[ForgeQueryEvidenceIdentity],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerResidueReport)
        .field_shape(ForgeQueryEvidenceTag::new("consumer_name"), consumer_name)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("audited_root"),
            audited_roots.iter().map(String::as_str),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("source_inventory_digest"),
            inventory_digest,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("scanned_file_count"),
            scanned_file_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("parsed_item_count"),
            parsed_item_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("visited_node_count"),
            visited_node_count,
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("finding_identity"),
            finding_identities.iter(),
        )
        .seal()
}

pub(crate) fn derive_consumer_residue_source_inventory_identity(
    consumer_name: &str,
    audited_roots: &[String],
    audited_source_paths: &[String],
    skipped_non_rust_file_count: usize,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerResidueReport)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "source-inventory")
        .field_shape(ForgeQueryEvidenceTag::new("consumer_name"), consumer_name)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("audited_root"),
            audited_roots.iter().map(String::as_str),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("audited_source_path"),
            audited_source_paths.iter().map(String::as_str),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("skipped_non_rust_file_count"),
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
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerResidueReport)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "certification-case")
        .field_shape(ForgeQueryEvidenceTag::new("case_id"), case_id)
        .field_usize(
            ForgeQueryEvidenceTag::new("checked_source_count"),
            checked_source_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("checked_class_count"),
            checked_class_count,
        )
        .field_usize(ForgeQueryEvidenceTag::new("finding_count"), finding_count)
        .field_bool(ForgeQueryEvidenceTag::new("satisfied"), satisfied)
        .seal()
}
