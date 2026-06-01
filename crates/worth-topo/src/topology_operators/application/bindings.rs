use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use super::TopologyMutationApplicationError;
use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;

#[derive(Debug, Clone)]
pub(crate) struct QueryEntityBinding {
    pub(crate) query_identity: String,
    pub(crate) kind: TopologyEntityKind,
}

#[derive(Debug, Clone)]
pub(crate) struct QueryRelationBinding {
    pub(crate) query_identity: String,
    pub(crate) kind: TopologyRelationKind,
    pub(crate) source_query_identity: String,
    pub(crate) target_query_identity: String,
}

pub(crate) trait EntityBindingLookup {
    fn entity_binding_lookup(
        &self,
        entity_id: EntityId,
    ) -> Result<Option<QueryEntityBinding>, TopologyMutationApplicationError>;

    fn entity_id_by_query_identity(
        &self,
        query_identity: &str,
    ) -> Result<Option<EntityId>, TopologyMutationApplicationError>;
}

pub(crate) trait RelationBindingLookup {
    fn outgoing_relation_target_identity_rows(
        &self,
        source_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Result<Vec<String>, TopologyMutationApplicationError>;

    fn incoming_relation_source_identity_rows(
        &self,
        target_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Result<Vec<String>, TopologyMutationApplicationError>;

    fn incoming_relation_id_rows(
        &self,
        target_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Result<Vec<RelationId>, TopologyMutationApplicationError>;

    fn relation_binding_lookup(
        &self,
        relation_id: RelationId,
    ) -> Result<Option<QueryRelationBinding>, TopologyMutationApplicationError>;
}

impl EntityBindingLookup for TopologyQueryBindingIndex {
    fn entity_binding_lookup(
        &self,
        entity_id: EntityId,
    ) -> Result<Option<QueryEntityBinding>, TopologyMutationApplicationError> {
        Ok(self.entity_binding(entity_id))
    }

    fn entity_id_by_query_identity(
        &self,
        query_identity: &str,
    ) -> Result<Option<EntityId>, TopologyMutationApplicationError> {
        Ok(self.entity_id_by_identity(query_identity))
    }
}

impl RelationBindingLookup for TopologyQueryBindingIndex {
    fn outgoing_relation_target_identity_rows(
        &self,
        source_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Result<Vec<String>, TopologyMutationApplicationError> {
        Ok(self.outgoing_relation_target_identities(source_query_identity, expected_kind))
    }

    fn incoming_relation_source_identity_rows(
        &self,
        target_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Result<Vec<String>, TopologyMutationApplicationError> {
        Ok(self.incoming_relation_source_identities(target_query_identity, expected_kind))
    }

    fn incoming_relation_id_rows(
        &self,
        target_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Result<Vec<RelationId>, TopologyMutationApplicationError> {
        Ok(self.incoming_relation_ids(target_query_identity, expected_kind))
    }

    fn relation_binding_lookup(
        &self,
        relation_id: RelationId,
    ) -> Result<Option<QueryRelationBinding>, TopologyMutationApplicationError> {
        Ok(self.relation_binding(relation_id))
    }
}

pub(crate) fn query_outgoing_relation_target_identities(
    bindings: &(impl RelationBindingLookup + ?Sized),
    source_query_identity: &str,
    expected_kind: TopologyRelationKind,
) -> Result<Vec<String>, TopologyMutationApplicationError> {
    bindings.outgoing_relation_target_identity_rows(source_query_identity, expected_kind)
}

pub(crate) fn query_incoming_relation_source_identities(
    bindings: &(impl RelationBindingLookup + ?Sized),
    target_query_identity: &str,
    expected_kind: TopologyRelationKind,
) -> Result<Vec<String>, TopologyMutationApplicationError> {
    bindings.incoming_relation_source_identity_rows(target_query_identity, expected_kind)
}

pub(crate) fn query_incoming_relation_ids(
    bindings: &(impl RelationBindingLookup + ?Sized),
    target_query_identity: &str,
    expected_kind: TopologyRelationKind,
) -> Result<Vec<RelationId>, TopologyMutationApplicationError> {
    bindings.incoming_relation_id_rows(target_query_identity, expected_kind)
}

pub(crate) fn query_entity_id_by_identity(
    bindings: &(impl EntityBindingLookup + ?Sized),
    query_identity: &str,
) -> Result<Option<EntityId>, TopologyMutationApplicationError> {
    bindings.entity_id_by_query_identity(query_identity)
}

pub(crate) fn query_entity_binding(
    bindings: &(impl EntityBindingLookup + ?Sized),
    entity_id: EntityId,
) -> Result<Option<QueryEntityBinding>, TopologyMutationApplicationError> {
    bindings.entity_binding_lookup(entity_id)
}

pub(crate) fn query_relation_binding(
    bindings: &(impl RelationBindingLookup + ?Sized),
    relation_id: RelationId,
) -> Result<Option<QueryRelationBinding>, TopologyMutationApplicationError> {
    bindings.relation_binding_lookup(relation_id)
}
