use std::collections::BTreeSet;
use std::sync::Arc;

use crate::identity::data::{EntityId, PartitionId, RelationId};
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::{
    CreateIntent, EntityMutationIntent, EntityReference, MergedCommitPlan, MutationIntent,
    RelationMutationIntent,
};
use crate::validation::engine::state_view::{InvariantStateView, VisibleRelationMetadata};

use super::planned_records::{
    CustomInvariantTouchedSummary, PlannedEntityCreate, PlannedRelationCreate,
    PlannedRelationEndpointUpdate,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TouchedStructuralSet {
    visible_entity_ids: Arc<[EntityId]>,
    visible_relation_ids: Arc<[RelationId]>,
    touched_partitions: Arc<[PartitionId]>,
    planned_entity_deletes: Arc<[EntityId]>,
    planned_entity_creates: Arc<[PlannedEntityCreate]>,
    planned_relation_creates: Arc<[PlannedRelationCreate]>,
    planned_relation_deletes: Arc<[RelationId]>,
    planned_relation_endpoint_updates: Arc<[PlannedRelationEndpointUpdate]>,
}

impl TouchedStructuralSet {
    pub(crate) fn new(
        visible_entity_ids: Arc<[EntityId]>,
        visible_relation_ids: Arc<[RelationId]>,
        touched_partitions: Arc<[PartitionId]>,
        planned_entity_deletes: Arc<[EntityId]>,
        planned_entity_creates: Arc<[PlannedEntityCreate]>,
        planned_relation_creates: Arc<[PlannedRelationCreate]>,
        planned_relation_deletes: Arc<[RelationId]>,
        planned_relation_endpoint_updates: Arc<[PlannedRelationEndpointUpdate]>,
    ) -> Self {
        Self {
            visible_entity_ids,
            visible_relation_ids,
            touched_partitions,
            planned_entity_deletes,
            planned_entity_creates,
            planned_relation_creates,
            planned_relation_deletes,
            planned_relation_endpoint_updates,
        }
    }

    pub fn visible_entity_ids(&self) -> &[EntityId] {
        &self.visible_entity_ids
    }

    pub fn visible_relation_ids(&self) -> &[RelationId] {
        &self.visible_relation_ids
    }

    pub fn touched_partitions(&self) -> &[PartitionId] {
        &self.touched_partitions
    }

    pub fn planned_entity_deletes(&self) -> &[EntityId] {
        &self.planned_entity_deletes
    }

    pub fn planned_entity_creates(&self) -> &[PlannedEntityCreate] {
        &self.planned_entity_creates
    }

    pub fn planned_relation_creates(&self) -> &[PlannedRelationCreate] {
        &self.planned_relation_creates
    }

    pub fn planned_relation_deletes(&self) -> &[RelationId] {
        &self.planned_relation_deletes
    }

    pub fn planned_relation_endpoint_updates(&self) -> &[PlannedRelationEndpointUpdate] {
        &self.planned_relation_endpoint_updates
    }

    pub(crate) fn provenance_summary(&self) -> CustomInvariantTouchedSummary {
        CustomInvariantTouchedSummary {
            visible_entity_ids: self.visible_entity_ids.clone(),
            visible_relation_ids: self.visible_relation_ids.clone(),
            touched_partition_ids: self.touched_partitions.clone(),
            planned_entity_delete_count: self.planned_entity_deletes.len(),
            planned_entity_create_count: self.planned_entity_creates.len(),
            planned_relation_create_count: self.planned_relation_creates.len(),
            planned_relation_delete_count: self.planned_relation_deletes.len(),
            planned_relation_endpoint_update_count: self.planned_relation_endpoint_updates.len(),
        }
    }
}

pub(crate) fn collect_touched_structural_set(
    runtime: &RelationalRuntime,
    state_view: &InvariantStateView<'_>,
    merged_plan: Option<&MergedCommitPlan>,
) -> TouchedStructuralSet {
    let mut visible_entities = BTreeSet::new();
    let mut visible_relations = BTreeSet::new();
    let mut touched_partitions = BTreeSet::new();
    let mut planned_entity_deletes = Vec::new();
    let mut planned_entity_creates = Vec::new();
    let mut planned_relation_creates = Vec::new();
    let mut planned_relation_deletes = Vec::new();
    let mut planned_relation_endpoint_updates = Vec::new();

    if let Some(ids) = state_view.touched_visible_entity_ids() {
        visible_entities.extend(ids);
    }
    if let Some(ids) = state_view.touched_visible_relation_ids() {
        visible_relations.extend(ids);
    }

    if let Some(plan) = merged_plan {
        for intent in &plan.merged_intents {
            intent.seed_touched_partitions(&mut touched_partitions);
            match intent {
                MutationIntent::Create(CreateIntent::Entity(spec)) => {
                    planned_entity_creates.push(PlannedEntityCreate::new(
                        spec.partition_id,
                        spec.kind_id,
                        spec.client_key.clone(),
                    ));
                }
                MutationIntent::Create(CreateIntent::EntityAspects(spec)) => {
                    planned_entity_creates.push(PlannedEntityCreate::new(
                        spec.partition_id,
                        spec.kind_id,
                        spec.client_key.clone(),
                    ));
                }
                MutationIntent::Create(CreateIntent::BulkEntities(spec)) => {
                    for client_key in spec.client_keys.iter() {
                        planned_entity_creates.push(PlannedEntityCreate::new(
                            spec.partition_id,
                            spec.kind_id,
                            client_key.clone(),
                        ));
                    }
                }
                MutationIntent::Create(CreateIntent::Relation(spec)) => {
                    include_existing_entity_reference(&mut visible_entities, &spec.source);
                    include_existing_entity_reference(&mut visible_entities, &spec.target);
                    planned_relation_creates.push(PlannedRelationCreate::new(
                        spec.partition_id,
                        spec.kind_id,
                        spec.client_key.clone(),
                        spec.source.clone(),
                        spec.target.clone(),
                    ));
                }
                MutationIntent::Create(CreateIntent::RelationAspects(spec)) => {
                    include_existing_entity_reference(&mut visible_entities, &spec.source);
                    include_existing_entity_reference(&mut visible_entities, &spec.target);
                    planned_relation_creates.push(PlannedRelationCreate::new(
                        spec.partition_id,
                        spec.kind_id,
                        spec.client_key.clone(),
                        spec.source.clone(),
                        spec.target.clone(),
                    ));
                }
                MutationIntent::Create(CreateIntent::BulkRelations(spec)) => {
                    for ((source, target), client_key) in
                        spec.endpoints.iter().zip(spec.client_keys.iter())
                    {
                        include_existing_entity_reference(&mut visible_entities, source);
                        include_existing_entity_reference(&mut visible_entities, target);
                        planned_relation_creates.push(PlannedRelationCreate::new(
                            spec.partition_id,
                            spec.kind_id,
                            client_key.clone(),
                            source.clone(),
                            target.clone(),
                        ));
                    }
                }
                MutationIntent::Entity(EntityMutationIntent::UpdateFields(spec)) => {
                    visible_entities.insert(spec.entity_id);
                }
                MutationIntent::Entity(EntityMutationIntent::ApplyAspectPatch(spec)) => {
                    visible_entities.insert(spec.entity_id);
                }
                MutationIntent::Entity(EntityMutationIntent::Replace(spec)) => {
                    visible_entities.insert(spec.entity_id);
                }
                MutationIntent::Entity(EntityMutationIntent::Delete(spec)) => {
                    visible_entities.insert(spec.entity_id);
                    planned_entity_deletes.push(spec.entity_id);
                }
                MutationIntent::Relation(RelationMutationIntent::UpdateEndpoints(spec)) => {
                    visible_relations.insert(spec.relation_id);
                    include_existing_entity_reference(&mut visible_entities, &spec.source);
                    include_existing_entity_reference(&mut visible_entities, &spec.target);
                    planned_relation_endpoint_updates.push(PlannedRelationEndpointUpdate::new(
                        spec.relation_id,
                        spec.kind_id,
                        spec.source.clone(),
                        spec.target.clone(),
                    ));
                    if let Some(metadata) = state_view.relation_metadata(spec.relation_id) {
                        include_relation_metadata(
                            &mut visible_entities,
                            &mut touched_partitions,
                            metadata,
                        );
                    }
                }
                MutationIntent::Relation(RelationMutationIntent::ApplyAspectPatch(spec)) => {
                    visible_relations.insert(spec.relation_id);
                    if let Some(metadata) = state_view.relation_metadata(spec.relation_id) {
                        include_relation_metadata(
                            &mut visible_entities,
                            &mut touched_partitions,
                            metadata,
                        );
                    }
                }
                MutationIntent::Relation(RelationMutationIntent::Delete(spec)) => {
                    visible_relations.insert(spec.relation_id);
                    planned_relation_deletes.push(spec.relation_id);
                    if let Some(metadata) = state_view.relation_metadata(spec.relation_id) {
                        include_relation_metadata(
                            &mut visible_entities,
                            &mut touched_partitions,
                            metadata,
                        );
                    }
                }
            }
        }
    }

    let seed_entities = visible_entities.iter().copied().collect::<Vec<_>>();
    for entity_id in seed_entities {
        for relation_id in runtime
            .storage_access()
            .all_relations_for_entity(entity_id, state_view.version_id())
        {
            visible_relations.insert(relation_id);
            if let Some(metadata) = state_view.relation_metadata(relation_id) {
                include_relation_metadata(&mut visible_entities, &mut touched_partitions, metadata);
            }
        }
    }

    TouchedStructuralSet::new(
        visible_entities.into_iter().collect::<Vec<_>>().into(),
        visible_relations.into_iter().collect::<Vec<_>>().into(),
        touched_partitions.into_iter().collect::<Vec<_>>().into(),
        planned_entity_deletes.into(),
        planned_entity_creates.into(),
        planned_relation_creates.into(),
        planned_relation_deletes.into(),
        planned_relation_endpoint_updates.into(),
    )
}

fn include_existing_entity_reference(
    visible_entities: &mut BTreeSet<EntityId>,
    entity_reference: &EntityReference,
) {
    if let EntityReference::Existing(entity_id) = entity_reference {
        visible_entities.insert(*entity_id);
    }
}

fn include_relation_metadata(
    visible_entities: &mut BTreeSet<EntityId>,
    touched_partitions: &mut BTreeSet<PartitionId>,
    metadata: VisibleRelationMetadata,
) {
    visible_entities.insert(metadata.source);
    visible_entities.insert(metadata.target);
    touched_partitions.insert(metadata.relation_id.partition_id);
    touched_partitions.insert(metadata.source.partition_id);
    touched_partitions.insert(metadata.target.partition_id);
}
