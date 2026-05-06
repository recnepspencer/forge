use forge_query::facade::{ForgeQueryBatchWriteReceipt, ForgeQueryEntity};
use worth_schema::facade::{
    WorthEntityReference, WorthTopologyEntityKind, WorthTopologyRelationKind,
};

use super::{WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner};
use crate::edit::{
    WorthBoundaryMembershipKind, WorthTopologyEditAction, WorthTopologyEditContract,
    WorthTopologyEditFamily,
};
use crate::query::topology_relation_dependency_path;

impl<'workspace, 'assembly> WorthTopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn compose_face_inner_loop_workflow(
        &mut self,
        contracts: &[WorthTopologyEditContract],
        entity_rows: &[ForgeQueryEntity],
    ) -> Result<ForgeQueryBatchWriteReceipt, WorthTopologyQueryEditExecutionError> {
        let [create, attach] = contracts else {
            return Err(WorthTopologyQueryEditExecutionError::UnsupportedFamilies(
                vec![WorthTopologyEditFamily::AttachBoundaryMembership],
            ));
        };
        let (
            WorthTopologyEditAction::CreateTopologyEntity {
                create_key,
                kind: WorthTopologyEntityKind::Loop,
                ..
            },
            WorthTopologyEditAction::AttachBoundaryMembership {
                kind: WorthBoundaryMembershipKind::FaceInnerLoop,
                owner: WorthEntityReference::Existing(face_id),
                member: WorthEntityReference::Created(member_key),
                ..
            },
        ) = (&create.action, &attach.action)
        else {
            return Err(WorthTopologyQueryEditExecutionError::UnsupportedFamilies(
                vec![WorthTopologyEditFamily::AttachBoundaryMembership],
            ));
        };
        if create_key.as_str() != member_key.as_str() {
            return Err(WorthTopologyQueryEditExecutionError::UnsupportedFamilies(
                vec![WorthTopologyEditFamily::AttachBoundaryMembership],
            ));
        }
        let face_binding = super::super::bindings::query_entity_binding(entity_rows, *face_id)?
            .ok_or(WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(*face_id))?;
        if face_binding.kind != WorthTopologyEntityKind::Face {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: *face_id,
                    expected: WorthTopologyEntityKind::Face,
                    actual: face_binding.kind,
                },
            );
        }
        let create_key = create_key.as_str().to_string();
        self.workspace
            .compose_graph(|graph| {
                let loop_symbol =
                    graph.insert_entity(create_key.clone(), "WorthTopologyEntity", |mutation| {
                        mutation
                            .aspect("topology.kind", WorthTopologyEntityKind::Loop.kind_name())
                            .aspect("topology.structure", create_key.clone())
                            .aspect("naming.persistent_name", create_key.clone())
                    })?;
                graph.insert_relation("WorthTopologyRelation", |relation| {
                    let relation = relation
                        .aspect(
                            "topology.kind",
                            WorthTopologyRelationKind::FaceInnerLoop.kind_name(),
                        )
                        .existing_entity_identity(
                            "topology.source_identity",
                            face_binding.query_identity.clone(),
                        )
                        .symbolic_entity_identity("topology.target_identity", &loop_symbol);
                    if let Some(path) = topology_relation_dependency_path(
                        worth_schema::facade::WorthRelationKind::Topology(
                            WorthTopologyRelationKind::FaceInnerLoop,
                        ),
                    ) {
                        relation.aspect(path, WorthTopologyRelationKind::FaceInnerLoop.kind_name())
                    } else {
                        relation
                    }
                })?;
                Ok(())
            })
            .map_err(Into::into)
    }
}
