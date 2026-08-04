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

pub(super) fn full_capacity_budget_without(
    unit: ForegroundResourceUnitKind,
) -> ForegroundResourceBudget {
    budget_without(full_capacity_budget(), unit)
}

pub(super) fn budget_without(
    budget: ForegroundResourceBudget,
    unit: ForegroundResourceUnitKind,
) -> ForegroundResourceBudget {
    let mut narrowed = ForegroundResourceBudget::new();
    for candidate in required_capacity_units() {
        if candidate != unit {
            narrowed = with_amount(narrowed, candidate, budget.amount_for(candidate));
        }
    }
    narrowed
}

fn required_capacity_units() -> [ForegroundResourceUnitKind; 9] {
    [
        ForegroundResourceUnitKind::QueueSlot,
        ForegroundResourceUnitKind::BandwidthToken,
        ForegroundResourceUnitKind::FlushPermit,
        ForegroundResourceUnitKind::SyncDebt,
        ForegroundResourceUnitKind::ReadAheadWindow,
        ForegroundResourceUnitKind::WriteBackWindow,
        ForegroundResourceUnitKind::DirtyPageBudget,
        ForegroundResourceUnitKind::WorkerPermit,
        ForegroundResourceUnitKind::CacheResidencyHint,
    ]
}

fn with_amount(
    budget: ForegroundResourceBudget,
    unit: ForegroundResourceUnitKind,
    amount: u64,
) -> ForegroundResourceBudget {
    if amount == 0 {
        return budget;
    }
    match unit {
        ForegroundResourceUnitKind::QueueSlot => {
            budget.with_queue_slots(QueueSlot::new(amount).unwrap())
        }
        ForegroundResourceUnitKind::BandwidthToken => {
            budget.with_bandwidth(BandwidthToken::bytes(amount).unwrap())
        }
        ForegroundResourceUnitKind::FlushPermit => {
            budget.with_flush_permits(FlushPermit::new(amount).unwrap())
        }
        ForegroundResourceUnitKind::SyncDebt => {
            budget.with_sync_debt(SyncDebt::units(amount).unwrap())
        }
        ForegroundResourceUnitKind::ReadAheadWindow => {
            budget.with_read_ahead(ReadAheadWindow::pages(amount).unwrap())
        }
        ForegroundResourceUnitKind::WriteBackWindow => {
            budget.with_write_back(WriteBackWindow::pages(amount).unwrap())
        }
        ForegroundResourceUnitKind::DirtyPageBudget => {
            budget.with_dirty_pages(DirtyPageBudget::pages(amount).unwrap())
        }
        ForegroundResourceUnitKind::WorkerPermit => {
            budget.with_worker_permits(WorkerPermit::new(amount).unwrap())
        }
        ForegroundResourceUnitKind::CacheResidencyHint => {
            budget.with_cache_residency(CacheResidencyHint::frames(amount).unwrap())
        }
        ForegroundResourceUnitKind::ReclaimPermit => {
            budget.with_reclaim_permits(ReclaimPermit::new(amount).unwrap())
        }
    }
}
