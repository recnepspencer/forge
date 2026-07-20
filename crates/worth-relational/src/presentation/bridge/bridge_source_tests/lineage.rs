use std::sync::Arc;

use crate::facade::identity::PartitionId;
use crate::facade::transactions::{
    EntityMutationIntent, MutationIntent, ReplaceEntityIntent, TransactionOptions,
    WorkerIntentBatch,
};
use crate::tests::support::{
    changed_entities, create_entity_outcome, field_key, single_string_aspect_field_patch,
};
use worth_runtime_bridge::facade::{
    BridgeContinuityAuthorityBasis, BridgeHistoricalResolvedRecordIdentity, BridgeLineageContext,
    BridgeMappingContext, BridgeRouteRequest, CommittedPatchSource,
    RelationalCommittedPatchRequest, TruthBranchIdentity, TruthCommitIdentity,
};

use super::super::{bridge_snapshot_identity_for_commit, RuntimeBridgeRelationalSource};
use super::support::{runtime_bridge_for_envelope, runtime_with_test_schema};

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
                    fields: single_string_aspect_field_patch(
                        crate::tests::support::aspect_key("name"),
                        field_key("name"),
                        "replacement",
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
    let expected_successor_record_identities = runtime
        .read_truth()
        .project_version(latest_bundle.commit.version_id)
        .all_authoritative_entity_records()
        .into_iter()
        .filter_map(|record| {
            record.lineage_id.map(|_| {
                BridgeHistoricalResolvedRecordIdentity::from_relational_record(
                    super::super::identities::record_ref_identity(
                        &crate::transactions::data::RecordRef::Entity(record.entity_id),
                    ),
                )
            })
        })
        .collect::<Vec<_>>();

    let runtime = Arc::new(runtime);
    let source = RuntimeBridgeRelationalSource::for_graph_role(Arc::clone(&runtime), "model")
        .expect("test graph role");
    let latest_commit_identity = RelationalCommittedPatchRequest::new(
        TruthCommitIdentity::from_relational_commit_id(latest_bundle.commit.commit_id.0),
    );
    let envelope = source
        .load_committed_patch(latest_commit_identity.clone())
        .expect("runtime bridge committed patch");
    let bridge = runtime_bridge_for_envelope(source.clone(), &envelope);
    let lineage_context = BridgeMappingContext::default().with_lineage_context(
        BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
            TruthBranchIdentity::from_relational_branch_id("main"),
            expected_snapshot_identity.clone(),
        )),
    );
    let planned_route = bridge
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit(latest_commit_identity.commit_identity().clone()),
            lineage_context,
        )
        .expect("lineage-context route should plan");
    let continuity_requests = bridge
        .plan_continuity_requests_from_planned_route(&planned_route)
        .expect("continuity requests should derive native prior slices");
    let lineage_packet = bridge
        .plan_historical_lineage_packet(&continuity_requests)
        .expect("historical lineage packet should resolve");
    let authority = lineage_packet
        .entries()
        .iter()
        .map(|entry| entry.lineage_authority())
        .find(|authority| !authority.traversed_event_ids().is_empty())
        .expect("lineage packet entry with traversed replacement history");

    assert_eq!(
        authority.branch_identity().relational_branch_id(),
        Some("main")
    );
    assert_eq!(authority.snapshot_identity(), &expected_snapshot_identity);
    assert_eq!(authority.canonical_resolved_lineage_identities().len(), 1);
    assert_eq!(
        authority.canonical_resolved_record_identities(),
        expected_successor_record_identities.as_slice()
    );
    assert_eq!(authority.traversed_event_ids().len(), 1);
}
