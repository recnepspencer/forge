use worth_store_physical_backend::{BackendDurabilityProfile, WalDurabilityBarrier};
use worth_store_recovery_physics::{
    DurableAckReceipt, IllegalAcknowledgmentDenial, IllegalAcknowledgmentDenialKind,
};

pub fn assert_ack_basis<P: BackendDurabilityProfile>(
    receipt: &DurableAckReceipt<P>,
    expected_completed_barrier: WalDurabilityBarrier,
) {
    assert_eq!(receipt.profile_id(), P::ID);
    assert_eq!(receipt.ack_basis().profile_id(), P::ID);
    assert_eq!(
        receipt.ack_basis().required_barriers(),
        P::REQUIRED_BARRIERS
    );
    assert_eq!(
        receipt.ack_basis().completed_barriers(),
        P::REQUIRED_BARRIERS
    );
    assert!(P::REQUIRED_BARRIERS.contains(expected_completed_barrier));
    assert_eq!(receipt.ack_basis().segment_id().get(), 42);
    assert_eq!(receipt.ack_basis().generation().get(), 7);
    assert_eq!(receipt.ack_basis().lsn_range().start().get(), 100);
    assert_eq!(receipt.ack_basis().lsn_range().end_exclusive().get(), 101);
    assert!(!receipt.ack_basis().frame_digest().as_str().is_empty());
}

pub fn assert_denial<T>(
    result: Result<T, IllegalAcknowledgmentDenial>,
    expected: IllegalAcknowledgmentDenialKind,
) {
    let denial = expect_denial(result);
    assert_eq!(denial.kind(), expected);
}

pub fn assert_missing_barrier<T>(
    result: Result<T, IllegalAcknowledgmentDenial>,
    expected_missing: WalDurabilityBarrier,
) {
    let denial = expect_denial(result);
    assert_eq!(
        denial.kind(),
        IllegalAcknowledgmentDenialKind::RequiredBarrierMissing
    );
    assert_eq!(denial.barrier(), Some(expected_missing));
    assert!(denial.required_barriers().is_some());
    assert!(denial.completed_barriers().is_some());
}

pub fn assert_short_write<T>(result: Result<T, IllegalAcknowledgmentDenial>) {
    let denial = expect_denial(result);
    assert_eq!(denial.kind(), IllegalAcknowledgmentDenialKind::ShortWrite);
    assert_eq!(denial.expected_bytes(), Some(4096));
    assert_eq!(denial.observed_bytes(), Some(128));
    assert_eq!(denial.segment_id().map(|segment| segment.get()), Some(42));
}

fn expect_denial<T>(result: Result<T, IllegalAcknowledgmentDenial>) -> IllegalAcknowledgmentDenial {
    match result {
        Ok(_) => panic!("expected illegal acknowledgment denial"),
        Err(denial) => denial,
    }
}
