use std::net::SocketAddr;

use bank_http_adapter::{
    BankHttpCommitDisposition, BankHttpDenialKind, BankHttpEstateDisbursementOutcome,
    BankHttpRecoveryInspectionOutcome, BankHttpRecoveryPosture, BankHttpRedoProgressionOutcome,
    BankHttpUndoAdmissionOutcome, BankHttpUndoCorrection, BankHttpUndoProgressionOutcome,
};
use bank_user_node::{
    BankUserNodeEstateDisbursementOutcome, BankUserNodeRecoveryInspectionOutcome,
    BankUserNodeRedoProgressionOutcome, BankUserNodeUndoAdmissionOutcome,
    BankUserNodeUndoProgressionOutcome,
};

use super::post_node;

pub(super) async fn assert_opaque_disbursement_undo_redo(
    client: &reqwest::Client,
    primary: SocketAddr,
    specialist: SocketAddr,
) {
    let request = disbursement_request();
    drop_node_response(client, specialist, "/v1/estate/disburse", &request).await;
    let replayed = post_node::<BankUserNodeEstateDisbursementOutcome>(
        client,
        specialist,
        "/v1/estate/disburse",
        &request,
    )
    .await;
    let recovery = replayed_disbursement(&replayed).to_owned();
    assert_cross_user_recovery_denied(client, primary, &recovery).await;
    assert_compensatable(client, specialist, &recovery).await;
    let undo = admit_compensation(client, specialist, &recovery).await;
    assert_cross_user_undo_denied(client, primary, &undo).await;
    let undo_request = progress_undo_request(&undo);
    drop_node_response(
        client,
        specialist,
        "/v1/recovery/progress-undo",
        &undo_request,
    )
    .await;
    let replayed_undo = progress_undo(client, specialist, &undo).await;
    let redo = replayed_undo_handle(&replayed_undo).to_owned();
    assert_cross_user_redo_denied(client, primary, &redo).await;
    let redo_request = progress_redo_request(&redo);
    drop_node_response(
        client,
        specialist,
        "/v1/recovery/progress-redo",
        &redo_request,
    )
    .await;
    let replayed_redo = progress_redo(client, specialist, &redo).await;
    assert!(matches!(
        replayed_redo,
        BankUserNodeRedoProgressionOutcome::Forwarded {
            response: BankHttpRedoProgressionOutcome::Applied {
                disposition: BankHttpCommitDisposition::Committed,
                ..
            }
        }
    ));
}

fn disbursement_request() -> serde_json::Value {
    serde_json::json!({
        "request_id": "process-disbursement",
        "controls": { "deadline_milliseconds": 5_000 },
        "idempotency_key": "process-disbursement-key",
        "estate": "fixture:3",
        "source_account": "fixture:100",
        "destination_account": "fixture:102",
        "beneficiary": "fixture:3",
        "amount_minor_units": 250
    })
}

fn replayed_disbursement(outcome: &BankUserNodeEstateDisbursementOutcome) -> &str {
    match outcome {
        BankUserNodeEstateDisbursementOutcome::Forwarded {
            response:
                BankHttpEstateDisbursementOutcome::Applied {
                    disposition: BankHttpCommitDisposition::AlreadyCommitted,
                    recovery,
                    ..
                },
        } => recovery,
        other => panic!("lost process disbursement did not replay: {other:?}"),
    }
}

async fn assert_cross_user_recovery_denied(
    client: &reqwest::Client,
    primary: SocketAddr,
    recovery: &str,
) {
    let outcome = post_node::<BankUserNodeRecoveryInspectionOutcome>(
        client,
        primary,
        "/v1/recovery/inspect",
        &recovery_request("cross-user-disbursement", recovery),
    )
    .await;
    assert!(matches!(
        outcome,
        BankUserNodeRecoveryInspectionOutcome::Forwarded {
            response: BankHttpRecoveryInspectionOutcome::Denied { denial, .. }
        } if denial.kind == BankHttpDenialKind::Stale
    ));
}

