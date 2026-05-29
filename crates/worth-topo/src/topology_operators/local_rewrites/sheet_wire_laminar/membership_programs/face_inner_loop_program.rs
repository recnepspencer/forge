use forge_query::facade::ForgeQueryBatchWriteReceipt;
use schema::facade::platform::authority::EntityReference;
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::{
    TopologyOperatorExecutionError, TopologyOperatorRunner,
};
use crate::topology_operators::topology_relation_dependency_path;
use crate::topology_operators::{
    BoundaryMembershipKind, TopologyEditAction, TopologyEditContract, TopologyEditFamily,
};

impl<'workspace, 'assembly> TopologyOperatorRunner<'workspace, 'assembly> {
    pub(super) fn compose_face_inner_loop_program(
        &mut self,
        contracts: &[TopologyEditContract],
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<ForgeQueryBatchWriteReceipt, TopologyOperatorExecutionError> {
        let [create, attach] = contracts else {
            return Err(TopologyOperatorExecutionError::UnsupportedFamilies(vec![
                TopologyEditFamily::AttachBoundaryMembership,
            ]));
        };
        let (
            TopologyEditAction::CreateTopologyEntity {
                create_key,
                kind: TopologyEntityKind::Loop,
                ..
            },
            TopologyEditAction::AttachBoundaryMembership {
                kind: BoundaryMembershipKind::FaceInnerLoop,
                owner: EntityReference::Existing(face_id),
                member: EntityReference::Created(member_key),
                ..
            },
        ) = (&create.action, &attach.action)
        else {
            return Err(TopologyOperatorExecutionError::UnsupportedFamilies(vec![
                TopologyEditFamily::AttachBoundaryMembership,
            ]));
        };
        if create_key.as_str() != member_key.as_str() {
            return Err(TopologyOperatorExecutionError::UnsupportedFamilies(vec![
                TopologyEditFamily::AttachBoundaryMembership,
            ]));
        }
        let face_binding = crate::topology_operators::application::bindings::query_entity_binding(
            bindings, *face_id,
        )?
        .ok_or(TopologyOperatorExecutionError::MissingExistingEntityBinding(*face_id))?;
        if face_binding.kind != TopologyEntityKind::Face {
            return Err(TopologyOperatorExecutionError::ExistingEntityKindMismatch {
                entity_id: *face_id,
                expected: TopologyEntityKind::Face,
                actual: face_binding.kind,
            });
        }
        let create_key = create_key.as_str().to_string();
        self.workspace
            .compose_graph(|graph| {
                let loop_symbol =
                    graph.insert_entity(create_key.clone(), "TopologyEntity", |mutation| {
                        mutation
                            .aspect("topology.kind", TopologyEntityKind::Loop.kind_name())
                            .aspect("topology.structure", create_key.clone())
                            .aspect("naming.persistent_name", create_key.clone())
                    })?;
                graph.insert_relation("TopologyRelation", |relation| {
                    let relation = relation
                        .aspect(
                            "topology.kind",
                            TopologyRelationKind::FaceInnerLoop.kind_name(),
                        )
                        .existing_entity_identity(
                            "topology.source_identity",
                            face_binding.query_identity.clone(),
                        )
                        .symbolic_entity_identity("topology.target_identity", &loop_symbol);
                    if let Some(path) = topology_relation_dependency_path(
                        schema::facade::platform::relations::RelationKind::Topology(TopologyRelationKind::FaceInnerLoop),
                    ) {
                        relation.aspect(path, TopologyRelationKind::FaceInnerLoop.kind_name())
                    } else {
                        relation
                    }
                })?;
                Ok(())
            })
            .map_err(Into::into)
    }
}




