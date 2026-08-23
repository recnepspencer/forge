use super::super::tests::{basis, installed_owner};
use super::super::*;

#[test]
fn owner_denies_distinct_pending_work_at_its_declared_capacity() {
    let mut owner = installed_owner();
    let mut receipts = Vec::new();
    for slot in 0..WORTH_UI_PRESENTATION_PENDING_CAPACITY {
        receipts.push(owner.admit_pending(basis(slot as u16 + 100)).unwrap());
    }

    assert!(matches!(
        owner.admit_pending(basis(999)),
        Err(WorthUiPresentationPendingAdmissionDenial::PendingCapacityExhausted)
    ));
    assert_eq!(receipts.len(), WORTH_UI_PRESENTATION_PENDING_CAPACITY);
}
