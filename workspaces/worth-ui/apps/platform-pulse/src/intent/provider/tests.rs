use super::{
    PlatformPulseActionAttempt, PlatformPulseActionAttemptReference,
    PlatformPulseActionAttemptState, PlatformPulseActionPort, PlatformPulseExecutorGate,
};
use crate::intent::PlatformPulseActionPayload;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use worth_ui::facade::intent::{UiIntentProviderPoll, UiIntentProviderSettlement};

#[test]
fn intent_product_port_is_bounded_and_accounts_for_exact_settlement() {
    let (port, owner) = PlatformPulseActionPort::bounded();
    assert_eq!(port.census().retained(), 0);
    drop(owner);
    assert_eq!(port.census().submitted(), 0);
}

#[test]
fn intent_executor_gate_requires_exact_successor_revisions() {
    let gate = PlatformPulseExecutorGate::held(4);
    let denial = gate.apply(4, false).expect_err("equal revision is stale");
    assert_eq!((denial.active(), denial.submitted()), (4, 4));
    gate.apply(5, false).expect("successor gate revision");
    assert!(!gate.is_held());
    assert_eq!(gate.revision(), 5);
}

#[test]
fn intent_product_port_admits_sixteen_requests_and_rejects_the_seventeenth() {
    let (port, owner) = PlatformPulseActionPort::bounded();
    let gate = PlatformPulseExecutorGate::at(1, false);
    let mut attempts = (0..16)
        .map(|revision| action_attempt(&port, &gate, revision + 1))
        .collect::<Vec<_>>();
    for attempt in &mut attempts {
        assert!(matches!(
            attempt.submit(),
            UiIntentProviderPoll::PendingEffectMayHaveBegun
        ));
    }
    let mut overflow = action_attempt(&port, &gate, 17);
    match overflow.submit() {
        UiIntentProviderPoll::Settled(UiIntentProviderSettlement::FailedBeforeEffect(stop)) => {
            assert_eq!(stop.code(), "pulse-action-port-full");
        }
        UiIntentProviderPoll::PendingBeforeEffect
        | UiIntentProviderPoll::PendingEffectMayHaveBegun
        | UiIntentProviderPoll::Settled(_) => {
            panic!("the seventeenth request must fail before effect because the port is full")
        }
    }
    assert_eq!(port.census().submitted(), 16);
    assert_eq!(port.census().retained(), 16);

    for _ in 0..16 {
        let request = owner.try_next().expect("one admitted product request");
        assert!(request.reject_before_effect());
    }
    let census = owner.census();
    assert_eq!(census.received(), 16);
    assert_eq!(census.settled(), 16);
    assert_eq!(census.retained(), 0);
}

fn action_attempt(
    port: &PlatformPulseActionPort,
    gate: &PlatformPulseExecutorGate,
    revision: u64,
) -> PlatformPulseActionAttempt {
    PlatformPulseActionAttempt {
        reference: PlatformPulseActionAttemptReference::for_test(revision),
        state: PlatformPulseActionAttemptState::AwaitingGate(PlatformPulseActionPayload::for_test(
            revision,
        )),
        port: port.clone(),
        gate: gate.clone(),
        cancellation: Arc::new(AtomicBool::new(false)),
    }
}
