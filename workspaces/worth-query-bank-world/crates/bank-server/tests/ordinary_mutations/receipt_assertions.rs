use bank_server::{BankMutationOutcome, BankMutationStatus};

pub(super) fn assert_committed(outcome: BankMutationOutcome) {
    let BankMutationStatus::Committed(receipt) = outcome.status() else {
        panic!("unexpected mutation outcome: {outcome:?}");
    };
    assert!(!receipt.branch_id().is_empty());
    let session_identity = receipt
        .graph_work_session_identity()
        .expect("committed mutation must retain graph-session identity");
    assert!(session_identity.iter().any(|byte| *byte != 0));
    assert_eq!(
        receipt.graph_work_provider_session_identity(),
        receipt.graph_work_session_identity_hex().as_deref()
    );
    assert_eq!(receipt.graph_work_branch_id(), Some(receipt.branch_id()));
    assert!(receipt
        .graph_work_plan_identity()
        .is_some_and(|identity| identity.iter().any(|byte| *byte != 0)));
    assert!(receipt
        .graph_work_obligation_identity()
        .is_some_and(|identity| identity.iter().any(|byte| *byte != 0)));
    assert!(receipt
        .graph_work_required_obligation_count()
        .is_some_and(|count| count > 0));
    assert!(receipt
        .graph_work_released_reservation_count()
        .is_some_and(|count| count > 0));
    assert_eq!(receipt.graph_work_basis_released(), Some(true));
    assert!(outcome.metadata().projection_work().is_some());
}

pub(super) fn assert_emitting_commit(outcome: BankMutationOutcome) {
    let BankMutationStatus::Committed(receipt) = outcome.status() else {
        panic!("unexpected mutation outcome: {outcome:?}");
    };
    assert!(receipt.emitted_effect_count() > 0);
    assert!(outcome.metadata().provider_work_units() > 0);
}
