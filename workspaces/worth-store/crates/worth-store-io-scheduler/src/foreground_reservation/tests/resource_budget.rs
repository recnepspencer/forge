use super::super::*;

pub(super) fn read_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(1).unwrap())
}

pub(super) fn full_capacity_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(8).unwrap())
        .with_bandwidth(BandwidthToken::bytes(1_048_576).unwrap())
        .with_flush_permits(FlushPermit::new(8).unwrap())
        .with_sync_debt(SyncDebt::units(8).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(8).unwrap())
        .with_write_back(WriteBackWindow::pages(8).unwrap())
        .with_dirty_pages(DirtyPageBudget::pages(8).unwrap())
        .with_worker_permits(WorkerPermit::new(8).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(8).unwrap())
        .with_reclaim_permits(ReclaimPermit::new(8).unwrap())
}
