use worth_store_io_scheduler::foreground_reservation::{
    admitted_point_read_reservation_for_certification_test,
    admitted_range_read_reservation_for_certification_test, ForegroundReservationState,
};

use crate::{certify_io_qos_foreground_reservation, S6ForegroundReservationCertificationDenial};

#[test]
fn io_qos_foreground_reservation_certification_preserves_exact_receipt_fields() {
    let receipt = admitted_point_read_reservation_for_certification_test();
    let expected = admitted_point_read_reservation_for_certification_test();

    let evidence = certify_io_qos_foreground_reservation(receipt, expected)
        .expect("matching Store reservation receipt should certify");

    assert_eq!(
        evidence.state(),
        ForegroundReservationState::ReservationAdmitted
    );
    assert_eq!(evidence.lane(), expected.lane());
    assert_eq!(
        evidence.backend_requirement(),
        expected.backend_requirement()
    );
    assert_eq!(evidence.backend_profile(), expected.backend_profile());
    assert_eq!(
        evidence.backend_evidence_class(),
        expected.backend_evidence_class()
    );
    assert_eq!(evidence.envelope(), expected.envelope());
    assert_eq!(evidence.counters(), expected.counters());
    assert_eq!(
        evidence.security_scope_identity(),
        expected.security_scope_identity()
    );
}

#[test]
fn io_qos_foreground_reservation_certification_denies_mismatched_receipts() {
    let receipt = admitted_point_read_reservation_for_certification_test();
    let mismatched = admitted_range_read_reservation_for_certification_test();

    assert_eq!(
        certify_io_qos_foreground_reservation(receipt, mismatched),
        Err(S6ForegroundReservationCertificationDenial::ReceiptMismatch)
    );
}
