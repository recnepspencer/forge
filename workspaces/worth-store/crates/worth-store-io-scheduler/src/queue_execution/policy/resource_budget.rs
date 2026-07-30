use worth_store_contracts::QueueProducerResourceShape;

use crate::{
    BackgroundResourceBudget, BandwidthToken, CacheResidencyHint, DirtyPageBudget, FlushPermit,
    QueueSlot, ReadAheadWindow, ReclaimPermit, SyncDebt, WorkerPermit, WriteBackWindow,
};

use super::QueueExecutionAdmissionDenial;

pub(super) fn budget_from_shape(
    shape: QueueProducerResourceShape,
) -> Result<BackgroundResourceBudget, QueueExecutionAdmissionDenial> {
    let mut budget = BackgroundResourceBudget::new();
    if shape.queue_slots() > 0 {
        budget = budget.with_queue_slots(
            QueueSlot::new(shape.queue_slots())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.bandwidth_tokens() > 0 {
        budget = budget.with_bandwidth(
            BandwidthToken::bytes(shape.bandwidth_tokens())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.flush_permits() > 0 {
        budget = budget.with_flush_permits(
            FlushPermit::new(shape.flush_permits())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.sync_debt() > 0 {
        budget = budget.with_sync_debt(
            SyncDebt::units(shape.sync_debt())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.read_ahead_windows() > 0 {
        budget = budget.with_read_ahead(
            ReadAheadWindow::pages(shape.read_ahead_windows())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.write_back_windows() > 0 {
        budget = budget.with_write_back(
            WriteBackWindow::pages(shape.write_back_windows())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.dirty_page_budget() > 0 {
        budget = budget.with_dirty_pages(
            DirtyPageBudget::pages(shape.dirty_page_budget())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.worker_permits() > 0 {
        budget = budget.with_worker_permits(
            WorkerPermit::new(shape.worker_permits())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.cache_residency_hints() > 0 {
        budget = budget.with_cache_residency(
            CacheResidencyHint::frames(shape.cache_residency_hints())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.reclaim_permits() > 0 {
        budget = budget.with_reclaim_permits(
            ReclaimPermit::new(shape.reclaim_permits())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    Ok(budget)
}
