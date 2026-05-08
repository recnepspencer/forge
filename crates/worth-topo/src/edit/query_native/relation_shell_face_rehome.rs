use std::collections::{BTreeMap, BTreeSet};

use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryExistingRelationTarget, ForgeQueryMutationBatchBuilder,
    ForgeQuerySymbolicTargetReference,
};
use schema::facade::TopologyEntityKind;

use super::bindings::{
    query_entity_binding, query_incoming_relation_ids, query_outgoing_relation_ids,
};
use super::relation_shell_face_rehome_support::parse_shell_face_rehome_workflow;
use super::{TopologyQueryEditExecutionError, TopologyQueryEditRunner};
use crate::edit::TopologyEditContract;
use crate::query::topology_relation_dependency_path;

impl<'workspace, 'assembly> TopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn lower_rehome_owned_face_set_to_new_shell_workflow(
        &self,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
        contracts: &[TopologyEditContract],
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyQueryEditExecutionError> {
        let Some(workflow) = parse_shell_face_rehome_workflow(contracts) else {
            return Err(TopologyQueryEditExecutionError::UnsupportedFamilies(vec![
                crate::edit::TopologyEditFamily::AttachShellOrWireMembership,
            ]));
        };
        let retired_shell_binding = query_entity_binding(entity_rows, workflow.retired_shell_id)?
            .ok_or(
            TopologyQueryEditExecutionError::MissingExistingEntityBinding(
                workflow.retired_shell_id,
            ),
        )?;
        let region_entity_binding = query_entity_binding(entity_rows, workflow.region_id)?.ok_or(
            TopologyQueryEditExecutionError::MissingExistingEntityBinding(workflow.region_id),
        )?;
        if region_entity_binding.kind != TopologyEntityKind::Region {
            return Err(
                TopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: workflow.region_id,
                    expected: TopologyEntityKind::Region,
                    actual: region_entity_binding.kind,
                },
            );
        }
        let mut face_entity_bindings = Vec::with_capacity(workflow.face_ids.len());
        let mut expected_face_identities = BTreeSet::new();
        for face_id in &workflow.face_ids {
            let face_entity_binding = query_entity_binding(entity_rows, *face_id)?
                .ok_or(TopologyQueryEditExecutionError::MissingExistingEntityBinding(*face_id))?;
            if face_entity_binding.kind != TopologyEntityKind::Face {
                return Err(
                    TopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                        entity_id: *face_id,
                        expected: TopologyEntityKind::Face,
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
            schema::facade::TopologyRelationKind::RegionOwnsShell,
        )?;
        let [region_owns_shell_relation_id] = incoming_region_relation_ids.as_slice() else {
            return Err(
                TopologyQueryEditExecutionError::ExistingEntityIncomingRelationCountMismatch {
                    entity_id: workflow.retired_shell_id,
                    relation_kind: schema::facade::TopologyRelationKind::RegionOwnsShell,
                    expected: 1,
                    actual: incoming_region_relation_ids.len(),
                },
            );
        };
        let outgoing_face_relation_ids = query_outgoing_relation_ids(
            relation_rows,
            &retired_shell_binding.query_identity,
            schema::facade::TopologyRelationKind::ShellOwnsFace,
        )?;
        if outgoing_face_relation_ids.len() != workflow.face_ids.len() {
            return Err(
                TopologyQueryEditExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                    entity_id: workflow.retired_shell_id,
                    relation_kind: schema::facade::TopologyRelationKind::ShellOwnsFace,
                    expected: workflow.face_ids.len(),
                    actual: outgoing_face_relation_ids.len(),
                },
            );
        }
        let region_relation_binding =
            super::bindings::query_relation_binding(relation_rows, *region_owns_shell_relation_id)?
                .ok_or(
                    TopologyQueryEditExecutionError::MissingExistingRelationBinding(
                        *region_owns_shell_relation_id,
                    ),
                )?;
        if region_relation_binding.source_query_identity != region_entity_binding.query_identity {
            return Err(
                TopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
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
                    TopologyQueryEditExecutionError::MissingExistingRelationBinding(relation_id),
                )?;
            if relation_binding.source_query_identity != retired_shell_binding.query_identity {
                return Err(
                    TopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
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
                TopologyQueryEditExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                    entity_id: workflow.retired_shell_id,
                    relation_kind: schema::facade::TopologyRelationKind::ShellOwnsFace,
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
            topology_relation_dependency_path(schema::facade::RelationKind::Topology(
                schema::facade::TopologyRelationKind::RegionOwnsShell,
            ));
        let face_dependency_path =
            topology_relation_dependency_path(schema::facade::RelationKind::Topology(
                schema::facade::TopologyRelationKind::ShellOwnsFace,
            ));
        let region_relation_handle = self.workspace.bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                format!("{region_owns_shell_relation_id:?}"),
                region_relation_binding.query_identity,
            )?
            .in_target_collection("TopologyRelation")?,
        )?;
        let mut builder = ForgeQueryMutationBatchBuilder::new().insert_symbolic(
            workflow.create_key.as_str(),
            "TopologyEntity",
            |mutation| {
                mutation
                    .aspect("topology.kind", TopologyEntityKind::Shell.kind_name())
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
                        schema::facade::TopologyRelationKind::RegionOwnsShell.kind_name(),
                    )
                    .aspect("topology.source_identity", region_entity_identity.clone())
                    .aspect("topology.target_identity", region_relation_target.clone());
                if let Some(path) = region_dependency_path {
                    verify.aspect(
                        path,
                        schema::facade::TopologyRelationKind::RegionOwnsShell.kind_name(),
                    )
                } else {
                    verify
                }
            },
            |update| {
                let update = update
                    .aspect(
                        "topology.kind",
                        schema::facade::TopologyRelationKind::RegionOwnsShell.kind_name(),
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
                        schema::facade::TopologyRelationKind::RegionOwnsShell.kind_name(),
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
                        TopologyQueryEditExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                            entity_id: workflow.retired_shell_id,
                            relation_kind:
                                schema::facade::TopologyRelationKind::ShellOwnsFace,
                            expected: workflow.face_ids.len(),
                            actual: 0,
                        },
                    )?;
            let face_relation_handle = self.workspace.bind_existing_relation(
                ForgeQueryExistingRelationTarget::new(
                    format!("{shell_owns_face_relation_id:?}"),
                    face_relation_binding.query_identity,
                )?
                .in_target_collection("TopologyRelation")?,
            )?;
            let face_entity_identity = face_entity_binding.query_identity.clone();
            builder = builder.update_existing_verified(
                face_relation_handle,
                |verify| {
                    let verify = verify
                        .aspect(
                            "topology.kind",
                            schema::facade::TopologyRelationKind::ShellOwnsFace.kind_name(),
                        )
                        .aspect("topology.source_identity", shell_entity_identity.clone())
                        .aspect("topology.target_identity", face_entity_identity.clone());
                    if let Some(path) = face_dependency_path {
                        verify.aspect(
                            path,
                            schema::facade::TopologyRelationKind::ShellOwnsFace.kind_name(),
                        )
                    } else {
                        verify
                    }
                },
                |update| {
                    let update = update
                        .aspect(
                            "topology.kind",
                            schema::facade::TopologyRelationKind::ShellOwnsFace.kind_name(),
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
                            schema::facade::TopologyRelationKind::ShellOwnsFace.kind_name(),
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
            TopologyEntityKind::Shell,
            contracts
                .last()
                .expect("shell rehome workflow always has retire contract"),
        )
    }
}
