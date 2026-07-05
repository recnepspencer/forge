use crate::IoSchedulerBackendCapabilityRequirement;

use super::super::*;
use super::common::*;

#[test]
fn certification_only_envelope_is_held_not_execution_ready() {
    let readiness = s6_readiness_admission();
    let security = s6_security_scope_admission();
    let backend = backend_admission(IoSchedulerBackendCapabilityRequirement::DirectIo);
    let lane = ForegroundLaneDeclaration::point_read()
        .with_latency_envelope(ForegroundLatencyEnvelope::certification_only_target(
            "certification-only",
        ))
        .with_budget(read_budget());
    let arbitration = ForegroundArbitrationDeclaration::for_lane(ForegroundIoLaneKind::PointRead);
    let capacity = capacity_admission(
        lane,
        &backend,
        &readiness,
        &security,
        arbitration,
        lane.requested_budget(),
        full_capacity_budget(),
    );

    let outcome = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &backend,
        &readiness,
        &security,
        arbitration,
        &capacity,
    ));

    assert_eq!(outcome.state(), ForegroundReservationState::ReservationHeld);
    assert!(matches!(
        outcome.into_result(),
        Err(ForegroundReservationAdmissionDenial::CertificationOnlyEnvelopeCannotExecute)
    ));
}

#[test]
fn rebind_required_basis_is_stateful_denial_not_execution_ready() {
    let readiness = s6_readiness_admission();
    let security = s6_security_scope_admission();
    let backend = backend_admission(IoSchedulerBackendCapabilityRequirement::DirectIo);
    let lane = point_read_lane();
    let arbitration = ForegroundArbitrationDeclaration::for_lane(ForegroundIoLaneKind::PointRead);
    let capacity = super::super::capacity_admission::rebind_required_capacity_admission_for_test(
        capacity_admission(
            lane,
            &backend,
            &readiness,
            &security,
            arbitration,
            lane.requested_budget(),
            full_capacity_budget(),
        ),
    );
    let outcome = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &backend,
        &readiness,
        &security,
        arbitration,
        &capacity,
    ));

    assert_eq!(
        outcome.state(),
        ForegroundReservationState::ReservationStaleRebindRequired
    );
    assert!(matches!(
        outcome.into_result(),
        Err(ForegroundReservationAdmissionDenial::ReservationBasisRebindRequired)
    ));
    if let ForegroundReservationAdmissionOutcome::StaleRebindRequired(stale) = outcome {
        assert_eq!(stale.lane(), ForegroundIoLaneKind::PointRead);
        assert_eq!(
            stale.counters().denied_budget(),
            point_read_lane().requested_budget()
        );
    } else {
        panic!("rebind outcome must carry stale/rebind proof state");
    }
}

#[test]
fn envelope_violation_reports_typed_cause() {
    let receipt = admit_point_read_reservation();
    let violation = receipt
        .observe_interference(3)
        .expect_err("interference above envelope must violate with cause");

    assert_eq!(violation.lane(), ForegroundIoLaneKind::PointRead);
    assert_eq!(
        violation.cause(),
        ForegroundReservationViolationCause::EnvelopeExceeded {
            allowed_interference_events: 2,
            observed_interference_events: 3,
        }
    );
}

#[test]
fn raw_shortcuts_are_typed_denials_not_reservation_authority() {
    assert_eq!(
        reject_raw_lane_label_as_foreground_reservation(),
        Err(ForegroundReservationAdmissionDenial::RawLaneLabelCannotReserve)
    );
    assert_eq!(
        reject_semantic_priority_as_foreground_reservation(),
        Err(ForegroundReservationAdmissionDenial::SemanticPriorityCannotReserve)
    );
    assert_eq!(
        reject_copied_s5_counters_as_foreground_reservation(),
        Err(ForegroundReservationAdmissionDenial::CopiedS5CountersCannotReserve)
    );
    assert_eq!(
        reject_copied_security_scope_fields_as_foreground_reservation(),
        Err(ForegroundReservationAdmissionDenial::CopiedSecurityScopeFieldsCannotReserve)
    );
    assert_eq!(
        reject_terminal_projection_as_foreground_reservation(),
        Err(ForegroundReservationAdmissionDenial::TerminalProjectionCannotReserve)
    );
}
