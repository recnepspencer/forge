use super::{OperationalCounterReceipt, OperationalCounterStructureDenial, OperationalSessionKind};

#[test]
fn a_structurally_empty_streaming_receipt_is_rejected() {
    let receipt =
        OperationalCounterReceipt::empty_for_test(OperationalSessionKind::ReplicaBootstrap);
    assert_eq!(
        receipt.validate_structure(),
        Err(OperationalCounterStructureDenial::EmptyWork)
    );
}

#[test]
fn promotion_requires_exactly_one_external_fence_and_authorization() {
    let mut receipt =
        OperationalCounterReceipt::empty_for_test(OperationalSessionKind::ReplicaPromotion);
    receipt.set_test_structure(0, 1, 0, 1, 0, 2);
    assert_eq!(
        receipt.validate_structure(),
        Err(OperationalCounterStructureDenial::InvalidFenceCount)
    );
    receipt.set_test_structure(0, 1, 0, 1, 1, 2);
    receipt.validate_structure().unwrap();
}

#[test]
fn offline_verification_requires_real_media_breadth_and_a_resident_bound() {
    let mut receipt =
        OperationalCounterReceipt::empty_for_test(OperationalSessionKind::OfflineVerification);
    receipt.set_test_structure(0, 1, 0, 0, 0, 0);
    assert_eq!(
        receipt.validate_structure(),
        Err(OperationalCounterStructureDenial::MissingStreamingBreadth)
    );
    receipt.set_test_structure(4096, 1, 0, 0, 0, 0);
    assert_eq!(
        receipt.validate_structure(),
        Err(OperationalCounterStructureDenial::MissingResidentBound)
    );
    receipt.set_test_structure(4096, 1, 4096, 0, 0, 0);
    receipt.validate_structure().unwrap();
}
