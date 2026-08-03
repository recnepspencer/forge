use crate::foreground_reservation::{ForegroundLaneDeclaration, ForegroundLatencyEnvelope};
use crate::{BackgroundCapacityAdmission, BackgroundIoPressureShape, BackgroundResourceBudget};

pub fn blob_ingest_throttled_background_capacity_for_certification_test(
    requested: BackgroundResourceBudget,
    admitted: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    blob_ingest_page_write_background_capacity_with_limits(
        requested,
        admitted,
        admitted,
        BackgroundResourceBudget::new(),
    )
}

pub fn blob_ingest_zero_admitted_throttle_background_capacity_for_certification_test(
    requested: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    blob_ingest_page_write_background_capacity_with_limits(
        requested,
        BackgroundResourceBudget::new(),
        requested,
        requested,
    )
}

pub fn blob_ingest_deferred_background_capacity_for_certification_test(
    requested: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    blob_ingest_page_write_background_capacity_with_limits(
        requested,
        requested,
        BackgroundResourceBudget::new(),
        BackgroundResourceBudget::new(),
    )
}

pub fn blob_ingest_denied_background_capacity_for_certification_test(
    requested: BackgroundResourceBudget,
    admitted: BackgroundResourceBudget,
    debt_limit: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    blob_ingest_page_write_background_capacity_with_limits(
        requested, admitted, admitted, debt_limit,
    )
}

fn blob_ingest_page_write_background_capacity_with_limits(
    requested: BackgroundResourceBudget,
    idle_available: BackgroundResourceBudget,
    policy_admitted: BackgroundResourceBudget,
    debt_limit: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    let lane = ForegroundLaneDeclaration::ordinary_page_write()
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "certification-blob-ingest-page-write",
            2,
        ))
        .with_budget(super::page_write_budget());
    super::background_capacity_for_lane(
        BackgroundIoPressureShape::blob_ingest_pressure().requesting(requested),
        lane,
        idle_available,
        policy_admitted,
        debt_limit,
    )
}
