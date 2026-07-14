use super::super::*;
use super::common::*;

#[test]
fn every_lane_denies_when_a_required_resource_unit_is_missing() {
    for lane in read_lane_kinds() {
        for unit in read_required_units() {
            assert_missing_required_unit(lane, unit);
        }
    }
    for unit in wal_required_units() {
        assert_missing_required_unit(ForegroundIoLaneKind::CommitCriticalWalWrite, unit);
    }
    for unit in page_write_required_units() {
        assert_missing_required_unit(ForegroundIoLaneKind::OrdinaryPageWrite, unit);
    }
}

#[test]
fn every_required_resource_unit_denies_when_capacity_is_insufficient() {
    for case in capacity_shortfall_cases() {
        let lane = lane_with_budget(case.lane, budget_for_lane(case.lane));
        let backend = backend_admission(lane.backend_requirement());
        let readiness = io_qos_readiness_admission();
        let security = io_qos_security_scope_admission();
        let arbitration = ForegroundArbitrationDeclaration::for_lane(case.lane);
        let requested = lane.requested_budget();
        let available = full_capacity_budget_without(case.unit);

        let denial = admit_foreground_reservation_capacity(
            ForegroundReservationCapacityAdmissionRequest::new(
                ForegroundReservationCapacityAuthority::store_owned(),
                lane,
                &backend,
                &readiness,
                &security,
                arbitration,
                requested,
                available,
                policy_receipt(requested, requested),
            ),
        )
        .expect_err("insufficient unit capacity must deny before reservation admission");

        assert_eq!(
            denial,
            ForegroundReservationCapacityAdmissionDenial::InsufficientCapacity(case.shortfall)
        );
    }
}

fn assert_missing_required_unit(lane_kind: ForegroundIoLaneKind, unit: ForegroundResourceUnitKind) {
    let lane = lane_with_budget(lane_kind, budget_for_lane_except(lane_kind, unit));
    let backend = backend_admission(lane.backend_requirement());
    let readiness = io_qos_readiness_admission();
    let security = io_qos_security_scope_admission();
    let arbitration = ForegroundArbitrationDeclaration::for_lane(lane_kind);
    let capacity = capacity_admission(
        lane,
        &backend,
        &readiness,
        &security,
        arbitration,
        lane.requested_budget(),
        full_capacity_budget(),
    );

    let denial = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &backend,
        &readiness,
        &security,
        arbitration,
        &capacity,
    ))
    .into_result()
    .expect_err("missing lane-required resource unit must deny before reservation");

    assert_eq!(
        denial,
        ForegroundReservationAdmissionDenial::MissingRequiredResourceUnit {
            lane: lane_kind,
            unit,
        }
    );
}

#[derive(Clone, Copy)]
struct CapacityShortfallCase {
    lane: ForegroundIoLaneKind,
    unit: ForegroundResourceUnitKind,
    shortfall: ForegroundReservationResourceShortfall,
}

fn read_lane_kinds() -> [ForegroundIoLaneKind; 4] {
    [
        ForegroundIoLaneKind::PointRead,
        ForegroundIoLaneKind::RangeRead,
        ForegroundIoLaneKind::InteractiveRead,
        ForegroundIoLaneKind::InternalForegroundRead,
    ]
}

fn read_required_units() -> [ForegroundResourceUnitKind; 5] {
    [
        ForegroundResourceUnitKind::QueueSlot,
        ForegroundResourceUnitKind::BandwidthToken,
        ForegroundResourceUnitKind::ReadAheadWindow,
        ForegroundResourceUnitKind::WorkerPermit,
        ForegroundResourceUnitKind::CacheResidencyHint,
    ]
}

fn wal_required_units() -> [ForegroundResourceUnitKind; 5] {
    [
        ForegroundResourceUnitKind::QueueSlot,
        ForegroundResourceUnitKind::BandwidthToken,
        ForegroundResourceUnitKind::FlushPermit,
        ForegroundResourceUnitKind::SyncDebt,
        ForegroundResourceUnitKind::WorkerPermit,
    ]
}

fn page_write_required_units() -> [ForegroundResourceUnitKind; 5] {
    [
        ForegroundResourceUnitKind::QueueSlot,
        ForegroundResourceUnitKind::BandwidthToken,
        ForegroundResourceUnitKind::WriteBackWindow,
        ForegroundResourceUnitKind::DirtyPageBudget,
        ForegroundResourceUnitKind::WorkerPermit,
    ]
}

