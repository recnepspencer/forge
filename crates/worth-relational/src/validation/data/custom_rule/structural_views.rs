use worth_foundational::facade::AuthoritativeRecordAspectState;

use crate::identity::data::{EntityId, KindId, RelationId};
use crate::logic::runtime::RelationalRuntime;
use crate::validation::engine::state_view::InvariantStateView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructuralRelationRecord {
    pub relation_id: RelationId,
    pub kind_id: KindId,
    pub source: EntityId,
    pub target: EntityId,
}

#[derive(Clone, Copy)]
pub struct StructuralAspectStateView<'runtime> {
    state_view: InvariantStateView<'runtime>,
}

impl<'runtime> StructuralAspectStateView<'runtime> {
    pub(crate) fn new(state_view: InvariantStateView<'runtime>) -> Self {
        Self { state_view }
    }

    pub fn entity_aspect_state(
        &self,
        entity_id: EntityId,
    ) -> Option<&'runtime AuthoritativeRecordAspectState> {
        self.state_view.entity_aspect_state(entity_id)
    }

    pub fn relation_aspect_state(
        &self,
        relation_id: RelationId,
    ) -> Option<&'runtime AuthoritativeRecordAspectState> {
        self.state_view.relation_aspect_state(relation_id)
    }
}

#[derive(Clone, Copy)]
pub struct StructuralRelationView<'runtime> {
    runtime: &'runtime RelationalRuntime,
    state_view: InvariantStateView<'runtime>,
}

impl<'runtime> StructuralRelationView<'runtime> {
    pub(crate) fn new(
        runtime: &'runtime RelationalRuntime,
        state_view: InvariantStateView<'runtime>,
    ) -> Self {
        Self {
            runtime,
            state_view,
        }
    }

    pub fn entity_kind(&self, entity_id: EntityId) -> Option<KindId> {
        self.state_view
            .entity_metadata(entity_id)
            .map(|metadata| metadata.kind_id)
    }

    pub fn relation(&self, relation_id: RelationId) -> Option<StructuralRelationRecord> {
        self.state_view
            .relation_metadata(relation_id)
            .map(|metadata| StructuralRelationRecord {
                relation_id: metadata.relation_id,
                kind_id: metadata.kind_id,
                source: metadata.source,
                target: metadata.target,
            })
    }

    pub fn outgoing_relations_for_entity(&self, entity_id: EntityId) -> Vec<RelationId> {
        self.runtime
            .storage_access()
            .outgoing_relations_for_entity(entity_id, self.state_view.version_id())
    }

    pub fn incoming_relations_for_entity(&self, entity_id: EntityId) -> Vec<RelationId> {
        self.runtime
            .storage_access()
            .incoming_relations_for_entity(entity_id, self.state_view.version_id())
    }

    pub fn all_relations_for_entity(&self, entity_id: EntityId) -> Vec<RelationId> {
        self.runtime
            .storage_access()
            .all_relations_for_entity(entity_id, self.state_view.version_id())
    }
}
