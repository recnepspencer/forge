use forge_query::facade::{ForgeQueryBatchWriteReceipt, ForgeQuerySymbolicTargetReference};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use super::shared::{
    bind_existing_entity_handle, bind_existing_relation_handle, delete_existing_entity_from_graph,
};
use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::topology_relation_dependency_path;
use crate::topology_operators::TopologyEditContract;
use crate::topology_operators::{TopologyOperatorExecutionError, TopologyOperatorRunner};

impl<'workspace, 'assembly> TopologyOperatorRunner<'workspace, 'assembly> {
    pub(super) fn compose_wire_rehome_program(
        &mut self,
        program: super::super::wire_rehome_support::WireRehomeProgram,
        contracts: &[TopologyEditContract],
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<ForgeQueryBatchWriteReceipt, TopologyOperatorExecutionError> {
        let retired_wire_binding =
            crate::topology_operators::application::bindings::query_entity_binding(
                bindings,
                program.retired_wire_id,
            )?
            .ok_or(
                TopologyOperatorExecutionError::MissingExistingEntityBinding(
                    program.retired_wire_id,
                ),
            )?;
        let dependency_path = topology_relation_dependency_path(
            schema::facade::platform::relations::RelationKind::Topology(TopologyRelationKind::WireOwnsHalfEdge),
        );
        let created_wire_key = program.create_key.clone();
        let retired_wire_identity = retired_wire_binding.query_identity.clone();
        let retire_contract = contracts
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
                    TopologyOperatorExecutionError::MissingExistingEntityBinding(*half_edge_id),
                )?;
            let incoming_relation_ids =
                crate::topology_operators::application::bindings::query_incoming_relation_ids(
                    bindings,
                    &half_edge_binding.query_identity,
                    TopologyRelationKind::WireOwnsHalfEdge,
                )?;
            let [relation_id] = incoming_relation_ids.as_slice() else {
                return Err(
                    TopologyOperatorExecutionError::ExistingEntityIncomingRelationCountMismatch {
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
                    TopologyOperatorExecutionError::MissingExistingRelationBinding(*relation_id),
                )?;
            relation_rows_to_move.push((
                bind_existing_relation_handle(
                    self,
                    *relation_id,
                    &relation_binding.query_identity,
                )?,
                *relation_id,
                half_edge_binding.query_identity,
            ));
        }
        self.workspace
            .compose_graph(|graph| {
                graph.insert_entity(created_wire_key.clone(), "TopologyEntity", |mutation| {
                    mutation
                        .aspect("topology.kind", TopologyEntityKind::Wire.kind_name())
                        .aspect("topology.structure", created_wire_key.clone())
                        .aspect("naming.persistent_name", created_wire_key.clone())
                })?;
                for (relation_handle, relation_id, half_edge_identity) in &relation_rows_to_move {
                    graph.retarget_existing_verified(
                        relation_handle.clone(),
                        |verify| {
                            let verify = verify
                                .aspect(
                                    "topology.kind",
                                    TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                                )
                                .aspect("topology.source_identity", retired_wire_identity.clone())
                                .aspect("topology.target_identity", half_edge_identity.clone());
                            if let Some(path) = dependency_path {
                                verify.aspect(
                                    path,
                                    TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                                )
                            } else {
                                verify
                            }
                        },
                        |update| {
                            let update = update
                                .continuity_rebind_existing_target(
                                    format!("{relation_id:?}"),
                                    format!("{relation_id:?}:successor"),
                                )
                                .aspect(
                                    "topology.kind",
                                    TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                                )
                                .symbolic_entity_identity(
                                    "topology.source_identity",
                                    ForgeQuerySymbolicTargetReference::new(
                                        created_wire_key.clone(),
                                    )
                                    .expect("created entity keys are non-empty"),
                                )
                                .aspect("topology.target_identity", half_edge_identity.clone());
                            if let Some(path) = dependency_path {
                                update.aspect(
                                    path,
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
                    retire_contract,
                )?;
                Ok(())
            })
            .map_err(Into::into)
    }

    pub(super) fn compose_wire_split_program(
        &mut self,
        program: super::super::wire_rehome_support::WireSplitProgram,
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<ForgeQueryBatchWriteReceipt, TopologyOperatorExecutionError> {
        let retained_wire_id = program
            .retained_wire_id
            .expect("resolved wire split program always sets retained wire id");
        let retained_wire_binding =
            crate::topology_operators::application::bindings::query_entity_binding(
                bindings,
                retained_wire_id,
            )?
            .ok_or(
                TopologyOperatorExecutionError::MissingExistingEntityBinding(retained_wire_id),
            )?;
        let dependency_path = topology_relation_dependency_path(
            schema::facade::platform::relations::RelationKind::Topology(TopologyRelationKind::WireOwnsHalfEdge),
        );
        let created_wire_key = program.create_key.clone();
        let retained_wire_identity = retained_wire_binding.query_identity.clone();
        let mut moved_relations = Vec::with_capacity(program.half_edge_ids.len());
        for half_edge_id in &program.half_edge_ids {
            let half_edge_binding =
                crate::topology_operators::application::bindings::query_entity_binding(
                    bindings,
                    *half_edge_id,
                )?
                .ok_or(
                    TopologyOperatorExecutionError::MissingExistingEntityBinding(*half_edge_id),
                )?;
            let incoming_relation_ids =
                crate::topology_operators::application::bindings::query_incoming_relation_ids(
                    bindings,
                    &half_edge_binding.query_identity,
                    TopologyRelationKind::WireOwnsHalfEdge,
                )?;
            let [relation_id] = incoming_relation_ids.as_slice() else {
                return Err(
                    TopologyOperatorExecutionError::ExistingEntityIncomingRelationCountMismatch {
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
                    TopologyOperatorExecutionError::MissingExistingRelationBinding(*relation_id),
                )?;
            moved_relations.push((
                bind_existing_relation_handle(
                    self,
                    *relation_id,
                    &relation_binding.query_identity,
                )?,
                *relation_id,
                half_edge_binding.query_identity,
            ));
        }
        self.workspace
            .compose_graph(|graph| {
                graph.insert_entity(created_wire_key.clone(), "TopologyEntity", |mutation| {
                    mutation
                        .aspect("topology.kind", TopologyEntityKind::Wire.kind_name())
                        .aspect("topology.structure", created_wire_key.clone())
                        .aspect("naming.persistent_name", created_wire_key.clone())
                })?;
                for (relation_handle, relation_id, half_edge_identity) in &moved_relations {
                    graph.retarget_existing_verified(
                        relation_handle.clone(),
                        |verify| {
                            let verify = verify
                                .aspect(
                                    "topology.kind",
                                    TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                                )
                                .aspect("topology.source_identity", retained_wire_identity.clone())
                                .aspect("topology.target_identity", half_edge_identity.clone());
                            if let Some(path) = dependency_path {
                                verify.aspect(
                                    path,
                                    TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                                )
                            } else {
                                verify
                            }
                        },
                        |update| {
                            let update = update
                                .continuity_rebind_existing_target(
                                    format!("{relation_id:?}"),
                                    format!("{relation_id:?}:successor"),
                                )
                                .aspect(
                                    "topology.kind",
                                    TopologyRelationKind::WireOwnsHalfEdge.kind_name(),
                                )
                                .symbolic_entity_identity(
                                    "topology.source_identity",
                                    ForgeQuerySymbolicTargetReference::new(
                                        created_wire_key.clone(),
                                    )
                                    .expect("created entity keys are non-empty"),
                                )
                                .aspect("topology.target_identity", half_edge_identity.clone());
                            if let Some(path) = dependency_path {
                                update.aspect(
                                    path,
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
            .map_err(Into::into)
    }
}




