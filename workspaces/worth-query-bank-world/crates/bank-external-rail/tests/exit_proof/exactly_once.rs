//! Atomic reservation and immutable-correlation evidence under real TCP races.

use std::sync::Arc;

use bank_external_rail::{
    dispatch, inquire_admission_count, inquire_completed_effect_count, inquire_completed_notice,
    inquire_notice, inquire_status, LedgerStatus, RailDispatch, RailEffectPayload,
    RailExchangeOutcome, RailRejection,
};
use tokio::sync::Barrier;
use worth_foundational::facade::BoundaryProtocolVersion;

use crate::support::{
    attempt_for, correlation_for, notice_payload, spawn_rail, ESTATE, EXPECTED_EFFECT,
    EXPECTED_PROTOCOL_IDENTITY, EXPECTED_PROTOCOL_VERSION, FRAME_TIMEOUT, NOTICE, NOTICE_BOUND,
    SUBJECT,
};

const CONCURRENT_DUPLICATES: usize = 16;

#[tokio::test]
async fn concurrent_duplicates_reserve_and_apply_one_physical_effect() {
    let rail = spawn_rail();
    let attempt = attempt_for("concurrent-duplicates");
    let correlation = attempt.correlation.clone();
    let barrier = Arc::new(Barrier::new(CONCURRENT_DUPLICATES));
    let mut tasks = Vec::with_capacity(CONCURRENT_DUPLICATES);

    for _ in 0..CONCURRENT_DUPLICATES {
        let barrier = Arc::clone(&barrier);
        let attempt = attempt.clone();
        let address = rail.addr;
        tasks.push(tokio::spawn(async move {
            barrier.wait().await;
            dispatch(address, attempt, FRAME_TIMEOUT).await
        }));
    }

    for task in tasks {
        let outcome = task.await.expect("duplicate dispatch task completes");
        assert!(matches!(
            outcome,
            RailExchangeOutcome::Acknowledged | RailExchangeOutcome::Completed
        ));
    }
    assert_exact_single_completion(rail.addr, correlation, (ESTATE, NOTICE, SUBJECT)).await;
}

#[tokio::test]
async fn concurrent_same_key_payload_drift_rejects_the_loser_without_mutating_truth() {
    let rail = spawn_rail();
    let correlation = correlation_for("concurrent-payload-drift");
    let original = RailDispatch {
        correlation: correlation.clone(),
        payload: notice_payload(),
    };
    let drifted = RailDispatch {
        correlation: correlation.clone(),
        payload: payload_for_subject(SUBJECT + 1),
    };
    let outcomes = race_dispatches(rail.addr, original.clone(), drifted.clone()).await;
    assert_one_completion_and_one_drift(&outcomes);
    let completed = observe_matching_completed_notice(rail.addr, correlation.clone()).await;
    assert!(completed.subject() == SUBJECT || completed.subject() == SUBJECT + 1);

    let rejected_retry = if completed.subject() == SUBJECT {
        drifted
    } else {
        original
    };
    assert_eq!(
        dispatch(rail.addr, rejected_retry, FRAME_TIMEOUT).await,
        RailExchangeOutcome::Rejected(RailRejection::CorrelationPayloadMismatch)
    );
    assert_exact_single_completion(
        rail.addr,
        correlation,
        (completed.estate(), completed.notice(), completed.subject()),
    )
    .await;
}

async fn race_dispatches(
    address: std::net::SocketAddr,
    original: RailDispatch,
    drifted: RailDispatch,
) -> [RailExchangeOutcome; 2] {
    let barrier = Arc::new(Barrier::new(2));
    let original_task = spawn_racing_dispatch(address, original, Arc::clone(&barrier));
    let drifted_task = spawn_racing_dispatch(address, drifted, barrier);
    [
        original_task.await.expect("original dispatch task"),
        drifted_task.await.expect("drifted dispatch task"),
    ]
}

