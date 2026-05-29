use std::sync::Arc;

use forge_foundational::facade::AspectValue;

use crate::config::data::CascadeDeletePolicy;
use crate::facade::identity::PartitionId;
use crate::facade::transactions::{
    EntityMutationIntent, MutationIntent, ReplaceEntityIntent, TransactionOptions,
    WorkerIntentBatch,
};
use crate::tests::support::{
    changed_entities, create_entity_outcome, runtime_with_declared_aspect_schema,
};
use forge_runtime_bridge::facade::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeDeliveryReceipt, BridgeMappingId,
    BridgeMappingRegistration, BridgeRouteRequest, CoarseRoutingMode, InvalidationSink,
    MappingSelector, RelationalCommittedPatchRequest, RuntimeBridgeBuilder, SignalBridgeSinkError,
    SignalInvalidationScope, SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind,
    TruthPatchScope,
};
use forge_runtime_bridge::facade::{
    CommittedPatchSource, ContinuityLineageSource, SnapshotReadSource,
};

use super::{
    bridge_snapshot_identity_for_commit, bridge_snapshot_identity_for_handle,
    RuntimeBridgeRelationalSource,
};

struct TestSink;

impl InvalidationSink for TestSink {
    fn deliver_invalidation(
        &self,
        delivery: forge_runtime_bridge::facade::BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

fn exact_registration(
    mapping_id: &str,
    entity_identity: &str,
    aspect_label: &str,
    surface_label: &str,
) -> BridgeMappingRegistration {
    BridgeMappingRegistration::new(
        BridgeMappingId::new(mapping_id),
        TruthPatchScope::new(
            MappingSelector::exact(entity_identity),
            MappingSelector::exact(aspect_label),
            MappingSelector::exact(surface_label),
        ),
        SignalInvalidationScope::new("signal.user.profile"),
        CoarseRoutingMode::Direct,
    )
}

fn exact_aspect_registration(
    registration_id: &str,
    entity_identity: &str,
    aspect_label: &str,
    surface_label: &str,
) -> BridgeAspectRegistration {
    BridgeAspectRegistration::new(
        BridgeAspectRegistrationId::new(registration_id),
        TruthPatchScope::new(
            MappingSelector::exact(entity_identity),
            MappingSelector::exact(aspect_label),
            MappingSelector::exact(surface_label),
        ),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceFallbackPolicy::Disallow,
    )
}

#[test]
fn runtime_bridge_lineage_source_resolves_real_relational_history() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let entity = changed_entities(&created)[0];

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("replace").push(MutationIntent::Entity(
            EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: entity,
                replacement: crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: crate::facade::identity::KindId(1),
                    client_key: crate::symbols::data::ClientKey::raw("replacement"),
                    fields: crate::tests::support::aspect_field_patch_from_compatibility_json(
                        serde_json::json!({"name":"replacement"}),
                    ),
                },
            }),
        )),
    );
    txn.commit().expect("replace should commit");
    let latest_bundle = runtime
        .publication()
        .latest_bundle()
        .expect("runtime publication bundle")
        .clone();
    let expected_snapshot_identity = bridge_snapshot_identity_for_commit(
        latest_bundle.commit.commit_id,
        latest_bundle.commit.version_id,
    );
    let expected_successor_record_keys = runtime
        .read_truth()
        .project_version(latest_bundle.commit.version_id)
        .all_entity_records()
        .into_iter()
        .filter_map(|record| {
            record.lineage_id.map(|_| {
                Arc::<str>::from(super::identities::record_ref_identity(
                    &crate::transactions::data::RecordRef::Entity(record.entity_id),
                ))
            })
        })
        .collect::<Vec<_>>();

    let runtime = Arc::new(runtime);
    let source = RuntimeBridgeRelationalSource::new(Arc::clone(&runtime));
    let request = forge_runtime_bridge::facade::BridgeHistoricalLineageRequest::new(
        forge_runtime_bridge::facade::BridgeContinuityAuthorityBasis::new(
            forge_runtime_bridge::facade::TruthBranchIdentity::new("main"),
            expected_snapshot_identity.clone(),
        ),
        forge_runtime_bridge::facade::PriorSubscriptionSlice::from_parts(
            forge_runtime_bridge::facade::BridgeSubscriptionSliceIdentity::new("slice:a"),
            format!(
                "entity:{}:{}:{}",
                entity.partition_id.0, entity.local_slot.0, entity.generation.0
            ),
            "profile.name",
            "name",
            forge_runtime_bridge::facade::SubscriptionSliceKind::SignalField,
            forge_runtime_bridge::facade::FineGrainedMatchStatus::Matched,
        ),
    );
    let authority = source
        .historical_lineage(request)
        .expect("runtime lineage source should resolve");

    assert_eq!(authority.branch_identity().as_str(), "main");
    assert_eq!(authority.snapshot_identity(), &expected_snapshot_identity);
    assert_eq!(authority.canonical_resolved_lineage_keys().len(), 1);
    assert_eq!(
        authority.canonical_resolved_record_keys(),
        expected_successor_record_keys.as_slice()
    );
    assert_eq!(authority.traversed_event_ids().len(), 1);
}

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
        forge_runtime_bridge::facade::RelationalCommittedPatchRequest::new(format!(
            "commit-{}",
            bundle.commit.commit_id.0
        ));

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
        .patch_items()
        .iter()
        .any(|item| item.aspect_label() == "name"));
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
    let commit_identity = format!("commit-{}", bundle.commit.commit_id.0);
    let expected_snapshot_identity =
        bridge_snapshot_identity_for_commit(bundle.commit.commit_id, bundle.commit.version_id);

    let source = RuntimeBridgeRelationalSource::new(Arc::new(runtime));
    let envelope = source
        .load_committed_patch(RelationalCommittedPatchRequest::new(
            commit_identity.clone(),
        ))
        .expect("runtime bridge committed patch");
    let first_patch_item = envelope
        .patch_items()
        .first()
        .expect("runtime bridge envelope should contain at least one patch item");
    let mut builder = RuntimeBridgeBuilder::new()
        .with_relational_source(source.clone())
        .with_signal_sink(TestSink)
        .with_continuity_lineage_source(source.clone())
        .register_mapping(exact_registration(
            "runtime-publication-item-0",
            first_patch_item.entity_identity(),
            first_patch_item.aspect_label(),
            first_patch_item.surface_label(),
        ))
        .register_aspect_mapping(exact_aspect_registration(
            "runtime-publication-item-field-0",
            first_patch_item.entity_identity(),
            first_patch_item.aspect_label(),
            first_patch_item.surface_label(),
        ));
    for (index, patch_item) in envelope.patch_items().iter().enumerate().skip(1) {
        builder = builder
            .register_mapping(exact_registration(
                &format!("runtime-publication-item-{index}"),
                patch_item.entity_identity(),
                patch_item.aspect_label(),
                patch_item.surface_label(),
            ))
            .register_aspect_mapping(exact_aspect_registration(
                &format!("runtime-publication-item-field-{index}"),
                patch_item.entity_identity(),
                patch_item.aspect_label(),
                patch_item.surface_label(),
            ));
    }
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
                    fields: crate::tests::support::aspect_field_patch_from_compatibility_json(
                        serde_json::json!({"name":"bob"}),
                    ),
                },
            ),
        )),
    );
    txn.commit().expect("second commit should publish");

    let source = RuntimeBridgeRelationalSource::new(Arc::new(runtime));
    let historical_commit_identity =
        RelationalCommittedPatchRequest::new(format!("commit-{}", historical_commit_id.0));
    let envelope = source
        .load_committed_patch(historical_commit_identity.clone())
        .expect("historical bridge committed patch");
    let expected_snapshot_identity = envelope.snapshot_identity().clone();
    let first_patch_item = envelope
        .patch_items()
        .first()
        .expect("historical bridge envelope should contain at least one patch item");
    let mut builder = RuntimeBridgeBuilder::new()
        .with_relational_source(source.clone())
        .with_signal_sink(TestSink)
        .with_continuity_lineage_source(source.clone())
        .register_mapping(exact_registration(
            "historical-publication-item-0",
            first_patch_item.entity_identity(),
            first_patch_item.aspect_label(),
            first_patch_item.surface_label(),
        ))
        .register_aspect_mapping(exact_aspect_registration(
            "historical-publication-item-field-0",
            first_patch_item.entity_identity(),
            first_patch_item.aspect_label(),
            first_patch_item.surface_label(),
        ));
    for (index, patch_item) in envelope.patch_items().iter().enumerate().skip(1) {
        builder = builder
            .register_mapping(exact_registration(
                &format!("historical-publication-item-{index}"),
                patch_item.entity_identity(),
                patch_item.aspect_label(),
                patch_item.surface_label(),
            ))
            .register_aspect_mapping(exact_aspect_registration(
                &format!("historical-publication-item-field-{index}"),
                patch_item.entity_identity(),
                patch_item.aspect_label(),
                patch_item.surface_label(),
            ));
    }
    let bridge = builder
        .build()
        .expect("runtime bridge should build from runtime-backed relational source");

    let planned = bridge
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            historical_commit_identity.commit_identity().to_string(),
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

