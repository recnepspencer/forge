//! Fault 2: acknowledge without completing.

use bank_external_rail::test_control::FaultScript;
use bank_external_rail::{
    dispatch, inquire_admission_count, inquire_completed_effect_count, inquire_completed_notice,
    inquire_notice, inquire_status, LedgerStatus, RailExchangeOutcome,
};

use crate::support::{attempt_for, correlation_for, spawn_rail, FRAME_TIMEOUT};

#[tokio::test]
async fn acknowledgement_alone_never_reports_completed() {
    let rail = spawn_rail();
    let correlation = correlation_for("ack-without-completion");
    rail.select_fault(FaultScript::AcknowledgeWithoutCompleting)
        .await;

    let outcome = dispatch(
        rail.addr,
        attempt_for("ack-without-completion"),
        FRAME_TIMEOUT,
    )
    .await;

    assert_eq!(outcome, RailExchangeOutcome::Acknowledged);

    let status = inquire_status(rail.addr, correlation.clone(), FRAME_TIMEOUT)
        .await
        .expect("a fresh connection can still ask the rail's ledger");
    assert_eq!(
        status,
        LedgerStatus::Acknowledged,
        "the rail's own truth agrees: it accepted the attempt but never completed it"
    );
    assert_eq!(
        inquire_admission_count(rail.addr, FRAME_TIMEOUT)
            .await
            .expect("the ledger reports its admission count"),
        1
    );
    assert!(
        inquire_notice(rail.addr, correlation.clone(), FRAME_TIMEOUT)
            .await
            .expect("the ledger reports its admitted notice")
            .is_some(),
        "admission must retain the decoded notice before physical completion"
    );
    assert_eq!(
        inquire_completed_effect_count(rail.addr, FRAME_TIMEOUT)
            .await
            .expect("the physical consequence owner reports its count"),
        0,
        "acknowledgement must not increment the independent consequence count"
    );
    assert_eq!(
        inquire_completed_notice(rail.addr, correlation, FRAME_TIMEOUT)
            .await
            .expect("the physical consequence owner answers"),
        None,
        "ledger notice retention must not impersonate physical completion"
    );
}
