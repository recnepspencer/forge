use bank_external_rail::{
    dispatch, inquire_admission_count, inquire_notice, inquire_status, FaultScript, LedgerStatus,
    RailCorrelation, RailDispatch, RailEffectPayload, RailExchangeOutcome,
    RailProtocolSupportProfile, RailRejection,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use worth_foundational::facade::{
    BoundaryProtocolIdentity, BoundaryProtocolUnsupportedVersionPosture, BoundaryProtocolVersion,
};

use crate::support::{
    spawn_rail, spawn_rail_with_protocol_support, EXPECTED_EFFECT, EXPECTED_PROTOCOL_IDENTITY,
    FRAME_TIMEOUT,
};

const V1_CORPUS: &str =
    include_str!("../../../../protocol-corpus/estate-death-notification/v1.hex");
const V2_CORPUS: &str =
    include_str!("../../../../protocol-corpus/estate-death-notification/v2.hex");

#[tokio::test]
async fn distinct_v1_v2_corpora_coexist_without_cross_version_fallback() {
    let rail = spawn_rail();
    for (scenario, version, maximum, corpus) in [
        ("healthy-v1", 1, 24, V1_CORPUS),
        ("healthy-v2", 2, 32, V2_CORPUS),
    ] {
        let correlation = correlation(scenario);
        assert_eq!(
            dispatch_payload(
                rail.addr,
                correlation.clone(),
                EXPECTED_PROTOCOL_IDENTITY,
                version,
                maximum,
                corpus,
            )
            .await,
            RailExchangeOutcome::Completed
        );
        let notice = inquire_notice(rail.addr, correlation, FRAME_TIMEOUT)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            (notice.estate(), notice.notice(), notice.subject()),
            (8101, 8102, 8103)
        );
    }
    assert_eq!(admission_count(rail.addr).await, 2);

    for (scenario, version, maximum, corpus) in [
        ("v1-bytes-as-v2", 2, 32, V1_CORPUS),
        ("v2-bytes-as-v1", 1, 24, V2_CORPUS),
    ] {
        let correlation = correlation(scenario);
        assert_eq!(
            dispatch_payload(
                rail.addr,
                correlation.clone(),
                EXPECTED_PROTOCOL_IDENTITY,
                version,
                maximum,
                corpus,
            )
            .await,
            RailExchangeOutcome::Rejected(RailRejection::MalformedNotice)
        );
        assert_unadmitted(rail.addr, correlation, 2).await;
    }

    let wrong_prefix_correlation = correlation("v2-wrong-prefix");
    let mut wrong_prefix = decode_hex(V2_CORPUS);
    wrong_prefix[0] ^= 0xFF;
    assert_eq!(
        dispatch_bytes(
            rail.addr,
            wrong_prefix_correlation.clone(),
            EXPECTED_PROTOCOL_IDENTITY,
            2,
            32,
            wrong_prefix,
        )
        .await,
        RailExchangeOutcome::Rejected(RailRejection::MalformedNotice)
    );
    assert_unadmitted(rail.addr, wrong_prefix_correlation, 2).await;

    let correlation = correlation("future-v3");
    let outcome = dispatch_payload(
        rail.addr,
        correlation.clone(),
        EXPECTED_PROTOCOL_IDENTITY,
        3,
        32,
        V2_CORPUS,
    )
    .await;
    let RailExchangeOutcome::Rejected(RailRejection::UnsupportedProtocolVersion(unsupported)) =
        outcome
    else {
        panic!("future version must retain its typed unsupported posture: {outcome:?}");
    };
    assert_eq!(unsupported.produced(), BoundaryProtocolVersion::new(3));
    assert_eq!(
        unsupported.posture(),
        BoundaryProtocolUnsupportedVersionPosture::ExceedsWindow
    );
    assert_unadmitted(rail.addr, correlation, 2).await;
}

#[tokio::test]
async fn external_owner_preserves_predating_retired_and_threshold_postures() {
    let v2_only = spawn_rail_with_protocol_support(RailProtocolSupportProfile::V2Only);
    assert_unsupported_v1(
        v2_only.addr,
        "predates-v2-only",
        BoundaryProtocolUnsupportedVersionPosture::PredatesWindow,
    )
    .await;

    let retired = spawn_rail_with_protocol_support(RailProtocolSupportProfile::V1Retired);
    assert_unsupported_v1(
        retired.addr,
        "retired-v1",
        BoundaryProtocolUnsupportedVersionPosture::Retired,
    )
    .await;
    let threshold = correlation("retirement-threshold-v2");
    assert_eq!(
        dispatch_payload(
            retired.addr,
            threshold.clone(),
            EXPECTED_PROTOCOL_IDENTITY,
            2,
            32,
            V2_CORPUS,
        )
        .await,
        RailExchangeOutcome::Completed
    );
    assert_eq!(admission_count(retired.addr).await, 1);
    assert!(inquire_notice(retired.addr, threshold, FRAME_TIMEOUT)
        .await
        .unwrap()
        .is_some());
}

