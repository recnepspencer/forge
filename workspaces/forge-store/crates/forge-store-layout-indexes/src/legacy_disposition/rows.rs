//! Canonical aggregation of historical surfaces by compatibility family and owner.

mod certification_reports;
mod compatibility_inputs;
mod legacy_root_api;
mod legacy_root_artifacts;
mod maintenance_inputs;
mod row;
mod subscription_support_inputs;

use super::surface_row::LegacySurfaceInventoryRow;
use std::sync::LazyLock;

static LEGACY_SURFACE_ROWS: LazyLock<Box<[LegacySurfaceInventoryRow]>> = LazyLock::new(|| {
    let mut rows = Vec::with_capacity(88);
    rows.extend_from_slice(legacy_root_api::ROWS);
    rows.extend_from_slice(legacy_root_artifacts::ROWS);
    rows.extend_from_slice(compatibility_inputs::ROWS);
    rows.extend_from_slice(maintenance_inputs::ROWS);
    rows.extend_from_slice(subscription_support_inputs::ROWS);
    rows.extend_from_slice(certification_reports::ROWS);
    assert_eq!(
        rows.len(),
        88,
        "legacy disposition catalog must remain complete"
    );
    rows.into_boxed_slice()
});

pub(crate) fn legacy_surface_rows() -> &'static [LegacySurfaceInventoryRow] {
    &LEGACY_SURFACE_ROWS
}
