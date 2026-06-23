use forge_query::facade::{
    ForgeQueryEntityIdentity, ForgeQueryExistingEntityTarget, ForgeQueryExistingRelationTarget,
    ForgeQueryExistingTruthBindingAuthorityLabel, ForgeQueryMutationAuthorityIdentity,
};
use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityParts;

fn main() {
    let entity_id = EntityId::new(PartitionId(1), 1, 1);
    let relation_id = RelationId::new(PartitionId(1), 2, 1);
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
