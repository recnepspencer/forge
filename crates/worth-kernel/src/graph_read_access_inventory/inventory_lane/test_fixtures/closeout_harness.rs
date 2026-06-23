use super::super::{
    WorthGraphReadAccessCoverageGuardReport, WorthGraphReadAccessInventoryCloseout,
    WorthGraphReadAccessInventoryCollector, WorthGraphReadAccessInventoryError,
    WorthGraphReadAccessInventoryRowBuilder, WorthGraphReadAccessInventorySeed,
};

pub(super) fn closeout_from_rows(
    seed: WorthGraphReadAccessInventorySeed,
    rows: Vec<WorthGraphReadAccessInventoryRowBuilder>,
) -> Result<WorthGraphReadAccessInventoryCloseout, WorthGraphReadAccessInventoryError> {
    let mut collector = WorthGraphReadAccessInventoryCollector::from_seed(seed);
    let guard_report = WorthGraphReadAccessCoverageGuardReport::clean_for_tests(rows.len());
    collector = collector.with_guard_report(guard_report);
    for row in rows {
        collector = collector.admit_row(row)?;
    }
    collector.closeout()
}
