use crate::IoSchedulerBackendCapabilityRequirement;

use super::super::*;
use super::common::*;

#[test]
fn capacity_witness_cannot_be_reused_for_different_lane_budget() {
    let readiness = io_qos_readiness_admission();
    let security = io_qos_security_scope_admission();
    let backend = backend_admission(IoSchedulerBackendCapabilityRequirement::DirectIo);
    let admitted_lane = point_read_lane();
    let arbitration = ForegroundArbitrationDeclaration::for_lane(ForegroundIoLaneKind::PointRead);
    let capacity = capacity_admission(
        admitted_lane,
        &backend,
        &readiness,
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
        &readiness,
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
    let readiness = io_qos_readiness_admission();
    let security = io_qos_security_scope_admission();
    let backend = backend_admission(IoSchedulerBackendCapabilityRequirement::DirectIo);
    let lane = point_read_lane();
    let admitted = full_capacity_budget();
    let denial =
        admit_foreground_reservation_capacity(ForegroundReservationCapacityAdmissionRequest::new(
            lane,
            ForegroundReservationCapacityBasis::new(&backend, &readiness, &security),
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
    let readiness = io_qos_readiness_admission();
    let security = io_qos_security_scope_admission();
    let requested_backend = backend_admission(IoSchedulerBackendCapabilityRequirement::DirectIo);
    let admitted_backend = backend_admission(IoSchedulerBackendCapabilityRequirement::BufferedFile);
    let lane = point_read_lane();
    let arbitration = ForegroundArbitrationDeclaration::for_lane(ForegroundIoLaneKind::PointRead);
    let capacity = capacity_admission(
        lane,
        &admitted_backend,
        &readiness,
        &security,
        arbitration,
        lane.requested_budget(),
        full_capacity_budget(),
    );

    let denial = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &requested_backend,
        &readiness,
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
    let readiness = io_qos_readiness_admission();
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
        &readiness,
        &security,
        arbitration,
        admitted_lane.requested_budget(),
        full_capacity_budget(),
    );

    let denial = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        requested_lane,
        &backend,
        &readiness,
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
    let readiness = io_qos_readiness_admission();
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
        &readiness,
        &security,
        attempted_arbitration,
        lane.requested_budget(),
        full_capacity_budget(),
    );

    let denial = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &backend,
        &readiness,
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

#[test]
fn capacity_witness_cannot_be_reused_with_different_readiness_counter_basis() {
    let admitted_readiness = io_qos_readiness_admission();
    let requested_readiness = io_qos_readiness_admission_with_counts(3, 2);
    let security = io_qos_security_scope_admission();
    let backend = backend_admission(IoSchedulerBackendCapabilityRequirement::DirectIo);
    let lane = point_read_lane();
    let arbitration = ForegroundArbitrationDeclaration::for_lane(ForegroundIoLaneKind::PointRead);
    let capacity = capacity_admission(
        lane,
        &backend,
        &admitted_readiness,
        &security,
        arbitration,
        lane.requested_budget(),
        full_capacity_budget(),
    );

    let denial = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &backend,
        &requested_readiness,
        &security,
        arbitration,
        &capacity,
    ))
    .into_result()
    .expect_err("capacity admitted for one readiness basis must not reserve another");

    assert_eq!(
        denial,
        ForegroundReservationAdmissionDenial::CapacityAdmissionReadinessCounterMismatch
    );
}
