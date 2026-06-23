use crate::evidence_identity::forge_query_evidence_identity;
use crate::{
    ForgeQueryBoundaryAuditSourceSite, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::registry::ForgeQueryGraphReadBypassRegistryRow;

pub(crate) fn derive_graph_read_bypass_candidate_identity(
    row: &ForgeQueryGraphReadBypassRegistryRow,
    source_site: &ForgeQueryBoundaryAuditSourceSite,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerGraphReadBypassFinding)
        .field_shape(
            ForgeQueryEvidenceTag::new("source_label"),
            source_site.source_label(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_path"),
            source_site.source_path().unwrap_or("none"),
        )
        .field_usize(ForgeQueryEvidenceTag::new("line"), source_site.line())
        .field_usize(ForgeQueryEvidenceTag::new("column"), source_site.column())
        .field_shape(ForgeQueryEvidenceTag::new("class"), row.class().as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("authority_violation"),
            row.authority_violation().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("detection_key"),
            row.detection_key(),
        )
        .seal()
}

pub(crate) fn derive_graph_read_bypass_report_identity(
    consumer_name: &str,
    audited_source_labels: &[String],
    source_inventory_identities: &[ForgeQueryEvidenceIdentity],
    counters: &super::ForgeQueryGraphReadBypassCounters,
    finding_identities: &[ForgeQueryEvidenceIdentity],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerGraphReadBypassReport)
        .field_shape(ForgeQueryEvidenceTag::new("consumer_name"), consumer_name)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("audited_source_label"),
            audited_source_labels.iter().map(String::as_str),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("source_inventory_identity"),
            source_inventory_identities.iter(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("evaluated_source_count"),
            counters.evaluated_source_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("finding_count"),
            counters.finding_count(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("finding_identity"),
            finding_identities.iter(),
        )
        .seal()
}

pub(crate) fn derive_graph_read_bypass_report_residue_certification_identity(
    report_identity: &ForgeQueryEvidenceIdentity,
    manifest_digest: &str,
    certified_finding_count: usize,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerGraphReadBypassResidue)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("report_identity"),
            report_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("residue_manifest_digest"),
            manifest_digest,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("certified_finding_count"),
            certified_finding_count,
        )
        .seal()
}