fn assert_one_completion_and_one_drift(outcomes: &[RailExchangeOutcome; 2]) {
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| **outcome == RailExchangeOutcome::Completed)
            .count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| {
                **outcome
                    == RailExchangeOutcome::Rejected(RailRejection::CorrelationPayloadMismatch)
            })
            .count(),
        1
    );
}

async fn observe_matching_completed_notice(
    address: std::net::SocketAddr,
    correlation: bank_external_rail::RailCorrelation,
) -> bank_external_rail::EstateDeathNotice {
    let completed = inquire_completed_notice(address, correlation.clone(), FRAME_TIMEOUT)
        .await
        .expect("the physical consequence owner answers")
        .expect("exactly one racing request completed");
    let admitted = inquire_notice(address, correlation, FRAME_TIMEOUT)
        .await
        .expect("the idempotency owner answers")
        .expect("exactly one racing request was admitted");
    assert_eq!(completed, admitted);
    completed
}

#[tokio::test]
async fn correlation_binds_exact_payload_even_when_another_version_decodes_the_same_notice() {
    let rail = spawn_rail();
    let original = attempt_for("cross-version-drift");
    let correlation = original.correlation.clone();
    assert_eq!(
        dispatch(rail.addr, original, FRAME_TIMEOUT).await,
        RailExchangeOutcome::Completed
    );

    let mut v2_bytes = b"DEATHV2!".to_vec();
    v2_bytes.extend_from_slice(&ESTATE.to_be_bytes());
    v2_bytes.extend_from_slice(&NOTICE.to_be_bytes());
    v2_bytes.extend_from_slice(&SUBJECT.to_be_bytes());
    let alternate_representation = RailDispatch {
        correlation: correlation.clone(),
        payload: RailEffectPayload::new(
            EXPECTED_EFFECT,
            EXPECTED_PROTOCOL_IDENTITY,
            BoundaryProtocolVersion::new(2),
            32,
            v2_bytes,
        ),
    };
    assert_eq!(
        dispatch(rail.addr, alternate_representation, FRAME_TIMEOUT).await,
        RailExchangeOutcome::Rejected(RailRejection::CorrelationPayloadMismatch)
    );
    assert_exact_single_completion(rail.addr, correlation, (ESTATE, NOTICE, SUBJECT)).await;
}

fn spawn_racing_dispatch(
    address: std::net::SocketAddr,
    attempt: RailDispatch,
    barrier: Arc<Barrier>,
) -> tokio::task::JoinHandle<RailExchangeOutcome> {
    tokio::spawn(async move {
        barrier.wait().await;
        dispatch(address, attempt, FRAME_TIMEOUT).await
    })
}

async fn assert_exact_single_completion(
    address: std::net::SocketAddr,
    correlation: bank_external_rail::RailCorrelation,
    expected: (u64, u64, u64),
) {
    assert_eq!(
        inquire_status(address, correlation.clone(), FRAME_TIMEOUT)
            .await
            .expect("the idempotency owner answers"),
        LedgerStatus::Completed
    );
    assert_eq!(
        inquire_admission_count(address, FRAME_TIMEOUT)
            .await
            .expect("the admission counter answers"),
        1
    );
    assert_eq!(
        inquire_completed_effect_count(address, FRAME_TIMEOUT)
            .await
            .expect("the physical consequence counter answers"),
        1
    );
    let completed = inquire_completed_notice(address, correlation, FRAME_TIMEOUT)
        .await
        .expect("the physical consequence owner answers")
        .expect("one physical consequence exists");
    assert_eq!(
        (completed.estate(), completed.notice(), completed.subject()),
        expected
    );
}

fn payload_for_subject(subject: u64) -> RailEffectPayload {
    let bytes = [
        ESTATE.to_be_bytes(),
        NOTICE.to_be_bytes(),
        subject.to_be_bytes(),
    ]
    .concat();
    RailEffectPayload::new(
        EXPECTED_EFFECT,
        EXPECTED_PROTOCOL_IDENTITY,
        EXPECTED_PROTOCOL_VERSION,
        NOTICE_BOUND,
        bytes,
    )
}
