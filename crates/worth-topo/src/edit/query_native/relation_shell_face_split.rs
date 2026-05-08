use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryExistingRelationTarget, ForgeQueryMutationBatchBuilder,
    ForgeQuerySymbolicTargetReference,
};
use schema::facade::TopologyEntityKind;

use super::bindings::{query_entity_binding, query_incoming_relation_ids};
use super::relation_shell_face_rehome_support::resolve_single_face_two_face_shell_split_workflow;
use super::{TopologyQueryEditExecutionError, TopologyQueryEditRunner};
use crate::edit::TopologyEditContract;
use crate::query::topology_relation_dependency_path;

impl<'workspace, 'assembly> TopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn lower_split_single_face_from_two_face_shell_to_new_shell_workflow(
        &self,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
        contracts: &[TopologyEditContract],
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyQueryEditExecutionError> {
        let Some(workflow) = resolve_single_face_two_face_shell_split_workflow(
            entity_rows,
            relation_rows,
            contracts,
        ) else {
            return Err(TopologyQueryEditExecutionError::UnsupportedFamilies(vec![
                crate::edit::TopologyEditFamily::AttachShellOrWireMembership,
            ]));
        };
        let retained_shell_id = workflow
            .retained_shell_id
            .expect("resolved shell split workflow always sets retained shell id");
        let region_entity_binding = query_entity_binding(entity_rows, workflow.region_id)?.ok_or(
            TopologyQueryEditExecutionError::MissingExistingEntityBinding(workflow.region_id),
        )?;
        let face_entity_binding = query_entity_binding(entity_rows, workflow.face_id)?.ok_or(
            TopologyQueryEditExecutionError::MissingExistingEntityBinding(workflow.face_id),
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
        if face_entity_binding.kind != TopologyEntityKind::Face {
            return Err(
                TopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: workflow.face_id,
                    expected: TopologyEntityKind::Face,
                    actual: face_entity_binding.kind,
                },
            );
        }
        let retained_shell_binding = query_entity_binding(entity_rows, retained_shell_id)?.ok_or(
            TopologyQueryEditExecutionError::MissingExistingEntityBinding(retained_shell_id),
        )?;
        let incoming_region_relation_ids = query_incoming_relation_ids(
            relation_rows,
            &retained_shell_binding.query_identity,
            schema::facade::TopologyRelationKind::RegionOwnsShell,
        )?;
        let [region_owns_shell_relation_id] = incoming_region_relation_ids.as_slice() else {
            return Err(
                TopologyQueryEditExecutionError::ExistingEntityIncomingRelationCountMismatch {
                    entity_id: retained_shell_id,
                    relation_kind: schema::facade::TopologyRelationKind::RegionOwnsShell,
                    expected: 1,
                    actual: incoming_region_relation_ids.len(),
                },
            );
        };
        let incoming_shell_face_relation_ids = query_incoming_relation_ids(
            relation_rows,
            &face_entity_binding.query_identity,
            schema::facade::TopologyRelationKind::ShellOwnsFace,
        )?;
        let [shell_owns_face_relation_id] = incoming_shell_face_relation_ids.as_slice() else {
            return Err(
                TopologyQueryEditExecutionError::ExistingEntityIncomingRelationCountMismatch {
                    entity_id: workflow.face_id,
                    relation_kind: schema::facade::TopologyRelationKind::ShellOwnsFace,
                    expected: 1,
                    actual: incoming_shell_face_relation_ids.len(),
                },
            );
        };
        let region_relation_binding =
            super::bindings::query_relation_binding(relation_rows, *region_owns_shell_relation_id)?
                .ok_or(
                    TopologyQueryEditExecutionError::MissingExistingRelationBinding(
                        *region_owns_shell_relation_id,
                    ),
                )?;
        let face_relation_binding =
            super::bindings::query_relation_binding(relation_rows, *shell_owns_face_relation_id)?
                .ok_or(
                TopologyQueryEditExecutionError::MissingExistingRelationBinding(
                    *shell_owns_face_relation_id,
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
        if face_relation_binding.source_query_identity != retained_shell_binding.query_identity {
            return Err(
                TopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
                    relation_id: *shell_owns_face_relation_id,
                    expected_source_entity_id: retained_shell_id,
                    actual_source_identity: face_relation_binding.source_query_identity,
                },
            );
        }

        let region_dependency_path =
            topology_relation_dependency_path(schema::facade::RelationKind::Topology(
                schema::facade::TopologyRelationKind::RegionOwnsShell,
            ));
        let face_dependency_path =
            topology_relation_dependency_path(schema::facade::RelationKind::Topology(
                schema::facade::TopologyRelationKind::ShellOwnsFace,
            ));
        let region_relation_target = region_entity_binding.query_identity.clone();
        let retained_shell_identity = retained_shell_binding.query_identity.clone();
        let face_identity = face_entity_binding.query_identity.clone();
        let created_shell_key = workflow.create_key.clone();

        let region_relation_create = |builder: ForgeQueryMutationBatchBuilder| {
            builder.insert("TopologyRelation", |mutation| {
                let mutation = mutation
                    .aspect(
                        "topology.kind",
                        schema::facade::TopologyRelationKind::RegionOwnsShell.kind_name(),
                    )
                    .aspect("topology.source_identity", region_relation_target.clone())
                    .symbolic_entity_identity(
                        "topology.target_identity",
                        ForgeQuerySymbolicTargetReference::new(created_shell_key.clone())
                            .expect("created entity keys are non-empty"),
                    );
                if let Some(path) = region_dependency_path {
                    mutation.aspect(
                        path,
                        schema::facade::TopologyRelationKind::RegionOwnsShell.kind_name(),
                    )
                } else {
                    mutation
                }
            })
        };
        let face_relation_handle = self.workspace.bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                format!("{shell_owns_face_relation_id:?}"),
                face_relation_binding.query_identity,
            )?
            .in_target_collection("TopologyRelation")?,
        )?;
        let builder = ForgeQueryMutationBatchBuilder::new().insert_symbolic(
            workflow.create_key.as_str(),
            "TopologyEntity",
            |mutation| {
                mutation
                    .aspect("topology.kind", TopologyEntityKind::Shell.kind_name())
                    .aspect("topology.structure", workflow.create_key.as_str())
                    .aspect("naming.persistent_name", workflow.create_key.as_str())
            },
        );
        let builder = region_relation_create(builder);
        Ok(builder.update_existing_verified(
            face_relation_handle,
            |verify| {
                let verify = verify
                    .aspect(
                        "topology.kind",
                        schema::facade::TopologyRelationKind::ShellOwnsFace.kind_name(),
                    )
                    .aspect("topology.source_identity", retained_shell_identity.clone())
                    .aspect("topology.target_identity", face_identity.clone());
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
                    .aspect("topology.target_identity", face_identity.clone());
                if let Some(path) = face_dependency_path {
                    update.aspect(
                        path,
                        schema::facade::TopologyRelationKind::ShellOwnsFace.kind_name(),
                    )
                } else {
                    update
                }
            },
        ))
    }
}
