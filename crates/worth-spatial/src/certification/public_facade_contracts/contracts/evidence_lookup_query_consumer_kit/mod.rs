use forge_query::facade::runtime::ForgeQueryRuntimeFacadeFamily;
use worth_spatial::facade::evidence_lookup_query_consumer_kit::{
    current_evidence_lookup_query_consumer_kit, EvidenceLookupQueryConsumerKitBindingRow,
    EvidenceLookupQueryConsumerKitCloseout, EvidenceLookupQueryConsumerKitCounters,
    EvidenceLookupQueryConsumerKitError, EvidenceLookupQueryConsumerKitErrorKind,
    EvidenceLookupQueryConsumerResidueRow, EvidenceLookupQuerySupportPinRow,
};

#[test]
fn spatial_public_api_exports_lookup_query_consumer_kit_closeout() {
    let _: fn() -> Result<
        EvidenceLookupQueryConsumerKitCloseout,
        EvidenceLookupQueryConsumerKitError,
    > = current_evidence_lookup_query_consumer_kit;
}

#[test]
fn spatial_public_api_exposes_lookup_query_consumer_kit_read_only_contract() {
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> &str =
        EvidenceLookupQueryConsumerKitCloseout::query_surface_matrix_digest;
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> &str =
        EvidenceLookupQueryConsumerKitCloseout::support_snapshot_digest;
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> &str =
        EvidenceLookupQueryConsumerKitCloseout::support_pin_contract_digest;
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> &str =
        EvidenceLookupQueryConsumerKitCloseout::support_pin_report_digest;
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> &str =
        EvidenceLookupQueryConsumerKitCloseout::evidence_report_identity;
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> &str =
        EvidenceLookupQueryConsumerKitCloseout::evidence_digest_participation_identity;
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> &str =
        EvidenceLookupQueryConsumerKitCloseout::boundary_audit_coverage_identity;
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> &str =
        EvidenceLookupQueryConsumerKitCloseout::boundary_audit_report_identity;
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> &str =
        EvidenceLookupQueryConsumerKitCloseout::consumer_residue_report_identity;
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> &str =
        EvidenceLookupQueryConsumerKitCloseout::consumer_residue_source_inventory_digest;
    let _: fn(
        &EvidenceLookupQueryConsumerKitCloseout,
    ) -> &[EvidenceLookupQueryConsumerKitBindingRow] =
        EvidenceLookupQueryConsumerKitCloseout::binding_rows;
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> &[EvidenceLookupQuerySupportPinRow] =
        EvidenceLookupQueryConsumerKitCloseout::support_rows;
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> &[EvidenceLookupQueryConsumerResidueRow] =
        EvidenceLookupQueryConsumerKitCloseout::query_residue_rows;
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> &EvidenceLookupQueryConsumerKitCounters =
        EvidenceLookupQueryConsumerKitCloseout::counters;
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> &str =
        EvidenceLookupQueryConsumerKitCloseout::closeout_digest;
    let _: fn(&EvidenceLookupQueryConsumerKitCloseout) -> bool =
        EvidenceLookupQueryConsumerKitCloseout::claims_spatial_lookup_residue_authority;
}

#[test]
fn spatial_public_api_exposes_lookup_query_consumer_kit_support_rows() {
    let _: fn(&EvidenceLookupQuerySupportPinRow) -> ForgeQueryRuntimeFacadeFamily =
        EvidenceLookupQuerySupportPinRow::runtime_family;
    let _: fn(&EvidenceLookupQuerySupportPinRow) -> &str =
        EvidenceLookupQuerySupportPinRow::query_support_surface;
    let _: fn(&EvidenceLookupQuerySupportPinRow) -> &str =
        EvidenceLookupQuerySupportPinRow::snapshot_row_digest;
    let _: fn(&EvidenceLookupQuerySupportPinRow) -> &str =
        EvidenceLookupQuerySupportPinRow::support_pin_report_digest;
    let _: fn(&EvidenceLookupQuerySupportPinRow) -> &str =
        EvidenceLookupQuerySupportPinRow::row_digest;

    let _: fn(&EvidenceLookupQueryConsumerKitBindingRow) -> &str =
        EvidenceLookupQueryConsumerKitBindingRow::family_identity;
    let _: fn(&EvidenceLookupQueryConsumerKitBindingRow) -> &str =
        EvidenceLookupQueryConsumerKitBindingRow::matrix_row_digest;
    let _: fn(&EvidenceLookupQueryConsumerKitBindingRow) -> Option<&str> =
        EvidenceLookupQueryConsumerKitBindingRow::query_surface_proof_digest;
    let _: fn(&EvidenceLookupQueryConsumerKitBindingRow) -> Option<&str> =
        EvidenceLookupQueryConsumerKitBindingRow::support_pin_report_digest;

    let _: fn(&EvidenceLookupQueryConsumerResidueRow) -> &str =
        EvidenceLookupQueryConsumerResidueRow::source_path;
    let _: fn(&EvidenceLookupQueryConsumerResidueRow) -> &str =
        EvidenceLookupQueryConsumerResidueRow::finding_identity;
    let _: fn(&EvidenceLookupQueryConsumerResidueRow) -> &str =
        EvidenceLookupQueryConsumerResidueRow::report_identity;
    let _: fn(&EvidenceLookupQueryConsumerResidueRow) -> &str =
        EvidenceLookupQueryConsumerResidueRow::source_inventory_digest;
}

#[test]
fn spatial_public_api_exposes_lookup_query_consumer_kit_error_and_counters() {
    let _: fn(&EvidenceLookupQueryConsumerKitError) -> EvidenceLookupQueryConsumerKitErrorKind =
        EvidenceLookupQueryConsumerKitError::kind;
    let _: fn(&EvidenceLookupQueryConsumerKitError) -> &str =
        EvidenceLookupQueryConsumerKitError::detail;

    let _: fn(&EvidenceLookupQueryConsumerKitCounters) -> usize =
        EvidenceLookupQueryConsumerKitCounters::binding_row_count;
    let _: fn(&EvidenceLookupQueryConsumerKitCounters) -> usize =
        EvidenceLookupQueryConsumerKitCounters::support_pinning_binding_row_count;
    let _: fn(&EvidenceLookupQueryConsumerKitCounters) -> usize =
        EvidenceLookupQueryConsumerKitCounters::support_row_count;
    let _: fn(&EvidenceLookupQueryConsumerKitCounters) -> usize =
        EvidenceLookupQueryConsumerKitCounters::query_residue_row_count;
    let _: fn(&EvidenceLookupQueryConsumerKitCounters) -> usize =
        EvidenceLookupQueryConsumerKitCounters::boundary_audit_finding_count;
}
