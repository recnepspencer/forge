use forge_query::facade::{ForgeQueryBatchWriteReceipt, ForgeQuerySymbolicTargetReference};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use super::shared::{
    bind_existing_entity_handle, bind_existing_relation_handle, delete_existing_entity_from_graph,
};
use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::{
    TopologyMutationApplicationError, TopologyMutationApplicationRunner,
};
use crate::topology_operators::{
    topology_relation_dependency_path, TopologyDeclaredMutationSequence,
};

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn compose_shell_rehome_program(
        &mut self,
        program: super::super::shell_face_rehome_support::ShellFaceRehomeProgram,
        sequence: &TopologyDeclaredMutationSequence,
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<ForgeQueryBatchWriteReceipt, TopologyMutationApplicationError> {
        let members = sequence.members().collect::<Vec<_>>();
        let retired_shell_binding =
            crate::topology_operators::application::bindings::query_entity_binding(
                bindings,
                program.retired_shell_id,
            )?
            .ok_or(
                TopologyMutationApplicationError::MissingExistingEntityBinding(
                    program.retired_shell_id,
                ),
            )?;
        let region_binding =
            crate::topology_operators::application::bindings::query_entity_binding(
                bindings,
                program.region_id,
            )?
            .ok_or(
                TopologyMutationApplicationError::MissingExistingEntityBinding(program.region_id),
            )?;
        let incoming_region_relation_ids =
            crate::topology_operators::application::bindings::query_incoming_relation_ids(
                bindings,
                &retired_shell_binding.query_identity_label,
                TopologyRelationKind::RegionOwnsShell,
            )?;
        let [region_relation_id] = incoming_region_relation_ids.as_slice() else {
            return Err(
                TopologyMutationApplicationError::ExistingEntityIncomingRelationCountMismatch {
                    entity_id: program.retired_shell_id,
                    relation_kind: TopologyRelationKind::RegionOwnsShell,
                    expected: 1,
                    actual: incoming_region_relation_ids.len(),
                },
            );
        };
        let region_relation_binding =
            crate::topology_operators::application::bindings::query_relation_binding(
                bindings,
                *region_relation_id,
            )?
            .ok_or(
                TopologyMutationApplicationError::MissingExistingRelationBinding(
                    *region_relation_id,
                ),
            )?;
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
        let created_shell_key = program.create_key.clone();
        let retired_shell_identity = retired_shell_binding.query_identity_label.clone();
        let retire_contract = members
            .last()
            .expect("shell rehome program always ends with retire contract");
        let region_handle = bind_existing_relation_handle(
            self,
            *region_relation_id,
            region_relation_binding.query_identity.clone(),
        )?;
        let retired_shell_handle = bind_existing_entity_handle(
            self,
            bindings,
            program.retired_shell_id,
            TopologyEntityKind::Shell,
        )?;
        let mut face_relation_rows = Vec::with_capacity(program.face_ids.len());
        for face_id in &program.face_ids {
            let face_binding =
                crate::topology_operators::application::bindings::query_entity_binding(
                    bindings, *face_id,
                )?
                .ok_or(TopologyMutationApplicationError::MissingExistingEntityBinding(*face_id))?;
            let incoming_face_relation_ids =
                crate::topology_operators::application::bindings::query_incoming_relation_ids(
                    bindings,
                    &face_binding.query_identity_label,
                    TopologyRelationKind::ShellOwnsFace,
                )?;
            let [face_relation_id] = incoming_face_relation_ids.as_slice() else {
                return Err(
                    TopologyMutationApplicationError::ExistingEntityIncomingRelationCountMismatch {
                        entity_id: *face_id,
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
                    TopologyMutationApplicationError::MissingExistingRelationBinding(
                        *face_relation_id,
                    ),
                )?;
            face_relation_rows.push((
                bind_existing_relation_handle(
                    self,
                    *face_relation_id,
                    face_relation_binding.query_identity.clone(),
                )?,
                *face_relation_id,
                face_binding.query_identity_label,
            ));
        }

        let region_rebind_authorities =
            crate::topology_operators::authority_identity::relation_continuity_rebind_authorities(
                *region_relation_id,
            )?;
        let mut face_rebind_authorities = std::collections::BTreeMap::new();
        for (_, face_relation_id, _) in &face_relation_rows {
            face_rebind_authorities.insert(
                *face_relation_id,
                crate::topology_operators::authority_identity::relation_continuity_rebind_authorities(
                    *face_relation_id,
                )
                ?,
            );
        }

        self.workspace
            .compose_graph(|graph| {
                graph.insert_entity(created_shell_key.clone(), "TopologyEntity", |mutation| {
                    mutation
                        .aspect("topology.kind", TopologyEntityKind::Shell.kind_name())
                        .aspect("topology.structure", created_shell_key.clone())
                        .aspect("naming.persistent_name", created_shell_key.clone())
                })?;
                graph.retarget_existing_verified(
                    region_handle.clone(),
                    |verify| {
                        let verify = verify
                            .aspect(
                                "topology.kind",
                                TopologyRelationKind::RegionOwnsShell.kind_name(),
                            )
                            .aspect(
                                "topology.source_identity",
                                region_binding.query_identity_label.clone(),
                            )
                            .aspect("topology.target_identity", retired_shell_identity.clone());
                        if let Some(path) = region_dependency_path {
                            verify.aspect(path, TopologyRelationKind::RegionOwnsShell.kind_name())
                        } else {
                            verify
                        }
                    },
                    |update| {
                        let update = update
                            .continuity_rebind_existing_target(
                                region_rebind_authorities.0.clone(),
                                region_rebind_authorities.1.clone(),
                            )
                            .aspect(
                                "topology.kind",
                                TopologyRelationKind::RegionOwnsShell.kind_name(),
                            )
                            .aspect(
                                "topology.source_identity",
                                region_binding.query_identity_label.clone(),
                            )
                            .symbolic_entity_identity(
                                "topology.target_identity",
                                ForgeQuerySymbolicTargetReference::new(created_shell_key.clone())
                                    .expect("created entity keys are non-empty"),
                            );
                        if let Some(path) = region_dependency_path {
                            update.aspect(path, TopologyRelationKind::RegionOwnsShell.kind_name())
                        } else {
                            update
                        }
                    },
                )?;
                for (face_handle, face_relation_id, face_identity) in &face_relation_rows {
                    graph.retarget_existing_verified(
                        face_handle.clone(),
                        |verify| {
                            let verify = verify
                                .aspect(
                                    "topology.kind",
                                    TopologyRelationKind::ShellOwnsFace.kind_name(),
                                )
                                .aspect("topology.source_identity", retired_shell_identity.clone())
                                .aspect("topology.target_identity", face_identity.clone());
                            if let Some(path) = face_dependency_path {
                                verify.aspect(path, TopologyRelationKind::ShellOwnsFace.kind_name())
                            } else {
                                verify
                            }
                        },
                        |update| {
                            let (prior, successor) =
                                face_rebind_authorities[face_relation_id].clone();
                            let update = update
                                .continuity_rebind_existing_target(prior, successor)
                                .aspect(
                                    "topology.kind",
                                    TopologyRelationKind::ShellOwnsFace.kind_name(),
                                )
                                .symbolic_entity_identity(
                                    "topology.source_identity",
                                    ForgeQuerySymbolicTargetReference::new(
                                        created_shell_key.clone(),
                                    )
                                    .expect("created entity keys are non-empty"),
                                )
                                .aspect("topology.target_identity", face_identity.clone());
                            if let Some(path) = face_dependency_path {
                                update.aspect(path, TopologyRelationKind::ShellOwnsFace.kind_name())
                            } else {
                                update
                            }
                        },
                    )?;
                }
                delete_existing_entity_from_graph(
                    graph,
                    retired_shell_handle.clone(),
                    "TopologyEntity",
                    TopologyEntityKind::Shell.kind_name(),
                    *retire_contract,
                )?;
                Ok(())
            })
            .map_err(TopologyMutationApplicationError::from)
    }
}
