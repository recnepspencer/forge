use forge_query::facade::{ForgeQueryBatchWriteReceipt, ForgeQuerySymbolicTargetReference};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use super::shared::bind_existing_relation_handle;
use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::{
    TopologyMutationApplicationError, TopologyMutationApplicationRunner,
};
use crate::topology_operators::{
    continuity_prior_relation_authority, continuity_successor_relation_authority,
    topology_relation_dependency_path,
};

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn compose_shell_split_program(
        &mut self,
        program: super::super::shell_face_rehome_support::ShellFaceSplitProgram,
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<ForgeQueryBatchWriteReceipt, TopologyMutationApplicationError> {
        let retained_shell_id = program
            .retained_shell_id
            .expect("resolved shell split program always sets retained shell id");
        let region_binding =
            crate::topology_operators::application::bindings::query_entity_binding(
                bindings,
                program.region_id,
            )?
            .ok_or(
                TopologyMutationApplicationError::MissingExistingEntityBinding(program.region_id),
            )?;
        let face_binding = crate::topology_operators::application::bindings::query_entity_binding(
            bindings,
            program.face_id,
        )?
        .ok_or(TopologyMutationApplicationError::MissingExistingEntityBinding(program.face_id))?;
        let incoming_face_relation_ids =
            crate::topology_operators::application::bindings::query_incoming_relation_ids(
                bindings,
                &face_binding.query_identity_label,
                TopologyRelationKind::ShellOwnsFace,
            )?;
        let [face_relation_id] = incoming_face_relation_ids.as_slice() else {
            return Err(
                TopologyMutationApplicationError::ExistingEntityIncomingRelationCountMismatch {
                    entity_id: program.face_id,
                    relation_kind: TopologyRelationKind::ShellOwnsFace,
                    expected: 1,
                    actual: incoming_face_relation_ids.len(),
                },
            );
        };
        let face_relation_binding =
            crate::topology_operators::application::bindings::query_relation_binding(
                bindings,
                *face_relation_id,
            )?
            .ok_or(
                TopologyMutationApplicationError::MissingExistingRelationBinding(*face_relation_id),
            )?;
        let retained_shell_binding =
            crate::topology_operators::application::bindings::query_entity_binding(
                bindings,
                retained_shell_id,
            )?
            .ok_or(
                TopologyMutationApplicationError::MissingExistingEntityBinding(retained_shell_id),
            )?;
        let created_shell_key = program.create_key.clone();
        let face_dependency_path = topology_relation_dependency_path(
            schema::facade::platform::relations::RelationKind::Topology(
                TopologyRelationKind::ShellOwnsFace,
            ),
        );
        let region_dependency_path = topology_relation_dependency_path(
            schema::facade::platform::relations::RelationKind::Topology(
                TopologyRelationKind::RegionOwnsShell,
            ),
        );
        let face_handle = bind_existing_relation_handle(
            self,
            *face_relation_id,
            face_relation_binding.query_identity.clone(),
        )?;
        self.workspace
            .compose_graph(|graph| {
                let shell_symbol = graph.insert_entity(
                    created_shell_key.clone(),
                    "TopologyEntity",
                    |mutation| {
                        mutation
                            .aspect("topology.kind", TopologyEntityKind::Shell.kind_name())
                            .aspect("topology.structure", created_shell_key.clone())
                            .aspect("naming.persistent_name", created_shell_key.clone())
                    },
                )?;
                graph.insert_relation("TopologyRelation", |relation| {
                    let relation = relation
                        .aspect(
                            "topology.kind",
                            TopologyRelationKind::RegionOwnsShell.kind_name(),
                        )
                        .existing_entity_identity(
                            "topology.source_identity",
                            region_binding.query_identity.clone(),
                        )
                        .symbolic_entity_identity("topology.target_identity", &shell_symbol);
                    if let Some(path) = region_dependency_path {
                        relation.aspect(path, TopologyRelationKind::RegionOwnsShell.kind_name())
                    } else {
                        relation
                    }
                })?;
                let face_prior_authority = continuity_prior_relation_authority(*face_relation_id)?;
                let face_successor_authority =
                    continuity_successor_relation_authority(*face_relation_id)?;
                graph.retarget_existing_verified(
                    face_handle.clone(),
                    |verify| {
                        let verify = verify
                            .aspect(
                                "topology.kind",
                                TopologyRelationKind::ShellOwnsFace.kind_name(),
                            )
                            .aspect(
                                "topology.source_identity",
                                retained_shell_binding.query_identity_label.clone(),
                            )
                            .aspect(
                                "topology.target_identity",
                                face_binding.query_identity_label.clone(),
                            );
                        if let Some(path) = face_dependency_path {
                            verify.aspect(path, TopologyRelationKind::ShellOwnsFace.kind_name())
                        } else {
                            verify
                        }
                    },
                    |update| {
                        let update = update
                            .continuity_rebind_existing_target(
                                face_prior_authority,
                                face_successor_authority,
                            )
                            .aspect(
                                "topology.kind",
                                TopologyRelationKind::ShellOwnsFace.kind_name(),
                            )
                            .symbolic_entity_identity(
                                "topology.source_identity",
                                ForgeQuerySymbolicTargetReference::new(created_shell_key.clone())
                                    .expect("created entity keys are non-empty"),
                            )
                            .aspect(
                                "topology.target_identity",
                                face_binding.query_identity_label.clone(),
                            );
                        if let Some(path) = face_dependency_path {
                            update.aspect(path, TopologyRelationKind::ShellOwnsFace.kind_name())
                        } else {
                            update
                        }
                    },
                )?;
                Ok(())
            })
            .map_err(Into::into)
    }
}