async fn assert_compensatable(client: &reqwest::Client, specialist: SocketAddr, recovery: &str) {
    let outcome = post_node::<BankUserNodeRecoveryInspectionOutcome>(
        client,
        specialist,
        "/v1/recovery/inspect",
        &recovery_request("inspect-disbursement", recovery),
    )
    .await;
    assert!(matches!(
        outcome,
        BankUserNodeRecoveryInspectionOutcome::Forwarded {
            response: BankHttpRecoveryInspectionOutcome::Inspected {
                posture: BankHttpRecoveryPosture::Compensatable,
                ..
            }
        }
    ));
}

async fn admit_compensation(
    client: &reqwest::Client,
    specialist: SocketAddr,
    recovery: &str,
) -> String {
    let outcome = post_node::<BankUserNodeUndoAdmissionOutcome>(
        client,
        specialist,
        "/v1/recovery/admit-undo",
        &recovery_request("admit-disbursement-undo", recovery),
    )
    .await;
    match outcome {
        BankUserNodeUndoAdmissionOutcome::Forwarded {
            response:
                BankHttpUndoAdmissionOutcome::Admitted {
                    correction: BankHttpUndoCorrection::Compensation,
                    undo,
                    ..
                },
        } => undo,
        other => panic!("process compensation did not admit: {other:?}"),
    }
}

async fn assert_cross_user_undo_denied(client: &reqwest::Client, primary: SocketAddr, undo: &str) {
    let outcome = progress_undo(client, primary, undo).await;
    assert!(matches!(
        outcome,
        BankUserNodeUndoProgressionOutcome::Forwarded {
            response: BankHttpUndoProgressionOutcome::Denied { denial, .. }
        } if denial.kind == BankHttpDenialKind::Stale
    ));
}

async fn progress_undo(
    client: &reqwest::Client,
    specialist: SocketAddr,
    undo: &str,
) -> BankUserNodeUndoProgressionOutcome {
    post_node(
        client,
        specialist,
        "/v1/recovery/progress-undo",
        &progress_undo_request(undo),
    )
    .await
}

fn replayed_undo_handle(outcome: &BankUserNodeUndoProgressionOutcome) -> &str {
    match outcome {
        BankUserNodeUndoProgressionOutcome::Forwarded {
            response:
                BankHttpUndoProgressionOutcome::Applied {
                    disposition: BankHttpCommitDisposition::Committed,
                    redo,
                    ..
                },
        } => redo,
        other => panic!("lost process compensation did not replay: {other:?}"),
    }
}

async fn assert_cross_user_redo_denied(client: &reqwest::Client, primary: SocketAddr, redo: &str) {
    let outcome = progress_redo(client, primary, redo).await;
    assert!(matches!(
        outcome,
        BankUserNodeRedoProgressionOutcome::Forwarded {
            response: BankHttpRedoProgressionOutcome::Denied { denial, .. }
        } if denial.kind == BankHttpDenialKind::Stale
    ));
}

async fn progress_redo(
    client: &reqwest::Client,
    specialist: SocketAddr,
    redo: &str,
) -> BankUserNodeRedoProgressionOutcome {
    post_node(
        client,
        specialist,
        "/v1/recovery/progress-redo",
        &progress_redo_request(redo),
    )
    .await
}

fn progress_undo_request(undo: &str) -> serde_json::Value {
    serde_json::json!({
        "request_id": "process-progress-undo",
        "controls": { "deadline_milliseconds": 5_000 },
        "undo": undo,
        "idempotency_key": "process-disbursement-undo-key"
    })
}

fn progress_redo_request(redo: &str) -> serde_json::Value {
    serde_json::json!({
        "request_id": "process-progress-redo",
        "controls": { "deadline_milliseconds": 5_000 },
        "redo": redo
    })
}

async fn drop_node_response(
    client: &reqwest::Client,
    address: SocketAddr,
    path: &str,
    request: &serde_json::Value,
) {
    drop(
        client
            .post(format!("http://{address}{path}"))
            .json(request)
            .send()
            .await
            .expect("response should be droppable after headers"),
    );
}

fn recovery_request(request_id: &str, recovery: &str) -> serde_json::Value {
    serde_json::json!({
        "request_id": request_id,
        "controls": { "deadline_milliseconds": 5_000 },
        "recovery": recovery
    })
}
