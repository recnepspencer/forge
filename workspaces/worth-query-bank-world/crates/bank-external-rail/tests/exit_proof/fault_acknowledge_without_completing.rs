//! Fault 2: acknowledge without completing.

use bank_external_rail::{
    dispatch, inquire_status, FaultScript, LedgerStatus, RailExchangeOutcome,
};

use crate::support::{attempt_for, correlation_for, spawn_rail, FRAME_TIMEOUT};

#[tokio::test]
async fn acknowledgement_alone_never_reports_completed() {
    let rail = spawn_rail();
    let correlation = correlation_for("ack-without-completion");

    let outcome = dispatch(
        rail.addr,
        attempt_for(
            "ack-without-completion",
            FaultScript::AcknowledgeWithoutCompleting,
        ),
        FRAME_TIMEOUT,
    )
    .await;

    assert_eq!(outcome, RailExchangeOutcome::Acknowledged);

    let status = inquire_status(rail.addr, correlation, FRAME_TIMEOUT)
        .await
        .expect("a fresh connection can still ask the rail's ledger");
    assert_eq!(
        status,
        LedgerStatus::Acknowledged,
        "the rail's own truth agrees: it accepted the attempt but never completed it"
    );
}
