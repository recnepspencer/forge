use super::{unique_test_sqlite_path, unique_test_store_path, StoreErrorKind, WORTHStoreBuilder};

use super::raw_exact;

#[test]
fn local_file_subscription_support_digest_drift_fails_open() {
    let path = unique_test_store_path("worth-store-subscription-support-drift");
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store.publish_subscription_support(publishable).unwrap();
    }

    let raw = std::fs::read_to_string(&path).unwrap();
    let mut payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let record_sets = payload
        .get_mut("subscription_support_record_sets")
        .and_then(serde_json::Value::as_object_mut)
        .expect("subscription support record set should persist");
    let first_record = record_sets
        .values_mut()
        .next()
        .expect("one subscription support record set should persist");
    first_record["artifact_digest"] = serde_json::Value::String(String::new());
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("durable subscription-support digest drift should fail open");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn sqlite_subscription_support_linkage_gap_fails_open() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-linkage-gap");
    {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store.publish_subscription_support(publishable).unwrap();
    }

    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute("DELETE FROM subscription_support_record_sets", [])
        .unwrap();

    let error = WORTHStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("missing durable support rows should fail open");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn sqlite_subscription_support_index_projection_drift_fails_open() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-index-drift");
    {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store.publish_subscription_support(publishable).unwrap();
    }

    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute(
            "UPDATE subscription_support_record_sets SET basis_digest = 'basis:index-drift'",
            [],
        )
        .unwrap();
    drop(connection);

    let error = WORTHStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("indexed support projections must not drift from the payload");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn sqlite_subscription_support_restart_shard_projection_drift_fails_open() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-restart-shard-drift");
    {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store.publish_subscription_support(publishable).unwrap();
    }

    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute(
            "UPDATE subscription_support_record_sets SET restart_shard = 'restart:wrong'",
            [],
        )
        .unwrap();
    drop(connection);

    let error = WORTHStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("restart-shard projection drift must fail open");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn duplicate_subscription_support_publication_rejects_durable_identity_collision() {
    let path = unique_test_store_path("worth-store-subscription-support-collision");
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store.publish_subscription_support(publishable).unwrap();
    }

    let raw = std::fs::read_to_string(&path).unwrap();
    let mut payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let record_sets = payload
        .get_mut("subscription_support_record_sets")
        .and_then(serde_json::Value::as_object_mut)
        .expect("subscription support record set should persist");
    let first_record = record_sets
        .values_mut()
        .next()
        .expect("one subscription support record set should persist");
    first_record["artifact_digest"] = serde_json::Value::String("collision-digest".into());
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let admitted = reopened
        .admit_subscription_support_declaration(raw_exact())
        .unwrap();
    let publishable = reopened
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();

    let error = reopened
        .publish_subscription_support(publishable)
        .expect_err("same durable identity with different projection must reject");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
    assert_eq!(
        reopened
            .subscription_support_counters()
            .identity_collisions(),
        1
    );
}