#[test]
fn runtime_bridge_snapshot_reader_prefers_active_snapshot_binding_over_later_commit_id_collision() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "alice");
    let active_snapshot = runtime.visibility_authority().snapshot();
    let active_snapshot_identity = bridge_snapshot_identity_for_handle(&active_snapshot);
    let active_entity_identity = format!(
        "entity:{}:{}:{}",
        changed_entities(&created)[0].partition_id.0,
        changed_entities(&created)[0].local_slot.0,
        changed_entities(&created)[0].generation.0
    );

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("update").push(MutationIntent::Entity(
            EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: changed_entities(&created)[0],
                replacement: crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: crate::facade::identity::KindId(1),
                    client_key: crate::symbols::data::ClientKey::raw("alice"),
                    fields: crate::tests::support::aspect_field_patch_from_compatibility_json(
                        serde_json::json!({"name":"alice-updated"}),
                    ),
                },
            }),
        )),
    );
    txn.commit().expect("second commit should publish");

    let source = RuntimeBridgeRelationalSource::new(Arc::new(runtime));
    let reader = source
        .open_snapshot(&active_snapshot_identity)
        .expect("active snapshot should remain bridge-readable after later commit id collision");
    let packet = forge_runtime_bridge::facade::SnapshotReadPacket::new(vec![
        forge_runtime_bridge::facade::SnapshotReadRequest::for_coarse(
            active_entity_identity,
            "name",
        ),
    ]);
    let result = reader
        .read_packet(&packet)
        .expect("bridge snapshot packet should read from active binding");

    assert_eq!(result.snapshot_identity(), &active_snapshot_identity);
    assert_eq!(result.records().len(), 1);
    assert_eq!(
        decode_snapshot_aspect_bytes(result.records()[0].aspect_bytes()),
        AspectValue::String("alice".into())
    );
}

fn runtime_with_test_schema() -> crate::facade::runtime::RelationalRuntime {
    runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations)
}

fn decode_snapshot_aspect_bytes(aspect_bytes: &[u8]) -> AspectValue {
    crate::aspect_wire::decode_aspect_value(aspect_bytes).expect("snapshot aspect bytes")
}
