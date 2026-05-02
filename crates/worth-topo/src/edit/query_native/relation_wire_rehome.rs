use std::collections::{BTreeMap, BTreeSet};

use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryExistingRelationTarget, ForgeQueryMutationBatchBuilder,
    ForgeQuerySymbolicTargetReference,
};
use worth_schema::facade::WorthTopologyEntityKind;

use super::bindings::{query_entity_binding, query_outgoing_relation_ids};
use super::relation_wire_rehome_support::{
    parse_wire_rehome_workflow, resolve_wire_split_workflow,
};
use super::{WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner};
use crate::edit::WorthTopologyEditContract;
use crate::query::topology_relation_dependency_path;

impl<'workspace, 'assembly> WorthTopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn lower_rehome_owned_half_edge_set_to_new_wire_workflow(
        &self,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
        contracts: &[WorthTopologyEditContract],
    ) -> Result<ForgeQueryMutationBatchBuilder, WorthTopologyQueryEditExecutionError> {
        let Some(workflow) = parse_wire_rehome_workflow(contracts) else {
            return Err(WorthTopologyQueryEditExecutionError::UnsupportedFamilies(
                vec![crate::edit::WorthTopologyEditFamily::AttachShellOrWireMembership],
            ));
        };
        let retired_wire_binding = query_entity_binding(entity_rows, workflow.retired_wire_id)?
            .ok_or(
                WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(
                    workflow.retired_wire_id,
                ),
            )?;
        let outgoing_relation_ids = query_outgoing_relation_ids(
            relation_rows,
            &retired_wire_binding.query_identity,
            worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge,
        )?;
        if outgoing_relation_ids.len() != workflow.half_edge_ids.len() {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                    entity_id: workflow.retired_wire_id,
                    relation_kind:
                        worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge,
                    expected: workflow.half_edge_ids.len(),
                    actual: outgoing_relation_ids.len(),
                },
            );
        }
        let mut half_edge_bindings = Vec::with_capacity(workflow.half_edge_ids.len());
        let mut expected_half_edge_identities = BTreeSet::new();
        for half_edge_id in &workflow.half_edge_ids {
            let half_edge_binding = query_entity_binding(entity_rows, *half_edge_id)?.ok_or(
                WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(*half_edge_id),
            )?;
            if half_edge_binding.kind != WorthTopologyEntityKind::HalfEdge {
                return Err(
                    WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                        entity_id: *half_edge_id,
                        expected: WorthTopologyEntityKind::HalfEdge,
                        actual: half_edge_binding.kind,
                    },
                );
            }
            expected_half_edge_identities.insert(half_edge_binding.query_identity.clone());
            half_edge_bindings.push((*half_edge_id, half_edge_binding));
        }
        let mut relation_bindings_by_target = BTreeMap::new();
        for relation_id in outgoing_relation_ids {
            let relation_binding =
                super::bindings::query_relation_binding(relation_rows, relation_id)?.ok_or(
                    WorthTopologyQueryEditExecutionError::MissingExistingRelationBinding(
                        relation_id,
                    ),
                )?;
            if relation_binding.source_query_identity != retired_wire_binding.query_identity {
                return Err(
                    WorthTopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
                        relation_id,
                        expected_source_entity_id: workflow.retired_wire_id,
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
                WorthTopologyQueryEditExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                    entity_id: workflow.retired_wire_id,
                    relation_kind:
                        worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge,
                    expected: workflow.half_edge_ids.len(),
                    actual: 0,
                },
            );
        }
        let dependency_path =
            topology_relation_dependency_path(worth_schema::facade::WorthRelationKind::Topology(
                worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge,
            ));
        let retired_wire_identity = retired_wire_binding.query_identity.clone();
        let created_wire_key = workflow.create_key.clone();
        let mut builder = ForgeQueryMutationBatchBuilder::new().insert_symbolic(
            workflow.create_key.as_str(),
            "WorthTopologyEntity",
            |mutation| {
                mutation
                    .aspect("topology.kind", WorthTopologyEntityKind::Wire.kind_name())
                    .aspect("topology.structure", workflow.create_key.as_str())
                    .aspect("naming.persistent_name", workflow.create_key.as_str())
            },
        );
        for (half_edge_id, half_edge_binding) in half_edge_bindings {
            let (relation_id, relation_binding) = relation_bindings_by_target
                .remove(&half_edge_binding.query_identity)
                .ok_or(
                    WorthTopologyQueryEditExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                        entity_id: workflow.retired_wire_id,
                        relation_kind:
                            worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge,
                        expected: workflow.half_edge_ids.len(),
                        actual: 0,
                    },
                )?;
            let relation_handle = self.workspace.bind_existing_relation(
                ForgeQueryExistingRelationTarget::new(
                    format!("{relation_id:?}"),
                    relation_binding.query_identity,
                )?
                .in_target_collection("WorthTopologyRelation")?,
            )?;
            let half_edge_identity = half_edge_binding.query_identity.clone();
            builder = builder.update_existing_verified(
                relation_handle,
                |verify| {
                    let verify = verify
                        .aspect(
                            "topology.kind",
                            worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge
                                .kind_name(),
                        )
                        .aspect("topology.source_identity", retired_wire_identity.clone())
                        .aspect("topology.target_identity", half_edge_identity.clone());
                    if let Some(path) = dependency_path {
                        verify.aspect(
                            path,
                            worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge
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
                            worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge
                                .kind_name(),
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
                            worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge
                                .kind_name(),
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
            entity_rows,
            workflow.retired_wire_id,
            WorthTopologyEntityKind::Wire,
            contracts
                .last()
                .expect("wire rehome workflow always has retire contract"),
        )
    }

    pub(super) fn lower_split_connected_half_edge_set_to_new_wire_workflow(
        &self,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
        contracts: &[WorthTopologyEditContract],
    ) -> Result<ForgeQueryMutationBatchBuilder, WorthTopologyQueryEditExecutionError> {
        let Some(workflow) = resolve_wire_split_workflow(entity_rows, relation_rows, contracts)
        else {
            return Err(WorthTopologyQueryEditExecutionError::UnsupportedFamilies(
                vec![crate::edit::WorthTopologyEditFamily::AttachShellOrWireMembership],
            ));
        };
        let retained_wire_id = workflow
            .retained_wire_id
            .expect("resolved wire split workflow always sets retained wire id");
        let retained_wire_binding = query_entity_binding(entity_rows, retained_wire_id)?.ok_or(
            WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(retained_wire_id),
        )?;
        let outgoing_relation_ids = query_outgoing_relation_ids(
            relation_rows,
            &retained_wire_binding.query_identity,
            worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge,
        )?;
        let mut half_edge_bindings = Vec::with_capacity(workflow.half_edge_ids.len());
        let mut moved_half_edge_identities = BTreeSet::new();
        for half_edge_id in &workflow.half_edge_ids {
            let half_edge_binding = query_entity_binding(entity_rows, *half_edge_id)?.ok_or(
                WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(*half_edge_id),
            )?;
            if half_edge_binding.kind != WorthTopologyEntityKind::HalfEdge {
                return Err(
                    WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                        entity_id: *half_edge_id,
                        expected: WorthTopologyEntityKind::HalfEdge,
                        actual: half_edge_binding.kind,
                    },
                );
            }
            moved_half_edge_identities.insert(half_edge_binding.query_identity.clone());
            half_edge_bindings.push(half_edge_binding);
        }
        let mut relation_bindings_by_target = BTreeMap::new();
        for relation_id in outgoing_relation_ids {
            let relation_binding =
                super::bindings::query_relation_binding(relation_rows, relation_id)?.ok_or(
                    WorthTopologyQueryEditExecutionError::MissingExistingRelationBinding(
                        relation_id,
                    ),
                )?;
            if relation_binding.source_query_identity != retained_wire_binding.query_identity {
                return Err(
                    WorthTopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
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
        if relation_bindings_by_target.len() != workflow.half_edge_ids.len() {
            return Err(
                WorthTopologyQueryEditExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                    entity_id: retained_wire_id,
                    relation_kind:
                        worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge,
                    expected: workflow.half_edge_ids.len(),
                    actual: relation_bindings_by_target.len(),
                },
            );
        }
        let dependency_path =
            topology_relation_dependency_path(worth_schema::facade::WorthRelationKind::Topology(
                worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge,
            ));
        let retained_wire_identity = retained_wire_binding.query_identity.clone();
        let created_wire_key = workflow.create_key.clone();
        let mut builder = ForgeQueryMutationBatchBuilder::new().insert_symbolic(
            workflow.create_key.as_str(),
            "WorthTopologyEntity",
            |mutation| {
                mutation
                    .aspect("topology.kind", WorthTopologyEntityKind::Wire.kind_name())
                    .aspect("topology.structure", workflow.create_key.as_str())
                    .aspect("naming.persistent_name", workflow.create_key.as_str())
            },
        );
        for half_edge_binding in half_edge_bindings {
            let (relation_id, relation_binding) = relation_bindings_by_target
                .remove(&half_edge_binding.query_identity)
                .ok_or(
                    WorthTopologyQueryEditExecutionError::ExistingEntityOutgoingRelationCountMismatch {
                        entity_id: retained_wire_id,
                        relation_kind:
                            worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge,
                        expected: workflow.half_edge_ids.len(),
                        actual: 0,
                    },
                )?;
            let relation_handle = self.workspace.bind_existing_relation(
                ForgeQueryExistingRelationTarget::new(
                    format!("{relation_id:?}"),
                    relation_binding.query_identity,
                )?
                .in_target_collection("WorthTopologyRelation")?,
            )?;
            let half_edge_identity = half_edge_binding.query_identity.clone();
            builder = builder.update_existing_verified(
                relation_handle,
                |verify| {
                    let verify = verify
                        .aspect(
                            "topology.kind",
                            worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge
                                .kind_name(),
                        )
                        .aspect("topology.source_identity", retained_wire_identity.clone())
                        .aspect("topology.target_identity", half_edge_identity.clone());
                    if let Some(path) = dependency_path {
                        verify.aspect(
                            path,
                            worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge
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
                            worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge
                                .kind_name(),
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
                            worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge
                                .kind_name(),
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
