use crate::workload_platform::evidence_lookup_workload_cutover::EvidenceLookupMilestoneTwelveSeed;

use super::counters::EvidenceLookupPublicCloseoutCounters;
use super::family_stage_row::EvidenceLookupPublicCloseoutFamilyStageRow;

pub(crate) fn lower_milestone_twelve_seed(
    closeout_digest: &str,
    selected_route_family_identity: &str,
    selected_compiled_product_identity_digest: &str,
    selected_equivalence_family_identity: &str,
    selected_reuse_basis_identity_digest: &str,
    query_surface_matrix_digest: &str,
    query_consumer_kit_closeout_digest: &str,
    source_firewall_digest: &str,
    residue_audit_digest: &str,
    family_coverage_digest: &str,
    family_rows: &[EvidenceLookupPublicCloseoutFamilyStageRow],
    counters: &EvidenceLookupPublicCloseoutCounters,
) -> EvidenceLookupMilestoneTwelveSeed {
    let selected_lookup_plan_digest = family_rows
        .iter()
        .find_map(EvidenceLookupPublicCloseoutFamilyStageRow::selected_lookup_plan_digest)
        .unwrap_or("no-covered-plan")
        .to_string();
    let lookup_execution_receipt_digest = family_rows
        .iter()
        .find_map(EvidenceLookupPublicCloseoutFamilyStageRow::lookup_execution_receipt_digest)
        .unwrap_or("no-covered-receipt")
        .to_string();
    let lookup_product_output_digest = family_rows
        .iter()
        .find_map(EvidenceLookupPublicCloseoutFamilyStageRow::lookup_product_output_digest)
        .unwrap_or("no-covered-output")
        .to_string();
    let covered_family_identities = family_rows
        .iter()
        .map(|row| row.family_identity().to_string())
        .collect::<Vec<_>>();
    EvidenceLookupMilestoneTwelveSeed::new_public_closeout(
        closeout_digest.to_string(),
        selected_route_family_identity.to_string(),
        selected_compiled_product_identity_digest.to_string(),
        selected_equivalence_family_identity.to_string(),
        selected_reuse_basis_identity_digest.to_string(),
        selected_lookup_plan_digest,
        lookup_execution_receipt_digest,
        lookup_product_output_digest,
        covered_family_identities,
        query_surface_matrix_digest.to_string(),
        query_consumer_kit_closeout_digest.to_string(),
        source_firewall_digest.to_string(),
        residue_audit_digest.to_string(),
        family_coverage_digest.to_string(),
        counters.family_stage_row_count(),
        counters.receipt_proof_row_count(),
        counters.non_ordinary_residue_row_count(),
        family_rows
            .iter()
            .filter(|row| row.query_import_evidence_digest().is_some())
            .count(),
        family_rows
            .iter()
            .filter(|row| {
                row.topology_input_summary()
                    .contains("DerivedProductReceiptRequired")
            })
            .count(),
    )
}
