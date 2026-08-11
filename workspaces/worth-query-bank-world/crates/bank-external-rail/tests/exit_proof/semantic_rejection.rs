//! Hostile protocol semantics must fail before the external owner admits work.

use bank_external_rail::{
    dispatch, inquire_admission_count, inquire_notice, inquire_status, LedgerStatus,
    RailCorrelation, RailDispatch, RailEffectPayload, RailExchangeOutcome, RailRejection,
};
use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};

use crate::support::{
    spawn_rail, EXPECTED_EFFECT, EXPECTED_PROTOCOL_IDENTITY, EXPECTED_PROTOCOL_VERSION,
    FRAME_TIMEOUT, NOTICE_BOUND,
};

const ESTATE: u64 = 8_101;
const NOTICE: u64 = 8_102;
const SUBJECT: u64 = 8_103;

struct HostileCase {
    scenario: &'static str,
    payload: RailEffectPayload,
    expected: RailRejection,
}

#[tokio::test]
async fn hostile_protocol_matrix_rejects_before_ledger_and_healthy_twin_completes() {
    let rail = spawn_rail();
    for (index, hostile) in hostile_cases().into_iter().enumerate() {
        assert_hostile_rejection(rail.addr, hostile, index as u8).await;
    }
    assert_healthy_twin(rail.addr).await;
}

fn hostile_cases() -> [HostileCase; 4] {
    [
        HostileCase {
            scenario: "wrong-effect",
            payload: payload(
                "OtherEffect",
                EXPECTED_PROTOCOL_IDENTITY,
                EXPECTED_PROTOCOL_VERSION,
                NOTICE_BOUND,
                bytes(),
            ),
            expected: RailRejection::UnknownEffect,
        },
        HostileCase {
            scenario: "wrong-protocol-identity",
            payload: payload(
                EXPECTED_EFFECT,
                BoundaryProtocolIdentity::new("bank.estate.other-notice"),
                EXPECTED_PROTOCOL_VERSION,
                NOTICE_BOUND,
                bytes(),
            ),
            expected: RailRejection::UnknownProtocolIdentity,
        },
        HostileCase {
            scenario: "dishonest-bound",
            payload: payload(
                EXPECTED_EFFECT,
                EXPECTED_PROTOCOL_IDENTITY,
                EXPECTED_PROTOCOL_VERSION,
                NOTICE_BOUND + 1,
                bytes(),
            ),
            expected: RailRejection::DeclaredBoundMismatch,
        },
        HostileCase {
            scenario: "malformed-bytes",
            payload: payload(
                EXPECTED_EFFECT,
                EXPECTED_PROTOCOL_IDENTITY,
                EXPECTED_PROTOCOL_VERSION,
                NOTICE_BOUND,
                vec![0xA5; NOTICE_BOUND as usize - 1],
            ),
            expected: RailRejection::MalformedNotice,
        },
    ]
}

async fn assert_hostile_rejection(address: std::net::SocketAddr, hostile: HostileCase, suffix: u8) {
    let correlation = correlation(hostile.scenario, suffix);
    let outcome = dispatch(
        address,
        attempt(correlation.clone(), hostile.payload),
        FRAME_TIMEOUT,
    )
    .await;
    assert_eq!(
        outcome,
        RailExchangeOutcome::Rejected(hostile.expected),
        "{}",
        hostile.scenario
    );
    assert_eq!(
        inquire_status(address, correlation.clone(), FRAME_TIMEOUT)
            .await
            .expect("the rail answers status after rejection"),
        LedgerStatus::NoRecord,
        "{}: rejection must create no status",
        hostile.scenario
    );
    assert_eq!(
        inquire_notice(address, correlation, FRAME_TIMEOUT)
            .await
            .expect("the rail answers notice inquiry after rejection"),
        None,
        "{}: rejection must retain no decoded meaning",
        hostile.scenario
    );
    assert_eq!(admission_count(address).await, 0, "{}", hostile.scenario);
}

async fn assert_healthy_twin(address: std::net::SocketAddr) {
    let healthy = correlation("healthy-after-hostility", 0xFE);
    assert_eq!(
        dispatch(
            address,
            attempt(
                healthy.clone(),
                payload(
                    EXPECTED_EFFECT,
                    EXPECTED_PROTOCOL_IDENTITY,
                    EXPECTED_PROTOCOL_VERSION,
                    NOTICE_BOUND,
                    bytes(),
                ),
            ),
            FRAME_TIMEOUT,
        )
        .await,
        RailExchangeOutcome::Completed
    );
    assert_eq!(admission_count(address).await, 1);
    let decoded = inquire_notice(address, healthy, FRAME_TIMEOUT)
        .await
        .expect("the healthy notice inquiry reaches the rail")
        .expect("the healthy payload is admitted with decoded meaning");
    assert_eq!(decoded.estate(), ESTATE);
    assert_eq!(decoded.notice(), NOTICE);
    assert_eq!(decoded.subject(), SUBJECT);
}

fn payload(
    effect: &str,
    protocol_identity: BoundaryProtocolIdentity,
    protocol_version: BoundaryProtocolVersion,
    maximum_bytes: u64,
    bytes: Vec<u8>,
) -> RailEffectPayload {
    RailEffectPayload::new(
        effect,
        protocol_identity,
        protocol_version,
        maximum_bytes,
        bytes,
    )
}

fn attempt(correlation: RailCorrelation, payload: RailEffectPayload) -> RailDispatch {
    RailDispatch {
        correlation,
        payload,
    }
}

fn correlation(scenario: &str, suffix: u8) -> RailCorrelation {
    RailCorrelation::new(
        "phase-8-f2-hostility",
        [scenario.as_bytes(), &[suffix]].concat(),
    )
}

fn bytes() -> Vec<u8> {
    [
        ESTATE.to_be_bytes(),
        NOTICE.to_be_bytes(),
        SUBJECT.to_be_bytes(),
    ]
    .concat()
}

async fn admission_count(address: std::net::SocketAddr) -> u64 {
    inquire_admission_count(address, FRAME_TIMEOUT)
        .await
        .expect("the rail answers admission-count inquiries")
}
