use std::collections::{BTreeMap, BTreeSet};

use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryExistingRelationTarget, ForgeQueryMutationBatchBuilder,
    ForgeQuerySymbolicTargetReference,
};
use worth_schema::facade::WorthTopologyEntityKind;

use super::bindings::{
    query_entity_binding, query_incoming_relation_ids, query_outgoing_relation_ids,
};
use super::relation_shell_face_rehome_support::parse_shell_face_rehome_workflow;
use super::{WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner};
use crate::edit::WorthTopologyEditContract;
use crate::query::topology_relation_dependency_path;

impl<'workspace, 'assembly> WorthTopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn lower_rehome_owned_face_set_to_new_shell_workflow(
        &self,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
        contracts: &[WorthTopologyEditContract],
    ) -> Result<ForgeQueryMutationBatchBuilder, WorthTopologyQueryEditExecutionError> {
        let Some(workflow) = parse_shell_face_rehome_workflow(contracts) else {
            return Err(WorthTopologyQueryEditExecutionError::UnsupportedFamilies(
                vec![crate::edit::WorthTopologyEditFamily::AttachShellOrWireMembership],
            ));
        };
        let retired_shell_binding = query_entity_binding(entity_rows, workflow.retired_shell_id)?
            .ok_or(
            WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(
                workflow.retired_shell_id,
            ),
        )?;
        let region_entity_binding = query_entity_binding(entity_rows, workflow.region_id)?.ok_or(
            WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(workflow.region_id),
        )?;
        if region_entity_binding.kind != WorthTopologyEntityKind::Region {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: workflow.region_id,
                    expected: WorthTopologyEntityKind::Region,
                    actual: region_entity_binding.kind,
                },
            );
        }
        let mut face_entity_bindings = Vec::with_capacity(workflow.face_ids.len());
        let mut expected_face_identities = BTreeSet::new();
        for face_id in &workflow.face_ids {
            let face_entity_binding = query_entity_binding(entity_rows, *face_id)?.ok_or(
                WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(*face_id),
            )?;
            if face_entity_binding.kind != WorthTopologyEntityKind::Face {
                return Err(
                    WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                        entity_id: *face_id,
                        expected: WorthTopologyEntityKind::Face,
                        actual: face_entity_binding.kind,
                    },
                );
            }
            expected_face_identities.insert(face_entity_binding.query_identity.clone());
            face_entity_bindings.push((*face_id, face_entity_binding));
        }
        let incoming_region_relation_ids = query_incoming_relation_ids(
            relation_rows,
            &retired_shell_binding.query_identity,
            worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell,
        )?;
        let [region_owns_shell_relation_id] = incoming_region_relation_ids.as_slice() else {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityIncomingRelationCountMismatch {
                    entity_id: workflow.retired_shell_id,
                    relation_kind: worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell,
                    expected: 1,
                    actual: incoming_region_relation_ids.len(),
                },
            );
        };
        let outgoing_face_relation_ids = query_outgoing_relation_ids(
            relation_rows,
            &retired_shell_binding.query_identity,
            worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace,
        )?;
        if outgoing_face_relation_ids.len() != workflow.face_ids.len() {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                    entity_id: workflow.retired_shell_id,
                    relation_kind: worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace,
                    expected: workflow.face_ids.len(),
                    actual: outgoing_face_relation_ids.len(),
                },
            );
        }
        let region_relation_binding =
            super::bindings::query_relation_binding(relation_rows, *region_owns_shell_relation_id)?
                .ok_or(
                    WorthTopologyQueryEditExecutionError::MissingExistingRelationBinding(
                        *region_owns_shell_relation_id,
                    ),
                )?;
        if region_relation_binding.source_query_identity != region_entity_binding.query_identity {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
                    relation_id: *region_owns_shell_relation_id,
                    expected_source_entity_id: workflow.region_id,
                    actual_source_identity: region_relation_binding.source_query_identity,
                },
            );
        }
        let mut face_relation_bindings_by_target = BTreeMap::new();
        for relation_id in outgoing_face_relation_ids {
            let relation_binding =
                super::bindings::query_relation_binding(relation_rows, relation_id)?.ok_or(
                    WorthTopologyQueryEditExecutionError::MissingExistingRelationBinding(
                        relation_id,
                    ),
                )?;
            if relation_binding.source_query_identity != retired_shell_binding.query_identity {
                return Err(
                    WorthTopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
                        relation_id,
                        expected_source_entity_id: workflow.retired_shell_id,
                        actual_source_identity: relation_binding.source_query_identity,
                    },
                );
            }
            face_relation_bindings_by_target.insert(
                relation_binding.target_query_identity.clone(),
                (relation_id, relation_binding),
            );
        }
        if face_relation_bindings_by_target
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>()
            != expected_face_identities
        {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                    entity_id: workflow.retired_shell_id,
                    relation_kind: worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace,
                    expected: workflow.face_ids.len(),
                    actual: 0,
                },
            );
        }

        let region_relation_target = retired_shell_binding.query_identity.clone();
        let region_entity_identity = region_entity_binding.query_identity.clone();
        let shell_entity_identity = retired_shell_binding.query_identity.clone();
        let created_shell_key = workflow.create_key.clone();
        let region_dependency_path =
            topology_relation_dependency_path(worth_schema::facade::WorthRelationKind::Topology(
                worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell,
            ));
        let face_dependency_path =
            topology_relation_dependency_path(worth_schema::facade::WorthRelationKind::Topology(
                worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace,
            ));
        let region_relation_handle = self.workspace.bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                format!("{region_owns_shell_relation_id:?}"),
                region_relation_binding.query_identity,
            )?
            .in_target_collection("WorthTopologyRelation")?,
        )?;
        let mut builder = ForgeQueryMutationBatchBuilder::new().insert_symbolic(
            workflow.create_key.as_str(),
            "WorthTopologyEntity",
            |mutation| {
                mutation
                    .aspect("topology.kind", WorthTopologyEntityKind::Shell.kind_name())
                    .aspect("topology.structure", workflow.create_key.as_str())
                    .aspect("naming.persistent_name", workflow.create_key.as_str())
            },
        );
        builder = builder.update_existing_verified(
            region_relation_handle,
            |verify| {
                let verify = verify
                    .aspect(
                        "topology.kind",
                        worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell
                            .kind_name(),
                    )
                    .aspect("topology.source_identity", region_entity_identity.clone())
                    .aspect("topology.target_identity", region_relation_target.clone());
                if let Some(path) = region_dependency_path {
                    verify.aspect(
                        path,
                        worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell
                            .kind_name(),
                    )
                } else {
                    verify
                }
            },
            |update| {
                let update = update
                    .aspect(
                        "topology.kind",
                        worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell
                            .kind_name(),
                    )
                    .aspect("topology.source_identity", region_entity_identity.clone())
                    .symbolic_entity_identity(
                        "topology.target_identity",
                        ForgeQuerySymbolicTargetReference::new(created_shell_key.clone())
                            .expect("created entity keys are non-empty"),
                    );
                if let Some(path) = region_dependency_path {
                    update.aspect(
                        path,
                        worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell
                            .kind_name(),
                    )
                } else {
                    update
                }
            },
        );
        for (face_id, face_entity_binding) in face_entity_bindings {
            let (shell_owns_face_relation_id, face_relation_binding) =
                face_relation_bindings_by_target
                    .remove(&face_entity_binding.query_identity)
                    .ok_or(
                        WorthTopologyQueryEditExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                            entity_id: workflow.retired_shell_id,
                            relation_kind:
                                worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace,
                            expected: workflow.face_ids.len(),
                            actual: 0,
                        },
                    )?;
            let face_relation_handle = self.workspace.bind_existing_relation(
                ForgeQueryExistingRelationTarget::new(
                    format!("{shell_owns_face_relation_id:?}"),
                    face_relation_binding.query_identity,
                )?
                .in_target_collection("WorthTopologyRelation")?,
            )?;
            let face_entity_identity = face_entity_binding.query_identity.clone();
            builder = builder.update_existing_verified(
                face_relation_handle,
                |verify| {
                    let verify = verify
                        .aspect(
                            "topology.kind",
                            worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace
                                .kind_name(),
                        )
                        .aspect("topology.source_identity", shell_entity_identity.clone())
                        .aspect("topology.target_identity", face_entity_identity.clone());
                    if let Some(path) = face_dependency_path {
                        verify.aspect(
                            path,
                            worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace
                                .kind_name(),
                        )
                    } else {
                        verify
                    }
                },
                |update| {
                    let update = update
                        .aspect(
                            "topology.kind",
                            worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace
                                .kind_name(),
                        )
                        .symbolic_entity_identity(
                            "topology.source_identity",
                            ForgeQuerySymbolicTargetReference::new(created_shell_key.clone())
                                .expect("created entity keys are non-empty"),
                        )
                        .aspect("topology.target_identity", face_entity_identity.clone());
                    if let Some(path) = face_dependency_path {
                        update.aspect(
                            path,
                            worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace
                                .kind_name(),
                        )
                    } else {
                        update
                    }
                },
            );
            let _ = face_id;
        }
        self.lower_retire_topology_entity(
            builder,
            entity_rows,
            workflow.retired_shell_id,
            WorthTopologyEntityKind::Shell,
            contracts
                .last()
                .expect("shell rehome workflow always has retire contract"),
        )
    }
}
