use crate::foreground_reservation::ForegroundResourceBudget;
use crate::{
    BandwidthToken, CacheResidencyHint, DirtyPageBudget, FlushPermit, QueueSlot, SyncDebt,
    WorkerPermit, WriteBackWindow,
};

pub(super) fn page_write_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_write_back(WriteBackWindow::pages(1).unwrap())
        .with_dirty_pages(DirtyPageBudget::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
}

pub(super) fn wal_write_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_flush_permits(FlushPermit::new(1).unwrap())
        .with_sync_debt(SyncDebt::units(1).unwrap())
        .with_write_back(WriteBackWindow::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
}

pub(super) fn full_foreground_capacity() -> ForegroundResourceBudget {
    page_write_budget()
        .with_queue_slots(QueueSlot::new(4).unwrap())
        .with_bandwidth(BandwidthToken::bytes(16_384).unwrap())
        .with_read_ahead(crate::ReadAheadWindow::pages(4).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(4).unwrap())
        .with_flush_permits(FlushPermit::new(4).unwrap())
        .with_sync_debt(SyncDebt::units(4).unwrap())
}