fn capacity_shortfall_cases() -> [CapacityShortfallCase; 9] {
    [
        shortfall(
            ForegroundIoLaneKind::PointRead,
            ForegroundResourceUnitKind::QueueSlot,
            ForegroundReservationResourceShortfall::QueueSlot {
                requested: 1,
                available: 0,
            },
        ),
        shortfall(
            ForegroundIoLaneKind::PointRead,
            ForegroundResourceUnitKind::BandwidthToken,
            ForegroundReservationResourceShortfall::BandwidthToken {
                requested: 4096,
                available: 0,
            },
        ),
        shortfall(
            ForegroundIoLaneKind::PointRead,
            ForegroundResourceUnitKind::ReadAheadWindow,
            ForegroundReservationResourceShortfall::ReadAheadWindow {
                requested: 1,
                available: 0,
            },
        ),
        shortfall(
            ForegroundIoLaneKind::PointRead,
            ForegroundResourceUnitKind::WorkerPermit,
            ForegroundReservationResourceShortfall::WorkerPermit {
                requested: 1,
                available: 0,
            },
        ),
        shortfall(
            ForegroundIoLaneKind::PointRead,
            ForegroundResourceUnitKind::CacheResidencyHint,
            ForegroundReservationResourceShortfall::CacheResidencyHint {
                requested: 1,
                available: 0,
            },
        ),
        shortfall(
            ForegroundIoLaneKind::CommitCriticalWalWrite,
            ForegroundResourceUnitKind::FlushPermit,
            ForegroundReservationResourceShortfall::FlushPermit {
                requested: 1,
                available: 0,
            },
        ),
        shortfall(
            ForegroundIoLaneKind::CommitCriticalWalWrite,
            ForegroundResourceUnitKind::SyncDebt,
            ForegroundReservationResourceShortfall::SyncDebt {
                requested: 1,
                available: 0,
            },
        ),
        shortfall(
            ForegroundIoLaneKind::OrdinaryPageWrite,
            ForegroundResourceUnitKind::WriteBackWindow,
            ForegroundReservationResourceShortfall::WriteBackWindow {
                requested: 1,
                available: 0,
            },
        ),
        shortfall(
            ForegroundIoLaneKind::OrdinaryPageWrite,
            ForegroundResourceUnitKind::DirtyPageBudget,
            ForegroundReservationResourceShortfall::DirtyPageBudget {
                requested: 1,
                available: 0,
            },
        ),
    ]
}

const fn shortfall(
    lane: ForegroundIoLaneKind,
    unit: ForegroundResourceUnitKind,
    shortfall: ForegroundReservationResourceShortfall,
) -> CapacityShortfallCase {
    CapacityShortfallCase {
        lane,
        unit,
        shortfall,
    }
}

fn lane_with_budget(
    lane: ForegroundIoLaneKind,
    budget: ForegroundResourceBudget,
) -> ForegroundLaneDeclaration {
    lane_declaration(lane)
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "resource-unit-contract",
            2,
        ))
        .with_budget(budget)
}

const fn lane_declaration(lane: ForegroundIoLaneKind) -> ForegroundLaneDeclaration {
    match lane {
        ForegroundIoLaneKind::PointRead => ForegroundLaneDeclaration::point_read(),
        ForegroundIoLaneKind::RangeRead => ForegroundLaneDeclaration::range_read(),
        ForegroundIoLaneKind::CommitCriticalWalWrite => {
            ForegroundLaneDeclaration::commit_critical_wal_write()
        }
        ForegroundIoLaneKind::OrdinaryPageWrite => ForegroundLaneDeclaration::ordinary_page_write(),
        ForegroundIoLaneKind::InteractiveRead => ForegroundLaneDeclaration::interactive_read(),
        ForegroundIoLaneKind::InternalForegroundRead => {
            ForegroundLaneDeclaration::internal_foreground_read()
        }
    }
}

fn budget_for_lane(lane: ForegroundIoLaneKind) -> ForegroundResourceBudget {
    match lane {
        ForegroundIoLaneKind::PointRead
        | ForegroundIoLaneKind::RangeRead
        | ForegroundIoLaneKind::InteractiveRead
        | ForegroundIoLaneKind::InternalForegroundRead => read_budget(),
        ForegroundIoLaneKind::CommitCriticalWalWrite => wal_write_budget(),
        ForegroundIoLaneKind::OrdinaryPageWrite => page_write_budget(),
    }
}

fn budget_for_lane_except(
    lane: ForegroundIoLaneKind,
    missing: ForegroundResourceUnitKind,
) -> ForegroundResourceBudget {
    without_unit(budget_for_lane(lane), missing)
}

fn wal_write_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_flush_permits(FlushPermit::new(1).unwrap())
        .with_sync_debt(SyncDebt::units(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
}

fn page_write_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_write_back(WriteBackWindow::pages(1).unwrap())
        .with_dirty_pages(DirtyPageBudget::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
}

fn full_capacity_budget_without(unit: ForegroundResourceUnitKind) -> ForegroundResourceBudget {
    without_unit(full_capacity_budget(), unit)
}

fn without_unit(
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
