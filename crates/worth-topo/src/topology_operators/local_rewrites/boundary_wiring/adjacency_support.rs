use forge_relational::facade::identity::EntityId;
use schema::facade::platform::relations::TopologyRelationKind;

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::bindings::{
    query_incoming_relation_source_identities, query_outgoing_relation_target_identities,
};
use crate::topology_operators::application::TopologyOperatorExecutionError;

pub(crate) fn single_outgoing_relation_target_identity(
    bindings: &TopologyQueryBindingIndex,
    entity_id: EntityId,
    source_query_identity: &str,
    relation_kind: TopologyRelationKind,
) -> Result<String, TopologyOperatorExecutionError> {
    let identities =
        query_outgoing_relation_target_identities(bindings, source_query_identity, relation_kind)?;
    if identities.len() != 1 {
        return Err(
            TopologyOperatorExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected: 1,
                actual: identities.len(),
            },
        );
    }
    Ok(identities[0].clone())
}

pub(crate) fn single_incoming_relation_source_identity(
    bindings: &TopologyQueryBindingIndex,
    entity_id: EntityId,
    target_query_identity: &str,
    relation_kind: TopologyRelationKind,
) -> Result<String, TopologyOperatorExecutionError> {
    let identities =
        query_incoming_relation_source_identities(bindings, target_query_identity, relation_kind)?;
    if identities.len() != 1 {
        return Err(
            TopologyOperatorExecutionError::ExistingEntityIncomingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected: 1,
                actual: identities.len(),
            },
        );
    }
    Ok(identities[0].clone())
}
