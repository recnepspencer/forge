use crate::facade::identity::KindId;
use crate::facade::inspection::{InspectionOrigin, InspectionScope};
use crate::facade::transactions::{
    EntityReference, MutationIntent, RecordRef, RelationMutationIntent, TransactionOptions,
    UpdateRelationEndpointsIntent, WorkerIntentBatch,
};
use crate::tests::support::*;

#[test]
fn structural_identity_origin_tracks_non_current_scope() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "scoped-identity");
    let version_id = runtime.current_version_id();
    let snapshot = runtime.visibility_authority().snapshot();

    let current = runtime
        .inspect_what_happened()
        .structural_identity(InspectionScope::Current, RecordRef::Entity(entity))
        .expect("current structural identity");
    let version = runtime
        .inspect_what_happened()
        .structural_identity(
            InspectionScope::Version(version_id),
            RecordRef::Entity(entity),
        )
        .expect("version structural identity");
    let snapshot = runtime
        .inspect_what_happened()
        .structural_identity(
            InspectionScope::Snapshot(snapshot),
            RecordRef::Entity(entity),
        )
        .expect("snapshot structural identity");

    assert_eq!(current.origin, InspectionOrigin::CurrentTruth);
    assert_eq!(version.origin, InspectionOrigin::VisibilitySnapshot);
    assert_eq!(snapshot.origin, InspectionOrigin::VisibilitySnapshot);
}

#[test]
fn historical_neighbors_follow_scoped_relation_endpoints_after_rewire() {
    let mut runtime = runtime_with_test_schema();
    let original_source = create_entity(&mut runtime, "original-source");
    let shared_target = create_entity(&mut runtime, "shared-target");
    let replacement_source = create_entity(&mut runtime, "replacement-source");
    let relation = create_relation(&mut runtime, original_source, shared_target, "edge");
    let historical_version = runtime.current_version_id();
    let historical_snapshot = runtime.visibility_authority().snapshot();

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("rewire-edge").push(MutationIntent::Relation(
            RelationMutationIntent::UpdateEndpoints(UpdateRelationEndpointsIntent {
                relation_id: relation,
                kind_id: KindId(2),
                source: EntityReference::Existing(replacement_source),
                target: EntityReference::Existing(shared_target),
            }),
        )),
    );
    txn.commit().expect("relation rewire should commit");

    let current_neighbors = runtime
        .inspect_what_happened()
        .neighbors(InspectionScope::Current, original_source);
    let historical_neighbors = runtime.inspect_what_happened().neighbors(
        InspectionScope::Version(historical_version),
        original_source,
    );
    let snapshot_neighbors = runtime.inspect_what_happened().neighbors(
        InspectionScope::Snapshot(historical_snapshot),
        original_source,
    );

    assert!(current_neighbors.outgoing_relation_ids.is_empty());
    assert_eq!(historical_neighbors.outgoing_relation_ids, vec![relation]);
    assert_eq!(snapshot_neighbors.outgoing_relation_ids, vec![relation]);
}
