use super::super::*;
use crate::IoSchedulerBackendCapabilityRequirement;

use super::backend_capability::backend_admission;
use super::capacity_policy::capacity_admission;
use super::resource_budget::{full_capacity_budget, read_budget};
use super::security_scope::io_qos_security_scope_admission;

pub(super) fn admit_point_read_reservation() -> ForegroundReservationReceipt {
    let security = io_qos_security_scope_admission();
    let backend = backend_admission(IoSchedulerBackendCapabilityRequirement::DirectIo);
    let lane = point_read_lane();
    let arbitration = ForegroundArbitrationDeclaration::for_lane(ForegroundIoLaneKind::PointRead);
    let capacity = capacity_admission(
        lane,
        &backend,
        &security,
        arbitration,
        lane.requested_budget(),
        full_capacity_budget(),
    );

    admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &backend,
        &security,
        arbitration,
        &capacity,
    ))
    .into_result()
    .expect("point read reservation should admit")
}

pub(super) fn point_read_lane() -> ForegroundLaneDeclaration {
    ForegroundLaneDeclaration::point_read()
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "point-read",
            2,
        ))
        .with_budget(read_budget())
}
