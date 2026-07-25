use worth_store::physical_runtime::ServingPhysicalRuntime;
use worth_store_io_scheduler::foreground_reservation::{
    BandwidthToken, CacheResidencyHint, DirtyPageBudget, ForegroundLaneDeclaration,
    ForegroundLatencyEnvelope, ForegroundResourceBudget, PhysicalInstanceForegroundReservation,
    QueueSlot, ReadAheadWindow, WorkerPermit, WriteBackWindow,
};

pub(crate) fn reserved_buffered_file_read(
    serving: &ServingPhysicalRuntime,
) -> PhysicalInstanceForegroundReservation {
    let lane = ForegroundLaneDeclaration::buffered_file_internal_foreground_read()
        .expect("buffered file is a Store-owned foreground lane")
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "physical-work-courtroom-read",
            2,
        ))
        .with_budget(read_budget());
    serving
        .reserve_physical_scheduler_foreground(lane)
        .expect("installed scheduler capacity should reserve the courtroom read")
        .0
}

pub(crate) fn reserved_page_write(
    serving: &ServingPhysicalRuntime,
) -> PhysicalInstanceForegroundReservation {
    let lane = ForegroundLaneDeclaration::ordinary_page_write()
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "physical-work-courtroom-write",
            2,
        ))
        .with_budget(write_budget());
    serving
        .reserve_physical_scheduler_foreground(lane)
        .expect("installed scheduler capacity should reserve the courtroom write")
        .0
}

fn read_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4_096).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(1).unwrap())
}

fn write_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4_096).unwrap())
        .with_write_back(WriteBackWindow::pages(1).unwrap())
        .with_dirty_pages(DirtyPageBudget::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
}
