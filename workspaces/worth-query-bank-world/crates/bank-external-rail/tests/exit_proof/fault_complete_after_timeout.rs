//! Fault 3: complete after timeout -- the rail delays past the caller's
//! deadline, then genuinely completes afterward.

use std::time::Duration;

use bank_external_rail::{
    dispatch, inquire_status, FaultScript, LedgerStatus, RailExchangeOutcome,
};

use crate::support::{attempt_for, correlation_for, spawn_rail};

const CALLER_DEADLINE: Duration = Duration::from_millis(300);
const RAIL_DELAY_MILLIS: u64 = 1_000;

#[tokio::test]
async fn late_completion_arrives_after_the_callers_deadline_has_already_elapsed() {
    let rail = spawn_rail();
    let correlation = correlation_for("complete-after-timeout");

    let outcome = dispatch(
        rail.addr,
        attempt_for(
            "complete-after-timeout",
            FaultScript::CompleteAfterDelay {
                delay_millis: RAIL_DELAY_MILLIS,
            },
        ),
        CALLER_DEADLINE,
    )
    .await;

    assert_eq!(
        outcome,
        RailExchangeOutcome::TimedOut,
        "the caller's deadline is far shorter than the rail's configured delay"
    );

    tokio::time::sleep(Duration::from_millis(RAIL_DELAY_MILLIS) + Duration::from_millis(500)).await;

    let status = inquire_status(rail.addr, correlation, Duration::from_secs(2))
        .await
        .expect("a fresh connection can still ask the rail's ledger");
    assert_eq!(
        status,
        LedgerStatus::Completed,
        "the rail actually completed the attempt after the caller had already given up"
    );
}
