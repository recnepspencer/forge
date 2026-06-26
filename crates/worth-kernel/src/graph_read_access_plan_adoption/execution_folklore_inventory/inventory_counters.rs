use super::inventory_disposition::WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition;
use super::inventory_row::WorthGraphReadAccessPlanAdoptionExecutionFolkloreRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionInventoryCounters {
    row_count: usize,
    migrate_count: usize,
    delete_count: usize,
    cap_count: usize,
    query_gap_count: usize,
}

impl WorthGraphReadAccessPlanAdoptionInventoryCounters {
    pub(in crate::graph_read_access_plan_adoption) fn from_rows(
        rows: &[WorthGraphReadAccessPlanAdoptionExecutionFolkloreRow],
    ) -> Self {
        Self {
            row_count: rows.len(),
            migrate_count: count(
                rows,
                WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::Migrate,
            ),
            delete_count: count(
                rows,
                WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::Delete,
            ),
            cap_count: count(
                rows,
                WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::Cap,
            ),
            query_gap_count: count(
                rows,
                WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::QueryGap,
            ),
        }
    }

    pub const fn row_count(&self) -> usize {
        self.row_count
    }

    pub const fn migrate_count(&self) -> usize {
        self.migrate_count
    }

    pub const fn delete_count(&self) -> usize {
        self.delete_count
    }

    pub const fn cap_count(&self) -> usize {
        self.cap_count
    }

    pub const fn query_gap_count(&self) -> usize {
        self.query_gap_count
    }
}

fn count(
    rows: &[WorthGraphReadAccessPlanAdoptionExecutionFolkloreRow],
    disposition: WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition,
) -> usize {
    rows.iter()
        .filter(|row| row.disposition() == disposition)
        .count()
}
