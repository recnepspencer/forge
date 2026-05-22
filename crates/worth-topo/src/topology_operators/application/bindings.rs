use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::{TopologyEntityKind, TopologyRelationKind};

use super::TopologyOperatorExecutionError;
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
    ) -> Result<Option<QueryEntityBinding>, TopologyOperatorExecutionError>;

    fn entity_id_by_query_identity(
        &self,
        query_identity: &str,
    ) -> Result<Option<EntityId>, TopologyOperatorExecutionError>;
}

pub(crate) trait RelationBindingLookup {
    fn outgoing_relation_target_identity_rows(
        &self,
        source_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Result<Vec<String>, TopologyOperatorExecutionError>;

    fn outgoing_relation_id_rows(
        &self,
        source_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Result<Vec<RelationId>, TopologyOperatorExecutionError>;

    fn incoming_relation_source_identity_rows(
        &self,
        target_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Result<Vec<String>, TopologyOperatorExecutionError>;

    fn incoming_relation_id_rows(
        &self,
        target_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Result<Vec<RelationId>, TopologyOperatorExecutionError>;

    fn relation_binding_lookup(
        &self,
        relation_id: RelationId,
    ) -> Result<Option<QueryRelationBinding>, TopologyOperatorExecutionError>;
}

impl EntityBindingLookup for TopologyQueryBindingIndex {
    fn entity_binding_lookup(
        &self,
        entity_id: EntityId,
    ) -> Result<Option<QueryEntityBinding>, TopologyOperatorExecutionError> {
        Ok(self.entity_binding(entity_id))
    }

    fn entity_id_by_query_identity(
        &self,
        query_identity: &str,
    ) -> Result<Option<EntityId>, TopologyOperatorExecutionError> {
        Ok(self.entity_id_by_identity(query_identity))
    }
}

impl RelationBindingLookup for TopologyQueryBindingIndex {
    fn outgoing_relation_target_identity_rows(
        &self,
        source_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Result<Vec<String>, TopologyOperatorExecutionError> {
        Ok(self.outgoing_relation_target_identities(source_query_identity, expected_kind))
    }

    fn outgoing_relation_id_rows(
        &self,
        source_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Result<Vec<RelationId>, TopologyOperatorExecutionError> {
        Ok(self.outgoing_relation_ids(source_query_identity, expected_kind))
    }

    fn incoming_relation_source_identity_rows(
        &self,
        target_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Result<Vec<String>, TopologyOperatorExecutionError> {
        Ok(self.incoming_relation_source_identities(target_query_identity, expected_kind))
    }

    fn incoming_relation_id_rows(
        &self,
        target_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Result<Vec<RelationId>, TopologyOperatorExecutionError> {
        Ok(self.incoming_relation_ids(target_query_identity, expected_kind))
    }

    fn relation_binding_lookup(
        &self,
        relation_id: RelationId,
    ) -> Result<Option<QueryRelationBinding>, TopologyOperatorExecutionError> {
        Ok(self.relation_binding(relation_id))
    }
}

pub(crate) fn query_outgoing_relation_target_identities(
    bindings: &(impl RelationBindingLookup + ?Sized),
    source_query_identity: &str,
    expected_kind: TopologyRelationKind,
) -> Result<Vec<String>, TopologyOperatorExecutionError> {
    bindings.outgoing_relation_target_identity_rows(source_query_identity, expected_kind)
}

pub(crate) fn query_outgoing_relation_ids(
    bindings: &(impl RelationBindingLookup + ?Sized),
    source_query_identity: &str,
    expected_kind: TopologyRelationKind,
) -> Result<Vec<RelationId>, TopologyOperatorExecutionError> {
    bindings.outgoing_relation_id_rows(source_query_identity, expected_kind)
}

pub(crate) fn query_incoming_relation_source_identities(
    bindings: &(impl RelationBindingLookup + ?Sized),
    target_query_identity: &str,
    expected_kind: TopologyRelationKind,
) -> Result<Vec<String>, TopologyOperatorExecutionError> {
    bindings.incoming_relation_source_identity_rows(target_query_identity, expected_kind)
}

pub(crate) fn query_incoming_relation_ids(
    bindings: &(impl RelationBindingLookup + ?Sized),
    target_query_identity: &str,
    expected_kind: TopologyRelationKind,
) -> Result<Vec<RelationId>, TopologyOperatorExecutionError> {
    bindings.incoming_relation_id_rows(target_query_identity, expected_kind)
}

pub(crate) fn query_entity_id_by_identity(
    bindings: &(impl EntityBindingLookup + ?Sized),
    query_identity: &str,
) -> Result<Option<EntityId>, TopologyOperatorExecutionError> {
    bindings.entity_id_by_query_identity(query_identity)
}

pub(crate) fn query_entity_binding(
    bindings: &(impl EntityBindingLookup + ?Sized),
    entity_id: EntityId,
) -> Result<Option<QueryEntityBinding>, TopologyOperatorExecutionError> {
    bindings.entity_binding_lookup(entity_id)
}

pub(crate) fn query_relation_binding(
    bindings: &(impl RelationBindingLookup + ?Sized),
    relation_id: RelationId,
) -> Result<Option<QueryRelationBinding>, TopologyOperatorExecutionError> {
    bindings.relation_binding_lookup(relation_id)
}
