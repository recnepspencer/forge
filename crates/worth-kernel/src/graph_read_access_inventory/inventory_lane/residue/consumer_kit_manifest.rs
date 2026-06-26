use forge_query::facade::consumer_kit::{
    ForgeQueryGraphReadBypassReport, ForgeQueryGraphReadBypassResidueManifest,
    ForgeQueryGraphReadBypassResidueRow,
};

use super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::bypass_residue_cap_inventory::{
    graph_read_bypass_residue_cap_inventory, WorthGraphReadBypassResidueCap,
};

pub(in crate::graph_read_access_inventory::inventory_lane) fn graph_read_bypass_residue_manifest_for_report(
    report: &ForgeQueryGraphReadBypassReport,
) -> Result<ForgeQueryGraphReadBypassResidueManifest, WorthGraphReadAccessInventoryError> {
    let residue_rows = graph_read_bypass_residue_cap_inventory()
        .iter()
        .map(|cap| residue_row_for_cap(report, *cap))
        .collect::<Result<Vec<_>, _>>()?;

    ForgeQueryGraphReadBypassResidueManifest::capped(residue_rows).map_err(|query_error| {
        error_with_message(
            WorthGraphReadAccessInventoryErrorKind::GraphReadBypassResidueManifestFailed,
            query_error.message(),
        )
    })
}

fn residue_row_for_cap(
    report: &ForgeQueryGraphReadBypassReport,
    cap: WorthGraphReadBypassResidueCap,
) -> Result<ForgeQueryGraphReadBypassResidueRow, WorthGraphReadAccessInventoryError> {
    let current_count = report.finding_count_for_class(cap.class());
    if current_count > cap.must_not_exceed_count() {
        return Err(error(
            WorthGraphReadAccessInventoryErrorKind::ResidueGrowthRequiresCapUpdate,
        ));
    }

    ForgeQueryGraphReadBypassResidueRow::explicit(
        cap.class(),
        cap.owner(),
        cap.introduced_in(),
        current_count,
        cap.must_not_exceed_count(),
        cap.blocker(),
        cap.removal_trigger(),
    )
    .map_err(|query_error| {
        error_with_message(
            WorthGraphReadAccessInventoryErrorKind::GraphReadBypassResidueManifestFailed,
            query_error.message(),
        )
    })
}

const fn error(kind: WorthGraphReadAccessInventoryErrorKind) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::new(kind)
}

fn error_with_message(
    kind: WorthGraphReadAccessInventoryErrorKind,
    message: impl Into<String>,
) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::with_message(kind, message)
}
