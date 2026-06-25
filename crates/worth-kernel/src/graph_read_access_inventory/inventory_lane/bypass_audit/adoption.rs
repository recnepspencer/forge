use forge_query::facade::consumer_kit::graph_read_bypass_adoption;

use super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::super::residue::graph_read_bypass_residue_manifest_for_report;
use super::super::row::WorthGraphReadAccessInventoryRow;
use super::adoption_report::WorthGraphReadBypassAdoptionReport;
use super::audit::audit_graph_read_bypass_inventory;
use super::source_inventory::{
    graph_read_bypass_source_inventory_from_rows, required_root_coverage_from_inventory,
};

pub(in crate::graph_read_access_inventory::inventory_lane) fn certify_graph_read_bypass_adoption(
    rows: &[WorthGraphReadAccessInventoryRow],
) -> Result<WorthGraphReadBypassAdoptionReport, WorthGraphReadAccessInventoryError> {
    let source_inventory = graph_read_bypass_source_inventory_from_rows(rows)?;
    let covered_roots = source_inventory.required_roots().to_vec();
    let required_root_coverage = required_root_coverage_from_inventory(&source_inventory);
    let source_inventory_identity = source_inventory
        .inventory_identity()
        .terminal_projection_for_reporting()
        .to_string();
    let source_inventory_count = source_inventory.source_count();
    let audit_report = audit_graph_read_bypass_inventory(&source_inventory)?;
    let evaluated_source_count = audit_report.counters().evaluated_source_count();
    let finding_count = audit_report.counters().finding_count();
    let skipped_empty_source_count = audit_report.counters().skipped_empty_source_count();
    let audited_source_labels = audit_report.audited_source_labels().to_vec();
    let residue_manifest = graph_read_bypass_residue_manifest_for_report(&audit_report)?;
    let adoption = graph_read_bypass_adoption("worth-kernel-graph-read-access-inventory")
        .audit_report(audit_report)
        .residue_manifest(residue_manifest)
        .certify()
        .map_err(|query_error| {
            error_with_message(
                WorthGraphReadAccessInventoryErrorKind::GraphReadBypassAdoptionFailed,
                query_error.message(),
            )
        })?;

    Ok(WorthGraphReadBypassAdoptionReport::from_query_adoption(
        adoption,
        covered_roots,
        required_root_coverage,
        audited_source_labels,
        source_inventory_identity,
        source_inventory_count,
        evaluated_source_count,
        finding_count,
        skipped_empty_source_count,
    ))
}

fn error_with_message(
    kind: WorthGraphReadAccessInventoryErrorKind,
    message: impl Into<String>,
) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::with_message(kind, message)
}
