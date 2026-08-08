//! Fault 5: disappear mid-dispatch -- the rail closes the connection before
//! any ledger record exists, distinguishing it from a lost response after a
//! real commit.

use bank_external_rail::{
    dispatch, inquire_status, FaultScript, LedgerStatus, RailExchangeOutcome,
};

use crate::support::{attempt_for, correlation_for, spawn_rail, FRAME_TIMEOUT};

#[tokio::test]
async fn disappearance_leaves_no_ledger_record_at_all() {
    let rail = spawn_rail();
    let correlation = correlation_for("disappear-mid-dispatch");

    let outcome = dispatch(
        rail.addr,
        attempt_for("disappear-mid-dispatch", FaultScript::DisappearMidDispatch),
        FRAME_TIMEOUT,
    )
    .await;

    assert_eq!(outcome, RailExchangeOutcome::Disconnected);

    let status = inquire_status(rail.addr, correlation, FRAME_TIMEOUT)
        .await
        .expect("a fresh connection can still ask the rail's ledger");
    assert_eq!(
        status,
        LedgerStatus::NoRecord,
        "unlike a lost response after commit, disappearance never touched the ledger"
    );
}
