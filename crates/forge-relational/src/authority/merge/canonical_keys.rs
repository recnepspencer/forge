use crate::identity::data::{EntityId, RelationId};
use crate::transactions::data::TransactionIntent;

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
        endpoints: Vec<(EntityId, EntityId)>,
    },
    DeleteRelation(RelationId),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RelationCreateKey {
    pub(crate) partition_id: crate::identity::data::PartitionId,
    pub(crate) kind_id: crate::identity::data::KindId,
    pub(crate) source: EntityId,
    pub(crate) target: EntityId,
    pub(crate) client_key: crate::symbols::data::InternedString,
}

pub(crate) fn canonical_intent_key(intent: &TransactionIntent) -> CanonicalIntentKey {
    match intent {
        TransactionIntent::CreateEntity(spec) => CanonicalIntentKey::CreateEntity {
            partition_id: spec.partition_id,
            kind_id: spec.kind_id,
            client_key: spec.client_key.clone(),
        },
        TransactionIntent::BulkCreateEntities {
            partition_id,
            kind_id,
            client_keys,
            ..
        } => CanonicalIntentKey::BulkCreateEntities {
            partition_id: *partition_id,
            kind_id: *kind_id,
            client_keys: client_keys.clone(),
        },
        TransactionIntent::UpdateEntity { entity_id, .. } => {
            CanonicalIntentKey::UpdateEntity(*entity_id)
        }
        TransactionIntent::ReplaceEntity {
            entity_id,
            replacement,
        } => CanonicalIntentKey::ReplaceEntity {
            entity_id: *entity_id,
            replacement_partition_id: replacement.partition_id,
            replacement_kind_id: replacement.kind_id,
            replacement_client_key: replacement.client_key.clone(),
        },
        TransactionIntent::DeleteEntity { entity_id } => {
            CanonicalIntentKey::DeleteEntity(*entity_id)
        }
        TransactionIntent::CreateRelation(spec) => {
            CanonicalIntentKey::CreateRelation(RelationCreateKey {
                partition_id: spec.partition_id,
                kind_id: spec.kind_id,
                source: spec.source,
                target: spec.target,
                client_key: spec.client_key.clone(),
            })
        }
        TransactionIntent::BulkCreateRelations {
            partition_id,
            kind_id,
            endpoints,
            ..
        } => CanonicalIntentKey::BulkCreateRelations {
            partition_id: *partition_id,
            kind_id: *kind_id,
            endpoints: endpoints.clone(),
        },
        TransactionIntent::DeleteRelation { relation_id } => {
            CanonicalIntentKey::DeleteRelation(*relation_id)
        }
    }
}
