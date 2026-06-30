use crate::replay_undo_inventory::inventory_lane::lowering::ReplayUndoInventoryDisposition;
use crate::replay_undo_inventory::inventory_lane::lowering::ReplayUndoInventoryReportRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoInventoryCloseoutCounters {
    total_declared_sources: usize,
    total_lowered_sources: usize,
    migrate_sources: usize,
    delete_sources: usize,
    capped_sources: usize,
    query_gap_sources: usize,
}

impl ReplayUndoInventoryCloseoutCounters {
    pub(crate) fn new(
        total_declared_sources: usize,
        rows: &[ReplayUndoInventoryReportRow],
    ) -> Self {
        let mut counters = Self {
            total_declared_sources,
            total_lowered_sources: rows.len(),
            migrate_sources: 0,
            delete_sources: 0,
            capped_sources: 0,
            query_gap_sources: 0,
        };
        for row in rows {
            match row.disposition() {
                ReplayUndoInventoryDisposition::Migrate => counters.migrate_sources += 1,
                ReplayUndoInventoryDisposition::Delete => counters.delete_sources += 1,
                ReplayUndoInventoryDisposition::Cap => counters.capped_sources += 1,
                ReplayUndoInventoryDisposition::QueryGap => counters.query_gap_sources += 1,
            }
        }
        counters
    }

    pub const fn total_declared_sources(&self) -> usize {
        self.total_declared_sources
    }

    pub const fn total_lowered_sources(&self) -> usize {
        self.total_lowered_sources
    }

    pub const fn migrate_sources(&self) -> usize {
        self.migrate_sources
    }

    pub const fn delete_sources(&self) -> usize {
        self.delete_sources
    }

    pub const fn capped_sources(&self) -> usize {
        self.capped_sources
    }

    pub const fn query_gap_sources(&self) -> usize {
        self.query_gap_sources
    }
}
