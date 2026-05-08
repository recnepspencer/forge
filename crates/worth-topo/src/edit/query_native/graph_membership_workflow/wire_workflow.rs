use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryEntity, ForgeQuerySymbolicTargetReference,
};
use schema::facade::{TopologyEntityKind, TopologyRelationKind};

use super::shared::{
    bind_existing_entity_handle, bind_existing_relation_handle, delete_existing_entity_from_graph,
};
use crate::edit::TopologyEditContract;
use crate::edit::{TopologyQueryEditExecutionError, TopologyQueryEditRunner};
use crate::query::topology_relation_dependency_path;

impl<'workspace, 'assembly> TopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn compose_wire_rehome_workflow(
        &mut self,
        workflow: super::super::relation_wire_rehome_support::WireRehomeWorkflow,
        contracts: &[TopologyEditContract],
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
    ) -> Result<ForgeQueryBatchWriteReceipt, TopologyQueryEditExecutionError> {
        let retired_wire_binding =
            super::super::bindings::query_entity_binding(entity_rows, workflow.retired_wire_id)?
                .ok_or(
                    TopologyQueryEditExecutionError::MissingExistingEntityBinding(
                        workflow.retired_wire_id,
                    ),
                )?;
        let dependency_path = topology_relation_dependency_path(
            schema::facade::RelationKind::Topology(TopologyRelationKind::WireOwnsHalfEdge),
        );
        let created_wire_key = workflow.create_key.clone();
        let retired_wire_identity = retired_wire_binding.query_identity.clone();
        let retire_contract = contracts
            .last()
            .expect("wire rehome workflow always ends with retire contract");
        let retired_wire_handle = bind_existing_entity_handle(
            self,
            entity_rows,
            workflow.retired_wire_id,
            TopologyEntityKind::Wire,
        )?;
        let mut relation_rows_to_move = Vec::with_capacity(workflow.half_edge_ids.len());
        for half_edge_id in &workflow.half_edge_ids {
            let half_edge_binding =
                super::super::bindings::query_entity_binding(entity_rows, *half_edge_id)?.ok_or(
                    TopologyQueryEditExecutionError::MissingExistingEntityBinding(*half_edge_id),
                )?;
            let incoming_relation_ids = super::super::bindings::query_incoming_relation_ids(
                relation_rows,
                &half_edge_binding.query_identity,
                TopologyRelationKind::WireOwnsHalfEdge,
            )?;
            let [relation_id] = incoming_relation_ids.as_slice() else {
                return Err(
                    TopologyQueryEditExecutionError::ExistingEntityIncomingRelationCountMismatch {
                        entity_id: *half_edge_id,
                        relation_kind: TopologyRelationKind::WireOwnsHalfEdge,
                        expected: 1,
                        actual: incoming_relation_ids.len(),
                    },
                );
            };
            let relation_binding =
                super::super::bindings::query_relation_binding(relation_rows, *relation_id)?
                    .ok_or(
                        TopologyQueryEditExecutionError::MissingExistingRelationBinding(
                            *relation_id,
                        ),
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

    pub(super) fn compose_wire_split_workflow(
        &mut self,
        workflow: super::super::relation_wire_rehome_support::WireSplitWorkflow,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
    ) -> Result<ForgeQueryBatchWriteReceipt, TopologyQueryEditExecutionError> {
        let retained_wire_id = workflow
            .retained_wire_id
            .expect("resolved wire split workflow always sets retained wire id");
        let retained_wire_binding =
            super::super::bindings::query_entity_binding(entity_rows, retained_wire_id)?.ok_or(
                TopologyQueryEditExecutionError::MissingExistingEntityBinding(retained_wire_id),
            )?;
        let dependency_path = topology_relation_dependency_path(
            schema::facade::RelationKind::Topology(TopologyRelationKind::WireOwnsHalfEdge),
        );
        let created_wire_key = workflow.create_key.clone();
        let retained_wire_identity = retained_wire_binding.query_identity.clone();
        let mut moved_relations = Vec::with_capacity(workflow.half_edge_ids.len());
        for half_edge_id in &workflow.half_edge_ids {
            let half_edge_binding =
                super::super::bindings::query_entity_binding(entity_rows, *half_edge_id)?.ok_or(
                    TopologyQueryEditExecutionError::MissingExistingEntityBinding(*half_edge_id),
                )?;
            let incoming_relation_ids = super::super::bindings::query_incoming_relation_ids(
                relation_rows,
                &half_edge_binding.query_identity,
                TopologyRelationKind::WireOwnsHalfEdge,
            )?;
            let [relation_id] = incoming_relation_ids.as_slice() else {
                return Err(
                    TopologyQueryEditExecutionError::ExistingEntityIncomingRelationCountMismatch {
                        entity_id: *half_edge_id,
                        relation_kind: TopologyRelationKind::WireOwnsHalfEdge,
                        expected: 1,
                        actual: incoming_relation_ids.len(),
                    },
                );
            };
            let relation_binding =
                super::super::bindings::query_relation_binding(relation_rows, *relation_id)?
                    .ok_or(
                        TopologyQueryEditExecutionError::MissingExistingRelationBinding(
                            *relation_id,
                        ),
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
