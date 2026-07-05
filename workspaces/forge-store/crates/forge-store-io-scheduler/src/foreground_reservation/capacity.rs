use super::{
    ForegroundReservationAdmissionDenial, ForegroundReservationResourceShortfall,
    ForegroundResourceBudget, ForegroundResourceUnitKind,
};

pub(super) fn require_capacity(
    requested: ForegroundResourceBudget,
    available: ForegroundResourceBudget,
) -> Result<
    (),
    (
        ForegroundReservationAdmissionDenial,
        ForegroundResourceBudget,
    ),
> {
    shortfall(
        requested.queue_slots(),
        available.queue_slots(),
        queue_slot_shortfall,
    )?;
    shortfall(
        requested.bandwidth_tokens(),
        available.bandwidth_tokens(),
        bandwidth_token_shortfall,
    )?;
    shortfall(
        requested.flush_permits(),
        available.flush_permits(),
        flush_permit_shortfall,
    )?;
    shortfall(
        requested.sync_debt(),
        available.sync_debt(),
        sync_debt_shortfall,
    )?;
    shortfall(
        requested.read_ahead_window(),
        available.read_ahead_window(),
        read_ahead_shortfall,
    )?;
    shortfall(
        requested.write_back_window(),
        available.write_back_window(),
        write_back_shortfall,
    )?;
    shortfall(
        requested.dirty_page_budget(),
        available.dirty_page_budget(),
        dirty_page_shortfall,
    )?;
    shortfall(
        requested.worker_permits(),
        available.worker_permits(),
        worker_shortfall,
    )?;
    shortfall(
        requested.cache_residency_hints(),
        available.cache_residency_hints(),
        cache_residency_shortfall,
    )?;
    shortfall(
        requested.reclaim_permits(),
        available.reclaim_permits(),
        reclaim_shortfall,
    )
}

fn shortfall(
    requested: u64,
    available: u64,
    build: fn(u64, u64) -> ForegroundReservationResourceShortfall,
) -> Result<
    (),
    (
        ForegroundReservationAdmissionDenial,
        ForegroundResourceBudget,
    ),
> {
    if requested > available {
        let shortfall = build(requested, available);
        Err((
            ForegroundReservationAdmissionDenial::InsufficientCapacity(shortfall),
            denied_budget_for_shortfall(shortfall),
        ))
    } else {
        Ok(())
    }
}

const fn denied_budget_for_shortfall(
    shortfall: ForegroundReservationResourceShortfall,
) -> ForegroundResourceBudget {
    match shortfall {
        ForegroundReservationResourceShortfall::QueueSlot { requested, .. } => {
            ForegroundResourceBudget::denied_unit(ForegroundResourceUnitKind::QueueSlot, requested)
        }
        ForegroundReservationResourceShortfall::BandwidthToken { requested, .. } => {
            ForegroundResourceBudget::denied_unit(
                ForegroundResourceUnitKind::BandwidthToken,
                requested,
            )
        }
        ForegroundReservationResourceShortfall::FlushPermit { requested, .. } => {
            ForegroundResourceBudget::denied_unit(
                ForegroundResourceUnitKind::FlushPermit,
                requested,
            )
        }
        ForegroundReservationResourceShortfall::SyncDebt { requested, .. } => {
            ForegroundResourceBudget::denied_unit(ForegroundResourceUnitKind::SyncDebt, requested)
        }
        ForegroundReservationResourceShortfall::ReadAheadWindow { requested, .. } => {
            ForegroundResourceBudget::denied_unit(
                ForegroundResourceUnitKind::ReadAheadWindow,
                requested,
            )
        }
        ForegroundReservationResourceShortfall::WriteBackWindow { requested, .. } => {
            ForegroundResourceBudget::denied_unit(
                ForegroundResourceUnitKind::WriteBackWindow,
                requested,
            )
        }
        ForegroundReservationResourceShortfall::DirtyPageBudget { requested, .. } => {
            ForegroundResourceBudget::denied_unit(
                ForegroundResourceUnitKind::DirtyPageBudget,
                requested,
            )
        }
        ForegroundReservationResourceShortfall::WorkerPermit { requested, .. } => {
            ForegroundResourceBudget::denied_unit(
                ForegroundResourceUnitKind::WorkerPermit,
                requested,
            )
        }
        ForegroundReservationResourceShortfall::CacheResidencyHint { requested, .. } => {
            ForegroundResourceBudget::denied_unit(
                ForegroundResourceUnitKind::CacheResidencyHint,
                requested,
            )
        }
        ForegroundReservationResourceShortfall::ReclaimPermit { requested, .. } => {
            ForegroundResourceBudget::denied_unit(
                ForegroundResourceUnitKind::ReclaimPermit,
                requested,
            )
        }
    }
}

const fn queue_slot_shortfall(
    requested: u64,
    available: u64,
) -> ForegroundReservationResourceShortfall {
    ForegroundReservationResourceShortfall::QueueSlot {
        requested,
        available,
    }
}

const fn bandwidth_token_shortfall(
    requested: u64,
    available: u64,
) -> ForegroundReservationResourceShortfall {
    ForegroundReservationResourceShortfall::BandwidthToken {
        requested,
        available,
    }
}

const fn flush_permit_shortfall(
    requested: u64,
    available: u64,
) -> ForegroundReservationResourceShortfall {
    ForegroundReservationResourceShortfall::FlushPermit {
        requested,
        available,
    }
}

const fn sync_debt_shortfall(
    requested: u64,
    available: u64,
) -> ForegroundReservationResourceShortfall {
    ForegroundReservationResourceShortfall::SyncDebt {
        requested,
        available,
    }
}

const fn read_ahead_shortfall(
    requested: u64,
    available: u64,
) -> ForegroundReservationResourceShortfall {
    ForegroundReservationResourceShortfall::ReadAheadWindow {
        requested,
        available,
    }
}

const fn write_back_shortfall(
    requested: u64,
    available: u64,
) -> ForegroundReservationResourceShortfall {
    ForegroundReservationResourceShortfall::WriteBackWindow {
        requested,
        available,
    }
}

const fn dirty_page_shortfall(
    requested: u64,
    available: u64,
) -> ForegroundReservationResourceShortfall {
    ForegroundReservationResourceShortfall::DirtyPageBudget {
        requested,
        available,
    }
}

const fn worker_shortfall(
    requested: u64,
    available: u64,
) -> ForegroundReservationResourceShortfall {
    ForegroundReservationResourceShortfall::WorkerPermit {
        requested,
        available,
    }
}

const fn cache_residency_shortfall(
    requested: u64,
    available: u64,
) -> ForegroundReservationResourceShortfall {
    ForegroundReservationResourceShortfall::CacheResidencyHint {
        requested,
        available,
    }
}

const fn reclaim_shortfall(
    requested: u64,
    available: u64,
) -> ForegroundReservationResourceShortfall {
    ForegroundReservationResourceShortfall::ReclaimPermit {
        requested,
        available,
    }
}
