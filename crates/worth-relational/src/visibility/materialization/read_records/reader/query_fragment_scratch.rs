use std::collections::{BTreeSet, VecDeque};

use crate::storage::data::{EntityReadRecord, RelationReadRecord};

#[derive(Default)]
pub(super) struct QueryFragmentScratch {
    entity_capacity_hint: usize,
    relation_capacity_hint: usize,
    entity_visit_key_capacity_hint: usize,
    relation_visit_key_capacity_hint: usize,
    visited_entities: BTreeSet<crate::identity::data::EntityId>,
    emitted_relations: BTreeSet<crate::identity::data::RelationId>,
    frontier: VecDeque<(
        crate::identity::data::EntityId,
        u32,
        crate::identity::data::EntityId,
        Option<crate::identity::data::RelationId>,
    )>,
}

impl QueryFragmentScratch {
    pub(super) fn entity_buffer(&self) -> Vec<EntityReadRecord> {
        Vec::with_capacity(self.entity_capacity_hint)
    }

    pub(super) fn relation_buffer(&self) -> Vec<RelationReadRecord> {
        Vec::with_capacity(self.relation_capacity_hint)
    }

    pub(super) fn entity_visit_key_buffer(
        &self,
    ) -> Vec<crate::query::data::TraversalEntityVisitKey> {
        Vec::with_capacity(self.entity_visit_key_capacity_hint)
    }

    pub(super) fn relation_visit_key_buffer(
        &self,
    ) -> Vec<crate::query::data::TraversalRelationVisitKey> {
        Vec::with_capacity(self.relation_visit_key_capacity_hint)
    }

    pub(super) fn remember_entity_capacity(&mut self, len: usize) {
        self.entity_capacity_hint = self.entity_capacity_hint.max(len);
    }

    pub(super) fn remember_relation_capacity(&mut self, len: usize) {
        self.relation_capacity_hint = self.relation_capacity_hint.max(len);
    }

    pub(super) fn remember_entity_visit_key_capacity(&mut self, len: usize) {
        self.entity_visit_key_capacity_hint = self.entity_visit_key_capacity_hint.max(len);
    }

    pub(super) fn remember_relation_visit_key_capacity(&mut self, len: usize) {
        self.relation_visit_key_capacity_hint = self.relation_visit_key_capacity_hint.max(len);
    }

    pub(super) fn reset_traversal(&mut self) {
        self.visited_entities.clear();
        self.emitted_relations.clear();
        self.frontier.clear();
    }

    pub(super) fn visited_entities_mut(
        &mut self,
    ) -> &mut BTreeSet<crate::identity::data::EntityId> {
        &mut self.visited_entities
    }

    pub(super) fn emitted_relations_mut(
        &mut self,
    ) -> &mut BTreeSet<crate::identity::data::RelationId> {
        &mut self.emitted_relations
    }

    pub(super) fn frontier_mut(
        &mut self,
    ) -> &mut VecDeque<(
        crate::identity::data::EntityId,
        u32,
        crate::identity::data::EntityId,
        Option<crate::identity::data::RelationId>,
    )> {
        &mut self.frontier
    }
}
