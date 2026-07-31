use super::super::*;
use super::backend_capability::backend_admission;
use super::capacity_policy::capacity_admission;
use super::resource_budget::{budget_without, full_capacity_budget, read_budget};
use super::security_scope::io_qos_security_scope_admission;

#[test]
fn every_lane_denies_when_a_required_resource_unit_is_missing() {
    for lane in read_lane_kinds() {
        for unit in read_required_units() {
            assert_missing_required_unit(lane, unit);
        }
    }
    for unit in wal_append_required_units() {
        assert_missing_required_unit(ForegroundIoLaneKind::CommitCriticalWalAppend, unit);
    }
    for unit in wal_write_required_units() {
        assert_missing_required_unit(ForegroundIoLaneKind::CommitCriticalWalWrite, unit);
    }
    for unit in page_write_required_units() {
        assert_missing_required_unit(ForegroundIoLaneKind::OrdinaryPageWrite, unit);
    }
    for unit in metadata_required_units() {
        assert_missing_required_unit(ForegroundIoLaneKind::ArtifactMetadataRead, unit);
    }
}

#[test]
fn wal_append_budget_excludes_barrier_only_resources() {
    let budget = wal_append_budget();
    assert_eq!(budget.flush_permits(), 0);
    assert_eq!(budget.sync_debt(), 0);
    assert_eq!(budget.queue_slots(), 1);
    assert_eq!(budget.bandwidth_tokens(), 4096);
    assert_eq!(budget.worker_permits(), 1);
}

fn assert_missing_required_unit(lane_kind: ForegroundIoLaneKind, unit: ForegroundResourceUnitKind) {
    let lane = lane_with_budget(lane_kind, budget_for_lane_except(lane_kind, unit));
    let backend = backend_admission(lane.backend_requirement());
    let security = io_qos_security_scope_admission();
    let arbitration = ForegroundArbitrationDeclaration::for_lane(lane_kind);
    let capacity = capacity_admission(
        lane,
        &backend,
        &security,
        arbitration,
        lane.requested_budget(),
        full_capacity_budget(),
    );

    let denial = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &backend,
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

fn wal_append_required_units() -> [ForegroundResourceUnitKind; 3] {
    [
        ForegroundResourceUnitKind::QueueSlot,
        ForegroundResourceUnitKind::BandwidthToken,
        ForegroundResourceUnitKind::WorkerPermit,
    ]
}

fn wal_write_required_units() -> [ForegroundResourceUnitKind; 5] {
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

fn metadata_required_units() -> [ForegroundResourceUnitKind; 2] {
    [
        ForegroundResourceUnitKind::QueueSlot,
        ForegroundResourceUnitKind::WorkerPermit,
    ]
}

pub(super) fn lane_with_budget(
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
        ForegroundIoLaneKind::CommitCriticalWalAppend => {
            ForegroundLaneDeclaration::commit_critical_wal_append()
        }
        ForegroundIoLaneKind::CommitCriticalWalWrite => {
            ForegroundLaneDeclaration::commit_critical_wal_write()
        }
        ForegroundIoLaneKind::OrdinaryPageWrite => ForegroundLaneDeclaration::ordinary_page_write(),
        ForegroundIoLaneKind::InteractiveRead => ForegroundLaneDeclaration::interactive_read(),
        ForegroundIoLaneKind::InternalForegroundRead => {
            ForegroundLaneDeclaration::internal_foreground_read()
        }
        ForegroundIoLaneKind::ArtifactMetadataRead => {
            ForegroundLaneDeclaration::artifact_metadata_read()
        }
    }
}

pub(super) fn budget_for_lane(lane: ForegroundIoLaneKind) -> ForegroundResourceBudget {
    match lane {
        ForegroundIoLaneKind::PointRead
        | ForegroundIoLaneKind::RangeRead
        | ForegroundIoLaneKind::InteractiveRead
        | ForegroundIoLaneKind::InternalForegroundRead => read_budget(),
        ForegroundIoLaneKind::ArtifactMetadataRead => metadata_budget(),
        ForegroundIoLaneKind::CommitCriticalWalAppend => wal_append_budget(),
        ForegroundIoLaneKind::CommitCriticalWalWrite => wal_write_budget(),
        ForegroundIoLaneKind::OrdinaryPageWrite => page_write_budget(),
    }
}

fn metadata_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
}

fn budget_for_lane_except(
    lane: ForegroundIoLaneKind,
    missing: ForegroundResourceUnitKind,
) -> ForegroundResourceBudget {
    budget_without(budget_for_lane(lane), missing)
}

fn wal_append_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
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
