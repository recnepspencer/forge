use crate::IoSchedulerBackendCapabilityRequirement;

use super::super::*;
use super::backend_capability::backend_admission;
use super::capacity_policy::{capacity_admission, policy_receipt};
use super::foreground_case::point_read_lane;
use super::resource_budget::{full_capacity_budget, read_budget};
use super::security_scope::io_qos_security_scope_admission;

#[test]
fn capacity_witness_cannot_be_reused_for_different_lane_budget() {
    let security = io_qos_security_scope_admission();
    let backend = backend_admission(IoSchedulerBackendCapabilityRequirement::DirectIo);
    let admitted_lane = point_read_lane();
    let arbitration = ForegroundArbitrationDeclaration::for_lane(ForegroundIoLaneKind::PointRead);
    let capacity = capacity_admission(
        admitted_lane,
        &backend,
        &security,
        arbitration,
        admitted_lane.requested_budget(),
        full_capacity_budget(),
    );
    let larger_lane = ForegroundLaneDeclaration::point_read()
        .with_latency_envelope(admitted_lane.envelope().unwrap())
        .with_budget(full_capacity_budget());

    let denial = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        larger_lane,
        &backend,
        &security,
        arbitration,
        &capacity,
    ))
    .into_result()
    .expect_err("capacity admitted for one budget must not reserve another");

    assert_eq!(
        denial,
        ForegroundReservationAdmissionDenial::CapacityAdmissionBudgetMismatch {
            lane_requested: larger_lane.requested_budget(),
            capacity_requested: admitted_lane.requested_budget(),
        }
    );
}

#[test]
fn policy_receipt_budget_mismatch_denies_capacity_admission() {
    let security = io_qos_security_scope_admission();
    let backend = backend_admission(IoSchedulerBackendCapabilityRequirement::DirectIo);
    let lane = point_read_lane();
    let admitted = full_capacity_budget();
    let denial =
        admit_foreground_reservation_capacity(ForegroundReservationCapacityAdmissionRequest::new(
            lane,
            ForegroundReservationCapacityBasis::new(&backend, &security),
            ForegroundArbitrationDeclaration::for_lane(ForegroundIoLaneKind::PointRead),
            admitted,
            admitted,
            policy_receipt(lane.requested_budget(), ForegroundResourceBudget::new()),
        ))
        .expect_err("Foundational budget decisions must match Store resource units");

    assert!(matches!(
        denial,
        ForegroundReservationCapacityAdmissionDenial::PolicyReceiptBudgetMismatch { .. }
    ));
}

#[test]
fn capacity_witness_cannot_be_reused_with_different_backend_basis() {
    let security = io_qos_security_scope_admission();
    let requested_backend = backend_admission(IoSchedulerBackendCapabilityRequirement::DirectIo);
    let admitted_backend = backend_admission(IoSchedulerBackendCapabilityRequirement::BufferedFile);
    let lane = point_read_lane();
    let arbitration = ForegroundArbitrationDeclaration::for_lane(ForegroundIoLaneKind::PointRead);
    let capacity = capacity_admission(
        lane,
        &admitted_backend,
        &security,
        arbitration,
        lane.requested_budget(),
        full_capacity_budget(),
    );

    let denial = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &requested_backend,
        &security,
        arbitration,
        &capacity,
    ))
    .into_result()
    .expect_err("capacity admitted for one backend basis must not reserve another");

    assert_eq!(
        denial,
        ForegroundReservationAdmissionDenial::CapacityAdmissionBackendMismatch
    );
}

#[test]
fn capacity_witness_cannot_be_reused_with_different_envelope_basis() {
    let security = io_qos_security_scope_admission();
    let backend = backend_admission(IoSchedulerBackendCapabilityRequirement::DirectIo);
    let admitted_lane = point_read_lane();
    let requested_lane = ForegroundLaneDeclaration::point_read()
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "different-point-read",
            3,
        ))
        .with_budget(read_budget());
    let arbitration = ForegroundArbitrationDeclaration::for_lane(ForegroundIoLaneKind::PointRead);
    let capacity = capacity_admission(
        admitted_lane,
        &backend,
        &security,
        arbitration,
        admitted_lane.requested_budget(),
        full_capacity_budget(),
    );

    let denial = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        requested_lane,
        &backend,
        &security,
        arbitration,
        &capacity,
    ))
    .into_result()
    .expect_err("capacity admitted for one envelope must not reserve another");

    assert_eq!(
        denial,
        ForegroundReservationAdmissionDenial::CapacityAdmissionEnvelopeMismatch
    );
}

#[test]
fn capacity_witness_cannot_be_reused_with_different_arbitration_basis() {
    let security = io_qos_security_scope_admission();
    let backend = backend_admission(IoSchedulerBackendCapabilityRequirement::DirectIo);
    let lane = point_read_lane();
    let admitted_arbitration =
        ForegroundArbitrationDeclaration::for_lane(ForegroundIoLaneKind::PointRead);
    let attempted_arbitration =
        ForegroundArbitrationDeclaration::for_lane(ForegroundIoLaneKind::InteractiveRead);
    let capacity = capacity_admission(
        lane,
        &backend,
        &security,
        attempted_arbitration,
        lane.requested_budget(),
        full_capacity_budget(),
    );

    let denial = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &backend,
        &security,
        admitted_arbitration,
        &capacity,
    ))
    .into_result()
    .expect_err("capacity admitted for one arbitration basis must not reserve another");

    assert_eq!(
        denial,
        ForegroundReservationAdmissionDenial::CapacityAdmissionArbitrationMismatch
    );
}
