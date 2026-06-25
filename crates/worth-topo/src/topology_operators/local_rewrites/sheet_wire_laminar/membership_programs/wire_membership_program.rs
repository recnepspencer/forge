use forge_query::facade::ForgeQuerySymbolicTargetReference;
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use super::shared::{
    bind_existing_entity_handle, bind_existing_relation_handle, delete_existing_entity_from_graph,
};
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
    pub(crate) fn compose_wire_rehome_program<I>(
        &mut self,
        retained_handoff: TopologyRetainedApplicationHandoff<I>,
        mode: TopologyMutationApplicationMode,
        semantic_family_key: &'static str,
        program: super::super::wire_rehome_support::WireRehomeProgram,
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
        let members = sequence.members().collect::<Vec<_>>();
        let retired_wire_binding =
            crate::topology_operators::application::bindings::query_entity_binding(
                bindings,
                program.retired_wire_id,
            )?
            .ok_or(
                TopologyMutationApplicationError::MissingExistingEntityBinding(
                    program.retired_wire_id,
                ),
            )?;
        let dependency_path = topology_relation_dependency_path(
            schema::facade::platform::relations::RelationKind::Topology(
                TopologyRelationKind::WireOwnsHalfEdge,
            ),
        );
        let created_wire_key = program.create_key.clone();
        let retired_wire_identity = retired_wire_binding.query_identity_label.clone();
        let retire_contract = members
            .last()
            .expect("wire rehome program always ends with retire contract");
        let retired_wire_handle = bind_existing_entity_handle(
            self,
            bindings,
            program.retired_wire_id,
            TopologyEntityKind::Wire,
        )?;
        let mut relation_rows_to_move = Vec::with_capacity(program.half_edge_ids.len());
        for half_edge_id in &program.half_edge_ids {
            let half_edge_binding =
                crate::topology_operators::application::bindings::query_entity_binding(
                    bindings,
                    *half_edge_id,
                )?
                .ok_or(
                    TopologyMutationApplicationError::MissingExistingEntityBinding(*half_edge_id),
                )?;
            let incoming_relation_ids =
                crate::topology_operators::application::bindings::query_incoming_relation_ids(
                    bindings,
                    &half_edge_binding.query_identity_label,
                    TopologyRelationKind::WireOwnsHalfEdge,
                )?;
            let [relation_id] = incoming_relation_ids.as_slice() else {
                return Err(
                    TopologyMutationApplicationError::ExistingEntityIncomingRelationCountMismatch {
                        entity_id: *half_edge_id,
                        relation_kind: TopologyRelationKind::WireOwnsHalfEdge,
                        expected: 1,
                        actual: incoming_relation_ids.len(),
                    },
                );
            };
            let relation_binding =
                crate::topology_operators::application::bindings::query_relation_binding(
                    bindings,
                    *relation_id,
                )?
                .ok_or(
                    TopologyMutationApplicationError::MissingExistingRelationBinding(*relation_id),
                )?;
            relation_rows_to_move.push((
                bind_existing_relation_handle(
                    self,
                    *relation_id,
                    relation_binding.query_identity.clone(),
                )?,
                *relation_id,
                half_edge_binding.query_identity_label,
            ));
        }
        let mut relation_rebind_authorities = std::collections::BTreeMap::new();
        for (_, relation_id, _) in &relation_rows_to_move {
            relation_rebind_authorities.insert(
                *relation_id,
                crate::topology_operators::authority_identity::relation_continuity_rebind_authorities(
                    *relation_id,
                )
                ?,
            );
        }
        let receipt = self
            .workspace
            .compose_graph(|graph| {
                graph.insert_entity(created_wire_key.clone(), "TopologyEntity", |mutation| {
                    TopologyNativeQueryRowField::NamingPersistentName.set_on(
                        TopologyNativeQueryRowField::TopologyStructure.set_on(
                            TopologyNativeQueryRowField::TopologyKind
                                .set_on(mutation, TopologyEntityKind::Wire.kind_name()),
                            created_wire_key.clone(),
                        ),
                        created_wire_key.clone(),
                    )
                })?;
                for (relation_handle, relation_id, half_edge_identity) in &relation_rows_to_move {
                    graph.retarget_existing_verified(
                        relation_handle.clone(),
                        |verify| {
                            let verify = TopologyNativeQueryRowField::TopologyTargetIdentity
                                .set_on(
                                    TopologyNativeQueryRowField::TopologySourceIdentity.set_on(
                                        TopologyNativeQueryRowField::TopologyKind.set_on(
                                            verify,
                                            TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                                        ),
                                        retired_wire_identity.clone(),
                                    ),
                                    half_edge_identity.clone(),
                                );
                            if let Some(field) = dependency_path
                                .and_then(TopologyNativeQueryRowField::from_query_aspect_path)
                            {
                                field.set_on(
                                    verify,
                                    TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                                )
                            } else {
                                verify
                            }
                        },
                        |update| {
                            let (prior, successor) =
                                relation_rebind_authorities[relation_id].clone();
                            let update = update.continuity_rebind_existing_target(prior, successor);
                            let update = TopologyNativeQueryRowField::TopologyKind
                                .set_on(update, TopologyRelationKind::WireOwnsHalfEdge.kind_name())
                                .symbolic_entity_identity(
                                    TopologyNativeQueryRowField::TopologySourceIdentity.touch(),
                                    ForgeQuerySymbolicTargetReference::new(
                                        created_wire_key.clone(),
                                    )
                                    .expect("created entity keys are non-empty"),
                                );
                            let update = TopologyNativeQueryRowField::TopologyTargetIdentity
                                .set_on(update, half_edge_identity.clone());
                            if let Some(field) = dependency_path
                                .and_then(TopologyNativeQueryRowField::from_query_aspect_path)
                            {
                                field.set_on(
                                    update,
                                    TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                                )
                            } else {
                                update
                            }
                        },
                    )?;
                }
                delete_existing_entity_from_graph(
                    graph,
                    retired_wire_handle.clone(),
                    "TopologyEntity",
                    TopologyEntityKind::Wire.kind_name(),
                    *retire_contract,
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

    pub(crate) fn compose_wire_split_program<I>(
        &mut self,
        retained_handoff: TopologyRetainedApplicationHandoff<I>,
        mode: TopologyMutationApplicationMode,
        semantic_family_key: &'static str,
        program: super::super::wire_rehome_support::WireSplitProgram,
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
        let retained_wire_id = program
            .retained_wire_id
            .expect("resolved wire split program always sets retained wire id");
        let retained_wire_binding =
            crate::topology_operators::application::bindings::query_entity_binding(
                bindings,
                retained_wire_id,
            )?
            .ok_or(
                TopologyMutationApplicationError::MissingExistingEntityBinding(retained_wire_id),
            )?;
        let dependency_path = topology_relation_dependency_path(
            schema::facade::platform::relations::RelationKind::Topology(
                TopologyRelationKind::WireOwnsHalfEdge,
            ),
        );
        let created_wire_key = program.create_key.clone();
        let retained_wire_identity = retained_wire_binding.query_identity_label.clone();
        let mut moved_relations = Vec::with_capacity(program.half_edge_ids.len());
        for half_edge_id in &program.half_edge_ids {
            let half_edge_binding =
                crate::topology_operators::application::bindings::query_entity_binding(
                    bindings,
                    *half_edge_id,
                )?
                .ok_or(
                    TopologyMutationApplicationError::MissingExistingEntityBinding(*half_edge_id),
                )?;
            let incoming_relation_ids =
                crate::topology_operators::application::bindings::query_incoming_relation_ids(
                    bindings,
                    &half_edge_binding.query_identity_label,
                    TopologyRelationKind::WireOwnsHalfEdge,
                )?;
            let [relation_id] = incoming_relation_ids.as_slice() else {
                return Err(
                    TopologyMutationApplicationError::ExistingEntityIncomingRelationCountMismatch {
                        entity_id: *half_edge_id,
                        relation_kind: TopologyRelationKind::WireOwnsHalfEdge,
                        expected: 1,
                        actual: incoming_relation_ids.len(),
                    },
                );
            };
            let relation_binding =
                crate::topology_operators::application::bindings::query_relation_binding(
                    bindings,
                    *relation_id,
                )?
                .ok_or(
                    TopologyMutationApplicationError::MissingExistingRelationBinding(*relation_id),
                )?;
            moved_relations.push((
                bind_existing_relation_handle(
                    self,
                    *relation_id,
                    relation_binding.query_identity.clone(),
                )?,
                *relation_id,
                half_edge_binding.query_identity_label,
            ));
        }
        let mut relation_rebind_authorities = std::collections::BTreeMap::new();
        for (_, relation_id, _) in &moved_relations {
            relation_rebind_authorities.insert(
                *relation_id,
                crate::topology_operators::authority_identity::relation_continuity_rebind_authorities(
                    *relation_id,
                )
                ?,
            );
        }
        let receipt = self
            .workspace
            .compose_graph(|graph| {
                graph.insert_entity(created_wire_key.clone(), "TopologyEntity", |mutation| {
                    TopologyNativeQueryRowField::NamingPersistentName.set_on(
                        TopologyNativeQueryRowField::TopologyStructure.set_on(
                            TopologyNativeQueryRowField::TopologyKind
                                .set_on(mutation, TopologyEntityKind::Wire.kind_name()),
                            created_wire_key.clone(),
                        ),
                        created_wire_key.clone(),
                    )
                })?;
                for (relation_handle, relation_id, half_edge_identity) in &moved_relations {
                    graph.retarget_existing_verified(
                        relation_handle.clone(),
                        |verify| {
                            let verify = TopologyNativeQueryRowField::TopologyTargetIdentity
                                .set_on(
                                    TopologyNativeQueryRowField::TopologySourceIdentity.set_on(
                                        TopologyNativeQueryRowField::TopologyKind.set_on(
                                            verify,
                                            TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                                        ),
                                        retained_wire_identity.clone(),
                                    ),
                                    half_edge_identity.clone(),
                                );
                            if let Some(field) = dependency_path
                                .and_then(TopologyNativeQueryRowField::from_query_aspect_path)
                            {
                                field.set_on(
                                    verify,
                                    TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                                )
                            } else {
                                verify
                            }
                        },
                        |update| {
                            let (prior, successor) =
                                relation_rebind_authorities[relation_id].clone();
                            let update = update.continuity_rebind_existing_target(prior, successor);
                            let update = TopologyNativeQueryRowField::TopologyKind
                                .set_on(update, TopologyRelationKind::WireOwnsHalfEdge.kind_name())
                                .symbolic_entity_identity(
                                    TopologyNativeQueryRowField::TopologySourceIdentity.touch(),
                                    ForgeQuerySymbolicTargetReference::new(
                                        created_wire_key.clone(),
                                    )
                                    .expect("created entity keys are non-empty"),
                                );
                            let update = TopologyNativeQueryRowField::TopologyTargetIdentity
                                .set_on(update, half_edge_identity.clone());
                            if let Some(field) = dependency_path
                                .and_then(TopologyNativeQueryRowField::from_query_aspect_path)
                            {
                                field.set_on(
                                    update,
                                    TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                                )
                            } else {
                                update
                            }
                        },
                    )?;
                }
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
