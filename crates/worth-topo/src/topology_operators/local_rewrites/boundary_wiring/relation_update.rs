use forge_query::facade::{
    ForgeQueryExistingRelationTarget, ForgeQueryExistingTruthTargetBinding,
    ForgeQueryMutationAuthorityIdentity, ForgeQueryMutationBatchBuilder,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::query_native_runtime_boundary::TopologyNativeQueryRowField;
use crate::topology_operators::application::bindings::{
    query_entity_binding, query_relation_binding,
};
use crate::topology_operators::application::{
    TopologyMutationApplicationError, TopologyMutationApplicationRunner,
};
use crate::topology_operators::authority_identity::{
    continuity_successor_relation_authority, existing_relation_authority,
};
use crate::topology_operators::local_rewrites::boundary_wiring::adjacency_support::single_incoming_relation_source_identity;
use crate::topology_operators::topology_relation_dependency_path;
use crate::topology_operators::{LoopEndpointKind, LoopSuccessorKind};

#[derive(Clone)]
pub(crate) struct ResolvedLoopSuccessorRewire {
    pub(crate) binding: ForgeQueryExistingTruthTargetBinding,
    pub(crate) relation_kind: TopologyRelationKind,
    pub(crate) authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    pub(crate) successor_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    pub(crate) source_query_identity: String,
    pub(crate) current_target_query_identity: String,
    pub(crate) updated_target_query_identity: String,
    pub(crate) dependency_path: Option<schema::facade::QueryAspectPath>,
}

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn resolve_loop_successor_rewire(
        &self,
        bindings: &TopologyQueryBindingIndex,
        relation_id: RelationId,
        kind: LoopSuccessorKind,
        half_edge_id: EntityId,
        successor_half_edge_id: EntityId,
    ) -> Result<ResolvedLoopSuccessorRewire, TopologyMutationApplicationError> {
        let relation_kind = kind.relation_kind();
        let relation_binding = query_relation_binding(bindings, relation_id)?
            .ok_or(TopologyMutationApplicationError::MissingExistingRelationBinding(relation_id))?;
        if relation_binding.kind != relation_kind {
            return Err(
                TopologyMutationApplicationError::ExistingRelationKindMismatch {
                    relation_id,
                    expected: relation_kind,
                    actual: relation_binding.kind,
                },
            );
        }
        let source_half_edge_binding = query_entity_binding(bindings, half_edge_id)?
            .ok_or(TopologyMutationApplicationError::MissingExistingEntityBinding(half_edge_id))?;
        if source_half_edge_binding.kind != TopologyEntityKind::HalfEdge {
            return Err(
                TopologyMutationApplicationError::ExistingEntityKindMismatch {
                    entity_id: half_edge_id,
                    expected: TopologyEntityKind::HalfEdge,
                    actual: source_half_edge_binding.kind,
                },
            );
        }
        if relation_binding.source_query_identity != source_half_edge_binding.query_identity_label {
            return Err(
                TopologyMutationApplicationError::ExistingRelationSourceMismatch {
                    relation_id,
                    expected_source_entity_id: half_edge_id,
                    actual_source_identity: relation_binding.source_query_identity,
                },
            );
        }
        let target_half_edge_binding = query_entity_binding(bindings, successor_half_edge_id)?
            .ok_or(
                TopologyMutationApplicationError::MissingExistingEntityBinding(
                    successor_half_edge_id,
                ),
            )?;
        if target_half_edge_binding.kind != TopologyEntityKind::HalfEdge {
            return Err(
                TopologyMutationApplicationError::ExistingEntityKindMismatch {
                    entity_id: successor_half_edge_id,
                    expected: TopologyEntityKind::HalfEdge,
                    actual: target_half_edge_binding.kind,
                },
            );
        }
        let source_loop_identity = single_incoming_relation_source_identity(
            bindings,
            half_edge_id,
            &source_half_edge_binding.query_identity_label,
            TopologyRelationKind::LoopOwnsHalfEdge,
        )?;
        let target_loop_identity = single_incoming_relation_source_identity(
            bindings,
            successor_half_edge_id,
            &target_half_edge_binding.query_identity_label,
            TopologyRelationKind::LoopOwnsHalfEdge,
        )?;
        if source_loop_identity != target_loop_identity {
            return Err(
                TopologyMutationApplicationError::ExistingHalfEdgesNotOnSameLoop {
                    relation_id,
                    source_half_edge_id: half_edge_id,
                    target_half_edge_id: successor_half_edge_id,
                    source_loop_identity,
                    target_loop_identity,
                },
            );
        }
        let authoritative_identity = existing_relation_authority(relation_id)?;
        let successor_authoritative_identity =
            continuity_successor_relation_authority(relation_id)?;
        let binding = ForgeQueryExistingTruthTargetBinding::from_relation_target(
            ForgeQueryExistingRelationTarget::new(
                authoritative_identity.clone(),
                relation_binding.query_identity,
            )?
            .in_target_collection("TopologyRelation")?,
        )?;
        Ok(ResolvedLoopSuccessorRewire {
            binding,
            relation_kind,
            authoritative_identity,
            successor_authoritative_identity,
            source_query_identity: source_half_edge_binding.query_identity_label,
            current_target_query_identity: relation_binding.target_query_identity,
            updated_target_query_identity: target_half_edge_binding.query_identity_label,
            dependency_path: topology_relation_dependency_path(
                schema::facade::platform::relations::RelationKind::Topology(relation_kind),
            ),
        })
    }

    pub(crate) fn lower_rewire_loop_successor(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        bindings: &TopologyQueryBindingIndex,
        relation_id: RelationId,
        kind: LoopSuccessorKind,
        half_edge_id: EntityId,
        successor_half_edge_id: EntityId,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyMutationApplicationError> {
        let resolved = self.resolve_loop_successor_rewire(
            bindings,
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
                let verify = TopologyNativeQueryRowField::TopologyTargetIdentity.set_on(
                    TopologyNativeQueryRowField::TopologySourceIdentity.set_on(
                        TopologyNativeQueryRowField::TopologyKind
                            .set_on(verify, relation_kind.kind_name()),
                        verified_source_query_identity,
                    ),
                    current_target_query_identity,
                );
                if let Some(field) =
                    dependency_path.and_then(TopologyNativeQueryRowField::from_query_aspect_path)
                {
                    field.set_on(verify, relation_kind.kind_name())
                } else {
                    verify
                }
            },
            |update| {
                let update = TopologyNativeQueryRowField::TopologyTargetIdentity.set_on(
                    TopologyNativeQueryRowField::TopologySourceIdentity.set_on(
                        TopologyNativeQueryRowField::TopologyKind
                            .set_on(update, relation_kind.kind_name()),
                        source_query_identity,
                    ),
                    updated_target_query_identity,
                );
                if let Some(field) =
                    dependency_path.and_then(TopologyNativeQueryRowField::from_query_aspect_path)
                {
                    field.set_on(update, relation_kind.kind_name())
                } else {
                    update
                }
            },
        ))
    }

    pub(crate) fn lower_rewire_loop_endpoint(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        bindings: &TopologyQueryBindingIndex,
        relation_id: RelationId,
        endpoint: LoopEndpointKind,
        half_edge_id: EntityId,
        vertex_id: EntityId,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyMutationApplicationError> {
        let relation_binding = query_relation_binding(bindings, relation_id)?
            .ok_or(TopologyMutationApplicationError::MissingExistingRelationBinding(relation_id))?;
        if relation_binding.kind != endpoint.relation_kind() {
            return Err(
                TopologyMutationApplicationError::ExistingRelationKindMismatch {
                    relation_id,
                    expected: endpoint.relation_kind(),
                    actual: relation_binding.kind,
                },
            );
        }
        let half_edge_binding = query_entity_binding(bindings, half_edge_id)?
            .ok_or(TopologyMutationApplicationError::MissingExistingEntityBinding(half_edge_id))?;
        if half_edge_binding.kind != TopologyEntityKind::HalfEdge {
            return Err(
                TopologyMutationApplicationError::ExistingEntityKindMismatch {
                    entity_id: half_edge_id,
                    expected: TopologyEntityKind::HalfEdge,
                    actual: half_edge_binding.kind,
                },
            );
        }
        if relation_binding.source_query_identity != half_edge_binding.query_identity_label {
            return Err(
                TopologyMutationApplicationError::ExistingRelationSourceMismatch {
                    relation_id,
                    expected_source_entity_id: half_edge_id,
                    actual_source_identity: relation_binding.source_query_identity,
                },
            );
        }
        let vertex_binding = query_entity_binding(bindings, vertex_id)?
            .ok_or(TopologyMutationApplicationError::MissingExistingEntityBinding(vertex_id))?;
        if vertex_binding.kind != TopologyEntityKind::Vertex {
            return Err(
                TopologyMutationApplicationError::ExistingEntityKindMismatch {
                    entity_id: vertex_id,
                    expected: TopologyEntityKind::Vertex,
                    actual: vertex_binding.kind,
                },
            );
        }
        let binding = ForgeQueryExistingTruthTargetBinding::from_relation_target(
            ForgeQueryExistingRelationTarget::new(
                existing_relation_authority(relation_id)?,
                relation_binding.query_identity,
            )?
            .in_target_collection("TopologyRelation")?,
        )?;
        let source_query_identity = half_edge_binding.query_identity_label;
        let verified_source_query_identity = source_query_identity.clone();
        let current_target_query_identity = relation_binding.target_query_identity;
        let updated_target_query_identity = vertex_binding.query_identity_label;
        let dependency_path = topology_relation_dependency_path(
            schema::facade::platform::relations::RelationKind::Topology(endpoint.relation_kind()),
        );
        Ok(builder.update_existing_verified(
            binding,
            |verify| {
                let verify = TopologyNativeQueryRowField::TopologyTargetIdentity.set_on(
                    TopologyNativeQueryRowField::TopologySourceIdentity.set_on(
                        TopologyNativeQueryRowField::TopologyKind
                            .set_on(verify, endpoint.relation_kind().kind_name()),
                        verified_source_query_identity,
                    ),
                    current_target_query_identity,
                );
                if let Some(field) =
                    dependency_path.and_then(TopologyNativeQueryRowField::from_query_aspect_path)
                {
                    field.set_on(verify, endpoint.relation_kind().kind_name())
                } else {
                    verify
                }
            },
            |update| {
                let update = TopologyNativeQueryRowField::TopologyTargetIdentity.set_on(
                    TopologyNativeQueryRowField::TopologySourceIdentity.set_on(
                        TopologyNativeQueryRowField::TopologyKind
                            .set_on(update, endpoint.relation_kind().kind_name()),
                        source_query_identity,
                    ),
                    updated_target_query_identity,
                );
                if let Some(field) =
                    dependency_path.and_then(TopologyNativeQueryRowField::from_query_aspect_path)
                {
                    field.set_on(update, endpoint.relation_kind().kind_name())
                } else {
                    update
                }
            },
        ))
    }
}
