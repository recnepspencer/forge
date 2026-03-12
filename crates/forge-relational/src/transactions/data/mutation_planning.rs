use crate::identity::data::{EntityId, PartitionId};
use crate::payloads::data::RecordPayload;
use super::{
    BulkRelationCreateIntent, CreateIntent, DeleteEntityIntent, EntityMutationIntent,
    ExistingRecordTarget, MutationIntent, RelationIdentity, RelationMutationIntent,
    ReplaceEntityIntent, RollbackEffect, UpdateEntityIntent,
};

impl MutationIntent {
    pub(crate) fn seed_touched_partitions(
        &self,
        touched: &mut std::collections::BTreeSet<PartitionId>,
    ) {
        match self {
            Self::Create(CreateIntent::Entity(spec)) => {
                touched.insert(spec.partition_id);
            }
            Self::Create(CreateIntent::BulkEntities(spec)) => {
                touched.insert(spec.partition_id);
            }
            Self::Entity(EntityMutationIntent::Update(spec)) => {
                touched.insert(spec.entity_id.partition_id);
            }
            Self::Entity(EntityMutationIntent::Replace(spec)) => {
                touched.insert(spec.entity_id.partition_id);
                touched.insert(spec.replacement.partition_id);
            }
            Self::Entity(EntityMutationIntent::Delete(spec)) => {
                touched.insert(spec.entity_id.partition_id);
            }
            Self::Create(CreateIntent::Relation(spec)) => {
                touched.insert(spec.partition_id);
                touched.insert(spec.source.partition_id);
                touched.insert(spec.target.partition_id);
            }
            Self::Create(CreateIntent::BulkRelations(spec)) => {
                touched.insert(spec.partition_id);
                for (source, target) in &spec.endpoints {
                    touched.insert(source.partition_id);
                    touched.insert(target.partition_id);
                }
            }
            Self::Relation(RelationMutationIntent::Delete(spec)) => {
                touched.insert(spec.relation_id.partition_id);
            }
        }
    }

    pub(crate) fn bulk_entity_reservation(&self) -> Option<(PartitionId, usize)> {
        match self {
            Self::Create(CreateIntent::BulkEntities(spec)) => {
                Some((spec.partition_id, spec.payloads.len()))
            }
            _ => None,
        }
    }

    pub(crate) fn bulk_relation_reservation(&self) -> Option<(PartitionId, usize)> {
        match self {
            Self::Create(CreateIntent::BulkRelations(spec)) => {
                Some((spec.partition_id, spec.endpoints.len()))
            }
            _ => None,
        }
    }

    pub(crate) fn rollback_effect(&self) -> RollbackEffect {
        match self {
            Self::Create(CreateIntent::Entity(_)) | Self::Create(CreateIntent::BulkEntities(_)) => {
                RollbackEffect::DiscardedEntityCreation
            }
            Self::Entity(EntityMutationIntent::Update(UpdateEntityIntent { entity_id, .. }))
            | Self::Entity(EntityMutationIntent::Replace(ReplaceEntityIntent { entity_id, .. }))
            | Self::Entity(EntityMutationIntent::Delete(DeleteEntityIntent { entity_id })) => {
                RollbackEffect::RestoredEntity(*entity_id)
            }
            Self::Create(CreateIntent::Relation(_))
            | Self::Create(CreateIntent::BulkRelations(_)) => {
                RollbackEffect::DiscardedRelationCreation
            }
            Self::Relation(RelationMutationIntent::Delete(spec)) => {
                RollbackEffect::RestoredRelation(spec.relation_id)
            }
        }
    }

    pub(crate) fn existing_record_target(&self) -> Option<ExistingRecordTarget> {
        match self {
            Self::Entity(EntityMutationIntent::Update(spec)) => {
                Some(ExistingRecordTarget::Entity(spec.entity_id))
            }
            Self::Entity(EntityMutationIntent::Replace(spec)) => {
                Some(ExistingRecordTarget::Entity(spec.entity_id))
            }
            Self::Entity(EntityMutationIntent::Delete(spec)) => {
                Some(ExistingRecordTarget::Entity(spec.entity_id))
            }
            Self::Relation(RelationMutationIntent::Delete(spec)) => {
                Some(ExistingRecordTarget::Relation(spec.relation_id))
            }
            Self::Create(_) => None,
        }
    }

    pub(crate) fn collect_relation_identities(
        &self,
        identities: &mut Vec<RelationIdentity>,
    ) {
        match self {
            Self::Create(CreateIntent::Relation(spec)) => identities.push(RelationIdentity {
                partition_id: spec.partition_id,
                kind_id: spec.kind_id,
                source: spec.source,
                target: spec.target,
            }),
            Self::Create(CreateIntent::BulkRelations(BulkRelationCreateIntent {
                partition_id,
                kind_id,
                endpoints,
                ..
            })) => {
                for (source, target) in endpoints {
                    identities.push(RelationIdentity {
                        partition_id: *partition_id,
                        kind_id: *kind_id,
                        source: *source,
                        target: *target,
                    });
                }
            }
            _ => {}
        }
    }

    pub(crate) fn collect_planned_entity_field_values(
        &self,
        field: &str,
        values: &mut Vec<(Option<EntityId>, String)>,
    ) -> bool {
        match self {
            Self::Create(CreateIntent::Entity(spec)) => {
                collect_payload_field_value(None, &spec.payload, field, values);
                true
            }
            Self::Create(CreateIntent::BulkEntities(spec)) => {
                for payload in &spec.payloads {
                    collect_payload_field_value(None, payload, field, values);
                }
                true
            }
            Self::Entity(EntityMutationIntent::Update(spec)) => {
                collect_payload_field_value(Some(spec.entity_id), &spec.payload, field, values);
                true
            }
            Self::Entity(EntityMutationIntent::Replace(spec)) => {
                collect_payload_field_value(
                    Some(spec.entity_id),
                    &spec.replacement.payload,
                    field,
                    values,
                );
                true
            }
            _ => false,
        }
    }
}

pub(super) fn collect_payload_field_value(
    entity_id: Option<EntityId>,
    payload: &RecordPayload,
    field: &str,
    values: &mut Vec<(Option<EntityId>, String)>,
) {
    if let Some(value) = payload
        .as_json()
        .and_then(|value| value.get(field))
        .and_then(|value| value.as_str())
    {
        values.push((entity_id, value.to_string()));
    }
}
