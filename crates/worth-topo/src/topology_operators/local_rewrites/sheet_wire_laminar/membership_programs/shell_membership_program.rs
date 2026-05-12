use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryEntity, ForgeQuerySymbolicTargetReference,
};
use schema::facade::{TopologyEntityKind, TopologyRelationKind};

use super::shared::{
    bind_existing_entity_handle, bind_existing_relation_handle, delete_existing_entity_from_graph,
};
use crate::topology_operators::application::{
    TopologyOperatorExecutionError, TopologyOperatorRunner,
};
use crate::topology_operators::topology_relation_dependency_path;
use crate::topology_operators::TopologyEditContract;

impl<'workspace, 'assembly> TopologyOperatorRunner<'workspace, 'assembly> {
    pub(super) fn compose_shell_rehome_program(
        &mut self,
        program: super::super::shell_face_rehome_support::ShellFaceRehomeProgram,
        contracts: &[TopologyEditContract],
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
    ) -> Result<ForgeQueryBatchWriteReceipt, TopologyOperatorExecutionError> {
        let retired_shell_binding =
            crate::topology_operators::application::bindings::query_entity_binding(
                entity_rows,
                program.retired_shell_id,
            )?
            .ok_or(
                TopologyOperatorExecutionError::MissingExistingEntityBinding(
                    program.retired_shell_id,
                ),
            )?;
        let region_binding =
            crate::topology_operators::application::bindings::query_entity_binding(
                entity_rows,
                program.region_id,
            )?
            .ok_or(
                TopologyOperatorExecutionError::MissingExistingEntityBinding(program.region_id),
            )?;
        let incoming_region_relation_ids =
            crate::topology_operators::application::bindings::query_incoming_relation_ids(
                relation_rows,
                &retired_shell_binding.query_identity,
                TopologyRelationKind::RegionOwnsShell,
            )?;
        let [region_relation_id] = incoming_region_relation_ids.as_slice() else {
            return Err(
                TopologyOperatorExecutionError::ExistingEntityIncomingRelationCountMismatch {
                    entity_id: program.retired_shell_id,
                    relation_kind: TopologyRelationKind::RegionOwnsShell,
                    expected: 1,
                    actual: incoming_region_relation_ids.len(),
                },
            );
        };
        let region_relation_binding =
            crate::topology_operators::application::bindings::query_relation_binding(
                relation_rows,
                *region_relation_id,
            )?
            .ok_or(
                TopologyOperatorExecutionError::MissingExistingRelationBinding(*region_relation_id),
            )?;
        let face_dependency_path = topology_relation_dependency_path(
            schema::facade::RelationKind::Topology(TopologyRelationKind::ShellOwnsFace),
        );
        let region_dependency_path = topology_relation_dependency_path(
            schema::facade::RelationKind::Topology(TopologyRelationKind::RegionOwnsShell),
        );
        let created_shell_key = program.create_key.clone();
        let retired_shell_identity = retired_shell_binding.query_identity.clone();
        let retire_contract = contracts
            .last()
            .expect("shell rehome program always ends with retire contract");
        let region_handle = bind_existing_relation_handle(
            self,
            *region_relation_id,
            &region_relation_binding.query_identity,
        )?;
        let retired_shell_handle = bind_existing_entity_handle(
            self,
            entity_rows,
            program.retired_shell_id,
            TopologyEntityKind::Shell,
        )?;
        let mut face_relation_rows = Vec::with_capacity(program.face_ids.len());
        for face_id in &program.face_ids {
            let face_binding =
                crate::topology_operators::application::bindings::query_entity_binding(
                    entity_rows,
                    *face_id,
                )?
                .ok_or(TopologyOperatorExecutionError::MissingExistingEntityBinding(*face_id))?;
            let incoming_face_relation_ids =
                crate::topology_operators::application::bindings::query_incoming_relation_ids(
                    relation_rows,
                    &face_binding.query_identity,
                    TopologyRelationKind::ShellOwnsFace,
                )?;
            let [face_relation_id] = incoming_face_relation_ids.as_slice() else {
                return Err(
                    TopologyOperatorExecutionError::ExistingEntityIncomingRelationCountMismatch {
                        entity_id: *face_id,
                        relation_kind: TopologyRelationKind::ShellOwnsFace,
                        expected: 1,
                        actual: incoming_face_relation_ids.len(),
                    },
                );
            };
            let face_relation_binding =
                crate::topology_operators::application::bindings::query_relation_binding(
                    relation_rows,
                    *face_relation_id,
                )?
                .ok_or(
                    TopologyOperatorExecutionError::MissingExistingRelationBinding(
                        *face_relation_id,
                    ),
                )?;
            face_relation_rows.push((
                bind_existing_relation_handle(
                    self,
                    *face_relation_id,
                    &face_relation_binding.query_identity,
                )?,
                *face_relation_id,
                face_binding.query_identity,
            ));
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
                                region_binding.query_identity.clone(),
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
                                format!("{region_relation_id:?}"),
                                format!("{region_relation_id:?}:successor"),
                            )
                            .aspect(
                                "topology.kind",
                                TopologyRelationKind::RegionOwnsShell.kind_name(),
                            )
                            .aspect(
                                "topology.source_identity",
                                region_binding.query_identity.clone(),
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
                            let update = update
                                .continuity_rebind_existing_target(
                                    format!("{face_relation_id:?}"),
                                    format!("{face_relation_id:?}:successor"),
                                )
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
                    retire_contract,
                )?;
                Ok(())
            })
            .map_err(Into::into)
    }

    pub(super) fn compose_shell_split_program(
        &mut self,
        program: super::super::shell_face_rehome_support::ShellFaceSplitProgram,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
    ) -> Result<ForgeQueryBatchWriteReceipt, TopologyOperatorExecutionError> {
        let retained_shell_id = program
            .retained_shell_id
            .expect("resolved shell split program always sets retained shell id");
        let region_binding =
            crate::topology_operators::application::bindings::query_entity_binding(
                entity_rows,
                program.region_id,
            )?
            .ok_or(
                TopologyOperatorExecutionError::MissingExistingEntityBinding(program.region_id),
            )?;
        let face_binding = crate::topology_operators::application::bindings::query_entity_binding(
            entity_rows,
            program.face_id,
        )?
        .ok_or(TopologyOperatorExecutionError::MissingExistingEntityBinding(program.face_id))?;
        let incoming_face_relation_ids =
            crate::topology_operators::application::bindings::query_incoming_relation_ids(
                relation_rows,
                &face_binding.query_identity,
                TopologyRelationKind::ShellOwnsFace,
            )?;
        let [face_relation_id] = incoming_face_relation_ids.as_slice() else {
            return Err(
                TopologyOperatorExecutionError::ExistingEntityIncomingRelationCountMismatch {
                    entity_id: program.face_id,
                    relation_kind: TopologyRelationKind::ShellOwnsFace,
                    expected: 1,
                    actual: incoming_face_relation_ids.len(),
                },
            );
        };
        let face_relation_binding =
            crate::topology_operators::application::bindings::query_relation_binding(
                relation_rows,
                *face_relation_id,
            )?
            .ok_or(
                TopologyOperatorExecutionError::MissingExistingRelationBinding(*face_relation_id),
            )?;
        let retained_shell_binding =
            crate::topology_operators::application::bindings::query_entity_binding(
                entity_rows,
                retained_shell_id,
            )?
            .ok_or(
                TopologyOperatorExecutionError::MissingExistingEntityBinding(retained_shell_id),
            )?;
        let created_shell_key = program.create_key.clone();
        let face_dependency_path = topology_relation_dependency_path(
            schema::facade::RelationKind::Topology(TopologyRelationKind::ShellOwnsFace),
        );
        let region_dependency_path = topology_relation_dependency_path(
            schema::facade::RelationKind::Topology(TopologyRelationKind::RegionOwnsShell),
        );
        let face_handle = bind_existing_relation_handle(
            self,
            *face_relation_id,
            &face_relation_binding.query_identity,
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
                                retained_shell_binding.query_identity.clone(),
                            )
                            .aspect(
                                "topology.target_identity",
                                face_binding.query_identity.clone(),
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
                                format!("{face_relation_id:?}"),
                                format!("{face_relation_id:?}:successor"),
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
                                face_binding.query_identity.clone(),
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
