use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryExistingRelationTarget, ForgeQueryMutationBatchBuilder,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use worth_schema::facade::{WorthTopologyEntityKind, WorthTopologyRelationKind};

use super::bindings::{
    query_entity_binding, query_incoming_relation_source_identities,
    query_outgoing_relation_target_identities, query_relation_binding,
};
use super::{WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner};
use crate::edit::{WorthLoopEndpointKind, WorthLoopSuccessorKind};
use crate::query::topology_relation_dependency_path;

impl<'workspace, 'assembly> WorthTopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn lower_rewire_loop_successor(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
        relation_id: RelationId,
        kind: WorthLoopSuccessorKind,
        half_edge_id: EntityId,
        successor_half_edge_id: EntityId,
    ) -> Result<ForgeQueryMutationBatchBuilder, WorthTopologyQueryEditExecutionError> {
        let relation_kind = kind.relation_kind();
        let relation_binding = query_relation_binding(relation_rows, relation_id)?.ok_or(
            WorthTopologyQueryEditExecutionError::MissingExistingRelationBinding(relation_id),
        )?;
        if relation_binding.kind != relation_kind {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingRelationKindMismatch {
                    relation_id,
                    expected: relation_kind,
                    actual: relation_binding.kind,
                },
            );
        }
        let source_half_edge_binding = query_entity_binding(entity_rows, half_edge_id)?.ok_or(
            WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(half_edge_id),
        )?;
        if source_half_edge_binding.kind != WorthTopologyEntityKind::HalfEdge {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: half_edge_id,
                    expected: WorthTopologyEntityKind::HalfEdge,
                    actual: source_half_edge_binding.kind,
                },
            );
        }
        if relation_binding.source_query_identity != source_half_edge_binding.query_identity {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
                    relation_id,
                    expected_source_entity_id: half_edge_id,
                    actual_source_identity: relation_binding.source_query_identity,
                },
            );
        }
        let target_half_edge_binding = query_entity_binding(entity_rows, successor_half_edge_id)?
            .ok_or(
            WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(
                successor_half_edge_id,
            ),
        )?;
        if target_half_edge_binding.kind != WorthTopologyEntityKind::HalfEdge {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: successor_half_edge_id,
                    expected: WorthTopologyEntityKind::HalfEdge,
                    actual: target_half_edge_binding.kind,
                },
            );
        }
        let source_loop_identity = single_incoming_relation_source_identity(
            relation_rows,
            half_edge_id,
            &source_half_edge_binding.query_identity,
            WorthTopologyRelationKind::LoopOwnsHalfEdge,
        )?;
        let target_loop_identity = single_incoming_relation_source_identity(
            relation_rows,
            successor_half_edge_id,
            &target_half_edge_binding.query_identity,
            WorthTopologyRelationKind::LoopOwnsHalfEdge,
        )?;
        if source_loop_identity != target_loop_identity {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingHalfEdgesNotOnSameLoop {
                    relation_id,
                    source_half_edge_id: half_edge_id,
                    target_half_edge_id: successor_half_edge_id,
                    source_loop_identity,
                    target_loop_identity,
                },
            );
        }
        let binding = self.workspace.bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                format!("{relation_id:?}"),
                relation_binding.query_identity,
            )?
            .in_target_collection("WorthTopologyRelation")?,
        )?;
        let source_query_identity = source_half_edge_binding.query_identity;
        let verified_source_query_identity = source_query_identity.clone();
        let current_target_query_identity = relation_binding.target_query_identity;
        let updated_target_query_identity = target_half_edge_binding.query_identity;
        let dependency_path = topology_relation_dependency_path(
            worth_schema::facade::WorthRelationKind::Topology(relation_kind),
        );
        Ok(builder.update_existing_verified(
            binding,
            |verify| {
                let verify = verify
                    .aspect("topology.kind", relation_kind.kind_name())
                    .aspect("topology.source_identity", verified_source_query_identity)
                    .aspect("topology.target_identity", current_target_query_identity);
                if let Some(path) = dependency_path {
                    verify.aspect(path, relation_kind.kind_name())
                } else {
                    verify
                }
            },
            |update| {
                let update = update
                    .aspect("topology.kind", relation_kind.kind_name())
                    .aspect("topology.source_identity", source_query_identity)
                    .aspect("topology.target_identity", updated_target_query_identity);
                if let Some(path) = dependency_path {
                    update.aspect(path, relation_kind.kind_name())
                } else {
                    update
                }
            },
        ))
    }

    pub(super) fn lower_rewire_loop_endpoint(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
        relation_id: RelationId,
        endpoint: WorthLoopEndpointKind,
        half_edge_id: EntityId,
        vertex_id: EntityId,
    ) -> Result<ForgeQueryMutationBatchBuilder, WorthTopologyQueryEditExecutionError> {
        let relation_binding = query_relation_binding(relation_rows, relation_id)?.ok_or(
            WorthTopologyQueryEditExecutionError::MissingExistingRelationBinding(relation_id),
        )?;
        if relation_binding.kind != endpoint.relation_kind() {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingRelationKindMismatch {
                    relation_id,
                    expected: endpoint.relation_kind(),
                    actual: relation_binding.kind,
                },
            );
        }
        let half_edge_binding = query_entity_binding(entity_rows, half_edge_id)?.ok_or(
            WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(half_edge_id),
        )?;
        if half_edge_binding.kind != WorthTopologyEntityKind::HalfEdge {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: half_edge_id,
                    expected: WorthTopologyEntityKind::HalfEdge,
                    actual: half_edge_binding.kind,
                },
            );
        }
        if relation_binding.source_query_identity != half_edge_binding.query_identity {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
                    relation_id,
                    expected_source_entity_id: half_edge_id,
                    actual_source_identity: relation_binding.source_query_identity,
                },
            );
        }
        let vertex_binding = query_entity_binding(entity_rows, vertex_id)?
            .ok_or(WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(vertex_id))?;
        if vertex_binding.kind != WorthTopologyEntityKind::Vertex {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: vertex_id,
                    expected: WorthTopologyEntityKind::Vertex,
                    actual: vertex_binding.kind,
                },
            );
        }
        let binding = self.workspace.bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                format!("{relation_id:?}"),
                relation_binding.query_identity,
            )?
            .in_target_collection("WorthTopologyRelation")?,
        )?;
        let source_query_identity = half_edge_binding.query_identity;
        let verified_source_query_identity = source_query_identity.clone();
        let current_target_query_identity = relation_binding.target_query_identity;
        let updated_target_query_identity = vertex_binding.query_identity;
        let dependency_path = topology_relation_dependency_path(
            worth_schema::facade::WorthRelationKind::Topology(endpoint.relation_kind()),
        );
        Ok(builder.update_existing_verified(
            binding,
            |verify| {
                let verify = verify
                    .aspect("topology.kind", endpoint.relation_kind().kind_name())
                    .aspect("topology.source_identity", verified_source_query_identity)
                    .aspect("topology.target_identity", current_target_query_identity);
                if let Some(path) = dependency_path {
                    verify.aspect(path, endpoint.relation_kind().kind_name())
                } else {
                    verify
                }
            },
            |update| {
                let update = update
                    .aspect("topology.kind", endpoint.relation_kind().kind_name())
                    .aspect("topology.source_identity", source_query_identity)
                    .aspect("topology.target_identity", updated_target_query_identity);
                if let Some(path) = dependency_path {
                    update.aspect(path, endpoint.relation_kind().kind_name())
                } else {
                    update
                }
            },
        ))
    }

    pub(super) fn lower_splice_radial_adjacency(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
        relation_id: RelationId,
        half_edge_id: EntityId,
        radial_next_half_edge_id: EntityId,
    ) -> Result<ForgeQueryMutationBatchBuilder, WorthTopologyQueryEditExecutionError> {
        let relation_kind = WorthTopologyRelationKind::HalfEdgeRadialNext;
        let relation_binding = query_relation_binding(relation_rows, relation_id)?.ok_or(
            WorthTopologyQueryEditExecutionError::MissingExistingRelationBinding(relation_id),
        )?;
        if relation_binding.kind != relation_kind {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingRelationKindMismatch {
                    relation_id,
                    expected: relation_kind,
                    actual: relation_binding.kind,
                },
            );
        }
        let source_half_edge_binding = query_entity_binding(entity_rows, half_edge_id)?.ok_or(
            WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(half_edge_id),
        )?;
        if source_half_edge_binding.kind != WorthTopologyEntityKind::HalfEdge {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: half_edge_id,
                    expected: WorthTopologyEntityKind::HalfEdge,
                    actual: source_half_edge_binding.kind,
                },
            );
        }
        if relation_binding.source_query_identity != source_half_edge_binding.query_identity {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
                    relation_id,
                    expected_source_entity_id: half_edge_id,
                    actual_source_identity: relation_binding.source_query_identity,
                },
            );
        }
        let target_half_edge_binding = query_entity_binding(entity_rows, radial_next_half_edge_id)?
            .ok_or(
                WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(
                    radial_next_half_edge_id,
                ),
            )?;
        if target_half_edge_binding.kind != WorthTopologyEntityKind::HalfEdge {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: radial_next_half_edge_id,
                    expected: WorthTopologyEntityKind::HalfEdge,
                    actual: target_half_edge_binding.kind,
                },
            );
        }
        let source_edge_identity = single_outgoing_relation_target_identity(
            relation_rows,
            half_edge_id,
            &source_half_edge_binding.query_identity,
            WorthTopologyRelationKind::HalfEdgeUsesEdge,
        )?;
        let target_edge_identity = single_outgoing_relation_target_identity(
            relation_rows,
            radial_next_half_edge_id,
            &target_half_edge_binding.query_identity,
            WorthTopologyRelationKind::HalfEdgeUsesEdge,
        )?;
        if source_edge_identity != target_edge_identity {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingHalfEdgesNotOnSameEdge {
                    relation_id,
                    source_half_edge_id: half_edge_id,
                    target_half_edge_id: radial_next_half_edge_id,
                    source_edge_identity,
                    target_edge_identity,
                },
            );
        }

        let binding = self.workspace.bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                format!("{relation_id:?}"),
                relation_binding.query_identity,
            )?
            .in_target_collection("WorthTopologyRelation")?,
        )?;
        let source_query_identity = source_half_edge_binding.query_identity;
        let verified_source_query_identity = source_query_identity.clone();
        let current_target_query_identity = relation_binding.target_query_identity;
        let updated_target_query_identity = target_half_edge_binding.query_identity;
        let dependency_path = topology_relation_dependency_path(
            worth_schema::facade::WorthRelationKind::Topology(relation_kind),
        );
        Ok(builder.update_existing_verified(
            binding,
            |verify| {
                let verify = verify
                    .aspect("topology.kind", relation_kind.kind_name())
                    .aspect("topology.source_identity", verified_source_query_identity)
                    .aspect("topology.target_identity", current_target_query_identity);
                if let Some(path) = dependency_path {
                    verify.aspect(path, relation_kind.kind_name())
                } else {
                    verify
                }
            },
            |update| {
                let update = update
                    .aspect("topology.kind", relation_kind.kind_name())
                    .aspect("topology.source_identity", source_query_identity)
                    .aspect("topology.target_identity", updated_target_query_identity);
                if let Some(path) = dependency_path {
                    update.aspect(path, relation_kind.kind_name())
                } else {
                    update
                }
            },
        ))
    }
}

fn single_outgoing_relation_target_identity(
    relation_rows: &[ForgeQueryEntity],
    entity_id: EntityId,
    source_query_identity: &str,
    relation_kind: WorthTopologyRelationKind,
) -> Result<String, WorthTopologyQueryEditExecutionError> {
    let identities = query_outgoing_relation_target_identities(
        relation_rows,
        source_query_identity,
        relation_kind,
    )?;
    if identities.len() != 1 {
        return Err(
            WorthTopologyQueryEditExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected: 1,
                actual: identities.len(),
            },
        );
    }
    Ok(identities[0].clone())
}

fn single_incoming_relation_source_identity(
    relation_rows: &[ForgeQueryEntity],
    entity_id: EntityId,
    target_query_identity: &str,
    relation_kind: WorthTopologyRelationKind,
) -> Result<String, WorthTopologyQueryEditExecutionError> {
    let identities = query_incoming_relation_source_identities(
        relation_rows,
        target_query_identity,
        relation_kind,
    )?;
    if identities.len() != 1 {
        return Err(
            WorthTopologyQueryEditExecutionError::ExistingEntityIncomingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected: 1,
                actual: identities.len(),
            },
        );
    }
    Ok(identities[0].clone())
}
