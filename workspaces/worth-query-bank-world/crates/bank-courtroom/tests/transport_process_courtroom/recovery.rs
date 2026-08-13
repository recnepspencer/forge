use std::net::SocketAddr;

use bank_http_adapter::{BankHttpCommitDisposition, BankHttpDenialKind};
use bank_user_node::{
    BankUserNodeEstateNotificationOutcome, BankUserNodeRecoveryInspectionOutcome,
    BankUserNodeUndoAdmissionOutcome, BankUserNodeUndoProgressionOutcome,
};

use super::post_node;

pub async fn assert_opaque_recovery_is_owned_by_the_authenticated_specialist(
    client: &reqwest::Client,
    primary: SocketAddr,
    specialist: SocketAddr,
) {
    let request = serde_json::json!({
        "request_id": "process-notify-death",
        "controls": { "deadline_milliseconds": 5_000 },
        "idempotency_key": "process-notify-death-key",
        "estate": "fixture:3",
        "notice": "fixture:12",
        "subject": "fixture:1"
    });
    drop(
        client
            .post(format!("http://{specialist}/v1/estate/notify-death"))
            .json(&request)
            .send()
            .await
            .expect("notification response should be droppable after headers"),
    );
    let notified = post_node::<BankUserNodeEstateNotificationOutcome>(
        client,
        specialist,
        "/v1/estate/notify-death",
        &request,
    )
    .await;
    let recovery = match notified {
        BankUserNodeEstateNotificationOutcome::Forwarded {
            response:
                bank_http_adapter::BankHttpEstateNotificationOutcome::Applied {
                    disposition: BankHttpCommitDisposition::AlreadyCommitted,
                    recovery,
                    ..
                },
        } => recovery,
        other => panic!("specialist notification did not commit: {other:?}"),
    };
    let crossed = inspect(client, primary, "cross-user-recovery", &recovery).await;
    assert!(matches!(
        crossed,
        BankUserNodeRecoveryInspectionOutcome::Forwarded {
            response: bank_http_adapter::BankHttpRecoveryInspectionOutcome::Denied {
                denial,
                ..
            }
        } if denial.kind == BankHttpDenialKind::Stale
    ));
    let inspected = inspect(client, specialist, "inspect-recovery", &recovery).await;
    assert!(matches!(
        inspected,
        BankUserNodeRecoveryInspectionOutcome::Forwarded {
            response: bank_http_adapter::BankHttpRecoveryInspectionOutcome::Inspected {
                posture: bank_http_adapter::BankHttpRecoveryPosture::Reconcilable,
                ..
            }
        }
    ));
    let admitted = post_node::<BankUserNodeUndoAdmissionOutcome>(
        client,
        specialist,
        "/v1/recovery/admit-undo",
        &recovery_request("admit-undo", &recovery),
    )
    .await;
    let undo = match admitted {
        BankUserNodeUndoAdmissionOutcome::Forwarded {
            response:
                bank_http_adapter::BankHttpUndoAdmissionOutcome::Admitted {
                    correction: bank_http_adapter::BankHttpUndoCorrection::Reconciliation,
                    undo,
                    ..
                },
        } => undo,
        other => panic!("notification undo did not admit: {other:?}"),
    };
    let progressed = progress_reconciliation(client, specialist, &undo).await;
    assert!(matches!(
        progressed,
        BankUserNodeUndoProgressionOutcome::Forwarded {
            response: bank_http_adapter::BankHttpUndoProgressionOutcome::Reconciled { .. }
        }
    ));
    let replayed = progress_reconciliation(client, specialist, &undo).await;
    assert_eq!(replayed, progressed);
}

async fn inspect(
    client: &reqwest::Client,
    address: SocketAddr,
    request_id: &str,
    recovery: &str,
) -> BankUserNodeRecoveryInspectionOutcome {
    post_node(
        client,
        address,
        "/v1/recovery/inspect",
        &recovery_request(request_id, recovery),
    )
    .await
}

fn recovery_request(request_id: &str, recovery: &str) -> serde_json::Value {
    serde_json::json!({
        "request_id": request_id,
        "controls": { "deadline_milliseconds": 5_000 },
        "recovery": recovery
    })
}

async fn progress_reconciliation(
    client: &reqwest::Client,
    address: SocketAddr,
    undo: &str,
) -> BankUserNodeUndoProgressionOutcome {
    post_node(
        client,
        address,
        "/v1/recovery/progress-undo",
        &serde_json::json!({
            "request_id": "process-progress-reconciliation",
            "controls": { "deadline_milliseconds": 5_000 },
            "undo": undo,
            "idempotency_key": "unused-process-reconciliation-key"
        }),
    )
    .await
}
