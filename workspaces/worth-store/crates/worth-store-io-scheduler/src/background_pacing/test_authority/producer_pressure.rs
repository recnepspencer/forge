use crate::foreground_reservation::{
    admit_foreground_reservation, admit_foreground_reservation_capacity,
    ForegroundArbitrationDeclaration, ForegroundLaneDeclaration, ForegroundLatencyEnvelope,
    ForegroundReservationAdmissionRequest, ForegroundReservationCapacityAdmissionRequest,
};
use crate::{
    admit_background_capacity, admit_background_pacing, BackgroundCapacityAdmissionRequest,
    BackgroundIoPressureShape, BackgroundResourceBudget,
};

use super::{
    backend_admission, background_capacity_for_lane, background_policy_receipt_for,
    foreground_policy_receipt, full_foreground_capacity, page_write_budget, point_read_budget,
    security_scope_admission, wal_write_budget,
};

pub fn execute_background_pressure_for_certification_test(
    pressure: BackgroundIoPressureShape,
) -> crate::BackgroundPacingOutcome {
    let lane = foreground_lane_for_pressure(pressure);
    let budget = pressure.requested_budget();
    let capacity = background_capacity_for_lane(
        pressure,
        lane,
        budget,
        budget,
        BackgroundResourceBudget::new(),
    );
    admit_background_pacing(crate::BackgroundIdleCapacityLeaseRequest::new(capacity))
}

fn foreground_lane_for_pressure(pressure: BackgroundIoPressureShape) -> ForegroundLaneDeclaration {
    use crate::IoSchedulerBackendCapabilityRequirement as Backend;

    let (lane, budget, envelope) = match pressure.backend_requirement() {
        Backend::Fsync => (
            ForegroundLaneDeclaration::commit_critical_wal_write(),
            wal_write_budget(),
            "certification-producer-wal-write",
        ),
        Backend::BufferedFile | Backend::AsyncIo => (
            ForegroundLaneDeclaration::ordinary_page_write(),
            page_write_budget(),
            "certification-producer-page-write",
        ),
        Backend::DirectIo => (
            ForegroundLaneDeclaration::internal_foreground_read(),
            point_read_budget(),
            "certification-producer-direct-read",
        ),
        Backend::SecureFrameIo => (
            ForegroundLaneDeclaration::secure_frame_internal_foreground_read()
                .expect("secure-frame foreground lane is store-owned"),
            point_read_budget(),
            "certification-producer-secure-read",
        ),
        unsupported @ (Backend::Mmap | Backend::DirectorySync | Backend::DurableRename) => {
            panic!(
                "certification pressure declares {unsupported:?}, which has no foreground preservation lane"
            )
        }
    };
    lane.with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(envelope, 2))
        .with_budget(budget)
}

pub fn mismatched_background_pressure_denial_for_certification_test(
    pressure: BackgroundIoPressureShape,
) -> crate::BackgroundPacingDenial {
    let lane = ForegroundLaneDeclaration::ordinary_page_write()
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "certification-producer-mismatch",
            2,
        ))
        .with_budget(page_write_budget());
    let security = security_scope_admission();
    let foreground_backend = backend_admission(lane.backend_requirement());
    let background_backend =
        backend_admission(crate::IoSchedulerBackendCapabilityRequirement::DirectIo);
    let arbitration = ForegroundArbitrationDeclaration::for_lane(lane.lane());
    let foreground_capacity =
        admit_foreground_reservation_capacity(ForegroundReservationCapacityAdmissionRequest::new(
            lane,
            crate::foreground_reservation::ForegroundReservationCapacityBasis::new(
                &foreground_backend,
                &security,
            ),
            arbitration,
            lane.requested_budget(),
            full_foreground_capacity(),
            foreground_policy_receipt(lane.requested_budget()),
        ))
        .expect("foreground producer-mismatch capacity should admit");
    let foreground = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &foreground_backend,
        &security,
        arbitration,
        &foreground_capacity,
    ))
    .into_result()
    .expect("foreground producer-mismatch reservation should admit");
    admit_background_capacity(BackgroundCapacityAdmissionRequest::new(
        pressure,
        &foreground,
        &background_backend,
        background_policy_receipt_for(pressure.requested_budget(), pressure.requested_budget()),
    ))
    .expect_err("mismatched producer pressure must deny")
}
