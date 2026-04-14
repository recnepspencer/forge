use crate::identity::data::{EntityId, RelationId};
use crate::transactions::data::{
    CreateIntent, EntityMutationIntent, EntityReference, MutationIntent, RelationMutationIntent,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CanonicalIntentKey {
    CreateEntity {
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        client_key: crate::symbols::data::InternedString,
    },
    BulkCreateEntities {
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        client_keys: Vec<crate::symbols::data::InternedString>,
    },
    UpdateEntity(EntityId),
    UpdateEntityFields(EntityId),
    ReplaceEntity {
        entity_id: EntityId,
        replacement_partition_id: crate::identity::data::PartitionId,
        replacement_kind_id: crate::identity::data::KindId,
        replacement_client_key: crate::symbols::data::InternedString,
    },
    DeleteEntity(EntityId),
    CreateRelation(RelationCreateKey),
    BulkCreateRelations {
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        endpoints: Vec<(EntityReference, EntityReference)>,
    },
    DeleteRelation(RelationId),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RelationCreateKey {
    pub(crate) partition_id: crate::identity::data::PartitionId,
    pub(crate) kind_id: crate::identity::data::KindId,
    pub(crate) source: EntityReference,
    pub(crate) target: EntityReference,
    pub(crate) client_key: crate::symbols::data::InternedString,
}

pub(crate) fn canonical_intent_key(intent: &MutationIntent) -> CanonicalIntentKey {
    match intent {
        MutationIntent::Create(CreateIntent::Entity(spec)) => CanonicalIntentKey::CreateEntity {
            partition_id: spec.partition_id,
            kind_id: spec.kind_id,
            client_key: spec.client_key.clone(),
        },
        MutationIntent::Create(CreateIntent::BulkEntities(spec)) => {
            CanonicalIntentKey::BulkCreateEntities {
                partition_id: spec.partition_id,
                kind_id: spec.kind_id,
                client_keys: spec.client_keys.clone(),
            }
        }
        MutationIntent::Entity(EntityMutationIntent::Update(spec)) => {
            CanonicalIntentKey::UpdateEntity(spec.entity_id)
        }
        MutationIntent::Entity(EntityMutationIntent::UpdateFields(spec)) => {
            CanonicalIntentKey::UpdateEntityFields(spec.entity_id)
        }
        MutationIntent::Entity(EntityMutationIntent::Replace(spec)) => {
            CanonicalIntentKey::ReplaceEntity {
                entity_id: spec.entity_id,
                replacement_partition_id: spec.replacement.partition_id,
                replacement_kind_id: spec.replacement.kind_id,
                replacement_client_key: spec.replacement.client_key.clone(),
            }
        }
        MutationIntent::Entity(EntityMutationIntent::Delete(spec)) => {
            CanonicalIntentKey::DeleteEntity(spec.entity_id)
        }
        MutationIntent::Create(CreateIntent::Relation(spec)) => {
            CanonicalIntentKey::CreateRelation(RelationCreateKey {
                partition_id: spec.partition_id,
                kind_id: spec.kind_id,
                source: spec.source.clone(),
                target: spec.target.clone(),
                client_key: spec.client_key.clone(),
            })
        }
        MutationIntent::Create(CreateIntent::BulkRelations(spec)) => {
            CanonicalIntentKey::BulkCreateRelations {
                partition_id: spec.partition_id,
                kind_id: spec.kind_id,
                endpoints: spec.endpoints.clone(),
            }
        }
        MutationIntent::Relation(RelationMutationIntent::Delete(spec)) => {
            CanonicalIntentKey::DeleteRelation(spec.relation_id)
        }
    }
}
