use std::sync::Arc;

use crate::facade::identity::PartitionId;
use crate::facade::transactions::{MutationIntent, TransactionOptions, WorkerIntentBatch};
use crate::tests::support::{create_entity_outcome, field_key, single_string_aspect_field_patch};
use forge_runtime_bridge::facade::{
    BridgeRouteRequest, CommittedPatchSource, RelationalCommittedPatchRequest,
    RuntimeBridgeBuilder, SnapshotReadSource, TruthCommitIdentity,
};

use super::super::{bridge_snapshot_identity_for_commit, RuntimeBridgeRelationalSource};
use super::support::{
    exact_aspect_registration, exact_registration, register_remaining_patch_items,
    runtime_with_test_schema, TestSink,
};

#[test]
fn runtime_bridge_relational_source_exposes_latest_publication_bundle_authoritatively() {
    let mut runtime = runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "alice");

    let bundle = runtime
        .publication()
        .latest_bundle()
        .expect("runtime publication bundle")
        .clone();
    let expected_snapshot_identity =
        bridge_snapshot_identity_for_commit(bundle.commit.commit_id, bundle.commit.version_id);
    let expected_commit_identity =
        forge_runtime_bridge::facade::RelationalCommittedPatchRequest::new(
            TruthCommitIdentity::new(format!("commit-{}", bundle.commit.commit_id.0)),
        );

    let source = RuntimeBridgeRelationalSource::new(Arc::new(runtime));
    let envelope = source
        .load_committed_patch(expected_commit_identity)
        .expect("runtime bridge committed patch");
    let reader = source
        .open_snapshot(&expected_snapshot_identity)
        .expect("runtime bridge snapshot reader");

    assert_eq!(envelope.snapshot_identity(), &expected_snapshot_identity);
    assert_eq!(reader.snapshot_identity(), expected_snapshot_identity);
    assert!(envelope
        .patch_body()
        .canonical_items()
        .iter()
        .any(|item| item.aspect_key().as_str() == "name"));
}

#[test]
fn runtime_bridge_relational_source_drives_public_bridge_delivery_with_canonical_snapshot_authority(
) {
    let mut runtime = runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "alice");

    let bundle = runtime
        .publication()
        .latest_bundle()
        .expect("runtime publication bundle")
        .clone();
    let commit_identity = TruthCommitIdentity::new(format!("commit-{}", bundle.commit.commit_id.0));
    let expected_snapshot_identity =
        bridge_snapshot_identity_for_commit(bundle.commit.commit_id, bundle.commit.version_id);

    let source = RuntimeBridgeRelationalSource::new(Arc::new(runtime));
    let envelope = source
        .load_committed_patch(RelationalCommittedPatchRequest::new(
            commit_identity.clone(),
        ))
        .expect("runtime bridge committed patch");
    let first_patch_item = envelope
        .patch_body()
        .canonical_items()
        .first()
        .expect("runtime bridge envelope should contain at least one patch item");
    let mut builder = RuntimeBridgeBuilder::new()
        .with_relational_source(source.clone())
        .with_signal_sink(TestSink)
        .with_continuity_lineage_source(source.clone())
        .register_mapping(exact_registration(
            "runtime-publication-item-0",
            first_patch_item,
        ))
        .register_aspect_mapping(exact_aspect_registration(
            "runtime-publication-item-field-0",
            first_patch_item,
        ));
    register_remaining_patch_items!(builder, envelope, "runtime-publication");
    let bridge = builder
        .build()
        .expect("runtime bridge should build from runtime-backed relational source");

    let route = bridge
        .plan_committed_patch(BridgeRouteRequest::for_commit(commit_identity))
        .expect("runtime-backed relational bridge route");
    let result = bridge
        .deliver_invalidation(route)
        .expect("runtime-backed relational bridge delivery");

    assert_eq!(
        result.result_summary().snapshot_identity(),
        &expected_snapshot_identity
    );
    assert_eq!(
        result.receipt().snapshot_identity(),
        &expected_snapshot_identity
    );
}

#[test]
fn runtime_bridge_replays_historical_commit_after_newer_publication_arrives() {
    let mut runtime = runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "alice");
    let historical_commit_id = runtime
        .publication()
        .latest_bundle()
        .expect("first runtime publication bundle")
        .commit
        .commit_id;

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("update").push(MutationIntent::Create(
            crate::transactions::data::CreateIntent::Entity(
                crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: crate::facade::identity::KindId(1),
                    client_key: crate::symbols::data::ClientKey::raw("bob"),
                    fields: single_string_aspect_field_patch(
                        crate::tests::support::aspect_key("name"),
                        field_key("name"),
                        "bob",
                    ),
                },
            ),
        )),
    );
    txn.commit().expect("second commit should publish");

    let source = RuntimeBridgeRelationalSource::new(Arc::new(runtime));
    let historical_commit_identity = RelationalCommittedPatchRequest::new(
        TruthCommitIdentity::new(format!("commit-{}", historical_commit_id.0)),
    );
    let envelope = source
        .load_committed_patch(historical_commit_identity.clone())
        .expect("historical bridge committed patch");
    let expected_snapshot_identity = envelope.snapshot_identity().clone();
    let first_patch_item = envelope
        .patch_body()
        .canonical_items()
        .first()
        .expect("historical bridge envelope should contain at least one patch item");
    let mut builder = RuntimeBridgeBuilder::new()
        .with_relational_source(source.clone())
        .with_signal_sink(TestSink)
        .with_continuity_lineage_source(source.clone())
        .register_mapping(exact_registration(
            "historical-publication-item-0",
            first_patch_item,
        ))
        .register_aspect_mapping(exact_aspect_registration(
            "historical-publication-item-field-0",
            first_patch_item,
        ));
    register_remaining_patch_items!(builder, envelope, "historical-publication");
    let bridge = builder
        .build()
        .expect("runtime bridge should build from runtime-backed relational source");

    let planned = bridge
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            historical_commit_identity.commit_identity().clone(),
        ))
        .expect("historical route should still plan after newer publication");
    let result = bridge
        .deliver_invalidation(planned)
        .expect("historical route should still deliver after newer publication");
    let canonical = bridge
        .diagnostics()
        .last_canonical_route_record()
        .expect("historical route record");
    let replay = bridge
        .replay_canonical_record(&canonical)
        .expect("historical replay should remain reconstructable");

    assert_eq!(
        result.result_summary().snapshot_identity(),
        &expected_snapshot_identity
    );
    assert_eq!(
        result.receipt().snapshot_identity(),
        &expected_snapshot_identity
    );
    assert_eq!(replay.source_snapshot(), &expected_snapshot_identity);
}
