use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryExistingRelationTarget, ForgeQueryExistingTruthTargetBinding,
    ForgeQueryMutationBatchBuilder,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::{TopologyEntityKind, TopologyRelationKind};

use super::bindings::{
    query_entity_binding, query_incoming_relation_source_identities,
    query_outgoing_relation_target_identities, query_relation_binding,
};
use super::{TopologyQueryEditExecutionError, TopologyQueryEditRunner};
use crate::edit::{LoopEndpointKind, LoopSuccessorKind};
use crate::query::topology_relation_dependency_path;

#[derive(Clone)]
pub(super) struct ResolvedLoopSuccessorRewire {
    pub(super) binding: ForgeQueryExistingTruthTargetBinding,
    pub(super) relation_kind: TopologyRelationKind,
    pub(super) authoritative_identity: String,
    pub(super) successor_authoritative_identity: String,
    pub(super) source_query_identity: String,
    pub(super) current_target_query_identity: String,
    pub(super) updated_target_query_identity: String,
    pub(super) dependency_path: Option<&'static str>,
}

impl<'workspace, 'assembly> TopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn resolve_loop_successor_rewire(
        &self,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
        relation_id: RelationId,
        kind: LoopSuccessorKind,
        half_edge_id: EntityId,
        successor_half_edge_id: EntityId,
    ) -> Result<ResolvedLoopSuccessorRewire, TopologyQueryEditExecutionError> {
        let relation_kind = kind.relation_kind();
        let relation_binding = query_relation_binding(relation_rows, relation_id)?
            .ok_or(TopologyQueryEditExecutionError::MissingExistingRelationBinding(relation_id))?;
        if relation_binding.kind != relation_kind {
            return Err(
                TopologyQueryEditExecutionError::ExistingRelationKindMismatch {
                    relation_id,
                    expected: relation_kind,
                    actual: relation_binding.kind,
                },
            );
        }
        let source_half_edge_binding = query_entity_binding(entity_rows, half_edge_id)?
            .ok_or(TopologyQueryEditExecutionError::MissingExistingEntityBinding(half_edge_id))?;
        if source_half_edge_binding.kind != TopologyEntityKind::HalfEdge {
            return Err(
                TopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: half_edge_id,
                    expected: TopologyEntityKind::HalfEdge,
                    actual: source_half_edge_binding.kind,
                },
            );
        }
        if relation_binding.source_query_identity != source_half_edge_binding.query_identity {
            return Err(
                TopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
                    relation_id,
                    expected_source_entity_id: half_edge_id,
                    actual_source_identity: relation_binding.source_query_identity,
                },
            );
        }
        let target_half_edge_binding = query_entity_binding(entity_rows, successor_half_edge_id)?
            .ok_or(
            TopologyQueryEditExecutionError::MissingExistingEntityBinding(successor_half_edge_id),
        )?;
        if target_half_edge_binding.kind != TopologyEntityKind::HalfEdge {
            return Err(
                TopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: successor_half_edge_id,
                    expected: TopologyEntityKind::HalfEdge,
                    actual: target_half_edge_binding.kind,
                },
            );
        }
        let source_loop_identity = single_incoming_relation_source_identity(
            relation_rows,
            half_edge_id,
            &source_half_edge_binding.query_identity,
            TopologyRelationKind::LoopOwnsHalfEdge,
        )?;
        let target_loop_identity = single_incoming_relation_source_identity(
            relation_rows,
            successor_half_edge_id,
            &target_half_edge_binding.query_identity,
            TopologyRelationKind::LoopOwnsHalfEdge,
        )?;
        if source_loop_identity != target_loop_identity {
            return Err(
                TopologyQueryEditExecutionError::ExistingHalfEdgesNotOnSameLoop {
                    relation_id,
                    source_half_edge_id: half_edge_id,
                    target_half_edge_id: successor_half_edge_id,
                    source_loop_identity,
                    target_loop_identity,
                },
            );
        }
        let authoritative_identity = format!("{relation_id:?}");
        let binding = self.workspace.bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                authoritative_identity.clone(),
                relation_binding.query_identity,
            )?
            .in_target_collection("TopologyRelation")?,
        )?;
        Ok(ResolvedLoopSuccessorRewire {
            binding,
            relation_kind,
            authoritative_identity: authoritative_identity.clone(),
            successor_authoritative_identity: format!("{authoritative_identity}:successor"),
            source_query_identity: source_half_edge_binding.query_identity,
            current_target_query_identity: relation_binding.target_query_identity,
            updated_target_query_identity: target_half_edge_binding.query_identity,
            dependency_path: topology_relation_dependency_path(
                schema::facade::RelationKind::Topology(relation_kind),
            ),
        })
    }

    pub(super) fn lower_rewire_loop_successor(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
        relation_id: RelationId,
        kind: LoopSuccessorKind,
        half_edge_id: EntityId,
        successor_half_edge_id: EntityId,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyQueryEditExecutionError> {
        let resolved = self.resolve_loop_successor_rewire(
            entity_rows,
            relation_rows,
            relation_id,
            kind,
            half_edge_id,
            successor_half_edge_id,
        )?;
        let relation_kind = resolved.relation_kind;
        let source_query_identity = resolved.source_query_identity;
        let verified_source_query_identity = source_query_identity.clone();
        let current_target_query_identity = resolved.current_target_query_identity;
        let updated_target_query_identity = resolved.updated_target_query_identity;
        let dependency_path = resolved.dependency_path;
        Ok(builder.update_existing_verified(
            resolved.binding,
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
        endpoint: LoopEndpointKind,
        half_edge_id: EntityId,
        vertex_id: EntityId,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyQueryEditExecutionError> {
        let relation_binding = query_relation_binding(relation_rows, relation_id)?
            .ok_or(TopologyQueryEditExecutionError::MissingExistingRelationBinding(relation_id))?;
        if relation_binding.kind != endpoint.relation_kind() {
            return Err(
                TopologyQueryEditExecutionError::ExistingRelationKindMismatch {
                    relation_id,
                    expected: endpoint.relation_kind(),
                    actual: relation_binding.kind,
                },
            );
        }
        let half_edge_binding = query_entity_binding(entity_rows, half_edge_id)?
            .ok_or(TopologyQueryEditExecutionError::MissingExistingEntityBinding(half_edge_id))?;
        if half_edge_binding.kind != TopologyEntityKind::HalfEdge {
            return Err(
                TopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: half_edge_id,
                    expected: TopologyEntityKind::HalfEdge,
                    actual: half_edge_binding.kind,
                },
            );
        }
        if relation_binding.source_query_identity != half_edge_binding.query_identity {
            return Err(
                TopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
                    relation_id,
                    expected_source_entity_id: half_edge_id,
                    actual_source_identity: relation_binding.source_query_identity,
                },
            );
        }
        let vertex_binding = query_entity_binding(entity_rows, vertex_id)?
            .ok_or(TopologyQueryEditExecutionError::MissingExistingEntityBinding(vertex_id))?;
        if vertex_binding.kind != TopologyEntityKind::Vertex {
            return Err(
                TopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: vertex_id,
                    expected: TopologyEntityKind::Vertex,
                    actual: vertex_binding.kind,
                },
            );
        }
        let binding = self.workspace.bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                format!("{relation_id:?}"),
                relation_binding.query_identity,
            )?
            .in_target_collection("TopologyRelation")?,
        )?;
        let source_query_identity = half_edge_binding.query_identity;
        let verified_source_query_identity = source_query_identity.clone();
        let current_target_query_identity = relation_binding.target_query_identity;
        let updated_target_query_identity = vertex_binding.query_identity;
        let dependency_path = topology_relation_dependency_path(
            schema::facade::RelationKind::Topology(endpoint.relation_kind()),
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
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyQueryEditExecutionError> {
        let relation_kind = TopologyRelationKind::HalfEdgeRadialNext;
        let relation_binding = query_relation_binding(relation_rows, relation_id)?
            .ok_or(TopologyQueryEditExecutionError::MissingExistingRelationBinding(relation_id))?;
        if relation_binding.kind != relation_kind {
            return Err(
                TopologyQueryEditExecutionError::ExistingRelationKindMismatch {
                    relation_id,
                    expected: relation_kind,
                    actual: relation_binding.kind,
                },
            );
        }
        let source_half_edge_binding = query_entity_binding(entity_rows, half_edge_id)?
            .ok_or(TopologyQueryEditExecutionError::MissingExistingEntityBinding(half_edge_id))?;
        if source_half_edge_binding.kind != TopologyEntityKind::HalfEdge {
            return Err(
                TopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: half_edge_id,
                    expected: TopologyEntityKind::HalfEdge,
                    actual: source_half_edge_binding.kind,
                },
            );
        }
        if relation_binding.source_query_identity != source_half_edge_binding.query_identity {
            return Err(
                TopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
                    relation_id,
                    expected_source_entity_id: half_edge_id,
                    actual_source_identity: relation_binding.source_query_identity,
                },
            );
        }
        let target_half_edge_binding = query_entity_binding(entity_rows, radial_next_half_edge_id)?
            .ok_or(
                TopologyQueryEditExecutionError::MissingExistingEntityBinding(
                    radial_next_half_edge_id,
                ),
            )?;
        if target_half_edge_binding.kind != TopologyEntityKind::HalfEdge {
            return Err(
                TopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: radial_next_half_edge_id,
                    expected: TopologyEntityKind::HalfEdge,
                    actual: target_half_edge_binding.kind,
                },
            );
        }
        let source_edge_identity = single_outgoing_relation_target_identity(
            relation_rows,
            half_edge_id,
            &source_half_edge_binding.query_identity,
            TopologyRelationKind::HalfEdgeUsesEdge,
        )?;
        let target_edge_identity = single_outgoing_relation_target_identity(
            relation_rows,
            radial_next_half_edge_id,
            &target_half_edge_binding.query_identity,
            TopologyRelationKind::HalfEdgeUsesEdge,
        )?;
        if source_edge_identity != target_edge_identity {
            return Err(
                TopologyQueryEditExecutionError::ExistingHalfEdgesNotOnSameEdge {
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
            .in_target_collection("TopologyRelation")?,
        )?;
        let source_query_identity = source_half_edge_binding.query_identity;
        let verified_source_query_identity = source_query_identity.clone();
        let current_target_query_identity = relation_binding.target_query_identity;
        let updated_target_query_identity = target_half_edge_binding.query_identity;
        let dependency_path = topology_relation_dependency_path(
            schema::facade::RelationKind::Topology(relation_kind),
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
    relation_kind: TopologyRelationKind,
) -> Result<String, TopologyQueryEditExecutionError> {
    let identities = query_outgoing_relation_target_identities(
        relation_rows,
        source_query_identity,
        relation_kind,
    )?;
    if identities.len() != 1 {
        return Err(
            TopologyQueryEditExecutionError::ExistingEntityOutgoingRelationCountMismatch {
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
    relation_kind: TopologyRelationKind,
) -> Result<String, TopologyQueryEditExecutionError> {
    let identities = query_incoming_relation_source_identities(
        relation_rows,
        target_query_identity,
        relation_kind,
    )?;
    if identities.len() != 1 {
        return Err(
            TopologyQueryEditExecutionError::ExistingEntityIncomingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected: 1,
                actual: identities.len(),
            },
        );
    }
    Ok(identities[0].clone())
}
