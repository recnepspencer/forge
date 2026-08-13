use std::time::Duration;

use axum::routing::post;
use axum::Router;
use tokio::net::TcpListener;

use super::send_with_deadline;
use crate::protocol::BankUserNodeDenialKind;

#[tokio::test]
async fn upstream_deadline_covers_the_network_response() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(
            listener,
            Router::new().route(
                "/slow",
                post(|| async {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    "late"
                }),
            ),
        )
        .await
        .unwrap();
    });
    let endpoint = url::Url::parse(&format!("http://{address}/slow")).unwrap();
    let denial = send_with_deadline(
        &reqwest::Client::new(),
        endpoint,
        &serde_json::json!({"request": "bounded"}),
        Duration::from_secs(1),
        25,
    )
    .await
    .expect_err("slow upstream must not outlive the node deadline");
    assert_eq!(denial, BankUserNodeDenialKind::UpstreamDeadlineExceeded);
    server.abort();
}

#[tokio::test]
async fn invalid_deadline_fails_before_network_contact() {
    let endpoint = url::Url::parse("http://127.0.0.1:1/unreachable").unwrap();
    for deadline in [0, 1_001] {
        let denial = send_with_deadline(
            &reqwest::Client::new(),
            endpoint.clone(),
            &serde_json::json!({"request": "invalid"}),
            Duration::from_secs(1),
            deadline,
        )
        .await
        .expect_err("invalid deadline must fail locally");
        assert_eq!(denial, BankUserNodeDenialKind::MalformedRequest);
    }
}
