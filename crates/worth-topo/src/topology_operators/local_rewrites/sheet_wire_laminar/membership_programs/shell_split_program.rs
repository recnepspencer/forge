use forge_query::facade::ForgeQuerySymbolicTargetReference;
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use super::shared::bind_existing_relation_handle;
use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::query_native_runtime_boundary::TopologyNativeQueryRowField;
use crate::topology_operators::application::{
    ensure_declared_touched_basis_covers_sequence_before_write, TopologyDeclaredMutationArtifact,
    TopologyMutationApplicationError, TopologyMutationApplicationRunner,
    TopologyRetainedApplicationHandoff,
};
use crate::topology_operators::topology_relation_dependency_path;
use crate::topology_operators::{
    TopologyDeclaredMutationSequence, TopologyMutationApplicationMode,
};

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn compose_shell_split_program<I>(
        &mut self,
        retained_handoff: TopologyRetainedApplicationHandoff<I>,
        mode: TopologyMutationApplicationMode,
        semantic_family_key: &'static str,
        program: super::super::shell_face_rehome_support::ShellFaceSplitProgram,
        sequence: &TopologyDeclaredMutationSequence,
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError>
    where
        I: forge_query::facade::ForgeQueryDeclarationInput<
            crate::query_domain::TopologyQueryDomain,
        >,
    {
        ensure_declared_touched_basis_covers_sequence_before_write(
            &retained_handoff,
            sequence,
            mode.clone(),
        )?;
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

        let face_rebind_authorities =
            crate::topology_operators::authority_identity::relation_continuity_rebind_authorities(
                *face_relation_id,
            )?;

        let receipt = self
            .workspace
            .compose_graph(|graph| {
                let shell_symbol = graph.insert_entity(
                    created_shell_key.clone(),
                    "TopologyEntity",
                    |mutation| {
                        TopologyNativeQueryRowField::NamingPersistentName.set_on(
                            TopologyNativeQueryRowField::TopologyStructure.set_on(
                                TopologyNativeQueryRowField::TopologyKind
                                    .set_on(mutation, TopologyEntityKind::Shell.kind_name()),
                                created_shell_key.clone(),
                            ),
                            created_shell_key.clone(),
                        )
                    },
                )?;
                graph.insert_relation("TopologyRelation", |relation| {
                    let relation = TopologyNativeQueryRowField::TopologyKind.set_on_relation(
                        relation,
                        TopologyRelationKind::RegionOwnsShell.kind_name(),
                    );
                    let relation = relation
                        .existing_entity_identity(
                            TopologyNativeQueryRowField::TopologySourceIdentity.touch(),
                            region_binding.query_identity.clone(),
                        )
                        .symbolic_entity_identity(
                            TopologyNativeQueryRowField::TopologyTargetIdentity.touch(),
                            &shell_symbol,
                        );
                    if let Some(field) = region_dependency_path
                        .and_then(TopologyNativeQueryRowField::from_query_aspect_path)
                    {
                        field.set_on_relation(
                            relation,
                            TopologyRelationKind::RegionOwnsShell.kind_name(),
                        )
                    } else {
                        relation
                    }
                })?;
                graph.retarget_existing_verified(
                    face_handle.clone(),
                    |verify| {
                        let verify = TopologyNativeQueryRowField::TopologyTargetIdentity.set_on(
                            TopologyNativeQueryRowField::TopologySourceIdentity.set_on(
                                TopologyNativeQueryRowField::TopologyKind.set_on(
                                    verify,
                                    TopologyRelationKind::ShellOwnsFace.kind_name(),
                                ),
                                retained_shell_binding.query_identity_label.clone(),
                            ),
                            face_binding.query_identity_label.clone(),
                        );
                        if let Some(field) = face_dependency_path
                            .and_then(TopologyNativeQueryRowField::from_query_aspect_path)
                        {
                            field.set_on(verify, TopologyRelationKind::ShellOwnsFace.kind_name())
                        } else {
                            verify
                        }
                    },
                    |update| {
                        let update = update.continuity_rebind_existing_target(
                            face_rebind_authorities.0.clone(),
                            face_rebind_authorities.1.clone(),
                        );
                        let update = TopologyNativeQueryRowField::TopologyKind
                            .set_on(update, TopologyRelationKind::ShellOwnsFace.kind_name())
                            .symbolic_entity_identity(
                                TopologyNativeQueryRowField::TopologySourceIdentity.touch(),
                                ForgeQuerySymbolicTargetReference::new(created_shell_key.clone())
                                    .expect("created entity keys are non-empty"),
                            );
                        let update = TopologyNativeQueryRowField::TopologyTargetIdentity
                            .set_on(update, face_binding.query_identity_label.clone());
                        if let Some(field) = face_dependency_path
                            .and_then(TopologyNativeQueryRowField::from_query_aspect_path)
                        {
                            field.set_on(update, TopologyRelationKind::ShellOwnsFace.kind_name())
                        } else {
                            update
                        }
                    },
                )?;
                Ok(())
            })
            .map_err(TopologyMutationApplicationError::from)?;
        self.finish_composed_membership_execution(
            mode,
            retained_handoff,
            semantic_family_key,
            sequence,
            receipt,
        )
    }
}
