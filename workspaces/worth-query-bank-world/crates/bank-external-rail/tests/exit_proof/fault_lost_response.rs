//! Fault 1: commit then lose the response.
//!
//! The caller's connection is closed with zero bytes, yet the rail's own
//! ledger -- reachable only by asking it, not by inference -- proves the
//! effect really completed. This is the exact indeterminacy Gate 8.2 must
//! not resolve by guessing.

use bank_external_rail::test_control::FaultScript;
use bank_external_rail::{dispatch, inquire_status, LedgerStatus, RailExchangeOutcome};

use crate::support::{attempt_for, correlation_for, spawn_rail, FRAME_TIMEOUT};

#[tokio::test]
async fn commit_then_lose_response_never_reports_completed_to_the_caller() {
    let rail = spawn_rail();
    let correlation = correlation_for("lost-response");
    rail.select_fault(FaultScript::CommitThenLoseResponse).await;

    let outcome = dispatch(rail.addr, attempt_for("lost-response"), FRAME_TIMEOUT).await;

    assert_eq!(outcome, RailExchangeOutcome::Disconnected);

    let status = inquire_status(rail.addr, correlation, FRAME_TIMEOUT)
        .await
        .expect("a fresh connection can still ask the rail's ledger");
    assert_eq!(
        status,
        LedgerStatus::Completed,
        "the rail's own truth shows the effect committed, even though the caller lost the reply"
    );
}
