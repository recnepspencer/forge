use std::sync::Arc;

use bank_domain::model::AccountId;

use super::super::super::protocol::{
    BankHttpCommitDisposition, BankHttpEstateDisbursementOutcome,
    BankHttpRecoveryInspectionOutcome, BankHttpRecoveryPosture, BankHttpRedoProgressionOutcome,
    BankHttpUndoAdmissionOutcome, BankHttpUndoCorrection, BankHttpUndoProgressionOutcome,
};
use super::fixture::application;
use super::{bind_application, credential_json, BankHttpServerConfiguration};

#[tokio::test]
async fn disbursement_aftermath_remains_opaque_through_undo_and_redo() {
    let server = bind_application(
        Arc::new(application(AccountId::new(100).unwrap())),
        BankHttpServerConfiguration::local_ephemeral(),
    )
    .await
    .expect("HTTP server should bind");
    let client = reqwest::Client::new();
    let disbursed = post::<BankHttpEstateDisbursementOutcome>(
        &client,
        server.local_address(),
        "/v1/estate/disburse",
        disbursement_request("disburse-estate-http"),
    )
    .await;
    let recovery = applied_disbursement(disbursed);
    assert_compensatable(&client, server.local_address(), &recovery).await;
    let undo = admit_compensation(&client, server.local_address(), &recovery).await;
    let first_undo = progress_undo(
        &client,
        server.local_address(),
        UndoProgressionSpec {
            token: &undo,
            request_id: "progress-disbursement-undo",
            idempotency_key: "http-disbursement-undo-key",
        },
    )
    .await;
    let replayed_undo = progress_undo(
        &client,
        server.local_address(),
        UndoProgressionSpec {
            token: &undo,
            request_id: "progress-disbursement-undo",
            idempotency_key: "http-disbursement-undo-key",
        },
    )
    .await;
    assert_eq!(replayed_undo, first_undo);
    let redo = applied_undo(first_undo);
    let first_redo = progress_redo(&client, server.local_address(), &redo).await;
    let replayed_redo = progress_redo(&client, server.local_address(), &redo).await;
    assert_eq!(replayed_redo, first_redo);
    assert!(matches!(
        first_redo,
        BankHttpRedoProgressionOutcome::Applied {
            disposition: BankHttpCommitDisposition::Committed,
            ..
        }
    ));
    server.shutdown().await.expect("server should shut down");
}

fn disbursement_request(request_id: &str) -> serde_json::Value {
    serde_json::json!({
        "protocol": "v1",
        "request_id": request_id,
        "credential": credential_json(),
        "controls": { "deadline_milliseconds": 5_000 },
        "idempotency_key": "http-disbursement-key",
        "estate": "fixture:3",
        "source_account": "fixture:101",
        "destination_account": "fixture:102",
        "beneficiary": "fixture:3",
        "amount_minor_units": 250
    })
}

fn applied_disbursement(outcome: BankHttpEstateDisbursementOutcome) -> String {
    match outcome {
        BankHttpEstateDisbursementOutcome::Applied {
            disposition: BankHttpCommitDisposition::Committed,
            recovery,
            ..
        } => recovery,
        other => panic!("disbursement did not commit: {other:?}"),
    }
}

async fn assert_compensatable(
    client: &reqwest::Client,
    address: std::net::SocketAddr,
    recovery: &str,
) {
    let inspected = post::<BankHttpRecoveryInspectionOutcome>(
        client,
        address,
        "/v1/recovery/inspect",
        recovery_request("inspect-disbursement", recovery),
    )
    .await;
    assert!(
        matches!(
            &inspected,
            BankHttpRecoveryInspectionOutcome::Inspected {
                posture: BankHttpRecoveryPosture::Compensatable,
                ..
            }
        ),
        "disbursement recovery was not compensatable: {inspected:?}"
    );
}

async fn admit_compensation(
    client: &reqwest::Client,
    address: std::net::SocketAddr,
    recovery: &str,
) -> String {
    let admitted = post::<BankHttpUndoAdmissionOutcome>(
        client,
        address,
        "/v1/recovery/admit-undo",
        recovery_request("admit-disbursement-undo", recovery),
    )
    .await;
    match admitted {
        BankHttpUndoAdmissionOutcome::Admitted {
            correction: BankHttpUndoCorrection::Compensation,
            undo,
            ..
        } => undo,
        other => panic!("compensation undo did not admit: {other:?}"),
    }
}

async fn progress_undo(
    client: &reqwest::Client,
    address: std::net::SocketAddr,
    spec: UndoProgressionSpec<'_>,
) -> BankHttpUndoProgressionOutcome {
    post(
        client,
        address,
        "/v1/recovery/progress-undo",
        serde_json::json!({
            "protocol": "v1",
            "request_id": spec.request_id,
            "credential": credential_json(),
            "controls": { "deadline_milliseconds": 5_000 },
            "undo": spec.token,
            "idempotency_key": spec.idempotency_key
        }),
    )
    .await
}

struct UndoProgressionSpec<'a> {
    token: &'a str,
    request_id: &'a str,
    idempotency_key: &'a str,
}

fn applied_undo(outcome: BankHttpUndoProgressionOutcome) -> String {
    match outcome {
        BankHttpUndoProgressionOutcome::Applied {
            disposition: BankHttpCommitDisposition::Committed,
            redo,
            ..
        } => redo,
        other => panic!("compensation undo did not commit: {other:?}"),
    }
}

async fn progress_redo(
    client: &reqwest::Client,
    address: std::net::SocketAddr,
    redo: &str,
) -> BankHttpRedoProgressionOutcome {
    post(
        client,
        address,
        "/v1/recovery/progress-redo",
        serde_json::json!({
            "protocol": "v1",
            "request_id": "progress-disbursement-redo",
            "credential": credential_json(),
            "controls": { "deadline_milliseconds": 5_000 },
            "redo": redo
        }),
    )
    .await
}

fn recovery_request(request_id: &str, recovery: &str) -> serde_json::Value {
    serde_json::json!({
        "protocol": "v1",
        "request_id": request_id,
        "credential": credential_json(),
        "controls": { "deadline_milliseconds": 5_000 },
        "recovery": recovery
    })
}

async fn post<T: serde::de::DeserializeOwned>(
    client: &reqwest::Client,
    address: std::net::SocketAddr,
    path: &str,
    request: serde_json::Value,
) -> T {
    client
        .post(format!("http://{address}{path}"))
        .json(&request)
        .send()
        .await
        .expect("typed HTTP request should complete")
        .json()
        .await
        .expect("typed HTTP response should decode")
}
