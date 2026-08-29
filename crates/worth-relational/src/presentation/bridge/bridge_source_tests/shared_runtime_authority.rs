use std::sync::{Arc, Mutex};

use crate::facade::identity::PartitionId;
use crate::facade::transactions::{CreateIntent, EntitySpec, MutationIntent, WorkerIntentBatch};
use crate::tests::support::{aspect_key, field_key, single_string_aspect_field_patch};
use worth_foundational::ScalarAspectType;
use worth_runtime_bridge::facade::{
    CommittedPatchSource, RelationalBridgeRecordIdentityParts, RelationalCommittedPatchRequest,
    SnapshotReadContract, SnapshotReadPacket, SnapshotReadRequest, SnapshotReadSource,
    TruthCommitIdentity,
};

use super::super::RuntimeBridgeRelationalSource;
use super::support::runtime_with_test_schema;

#[test]
fn shared_source_retains_the_live_runtime_authority_and_observes_later_commits() {
    let runtime = Arc::new(Mutex::new(runtime_with_test_schema()));
    let source =
        RuntimeBridgeRelationalSource::for_shared_graph_role(Arc::clone(&runtime), "model")
            .expect("shared source should accept the installed graph role");
    let expected_runtime_id = runtime
        .lock()
        .expect("test runtime lock")
        .runtime_instance_id();

    assert_eq!(
        source.authoritative_source_profile().runtime_instance_id(),
        expected_runtime_id
    );

    let committed = {
        let runtime = runtime.lock().expect("test runtime lock");
        let mut transaction =
            crate::tests::support::test_owner_begin_transaction_for_main(&runtime);
        transaction
            .push_batch(WorkerIntentBatch::new("shared-authority-create").push(
                MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: crate::facade::identity::KindId(1),
                    client_key: crate::facade::symbols::ClientKey::raw("alice"),
                    fields: single_string_aspect_field_patch(
                        aspect_key("name"),
                        field_key("name"),
                        "alice",
                    ),
                })),
            ))
            .expect("test staging stays within configured resource budgets");
        transaction
            .commit(&runtime)
            .expect("real shared-runtime commit")
    };
    let entity = committed
        .changed_records
        .iter()
        .find_map(|record| match record {
            crate::facade::transactions::RecordRef::Entity(entity) => Some(entity),
            crate::facade::transactions::RecordRef::Relation(_) => None,
        })
        .expect("created entity");
    let branch_identity = runtime
        .lock()
        .expect("test runtime lock")
        .branch_identity(&committed.commit.branch_id)
        .expect("committed branch identity");
    let (_, basis) = source
        .observe_branch_basis(&branch_identity)
        .expect("source must observe the live owner basis");
    let _lease = source
        .retain_branch_basis_for_bridge(&basis)
        .expect("source must retain the live owner observation");
    let envelope = source
        .load_committed_patch(RelationalCommittedPatchRequest::new(
            TruthCommitIdentity::from_relational_commit_id(committed.commit.commit_id.0),
        ))
        .expect("source must observe commits made after its construction");
    let reader = source
        .open_snapshot(envelope.snapshot_identity())
        .expect("source must open the live runtime snapshot");
    let packet = SnapshotReadPacket::new(vec![SnapshotReadRequest::for_relational_record(
        RelationalBridgeRecordIdentityParts::entity(
            entity.partition_id.0,
            entity.local_slot.0,
            entity.generation.0,
        ),
        SnapshotReadContract::scalar(aspect_key("name"), ScalarAspectType::String),
    )]);
    let result = reader
        .read_packet(&packet)
        .expect("snapshot must be read through the same shared runtime");

    assert_eq!(result.records().len(), 1);
    assert_eq!(
        result.records()[0].scalar_aspect_value(),
        Some(&worth_foundational::facade::AspectValue::String(
            "alice".into()
        ))
    );
}
