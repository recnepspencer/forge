use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::super::counters::EvidenceLookupQueryConsumerKitCounters;
use super::super::row::{
    EvidenceLookupQueryConsumerKitBindingRow, EvidenceLookupQueryConsumerResidueRow,
    EvidenceLookupQuerySupportPinRow,
};

#[allow(clippy::too_many_arguments)]
pub(super) fn closeout_digest(
    binding_rows: &[EvidenceLookupQueryConsumerKitBindingRow],
    support_rows: &[EvidenceLookupQuerySupportPinRow],
    query_residue_rows: &[EvidenceLookupQueryConsumerResidueRow],
    counters: &EvidenceLookupQueryConsumerKitCounters,
    matrix_digest: &str,
    support_snapshot_digest: &str,
    support_pin_contract_digest: &str,
    support_pin_report_digest: &str,
    evidence_report_identity: &str,
    evidence_digest_participation_identity: &str,
    boundary_audit_coverage_identity: &str,
    boundary_audit_report_identity: &str,
    consumer_residue_report_identity: &str,
    consumer_residue_source_inventory_digest: &str,
) -> String {
    let mut parts = vec![
        "worth-spatial:evidence-lookup-query-consumer-kit-closeout:v1".to_string(),
        matrix_digest.to_string(),
        support_snapshot_digest.to_string(),
        support_pin_contract_digest.to_string(),
        support_pin_report_digest.to_string(),
        evidence_report_identity.to_string(),
        evidence_digest_participation_identity.to_string(),
        boundary_audit_coverage_identity.to_string(),
        boundary_audit_report_identity.to_string(),
        consumer_residue_report_identity.to_string(),
        consumer_residue_source_inventory_digest.to_string(),
        counters.binding_row_count().to_string(),
        counters.support_row_count().to_string(),
        counters.query_residue_row_count().to_string(),
        counters.boundary_audit_finding_count().to_string(),
    ];
    parts.extend(binding_rows.iter().map(|row| row.row_digest().to_string()));
    parts.extend(support_rows.iter().map(|row| row.row_digest().to_string()));
    parts.extend(
        query_residue_rows
            .iter()
            .map(|row| row.row_digest().to_string()),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
