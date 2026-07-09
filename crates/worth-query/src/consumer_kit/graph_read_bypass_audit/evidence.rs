use crate::evidence_identity::worth_query_evidence_identity;
use crate::{
    WorthQueryBoundaryAuditSourceSite, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::registry::WorthQueryGraphReadBypassRegistryRow;

pub(crate) fn derive_graph_read_bypass_candidate_identity(
    row: &WorthQueryGraphReadBypassRegistryRow,
    source_site: &WorthQueryBoundaryAuditSourceSite,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerGraphReadBypassFinding)
        .field_shape(
            WorthQueryEvidenceTag::new("source_label"),
            source_site.source_label(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("source_path"),
            source_site.source_path().unwrap_or("none"),
        )
        .field_usize(WorthQueryEvidenceTag::new("line"), source_site.line())
        .field_usize(WorthQueryEvidenceTag::new("column"), source_site.column())
        .field_shape(WorthQueryEvidenceTag::new("class"), row.class().as_str())
        .field_shape(
            WorthQueryEvidenceTag::new("authority_violation"),
            row.authority_violation().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("detection_key"),
            row.detection_key(),
        )
        .seal()
}

pub(crate) fn derive_graph_read_bypass_report_identity(
    consumer_name: &str,
    audited_source_labels: &[String],
    source_inventory_identities: &[WorthQueryEvidenceIdentity],
    counters: &super::WorthQueryGraphReadBypassCounters,
    finding_identities: &[WorthQueryEvidenceIdentity],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerGraphReadBypassReport)
        .field_shape(WorthQueryEvidenceTag::new("consumer_name"), consumer_name)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("audited_source_label"),
            audited_source_labels.iter().map(String::as_str),
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("source_inventory_identity"),
            source_inventory_identities.iter(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("evaluated_source_count"),
            counters.evaluated_source_count(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("finding_count"),
            counters.finding_count(),
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("finding_identity"),
            finding_identities.iter(),
        )
        .seal()
}

pub(crate) fn derive_graph_read_bypass_report_residue_certification_identity(
    report_identity: &WorthQueryEvidenceIdentity,
    manifest_digest: &str,
    certified_finding_count: usize,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerGraphReadBypassResidue)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("report_identity"),
            report_identity,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("residue_manifest_digest"),
            manifest_digest,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("certified_finding_count"),
            certified_finding_count,
        )
        .seal()
}
