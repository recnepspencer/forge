use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryEntity, ForgeQuerySymbolicTargetReference,
};
use worth_schema::facade::{WorthTopologyEntityKind, WorthTopologyRelationKind};

use super::shared::{
    bind_existing_entity_handle, bind_existing_relation_handle, delete_existing_entity_from_graph,
};
use super::{WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner};
use crate::edit::WorthTopologyEditContract;
use crate::query::topology_relation_dependency_path;

impl<'workspace, 'assembly> WorthTopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn compose_shell_rehome_workflow(
        &mut self,
        workflow: super::super::relation_shell_face_rehome_support::ShellFaceRehomeWorkflow,
        contracts: &[WorthTopologyEditContract],
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
    ) -> Result<ForgeQueryBatchWriteReceipt, WorthTopologyQueryEditExecutionError> {
        let retired_shell_binding =
            super::super::bindings::query_entity_binding(entity_rows, workflow.retired_shell_id)?
                .ok_or(
                WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(
                    workflow.retired_shell_id,
                ),
            )?;
        let region_binding =
            super::super::bindings::query_entity_binding(entity_rows, workflow.region_id)?.ok_or(
                WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(
                    workflow.region_id,
                ),
            )?;
        let incoming_region_relation_ids = super::super::bindings::query_incoming_relation_ids(
            relation_rows,
            &retired_shell_binding.query_identity,
            WorthTopologyRelationKind::RegionOwnsShell,
        )?;
        let [region_relation_id] = incoming_region_relation_ids.as_slice() else {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityIncomingRelationCountMismatch {
                    entity_id: workflow.retired_shell_id,
                    relation_kind: WorthTopologyRelationKind::RegionOwnsShell,
                    expected: 1,
                    actual: incoming_region_relation_ids.len(),
                },
            );
        };
        let region_relation_binding =
            super::super::bindings::query_relation_binding(relation_rows, *region_relation_id)?
                .ok_or(
                    WorthTopologyQueryEditExecutionError::MissingExistingRelationBinding(
                        *region_relation_id,
                    ),
                )?;
        let face_dependency_path =
            topology_relation_dependency_path(worth_schema::facade::WorthRelationKind::Topology(
                WorthTopologyRelationKind::ShellOwnsFace,
            ));
        let region_dependency_path =
            topology_relation_dependency_path(worth_schema::facade::WorthRelationKind::Topology(
                WorthTopologyRelationKind::RegionOwnsShell,
            ));
        let created_shell_key = workflow.create_key.clone();
        let retired_shell_identity = retired_shell_binding.query_identity.clone();
        let retire_contract = contracts
            .last()
            .expect("shell rehome workflow always ends with retire contract");
        let region_handle = bind_existing_relation_handle(
            self,
            *region_relation_id,
            &region_relation_binding.query_identity,
        )?;
        let retired_shell_handle = bind_existing_entity_handle(
            self,
            entity_rows,
            workflow.retired_shell_id,
            WorthTopologyEntityKind::Shell,
        )?;
        let mut face_relation_rows = Vec::with_capacity(workflow.face_ids.len());
        for face_id in &workflow.face_ids {
            let face_binding = super::super::bindings::query_entity_binding(entity_rows, *face_id)?
                .ok_or(
                    WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(*face_id),
                )?;
            let incoming_face_relation_ids = super::super::bindings::query_incoming_relation_ids(
                relation_rows,
                &face_binding.query_identity,
                WorthTopologyRelationKind::ShellOwnsFace,
            )?;
            let [face_relation_id] = incoming_face_relation_ids.as_slice() else {
                return Err(
                    WorthTopologyQueryEditExecutionError::ExistingEntityIncomingRelationCountMismatch {
                        entity_id: *face_id,
                        relation_kind: WorthTopologyRelationKind::ShellOwnsFace,
                        expected: 1,
                        actual: incoming_face_relation_ids.len(),
                    },
                );
            };
            let face_relation_binding =
                super::super::bindings::query_relation_binding(relation_rows, *face_relation_id)?
                    .ok_or(
                    WorthTopologyQueryEditExecutionError::MissingExistingRelationBinding(
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
                graph.insert_entity(
                    created_shell_key.clone(),
                    "WorthTopologyEntity",
                    |mutation| {
                        mutation
                            .aspect("topology.kind", WorthTopologyEntityKind::Shell.kind_name())
                            .aspect("topology.structure", created_shell_key.clone())
                            .aspect("naming.persistent_name", created_shell_key.clone())
                    },
                )?;
                graph.retarget_existing_verified(
                    region_handle.clone(),
                    |verify| {
                        let verify = verify
                            .aspect(
                                "topology.kind",
                                WorthTopologyRelationKind::RegionOwnsShell.kind_name(),
                            )
                            .aspect(
                                "topology.source_identity",
                                region_binding.query_identity.clone(),
                            )
                            .aspect("topology.target_identity", retired_shell_identity.clone());
                        if let Some(path) = region_dependency_path {
                            verify.aspect(
                                path,
                                WorthTopologyRelationKind::RegionOwnsShell.kind_name(),
                            )
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
                                WorthTopologyRelationKind::RegionOwnsShell.kind_name(),
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
                            update.aspect(
                                path,
                                WorthTopologyRelationKind::RegionOwnsShell.kind_name(),
                            )
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
                                    WorthTopologyRelationKind::ShellOwnsFace.kind_name(),
                                )
                                .aspect("topology.source_identity", retired_shell_identity.clone())
                                .aspect("topology.target_identity", face_identity.clone());
                            if let Some(path) = face_dependency_path {
                                verify.aspect(
                                    path,
                                    WorthTopologyRelationKind::ShellOwnsFace.kind_name(),
                                )
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
                                    WorthTopologyRelationKind::ShellOwnsFace.kind_name(),
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
                                update.aspect(
                                    path,
                                    WorthTopologyRelationKind::ShellOwnsFace.kind_name(),
                                )
                            } else {
                                update
                            }
                        },
                    )?;
                }
                delete_existing_entity_from_graph(
                    graph,
                    retired_shell_handle.clone(),
                    "WorthTopologyEntity",
                    WorthTopologyEntityKind::Shell.kind_name(),
                    retire_contract,
                )?;
                Ok(())
            })
            .map_err(Into::into)
    }

    pub(super) fn compose_shell_split_workflow(
        &mut self,
        workflow: super::super::relation_shell_face_rehome_support::ShellFaceSplitWorkflow,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
    ) -> Result<ForgeQueryBatchWriteReceipt, WorthTopologyQueryEditExecutionError> {
        let retained_shell_id = workflow
            .retained_shell_id
            .expect("resolved shell split workflow always sets retained shell id");
        let region_binding =
            super::super::bindings::query_entity_binding(entity_rows, workflow.region_id)?.ok_or(
                WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(
                    workflow.region_id,
                ),
            )?;
        let face_binding =
            super::super::bindings::query_entity_binding(entity_rows, workflow.face_id)?.ok_or(
                WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(
                    workflow.face_id,
                ),
            )?;
        let incoming_face_relation_ids = super::super::bindings::query_incoming_relation_ids(
            relation_rows,
            &face_binding.query_identity,
            WorthTopologyRelationKind::ShellOwnsFace,
        )?;
        let [face_relation_id] = incoming_face_relation_ids.as_slice() else {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityIncomingRelationCountMismatch {
                    entity_id: workflow.face_id,
                    relation_kind: WorthTopologyRelationKind::ShellOwnsFace,
                    expected: 1,
                    actual: incoming_face_relation_ids.len(),
                },
            );
        };
        let face_relation_binding =
            super::super::bindings::query_relation_binding(relation_rows, *face_relation_id)?
                .ok_or(
                    WorthTopologyQueryEditExecutionError::MissingExistingRelationBinding(
                        *face_relation_id,
                    ),
                )?;
        let retained_shell_binding =
            super::super::bindings::query_entity_binding(entity_rows, retained_shell_id)?.ok_or(
                WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(
                    retained_shell_id,
                ),
            )?;
        let created_shell_key = workflow.create_key.clone();
        let face_dependency_path =
            topology_relation_dependency_path(worth_schema::facade::WorthRelationKind::Topology(
                WorthTopologyRelationKind::ShellOwnsFace,
            ));
        let region_dependency_path =
            topology_relation_dependency_path(worth_schema::facade::WorthRelationKind::Topology(
                WorthTopologyRelationKind::RegionOwnsShell,
            ));
        let face_handle = bind_existing_relation_handle(
            self,
            *face_relation_id,
            &face_relation_binding.query_identity,
        )?;

        self.workspace
            .compose_graph(|graph| {
                let shell_symbol = graph.insert_entity(
                    created_shell_key.clone(),
                    "WorthTopologyEntity",
                    |mutation| {
                        mutation
                            .aspect("topology.kind", WorthTopologyEntityKind::Shell.kind_name())
                            .aspect("topology.structure", created_shell_key.clone())
                            .aspect("naming.persistent_name", created_shell_key.clone())
                    },
                )?;
                graph.insert_relation("WorthTopologyRelation", |relation| {
                    let relation = relation
                        .aspect(
                            "topology.kind",
                            WorthTopologyRelationKind::RegionOwnsShell.kind_name(),
                        )
                        .existing_entity_identity(
                            "topology.source_identity",
                            region_binding.query_identity.clone(),
                        )
                        .symbolic_entity_identity("topology.target_identity", &shell_symbol);
                    if let Some(path) = region_dependency_path {
                        relation
                            .aspect(path, WorthTopologyRelationKind::RegionOwnsShell.kind_name())
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
                                WorthTopologyRelationKind::ShellOwnsFace.kind_name(),
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
                            verify
                                .aspect(path, WorthTopologyRelationKind::ShellOwnsFace.kind_name())
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
                                WorthTopologyRelationKind::ShellOwnsFace.kind_name(),
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
                            update
                                .aspect(path, WorthTopologyRelationKind::ShellOwnsFace.kind_name())
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
