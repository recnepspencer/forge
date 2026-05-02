use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryExistingRelationTarget, ForgeQueryMutationBatchBuilder,
    ForgeQuerySymbolicTargetReference,
};
use worth_schema::facade::WorthTopologyEntityKind;

use super::bindings::{query_entity_binding, query_incoming_relation_ids};
use super::relation_shell_face_rehome_support::resolve_single_face_two_face_shell_split_workflow;
use super::{WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner};
use crate::edit::WorthTopologyEditContract;
use crate::query::topology_relation_dependency_path;

impl<'workspace, 'assembly> WorthTopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn lower_split_single_face_from_two_face_shell_to_new_shell_workflow(
        &self,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
        contracts: &[WorthTopologyEditContract],
    ) -> Result<ForgeQueryMutationBatchBuilder, WorthTopologyQueryEditExecutionError> {
        let Some(workflow) = resolve_single_face_two_face_shell_split_workflow(
            entity_rows,
            relation_rows,
            contracts,
        ) else {
            return Err(WorthTopologyQueryEditExecutionError::UnsupportedFamilies(
                vec![crate::edit::WorthTopologyEditFamily::AttachShellOrWireMembership],
            ));
        };
        let retained_shell_id = workflow
            .retained_shell_id
            .expect("resolved shell split workflow always sets retained shell id");
        let region_entity_binding = query_entity_binding(entity_rows, workflow.region_id)?.ok_or(
            WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(workflow.region_id),
        )?;
        let face_entity_binding = query_entity_binding(entity_rows, workflow.face_id)?.ok_or(
            WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(workflow.face_id),
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
        if face_entity_binding.kind != WorthTopologyEntityKind::Face {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                    entity_id: workflow.face_id,
                    expected: WorthTopologyEntityKind::Face,
                    actual: face_entity_binding.kind,
                },
            );
        }
        let retained_shell_binding = query_entity_binding(entity_rows, retained_shell_id)?.ok_or(
            WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(retained_shell_id),
        )?;
        let incoming_region_relation_ids = query_incoming_relation_ids(
            relation_rows,
            &retained_shell_binding.query_identity,
            worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell,
        )?;
        let [region_owns_shell_relation_id] = incoming_region_relation_ids.as_slice() else {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityIncomingRelationCountMismatch {
                    entity_id: retained_shell_id,
                    relation_kind: worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell,
                    expected: 1,
                    actual: incoming_region_relation_ids.len(),
                },
            );
        };
        let incoming_shell_face_relation_ids = query_incoming_relation_ids(
            relation_rows,
            &face_entity_binding.query_identity,
            worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace,
        )?;
        let [shell_owns_face_relation_id] = incoming_shell_face_relation_ids.as_slice() else {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityIncomingRelationCountMismatch {
                    entity_id: workflow.face_id,
                    relation_kind: worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace,
                    expected: 1,
                    actual: incoming_shell_face_relation_ids.len(),
                },
            );
        };
        let region_relation_binding =
            super::bindings::query_relation_binding(relation_rows, *region_owns_shell_relation_id)?
                .ok_or(
                    WorthTopologyQueryEditExecutionError::MissingExistingRelationBinding(
                        *region_owns_shell_relation_id,
                    ),
                )?;
        let face_relation_binding =
            super::bindings::query_relation_binding(relation_rows, *shell_owns_face_relation_id)?
                .ok_or(
                WorthTopologyQueryEditExecutionError::MissingExistingRelationBinding(
                    *shell_owns_face_relation_id,
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
        if face_relation_binding.source_query_identity != retained_shell_binding.query_identity {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
                    relation_id: *shell_owns_face_relation_id,
                    expected_source_entity_id: retained_shell_id,
                    actual_source_identity: face_relation_binding.source_query_identity,
                },
            );
        }

        let region_dependency_path =
            topology_relation_dependency_path(worth_schema::facade::WorthRelationKind::Topology(
                worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell,
            ));
        let face_dependency_path =
            topology_relation_dependency_path(worth_schema::facade::WorthRelationKind::Topology(
                worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace,
            ));
        let region_relation_target = region_entity_binding.query_identity.clone();
        let retained_shell_identity = retained_shell_binding.query_identity.clone();
        let face_identity = face_entity_binding.query_identity.clone();
        let created_shell_key = workflow.create_key.clone();

        let region_relation_create = |builder: ForgeQueryMutationBatchBuilder| {
            builder.insert("WorthTopologyRelation", |mutation| {
                let mutation = mutation
                    .aspect(
                        "topology.kind",
                        worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell
                            .kind_name(),
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
                        worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell
                            .kind_name(),
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
            .in_target_collection("WorthTopologyRelation")?,
        )?;
        let builder = ForgeQueryMutationBatchBuilder::new().insert_symbolic(
            workflow.create_key.as_str(),
            "WorthTopologyEntity",
            |mutation| {
                mutation
                    .aspect("topology.kind", WorthTopologyEntityKind::Shell.kind_name())
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
                        worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace.kind_name(),
                    )
                    .aspect("topology.source_identity", retained_shell_identity.clone())
                    .aspect("topology.target_identity", face_identity.clone());
                if let Some(path) = face_dependency_path {
                    verify.aspect(
                        path,
                        worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace.kind_name(),
                    )
                } else {
                    verify
                }
            },
            |update| {
                let update = update
                    .aspect(
                        "topology.kind",
                        worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace.kind_name(),
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
                        worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace.kind_name(),
                    )
                } else {
                    update
                }
            },
        ))
    }
}
