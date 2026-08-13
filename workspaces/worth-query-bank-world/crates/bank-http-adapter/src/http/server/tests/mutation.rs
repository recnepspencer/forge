use std::sync::Arc;

use bank_domain::model::{AccountId, InstitutionId};

use super::super::super::protocol::{
    BankHttpAccountSummaryOutcome, BankHttpCommitDisposition, BankHttpDenialKind,
    BankHttpMutationOutcome, BankHttpMutationRequest,
};
use super::fixture::application;
use super::{bind_application, controls_json, credential_json, BankHttpServerConfiguration};

#[tokio::test]
async fn malformed_json_returns_a_typed_denial() {
    let server = bind_application(
        Arc::new(application(AccountId::new(100).unwrap())),
        BankHttpServerConfiguration::local_ephemeral(),
    )
    .await
    .expect("HTTP server should bind");
    let outcome = post_mutation(
        &reqwest::Client::new(),
        &format!("http://{}/v1/mutations", server.local_address()),
        &serde_json::json!({ "protocol": "v1", "request_id": 42 }),
    )
    .await;
    assert!(matches!(
        outcome,
        BankHttpMutationOutcome::NotApplied { denial, .. }
            if denial.kind == BankHttpDenialKind::MalformedRequest
    ));
    server.shutdown().await.expect("server should shut down");
}

#[tokio::test]
async fn response_loss_reuses_domain_idempotency_without_duplicate_effect() {
    let account = AccountId::new(100).unwrap();
    let institution = InstitutionId::new(1).unwrap();
    let server = bind_application(
        Arc::new(application(account)),
        BankHttpServerConfiguration::local_ephemeral(),
    )
    .await
    .expect("HTTP server should bind");
    let client = reqwest::Client::new();
    let endpoint = format!("http://{}/v1/mutations", server.local_address());
    let request = deposit_request(institution, account);
    serde_json::from_value::<BankHttpMutationRequest>(request.clone())
        .expect("mutation specimen must match the wire contract");
    let committed = post_mutation(&client, &endpoint, &request).await;
    assert_disposition(&committed, BankHttpCommitDisposition::Committed);
    let replayed = post_mutation(&client, &endpoint, &request).await;
    assert_disposition(&replayed, BankHttpCommitDisposition::AlreadyCommitted);
    assert_summary_balance(&client, server.local_address(), account, 310).await;
    server.shutdown().await.expect("server should shut down");
}

fn deposit_request(institution: InstitutionId, account: AccountId) -> serde_json::Value {
    serde_json::json!({
        "protocol": "v1",
        "request_id": "deposit-response-loss",
        "credential": credential_json(),
        "controls": { "deadline_milliseconds": 5_000 },
        "idempotency_key": "deposit-response-loss-key",
        "operation": "deposit",
        "institution": institution.canonical_text(),
        "account": account.canonical_text(),
        "amount_minor_units": 10
    })
}

fn assert_disposition(outcome: &BankHttpMutationOutcome, expected: BankHttpCommitDisposition) {
    assert!(
        matches!(
            outcome,
            BankHttpMutationOutcome::Applied { disposition, .. } if *disposition == expected
        ),
        "unexpected mutation outcome: {outcome:?}"
    );
}

async fn assert_summary_balance(
    client: &reqwest::Client,
    address: std::net::SocketAddr,
    account: AccountId,
    expected: i64,
) {
    let summary = client
        .post(format!("http://{address}/v1/queries/account-summary"))
        .json(&serde_json::json!({
            "protocol": "v1",
            "request_id": "summary-after-deposit",
            "credential": credential_json(),
            "controls": controls_json(1),
            "account": account.canonical_text()
        }))
        .send()
        .await
        .unwrap()
        .json::<BankHttpAccountSummaryOutcome>()
        .await
        .unwrap();
    assert!(matches!(
        summary,
        BankHttpAccountSummaryOutcome::Delivered { summary, .. }
            if summary.current_balance_minor == expected
    ));
}

async fn post_mutation(
    client: &reqwest::Client,
    endpoint: &str,
    request: &serde_json::Value,
) -> BankHttpMutationOutcome {
    client
        .post(endpoint)
        .json(request)
        .send()
        .await
        .expect("mutation request should complete")
        .json()
        .await
        .expect("mutation response should be typed")
}
