use super::super::*;
use super::backend_capability::backend_admission;
use super::capacity_policy::policy_receipt;
use super::resource_budget::full_capacity_budget_without;
use super::resource_units::{budget_for_lane, lane_with_budget};
use super::security_scope::io_qos_security_scope_admission;

#[test]
fn every_required_resource_unit_denies_when_capacity_is_insufficient() {
    for case in capacity_shortfall_cases() {
        let lane = lane_with_budget(case.lane, budget_for_lane(case.lane));
        let backend = backend_admission(lane.backend_requirement());
        let security = io_qos_security_scope_admission();
        let arbitration = ForegroundArbitrationDeclaration::for_lane(case.lane);
        let requested = lane.requested_budget();
        let available = full_capacity_budget_without(case.unit);

        let denial = admit_foreground_reservation_capacity(
            ForegroundReservationCapacityAdmissionRequest::new(
                lane,
                ForegroundReservationCapacityBasis::new(&backend, &security),
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

#[derive(Clone, Copy)]
struct CapacityShortfallCase {
    lane: ForegroundIoLaneKind,
    unit: ForegroundResourceUnitKind,
    shortfall: ForegroundReservationResourceShortfall,
}

fn capacity_shortfall_cases() -> [CapacityShortfallCase; 12] {
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
            ForegroundIoLaneKind::CommitCriticalWalAppend,
            ForegroundResourceUnitKind::QueueSlot,
            ForegroundReservationResourceShortfall::QueueSlot {
                requested: 1,
                available: 0,
            },
        ),
        shortfall(
            ForegroundIoLaneKind::CommitCriticalWalAppend,
            ForegroundResourceUnitKind::BandwidthToken,
            ForegroundReservationResourceShortfall::BandwidthToken {
                requested: 4096,
                available: 0,
            },
        ),
        shortfall(
            ForegroundIoLaneKind::CommitCriticalWalAppend,
            ForegroundResourceUnitKind::WorkerPermit,
            ForegroundReservationResourceShortfall::WorkerPermit {
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
