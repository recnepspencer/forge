use crate::foreground_reservation::{
    ForegroundLaneDeclaration, ForegroundLatencyEnvelope, ForegroundResourceBudget,
};
use crate::{
    BackgroundCapacityAdmission, BackgroundIoPressureShape, BackgroundResourceBudget,
    BandwidthToken, CacheResidencyHint, QueueSlot, ReadAheadWindow, WorkerPermit,
};

pub fn verification_throttled_background_capacity_for_certification_test(
    requested: BackgroundResourceBudget,
    admitted: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    verification_background_capacity_with_limits(
        requested,
        admitted,
        admitted,
        BackgroundResourceBudget::new(),
    )
}

pub fn verification_zero_admitted_throttle_background_capacity_for_certification_test(
    requested: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    verification_background_capacity_with_limits(
        requested,
        BackgroundResourceBudget::new(),
        requested,
        requested,
    )
}

pub fn verification_deferred_background_capacity_for_certification_test(
    requested: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    verification_background_capacity_with_limits(
        requested,
        requested,
        BackgroundResourceBudget::new(),
        BackgroundResourceBudget::new(),
    )
}

pub fn verification_denied_background_capacity_for_certification_test(
    requested: BackgroundResourceBudget,
    admitted: BackgroundResourceBudget,
    debt_limit: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    verification_background_capacity_with_limits(requested, admitted, admitted, debt_limit)
}

fn verification_background_capacity_with_limits(
    requested: BackgroundResourceBudget,
    idle_available: BackgroundResourceBudget,
    policy_admitted: BackgroundResourceBudget,
    debt_limit: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    let lane = ForegroundLaneDeclaration::point_read()
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "certification-verification-point-read",
            2,
        ))
        .with_budget(point_read_budget());
    super::background_capacity_for_lane(
        BackgroundIoPressureShape::verification_pressure().requesting(requested),
        lane,
        idle_available,
        policy_admitted,
        debt_limit,
    )
}

fn point_read_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(1).unwrap())
}
