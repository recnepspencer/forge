use serde_json::json;
use worth_server::{
    WorthServerProductResultArtifact, WorthServerProductResultArtifactErrorCode,
    WorthServerProductResultContract,
};

#[path = "support/product_result/schema_bound_json.rs"]
mod schema_bound_json;

use schema_bound_json::SchemaBoundJsonResult;

#[test]
fn canonical_json_order_and_semantic_change_have_exact_identity_behavior() {
    let contract =
        WorthServerProductResultContract::canonical_json("product.connection.result.v1", 1, 1024)
            .expect("result contract should validate");
    let left = publish(
        &contract,
        json!({ "connection": { "state": "ready", "id": "host-7" }, "version": 1 }),
    );
    let reordered = publish(
        &contract,
        json!({ "version": 1, "connection": { "id": "host-7", "state": "ready" } }),
    );
    let changed = publish(
        &contract,
        json!({ "version": 1, "connection": { "id": "host-7", "state": "failed" } }),
    );

    assert_eq!(
        left.body().canonical_bytes(),
        reordered.body().canonical_bytes()
    );
    assert_eq!(left.body_digest(), reordered.body_digest());
    assert_eq!(left.artifact_digest(), reordered.artifact_digest());
    assert_ne!(left.body_digest(), changed.body_digest());
    assert_ne!(left.artifact_digest(), changed.artifact_digest());
    assert_eq!(left.artifact_digest().len(), 64);
}

#[test]
fn typed_result_schema_must_match_the_declared_contract_before_publication() {
    let contract =
        WorthServerProductResultContract::canonical_json("product.connection.result.v1", 1, 1024)
            .expect("result contract should validate");
    let wrong_schema = SchemaBoundJsonResult::v1(
        "product.deployment.result.v1",
        json!({ "connection_id": "host-7" }),
    );
    let error = WorthServerProductResultArtifact::publish_json(&contract, &wrong_schema)
        .expect_err("typed schema mismatch must fail before artifact construction");

    assert_eq!(
        error.code(),
        WorthServerProductResultArtifactErrorCode::SchemaContractMismatch
    );
}

#[test]
fn declared_inline_budget_rejects_oversized_result_bodies() {
    let contract =
        WorthServerProductResultContract::canonical_json("product.connection.result.v1", 1, 16)
            .expect("result contract should validate");
    let result = SchemaBoundJsonResult::v1(
        "product.connection.result.v1",
        json!({ "body": "this result is intentionally too large" }),
    );
    let error = WorthServerProductResultArtifact::publish_json(&contract, &result)
        .expect_err("oversized inline body must fail closed");

    assert_eq!(
        error.code(),
        WorthServerProductResultArtifactErrorCode::InlineBudgetExceeded
    );
}

#[test]
fn canonical_contract_digest_frames_delimiter_bearing_schema_identity() {
    let left = WorthServerProductResultContract::canonical_json("schema|version=2", 1, 1024)
        .expect("delimiter-bearing identity should remain representable");
    let right = WorthServerProductResultContract::canonical_json("schema", 2, 1024)
        .expect("neighboring schema should validate");

    assert_ne!(left.canonical_digest(), right.canonical_digest());
    assert_eq!(left.canonical_digest().len(), 64);
    assert_eq!(right.canonical_digest().len(), 64);
}

fn publish(
    contract: &WorthServerProductResultContract,
    body: serde_json::Value,
) -> WorthServerProductResultArtifact {
    WorthServerProductResultArtifact::publish_json(
        contract,
        &SchemaBoundJsonResult::v1("product.connection.result.v1", body),
    )
    .expect("schema-bound result should publish")
}
