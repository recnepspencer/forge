//! Fault 4: duplicate acknowledgement -- the rail sends the acknowledgement
//! twice, and neither one may be mistaken for completion.

use bank_external_rail::{
    dispatch, inquire_status, FaultScript, LedgerStatus, RailExchangeOutcome,
};

use crate::support::{attempt_for, correlation_for, spawn_rail, FRAME_TIMEOUT};

#[tokio::test]
async fn duplicate_acknowledgement_is_distinct_from_completion() {
    let rail = spawn_rail();
    let correlation = correlation_for("duplicate-acknowledgement");

    let outcome = dispatch(
        rail.addr,
        attempt_for(
            "duplicate-acknowledgement",
            FaultScript::DuplicateAcknowledgement,
        ),
        FRAME_TIMEOUT,
    )
    .await;

    assert_eq!(outcome, RailExchangeOutcome::DuplicateAcknowledgement);

    let status = inquire_status(rail.addr, correlation, FRAME_TIMEOUT)
        .await
        .expect("a fresh connection can still ask the rail's ledger");
    assert_eq!(
        status,
        LedgerStatus::Acknowledged,
        "a duplicate acknowledgement is still only an acknowledgement, never a completion"
    );
}
