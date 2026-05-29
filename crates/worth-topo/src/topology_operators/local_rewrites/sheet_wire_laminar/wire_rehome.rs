use std::collections::{BTreeMap, BTreeSet};

use forge_query::facade::{
    ForgeQueryExistingRelationTarget, ForgeQueryMutationBatchBuilder,
    ForgeQuerySymbolicTargetReference,
};
use schema::facade::platform::entities::TopologyEntityKind;

use super::wire_rehome_support::{parse_wire_rehome_program, resolve_wire_split_program};
use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::bindings::{
    query_entity_binding, query_outgoing_relation_ids,
};
use crate::topology_operators::application::{
    TopologyOperatorExecutionError, TopologyOperatorRunner,
};
use crate::topology_operators::topology_relation_dependency_path;
use crate::topology_operators::TopologyEditContract;

impl<'workspace, 'assembly> TopologyOperatorRunner<'workspace, 'assembly> {
    pub(super) fn lower_rehome_owned_half_edge_set_to_new_wire_program(
        &self,
        bindings: &TopologyQueryBindingIndex,
        contracts: &[TopologyEditContract],
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyOperatorExecutionError> {
        let Some(program) = parse_wire_rehome_program(contracts) else {
            return Err(TopologyOperatorExecutionError::UnsupportedFamilies(vec![
                crate::topology_operators::TopologyEditFamily::AttachShellOrWireMembership,
            ]));
        };
        let retired_wire_binding = query_entity_binding(bindings, program.retired_wire_id)?.ok_or(
            TopologyOperatorExecutionError::MissingExistingEntityBinding(program.retired_wire_id),
        )?;
        let outgoing_relation_ids = query_outgoing_relation_ids(
            bindings,
            &retired_wire_binding.query_identity,
            schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge,
        )?;
        if outgoing_relation_ids.len() != program.half_edge_ids.len() {
            return Err(
                TopologyOperatorExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                    entity_id: program.retired_wire_id,
                    relation_kind:
                        schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge,
                    expected: program.half_edge_ids.len(),
                    actual: outgoing_relation_ids.len(),
                },
            );
        }
        let mut half_edge_bindings = Vec::with_capacity(program.half_edge_ids.len());
        let mut expected_half_edge_identities = BTreeSet::new();
        for half_edge_id in &program.half_edge_ids {
            let half_edge_binding = query_entity_binding(bindings, *half_edge_id)?.ok_or(
                TopologyOperatorExecutionError::MissingExistingEntityBinding(*half_edge_id),
            )?;
            if half_edge_binding.kind != TopologyEntityKind::HalfEdge {
                return Err(TopologyOperatorExecutionError::ExistingEntityKindMismatch {
                    entity_id: *half_edge_id,
                    expected: TopologyEntityKind::HalfEdge,
                    actual: half_edge_binding.kind,
                });
            }
            expected_half_edge_identities.insert(half_edge_binding.query_identity.clone());
            half_edge_bindings.push((*half_edge_id, half_edge_binding));
        }
        let mut relation_bindings_by_target = BTreeMap::new();
        for relation_id in outgoing_relation_ids {
            let relation_binding =
                crate::topology_operators::application::bindings::query_relation_binding(
                    bindings,
                    relation_id,
                )?
                .ok_or(
                    TopologyOperatorExecutionError::MissingExistingRelationBinding(relation_id),
                )?;
            if relation_binding.source_query_identity != retired_wire_binding.query_identity {
                return Err(
                    TopologyOperatorExecutionError::ExistingRelationSourceMismatch {
                        relation_id,
                        expected_source_entity_id: program.retired_wire_id,
                        actual_source_identity: relation_binding.source_query_identity,
                    },
                );
            }
            relation_bindings_by_target.insert(
                relation_binding.target_query_identity.clone(),
                (relation_id, relation_binding),
            );
        }
        if relation_bindings_by_target
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>()
            != expected_half_edge_identities
        {
            return Err(
                TopologyOperatorExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                    entity_id: program.retired_wire_id,
                    relation_kind:
                        schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge,
                    expected: program.half_edge_ids.len(),
                    actual: 0,
                },
            );
        }
        let dependency_path = topology_relation_dependency_path(
            schema::facade::platform::relations::RelationKind::Topology(
                schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge,
            ),
        );
        let retired_wire_identity = retired_wire_binding.query_identity.clone();
        let created_wire_key = program.create_key.clone();
        let mut builder = ForgeQueryMutationBatchBuilder::new().insert_symbolic(
            program.create_key.as_str(),
            "TopologyEntity",
            |mutation| {
                mutation
                    .aspect("topology.kind", TopologyEntityKind::Wire.kind_name())
                    .aspect("topology.structure", program.create_key.as_str())
                    .aspect("naming.persistent_name", program.create_key.as_str())
            },
        );
        for (half_edge_id, half_edge_binding) in half_edge_bindings {
            let (relation_id, relation_binding) = relation_bindings_by_target
                .remove(&half_edge_binding.query_identity)
                .ok_or(
                    TopologyOperatorExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                        entity_id: program.retired_wire_id,
                        relation_kind: schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge,
                        expected: program.half_edge_ids.len(),
                        actual: 0,
                    },
                )?;
            let relation_handle = self.workspace.bind_existing_relation(
                ForgeQueryExistingRelationTarget::new(
                    format!("{relation_id:?}"),
                    relation_binding.query_identity,
                )?
                .in_target_collection("TopologyRelation")?,
            )?;
            let half_edge_identity = half_edge_binding.query_identity.clone();
            builder = builder.update_existing_verified(
                relation_handle,
                |verify| {
                    let verify = verify
                        .aspect(
                            "topology.kind",
                            schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                        )
                        .aspect("topology.source_identity", retired_wire_identity.clone())
                        .aspect("topology.target_identity", half_edge_identity.clone());
                    if let Some(path) = dependency_path {
                        verify.aspect(
                            path,
                            schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                        )
                    } else {
                        verify
                    }
                },
                |update| {
                    let update = update
                        .aspect(
                            "topology.kind",
                            schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                        )
                        .symbolic_entity_identity(
                            "topology.source_identity",
                            ForgeQuerySymbolicTargetReference::new(created_wire_key.clone())
                                .expect("created entity keys are non-empty"),
                        )
                        .aspect("topology.target_identity", half_edge_identity.clone());
                    if let Some(path) = dependency_path {
                        update.aspect(
                            path,
                            schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                        )
                    } else {
                        update
                    }
                },
            );
            let _ = half_edge_id;
        }
        self.lower_retire_topology_entity(
            builder,
            bindings,
            program.retired_wire_id,
            TopologyEntityKind::Wire,
            contracts
                .last()
                .expect("wire rehome program always has retire contract"),
        )
    }

    pub(super) fn lower_split_connected_half_edge_set_to_new_wire_program(
        &self,
        bindings: &TopologyQueryBindingIndex,
        contracts: &[TopologyEditContract],
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyOperatorExecutionError> {
        let Some(program) = resolve_wire_split_program(bindings, contracts) else {
            return Err(TopologyOperatorExecutionError::UnsupportedFamilies(vec![
                crate::topology_operators::TopologyEditFamily::AttachShellOrWireMembership,
            ]));
        };
        let retained_wire_id = program
            .retained_wire_id
            .expect("resolved wire split program always sets retained wire id");
        let retained_wire_binding = query_entity_binding(bindings, retained_wire_id)?.ok_or(
            TopologyOperatorExecutionError::MissingExistingEntityBinding(retained_wire_id),
        )?;
        let outgoing_relation_ids = query_outgoing_relation_ids(
            bindings,
            &retained_wire_binding.query_identity,
            schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge,
        )?;
        let mut half_edge_bindings = Vec::with_capacity(program.half_edge_ids.len());
        let mut moved_half_edge_identities = BTreeSet::new();
        for half_edge_id in &program.half_edge_ids {
            let half_edge_binding = query_entity_binding(bindings, *half_edge_id)?.ok_or(
                TopologyOperatorExecutionError::MissingExistingEntityBinding(*half_edge_id),
            )?;
            if half_edge_binding.kind != TopologyEntityKind::HalfEdge {
                return Err(TopologyOperatorExecutionError::ExistingEntityKindMismatch {
                    entity_id: *half_edge_id,
                    expected: TopologyEntityKind::HalfEdge,
                    actual: half_edge_binding.kind,
                });
            }
            moved_half_edge_identities.insert(half_edge_binding.query_identity.clone());
            half_edge_bindings.push(half_edge_binding);
        }
        let mut relation_bindings_by_target = BTreeMap::new();
        for relation_id in outgoing_relation_ids {
            let relation_binding =
                crate::topology_operators::application::bindings::query_relation_binding(
                    bindings,
                    relation_id,
                )?
                .ok_or(
                    TopologyOperatorExecutionError::MissingExistingRelationBinding(relation_id),
                )?;
            if relation_binding.source_query_identity != retained_wire_binding.query_identity {
                return Err(
                    TopologyOperatorExecutionError::ExistingRelationSourceMismatch {
                        relation_id,
                        expected_source_entity_id: retained_wire_id,
                        actual_source_identity: relation_binding.source_query_identity,
                    },
                );
            }
            if moved_half_edge_identities.contains(&relation_binding.target_query_identity) {
                relation_bindings_by_target.insert(
                    relation_binding.target_query_identity.clone(),
                    (relation_id, relation_binding),
                );
            }
        }
        if relation_bindings_by_target.len() != program.half_edge_ids.len() {
            return Err(
                TopologyOperatorExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                    entity_id: retained_wire_id,
                    relation_kind:
                        schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge,
                    expected: program.half_edge_ids.len(),
                    actual: relation_bindings_by_target.len(),
                },
            );
        }
        let dependency_path = topology_relation_dependency_path(
            schema::facade::platform::relations::RelationKind::Topology(
                schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge,
            ),
        );
        let retained_wire_identity = retained_wire_binding.query_identity.clone();
        let created_wire_key = program.create_key.clone();
        let mut builder = ForgeQueryMutationBatchBuilder::new().insert_symbolic(
            program.create_key.as_str(),
            "TopologyEntity",
            |mutation| {
                mutation
                    .aspect("topology.kind", TopologyEntityKind::Wire.kind_name())
                    .aspect("topology.structure", program.create_key.as_str())
                    .aspect("naming.persistent_name", program.create_key.as_str())
            },
        );
        for half_edge_binding in half_edge_bindings {
            let (relation_id, relation_binding) = relation_bindings_by_target
                .remove(&half_edge_binding.query_identity)
                .ok_or(
                    TopologyOperatorExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                        entity_id: retained_wire_id,
                        relation_kind: schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge,
                        expected: program.half_edge_ids.len(),
                        actual: 0,
                    },
                )?;
            let relation_handle = self.workspace.bind_existing_relation(
                ForgeQueryExistingRelationTarget::new(
                    format!("{relation_id:?}"),
                    relation_binding.query_identity,
                )?
                .in_target_collection("TopologyRelation")?,
            )?;
            let half_edge_identity = half_edge_binding.query_identity.clone();
            builder = builder.update_existing_verified(
                relation_handle,
                |verify| {
                    let verify = verify
                        .aspect(
                            "topology.kind",
                            schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                        )
                        .aspect("topology.source_identity", retained_wire_identity.clone())
                        .aspect("topology.target_identity", half_edge_identity.clone());
                    if let Some(path) = dependency_path {
                        verify.aspect(
                            path,
                            schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                        )
                    } else {
                        verify
                    }
                },
                |update| {
                    let update = update
                        .aspect(
                            "topology.kind",
                            schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                        )
                        .symbolic_entity_identity(
                            "topology.source_identity",
                            ForgeQuerySymbolicTargetReference::new(created_wire_key.clone())
                                .expect("created entity keys are non-empty"),
                        )
                        .aspect("topology.target_identity", half_edge_identity.clone());
                    if let Some(path) = dependency_path {
                        update.aspect(
                            path,
                            schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                        )
                    } else {
                        update
                    }
                },
            );
        }
        Ok(builder)
    }
}