#[tokio::test]
async fn untrusted_frame_deserialization_denies_invalid_protocol_values_before_admission() {
    let rail = spawn_rail();
    for (scenario, field, invalid) in [
        (
            "raw-version-fused-identity",
            "protocol_identity",
            serde_json::json!("bank.estate.death-notification.v1"),
        ),
        (
            "raw-uppercase-identity",
            "protocol_identity",
            serde_json::json!("Bank.estate.death-notification"),
        ),
        ("raw-zero-version", "protocol_version", serde_json::json!(0)),
    ] {
        let correlation = correlation(scenario);
        let mut request = serde_json::json!({
            "Dispatch": serde_json::to_value(RailDispatch {
                correlation: correlation.clone(),
                payload: RailEffectPayload::new(
                    EXPECTED_EFFECT,
                    EXPECTED_PROTOCOL_IDENTITY,
                    BoundaryProtocolVersion::new(1),
                    24,
                    decode_hex(V1_CORPUS),
                ),
                fault_script: FaultScript::Succeed,
            })
            .unwrap()
        });
        request["Dispatch"]["payload"][field] = invalid;
        send_invalid_raw_frame(rail.addr, &request).await;
        assert_unadmitted(rail.addr, correlation, 0).await;
    }
}

async fn send_invalid_raw_frame(address: std::net::SocketAddr, request: &serde_json::Value) {
    let payload = serde_json::to_vec(request).unwrap();
    let mut stream = tokio::net::TcpStream::connect(address).await.unwrap();
    stream
        .write_all(&u32::try_from(payload.len()).unwrap().to_le_bytes())
        .await
        .unwrap();
    stream.write_all(&payload).await.unwrap();
    stream.shutdown().await.unwrap();
    let mut response = [0u8; 1];
    assert_eq!(stream.read(&mut response).await.unwrap(), 0);
}

async fn assert_unsupported_v1(
    address: std::net::SocketAddr,
    scenario: &str,
    expected: BoundaryProtocolUnsupportedVersionPosture,
) {
    let correlation = correlation(scenario);
    let outcome = dispatch_payload(
        address,
        correlation.clone(),
        EXPECTED_PROTOCOL_IDENTITY,
        1,
        24,
        V1_CORPUS,
    )
    .await;
    let RailExchangeOutcome::Rejected(RailRejection::UnsupportedProtocolVersion(unsupported)) =
        outcome
    else {
        panic!("external owner must preserve typed unsupported posture: {outcome:?}");
    };
    assert_eq!(unsupported.produced(), BoundaryProtocolVersion::new(1));
    assert_eq!(unsupported.posture(), expected);
    assert_unadmitted(address, correlation, 0).await;
}

async fn dispatch_payload(
    address: std::net::SocketAddr,
    correlation: RailCorrelation,
    identity: BoundaryProtocolIdentity,
    version: u32,
    maximum: u64,
    corpus: &str,
) -> RailExchangeOutcome {
    dispatch_bytes(
        address,
        correlation,
        identity,
        version,
        maximum,
        decode_hex(corpus),
    )
    .await
}

async fn dispatch_bytes(
    address: std::net::SocketAddr,
    correlation: RailCorrelation,
    identity: BoundaryProtocolIdentity,
    version: u32,
    maximum: u64,
    bytes: Vec<u8>,
) -> RailExchangeOutcome {
    dispatch(
        address,
        RailDispatch {
            correlation,
            payload: RailEffectPayload::new(
                EXPECTED_EFFECT,
                identity,
                BoundaryProtocolVersion::new(version),
                maximum,
                bytes,
            ),
            fault_script: FaultScript::Succeed,
        },
        FRAME_TIMEOUT,
    )
    .await
}

async fn assert_unadmitted(
    address: std::net::SocketAddr,
    correlation: RailCorrelation,
    expected_admissions: u64,
) {
    assert_eq!(
        inquire_status(address, correlation.clone(), FRAME_TIMEOUT)
            .await
            .unwrap(),
        LedgerStatus::NoRecord
    );
    assert_eq!(
        inquire_notice(address, correlation, FRAME_TIMEOUT)
            .await
            .unwrap(),
        None
    );
    assert_eq!(admission_count(address).await, expected_admissions);
}

fn correlation(scenario: &str) -> RailCorrelation {
    RailCorrelation::new("phase-8-c3-protocol", scenario.as_bytes().to_vec())
}

async fn admission_count(address: std::net::SocketAddr) -> u64 {
    inquire_admission_count(address, FRAME_TIMEOUT)
        .await
        .unwrap()
}

fn decode_hex(corpus: &str) -> Vec<u8> {
    let corpus = corpus.trim();
    assert!(corpus.len().is_multiple_of(2), "hex corpus has a remainder");
    let bytes = corpus
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(bytes.len() * 2, corpus.len());
    bytes
}
