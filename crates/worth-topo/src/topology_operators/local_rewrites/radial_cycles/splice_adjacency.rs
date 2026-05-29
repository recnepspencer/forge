use forge_query::facade::{ForgeQueryExistingRelationTarget, ForgeQueryMutationBatchBuilder};
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::bindings::{
    query_entity_binding, query_relation_binding,
};
use crate::topology_operators::application::{
    TopologyOperatorExecutionError, TopologyOperatorRunner,
};
use crate::topology_operators::local_rewrites::boundary_wiring::adjacency_support::single_outgoing_relation_target_identity;
use crate::topology_operators::topology_relation_dependency_path;

impl<'workspace, 'assembly> TopologyOperatorRunner<'workspace, 'assembly> {
    pub(crate) fn lower_splice_radial_adjacency(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        bindings: &TopologyQueryBindingIndex,
        relation_id: RelationId,
        half_edge_id: EntityId,
        radial_next_half_edge_id: EntityId,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyOperatorExecutionError> {
        let relation_kind = TopologyRelationKind::HalfEdgeRadialNext;
        let relation_binding = query_relation_binding(bindings, relation_id)?
            .ok_or(TopologyOperatorExecutionError::MissingExistingRelationBinding(relation_id))?;
        if relation_binding.kind != relation_kind {
            return Err(
                TopologyOperatorExecutionError::ExistingRelationKindMismatch {
                    relation_id,
                    expected: relation_kind,
                    actual: relation_binding.kind,
                },
            );
        }
        let source_half_edge_binding = query_entity_binding(bindings, half_edge_id)?
            .ok_or(TopologyOperatorExecutionError::MissingExistingEntityBinding(half_edge_id))?;
        if source_half_edge_binding.kind != TopologyEntityKind::HalfEdge {
            return Err(TopologyOperatorExecutionError::ExistingEntityKindMismatch {
                entity_id: half_edge_id,
                expected: TopologyEntityKind::HalfEdge,
                actual: source_half_edge_binding.kind,
            });
        }
        if relation_binding.source_query_identity != source_half_edge_binding.query_identity {
            return Err(
                TopologyOperatorExecutionError::ExistingRelationSourceMismatch {
                    relation_id,
                    expected_source_entity_id: half_edge_id,
                    actual_source_identity: relation_binding.source_query_identity,
                },
            );
        }
        let target_half_edge_binding = query_entity_binding(bindings, radial_next_half_edge_id)?
            .ok_or(
                TopologyOperatorExecutionError::MissingExistingEntityBinding(
                    radial_next_half_edge_id,
                ),
            )?;
        if target_half_edge_binding.kind != TopologyEntityKind::HalfEdge {
            return Err(TopologyOperatorExecutionError::ExistingEntityKindMismatch {
                entity_id: radial_next_half_edge_id,
                expected: TopologyEntityKind::HalfEdge,
                actual: target_half_edge_binding.kind,
            });
        }
        let source_edge_identity = single_outgoing_relation_target_identity(
            bindings,
            half_edge_id,
            &source_half_edge_binding.query_identity,
            TopologyRelationKind::HalfEdgeUsesEdge,
        )?;
        let target_edge_identity = single_outgoing_relation_target_identity(
            bindings,
            radial_next_half_edge_id,
            &target_half_edge_binding.query_identity,
            TopologyRelationKind::HalfEdgeUsesEdge,
        )?;
        if source_edge_identity != target_edge_identity {
            return Err(
                TopologyOperatorExecutionError::ExistingHalfEdgesNotOnSameEdge {
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
            schema::facade::platform::relations::RelationKind::Topology(relation_kind),
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
