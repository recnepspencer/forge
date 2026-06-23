use super::coverage_row::{
    PlanarBooleanOverlapChainCoverageRow, PlanarBooleanSplitFragmentCoverageRow,
};

pub(super) fn fragment_coverage_row_identity(
    fragment_set_identity: &str,
    schedule_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
) -> String {
    format!(
        "split-fragment-coverage:{fragment_set_identity}:{schedule_identity}:{source_edge_identity}:{carrier_identity}"
    )
}

pub(super) fn overlap_coverage_row_identity(
    chain_set_identity: &str,
    chain_identity: &str,
    source_interval_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
) -> String {
    format!(
        "overlap-chain-coverage:{chain_set_identity}:{chain_identity}:{source_interval_identity}:{source_edge_identity}:{carrier_identity}"
    )
}

pub(super) fn validation_receipt_identity(
    fragment_set_identity: &str,
    chain_set_identity: &str,
    fragment_rows: &[PlanarBooleanSplitFragmentCoverageRow],
    overlap_rows: &[PlanarBooleanOverlapChainCoverageRow],
) -> String {
    let mut identity =
        format!("split-chain-validation:{fragment_set_identity}:{chain_set_identity}");
    for row in fragment_rows {
        identity.push_str(":fragment:");
        identity.push_str(row.row_identity());
    }
    for row in overlap_rows {
        identity.push_str(":overlap:");
        identity.push_str(row.row_identity());
    }
    identity
}
