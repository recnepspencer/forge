use std::sync::{Arc, Mutex};

use crate::facade::identity::PartitionId;
use crate::facade::transactions::{
    EntityMutationIntent, MutationIntent, ReplaceEntityIntent, WorkerIntentBatch,
};
use crate::history::data::BranchId;
use crate::tests::support::{
    changed_entities, create_entity_outcome, field_key, single_string_aspect_field_patch,
};
use worth_runtime_bridge::facade::{
    BridgeContinuityAuthorityBasis, BridgeHistoricalLineageAuthority,
    BridgeHistoricalLineagePacket, BridgeLineageContext, BridgeMappingContext, BridgeRouteRequest,
    CommittedPatchSource, RelationalCommittedPatchRequest, RuntimeBridge, TruthBranchIdentity,
    TruthCommitIdentity, TruthSnapshotIdentity,
};

use super::super::RuntimeBridgeRelationalSource;
use super::support::{runtime_bridge_for_envelope, runtime_with_test_schema};

#[test]
fn retained_fork_observation_includes_ancestors_and_excludes_later_authoring_branch_events() {
    assert_retained_fork_excludes_later_event(BranchId("main".to_owned()));
}

fn assert_retained_fork_excludes_later_event(later_branch: BranchId) {
    let fixture = retained_fork_fixture();
    let before = fixture.lineage_authority();
    assert_eq!(before.traversed_event_ids().len(), 1);

    replace_entity_on_branch(
        &fixture.runtime.lock().unwrap(),
        fixture.successor,
        &format!("later-{}-replacement", later_branch.0),
        later_branch,
    );

    assert_eq!(fixture.lineage_authority(), before);
}

struct RetainedForkFixture {
    runtime: Arc<Mutex<crate::runtime::RelationalRuntime>>,
    bridge: RuntimeBridge,
    commit: TruthCommitIdentity,
    snapshot: TruthSnapshotIdentity,
    successor: crate::identity::data::EntityId,
    _lease: super::super::RelationalBridgeObservationLease,
}

impl RetainedForkFixture {
    fn lineage_authority(&self) -> BridgeHistoricalLineageAuthority {
        let packet = plan_lineage_packet(
            &self.bridge,
            self.commit.clone(),
            TruthBranchIdentity::from_relational_branch_id("feature"),
            self.snapshot.clone(),
        )
        .expect("retained fork lineage must resolve");
        packet
            .entries()
            .iter()
            .map(|entry| entry.lineage_authority())
            .find(|authority| !authority.traversed_event_ids().is_empty())
            .expect("fork lineage packet entry with inherited replacement history")
            .clone()
    }
}

fn retained_fork_fixture() -> RetainedForkFixture {
    let runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&runtime, "fork-source");
    let entity = changed_entities(&created)[0];
    replace_entity_on_branch(
        &runtime,
        entity,
        "fork-inherited-replacement",
        BranchId("main".to_owned()),
    );
    let inherited = runtime
        .publication()
        .latest_bundle()
        .expect("inherited replacement publication")
        .clone();
    let successor = runtime
        .read_truth()
        .project_historical_version(inherited.commit.version_id)
        .all_authoritative_entity_records()
        .into_iter()
        .find(|record| record.entity_id != entity && record.lineage_id.is_some())
        .expect("inherited replacement successor")
        .entity_id;
    let feature = BranchId("feature".to_owned());
    runtime
        .history_authority()
        .fork_branch_from(feature.clone(), &BranchId("main".to_owned()))
        .expect("feature fork from inherited main root");
    let feature_identity = runtime
        .branch_identity(&feature)
        .expect("feature branch identity");
    let runtime = Arc::new(Mutex::new(runtime));
    let source =
        RuntimeBridgeRelationalSource::for_shared_graph_role(Arc::clone(&runtime), "model")
            .expect("shared fork lineage graph role");
    let (_, basis) = source
        .observe_branch_basis(&feature_identity)
        .expect("owner-admitted feature basis");
    let lease = source
        .retain_branch_basis_for_bridge(&basis)
        .expect("retained feature observation");
    let snapshot = lease.snapshot_identity().clone();
    let commit = TruthCommitIdentity::from_relational_commit_id(inherited.commit.commit_id.0);
    let envelope = source
        .load_committed_patch(RelationalCommittedPatchRequest::at_snapshot(
            commit.clone(),
            snapshot.clone(),
        ))
        .expect("feature observation must authorize its inherited commit");
    assert_eq!(
        envelope.branch_identity().relational_branch_id(),
        Some("feature")
    );
    let bridge = runtime_bridge_for_envelope(source, &envelope);
    RetainedForkFixture {
        runtime,
        bridge,
        commit,
        snapshot,
        successor,
        _lease: lease,
    }
}

pub(super) fn replace_entity_on_branch(
    runtime: &crate::runtime::RelationalRuntime,
    entity: crate::identity::data::EntityId,
    replacement: &str,
    branch: BranchId,
) -> crate::identity::data::EntityId {
    let mut txn = crate::tests::support::test_owner_begin_transaction_for_branch(runtime, branch);
    txn.push_batch(
        WorkerIntentBatch::new(replacement).push(MutationIntent::Entity(
            EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: entity,
                replacement: crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: crate::facade::identity::KindId(1),
                    client_key: crate::symbols::data::ClientKey::raw(replacement),
                    fields: single_string_aspect_field_patch(
                        crate::tests::support::aspect_key("name"),
                        field_key("name"),
                        replacement,
                    ),
                },
            }),
        )),
    )
    .expect("test staging stays within configured resource budgets");
    let outcome = txn.commit(runtime).expect("replacement should commit");
    changed_entities(&outcome)[0]
}

pub(super) fn plan_lineage_packet(
    bridge: &RuntimeBridge,
    commit: TruthCommitIdentity,
    branch: TruthBranchIdentity,
    snapshot: TruthSnapshotIdentity,
) -> Result<BridgeHistoricalLineagePacket, String> {
    let context = BridgeMappingContext::default().with_lineage_context(BridgeLineageContext::new(
        BridgeContinuityAuthorityBasis::new(branch, snapshot),
    ));
    let route = bridge
        .plan_committed_patch_with_mapping_context(BridgeRouteRequest::for_commit(commit), context)
        .map_err(|error| error.to_string())?;
    let requests = bridge
        .plan_continuity_requests_from_planned_route(&route)
        .map_err(|error| error.to_string())?;
    bridge
        .plan_historical_lineage_packet(&requests)
        .map_err(|error| error.to_string())
}
