use std::sync::atomic::Ordering;
use std::sync::Arc;

use super::provider_session_protocol::{
    cleanup, session_run, SessionCallCounts, SessionFailurePoint,
};

#[test]
fn abandoned_or_failed_protocol_states_abort_the_physical_session() {
    let calls = Arc::new(SessionCallCounts::default());
    let (mut running, graph) = session_run(SessionFailurePoint::None, Arc::clone(&calls), false);
    drop(
        running
            .admit_provider_execution_plan(&graph)
            .unwrap()
            .readmit()
            .unwrap(),
    );
    assert_eq!(calls.aborts.load(Ordering::Acquire), 1);
    cleanup(running);

    let calls = Arc::new(SessionCallCounts::default());
    let (mut running, graph) = session_run(SessionFailurePoint::None, Arc::clone(&calls), false);
    drop(
        running
            .admit_provider_execution_plan(&graph)
            .unwrap()
            .readmit()
            .unwrap()
            .prepare()
            .unwrap(),
    );
    assert_eq!(calls.aborts.load(Ordering::Acquire), 1);
    cleanup(running);

    let calls = Arc::new(SessionCallCounts::default());
    let (mut running, graph) = session_run(
        SessionFailurePoint::PreparationPanic,
        Arc::clone(&calls),
        false,
    );
    let _ = running
        .admit_provider_execution_plan(&graph)
        .unwrap()
        .readmit()
        .unwrap()
        .prepare()
        .expect_err("preparation panic must be contained");
    assert_eq!(calls.aborts.load(Ordering::Acquire), 1);
    cleanup(running);
}
