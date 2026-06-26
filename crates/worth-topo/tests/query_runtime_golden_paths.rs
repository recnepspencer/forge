use std::sync::Arc;

use forge_query::facade::{
    admit_external_commit_token, ForgeQueryCommitIdentity, ForgeQueryEntityIdentity,
    ForgeQueryExistingEntityTarget, ForgeQueryExistingRelationTarget,
    ForgeQueryExistingTruthBindingAuthorityLabel, ForgeQueryMutationAuthorityIdentity,
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
    ForgeQueryRuntimeSnapshotIdentityAdapter, ForgeQuerySnapshotIdentity,
    QueryExternalIdentityToken,
};
use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use forge_runtime_bridge::facade::{
    RelationalBridgeRecordIdentityParts, RelationalBridgeSnapshotIdentityParts,
};

#[test]
fn phase_eight_golden_path_admits_derived_surface_commit_identity() {
    let _commit = admit_external_commit_token(QueryExternalIdentityToken::new(Arc::from(
        "derived-surface:test",
    )));
}

#[test]
fn phase_eight_golden_path_supports_snapshot_identity_adapter() {
    struct CurrentSnapshotIdentity;

    impl ForgeQueryRuntimeSnapshotIdentityAdapter for CurrentSnapshotIdentity {
        fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
            ForgeQuerySnapshotIdentity::empty_relational_state()
        }
    }

    let adapter = CurrentSnapshotIdentity;
    assert_eq!(
        adapter.current_snapshot_identity(),
        ForgeQuerySnapshotIdentity::empty_relational_state()
    );
}

#[test]
fn phase_eight_golden_path_admits_existing_truth_authority_targets() {
    let _entity_id = EntityId::new(PartitionId(1), 1, 1);
    let _relation_id = RelationId::new(PartitionId(1), 2, 1);
    let entity_identity = ForgeQueryEntityIdentity::from_relational_record(
        RelationalBridgeRecordIdentityParts::entity(1, 1, 1),
    );
    let relation_identity = ForgeQueryEntityIdentity::from_relational_record(
        RelationalBridgeRecordIdentityParts::relation(1, 2, 1),
    );
    let entity_authority = ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
        ForgeQueryExistingTruthBindingAuthorityLabel::new("entity:1:1:1".to_string())
            .expect("label"),
    )
    .expect("authority");
    let relation_authority = ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
        ForgeQueryExistingTruthBindingAuthorityLabel::new("relation:1:2:1".to_string())
            .expect("label"),
    )
    .expect("authority");
    let _entity_target =
        ForgeQueryExistingEntityTarget::new(entity_authority, entity_identity).expect("target");
    let _relation_target =
        ForgeQueryExistingRelationTarget::new(relation_authority, relation_identity)
            .expect("target");
}

#[test]
fn phase_nine_golden_path_admits_typed_mutation_receipt() {
    let _receipt = ForgeQueryMutationReceipt::from_authoritative_parts(
        ForgeQueryCommitIdentity::from_relational_commit_id(1),
        ForgeQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(1, 1),
        ),
        vec![ForgeQueryMutationDelta::from_touched_aspects(
            "TopologyEntity",
            ForgeQueryEntityIdentity::from_relational_record(
                RelationalBridgeRecordIdentityParts::entity(1, 1, 1),
            ),
            ForgeQueryMutationKind::Updated,
            vec![],
        )],
    );
}
