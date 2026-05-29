use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryExistingEntityTarget, ForgeQueryExistingRelationTarget,
    ForgeQueryRuntimeError, ForgeQueryWorkspace, ForgeQueryWorkspaceError,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::{
    admit_query_mutation_batch, DerivedTopologyReadBasis, EntityKind, EntityReference,
    QueryMutationAdmission, QueryMutationAdmissionReport, RawTopologyIntent, RelationKind,
    TopologyMutation,
};

use crate::projection::TopologyQueryMutationEvidence;
use crate::topology_operators::topology_relation_dependency_path;

use super::authority_support::{
    index_imported_entities, index_imported_relations, mutation_evidence_for_intent,
    relation_touched_aspects,
};
use super::TopologyQueryAssembly;

#[derive(Debug, Clone)]
pub struct TopologyQueryAppliedIntent {
    pub receipt: ForgeQueryBatchWriteReceipt,
    pub mutation_evidence: TopologyQueryMutationEvidence,
}

#[derive(Debug)]
pub enum TopologyQueryApplyError {
    AdmissionBlocked(Vec<QueryMutationAdmissionReport>),
    MissingCreatedEntityReference(String),
    MissingExistingEntityBinding(EntityId),
    MissingExistingRelationBinding(RelationId),
    MissingExistingEntityKind(EntityId),
    MissingExistingRelationKind(RelationId),
    UnsupportedMutation(String),
    Query(ForgeQueryRuntimeError),
}

impl std::fmt::Display for TopologyQueryApplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AdmissionBlocked(rows) => write!(
                f,
                " query apply is blocked for this raw intent: {}",
                rows.iter()
                    .map(|row| row.blocker.message())
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
            Self::MissingCreatedEntityReference(key) => {
                write!(
                    f,
                    "missing same-batch created entity `{key}` during query lowering"
                )
            }
            Self::MissingExistingEntityBinding(entity_id) => write!(
                f,
                "missing imported query entity binding for authoritative `{entity_id:?}`"
            ),
            Self::MissingExistingRelationBinding(relation_id) => write!(
                f,
                "missing imported query relation binding for authoritative `{relation_id:?}`"
            ),
            Self::MissingExistingEntityKind(entity_id) => write!(
                f,
                "missing imported query topology kind for authoritative entity `{entity_id:?}`"
            ),
            Self::MissingExistingRelationKind(relation_id) => write!(
                f,
                "missing imported query topology kind for authoritative relation `{relation_id:?}`"
            ),
            Self::UnsupportedMutation(summary) => {
                write!(f, "query apply does not support `{summary}`")
            }
            Self::Query(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for TopologyQueryApplyError {}

impl From<ForgeQueryRuntimeError> for TopologyQueryApplyError {
    fn from(value: ForgeQueryRuntimeError) -> Self {
        Self::Query(value)
    }
}

impl From<ForgeQueryWorkspaceError> for TopologyQueryApplyError {
    fn from(value: ForgeQueryWorkspaceError) -> Self {
        Self::Query(ForgeQueryRuntimeError::Workspace(value))
    }
}

#[derive(Debug, Clone)]
pub(super) struct ImportedTopologyEntity {
    pub(super) query_identity: String,
    pub(super) kind: EntityKind,
}

#[derive(Debug, Clone)]
pub(super) struct ImportedTopologyRelation {
    pub(super) query_identity: String,
    pub(super) kind: RelationKind,
    pub(super) source_query_identity: String,
    pub(super) target_query_identity: String,
}

impl TopologyQueryAssembly {
    pub fn apply_raw_intent(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        intent: RawTopologyIntent,
        read_basis: &DerivedTopologyReadBasis,
    ) -> Result<TopologyQueryAppliedIntent, TopologyQueryApplyError> {
        let admission = admit_query_mutation_batch(&intent);
        if let QueryMutationAdmission::Blocked(rows) = admission {
            return Err(TopologyQueryApplyError::AdmissionBlocked(rows));
        }

        let entities = index_imported_entities(workspace.read(self.entities()))?;
        let relations = index_imported_relations(workspace.read(self.relations()))?;
        let mutation_evidence =
            mutation_evidence_for_intent(read_basis, &intent, &entities, &relations)?;
        let mut created_entities = BTreeMap::<String, String>::new();
        let mut receipts = Vec::new();

        for mutation in intent.mutations {
            match mutation {
                TopologyMutation::CreateEntity { create_key, kind } => {
                    let EntityKind::Topology(_) = kind else {
                        return Err(TopologyQueryApplyError::UnsupportedMutation(format!(
                            "{kind:?}"
                        )));
                    };
                    let receipt = workspace.insert("TopologyEntity", |builder| {
                        builder
                            .metadata(
                                TopologyQueryMutationEvidence::metadata_key(),
                                &mutation_evidence,
                            )
                            .aspect("topology.kind", kind.kind_name())
                            .aspect("topology.structure", create_key.as_str())
                            .aspect("naming.persistent_name", create_key.as_str())
                    })?;
                    created_entities.insert(
                        create_key.as_str().to_string(),
                        receipt.deltas()[0].entity_identity.clone(),
                    );
                    receipts.push(receipt);
                }
                TopologyMutation::CreateRelation {
                    kind,
                    source,
                    target,
                    ..
                } => {
                    let source_identity =
                        resolve_entity_reference(&source, &entities, &created_entities)?;
                    let target_identity =
                        resolve_entity_reference(&target, &entities, &created_entities)?;
                    receipts.push(workspace.insert("TopologyRelation", |builder| {
                        let builder = builder
                            .metadata(
                                TopologyQueryMutationEvidence::metadata_key(),
                                &mutation_evidence,
                            )
                            .aspect("topology.kind", kind.kind_name())
                            .aspect("topology.source_identity", source_identity.clone())
                            .aspect("topology.target_identity", target_identity.clone());
                        if let Some(path) = topology_relation_dependency_path(kind) {
                            builder.aspect(path, kind.kind_name())
                        } else {
                            builder
                        }
                    })?);
                }
                TopologyMutation::RemoveEntity { entity_id } => {
                    let imported = entities.get(&entity_id).ok_or(
                        TopologyQueryApplyError::MissingExistingEntityBinding(entity_id),
                    )?;
                    let binding = workspace.bind_existing_entity(
                        ForgeQueryExistingEntityTarget::new(
                            format!("{entity_id:?}"),
                            imported.query_identity.clone(),
                        )?
                        .in_target_collection("TopologyEntity")?,
                    )?;
                    receipts.push(workspace.delete_existing_with(binding, |delete| {
                        delete
                            .target_collection("TopologyEntity")
                            .touches(mutation_evidence.touched_aspect_paths.clone())
                            .metadata(
                                TopologyQueryMutationEvidence::metadata_key(),
                                &mutation_evidence,
                            )
                    })?);
                }
                TopologyMutation::RemoveRelation { relation_id } => {
                    let imported = relations.get(&relation_id).ok_or(
                        TopologyQueryApplyError::MissingExistingRelationBinding(relation_id),
                    )?;
                    let binding = workspace.bind_existing_relation(
                        ForgeQueryExistingRelationTarget::new(
                            format!("{relation_id:?}"),
                            imported.query_identity.clone(),
                        )?
                        .in_target_collection("TopologyRelation")?,
                    )?;
                    receipts.push(workspace.delete_existing_with(binding, |delete| {
                        delete
                            .target_collection("TopologyRelation")
                            .touches(schema::facade::query_aspect_path_strings(
                                relation_touched_aspects(imported.kind),
                            ))
                            .metadata(
                                TopologyQueryMutationEvidence::metadata_key(),
                                &mutation_evidence,
                            )
                    })?);
                }
                TopologyMutation::UpsertEntity { entity_id, kind } => {
                    let imported = entities.get(&entity_id).ok_or(
                        TopologyQueryApplyError::MissingExistingEntityBinding(entity_id),
                    )?;
                    let binding = workspace.bind_existing_entity(
                        ForgeQueryExistingEntityTarget::new(
                            format!("{entity_id:?}"),
                            imported.query_identity.clone(),
                        )?
                        .in_target_collection("TopologyEntity")?,
                    )?;
                    receipts.push(workspace.verify_existing(binding, |assertion| {
                        assertion
                            .metadata(
                                TopologyQueryMutationEvidence::metadata_key(),
                                &mutation_evidence,
                            )
                            .aspect("topology.kind", kind.kind_name())
                    })?);
                }
                TopologyMutation::UpsertRelation {
                    relation_id,
                    kind,
                    source,
                    target,
                } => {
                    let imported = relations.get(&relation_id).ok_or(
                        TopologyQueryApplyError::MissingExistingRelationBinding(relation_id),
                    )?;
                    let expected_source = entities
                        .get(&source)
                        .ok_or(TopologyQueryApplyError::MissingExistingEntityBinding(
                            source,
                        ))?
                        .query_identity
                        .clone();
                    let expected_target = entities
                        .get(&target)
                        .ok_or(TopologyQueryApplyError::MissingExistingEntityBinding(
                            target,
                        ))?
                        .query_identity
                        .clone();
                    let binding = workspace.bind_existing_relation(
                        ForgeQueryExistingRelationTarget::new(
                            format!("{relation_id:?}"),
                            imported.query_identity.clone(),
                        )?
                        .in_target_collection("TopologyRelation")?,
                    )?;
                    if imported.kind != kind
                        || imported.source_query_identity != expected_source
                        || imported.target_query_identity != expected_target
                    {
                        receipts.push(workspace.update_existing_verified(
                            binding,
                            |assertion| {
                                assertion
                                    .aspect("topology.kind", imported.kind.kind_name())
                                    .aspect(
                                        "topology.source_identity",
                                        imported.source_query_identity.clone(),
                                    )
                                    .aspect(
                                        "topology.target_identity",
                                        imported.target_query_identity.clone(),
                                    )
                            },
                            |update| {
                                let update = update
                                    .metadata(
                                        TopologyQueryMutationEvidence::metadata_key(),
                                        &mutation_evidence,
                                    )
                                    .aspect("topology.kind", kind.kind_name())
                                    .aspect("topology.source_identity", expected_source)
                                    .aspect("topology.target_identity", expected_target);
                                if let Some(path) = topology_relation_dependency_path(kind) {
                                    update.aspect(path, kind.kind_name())
                                } else {
                                    update
                                }
                            },
                        )?);
                    } else {
                        receipts.push(workspace.verify_existing(binding, |assertion| {
                            let assertion = assertion
                                .metadata(
                                    TopologyQueryMutationEvidence::metadata_key(),
                                    &mutation_evidence,
                                )
                                .aspect("topology.kind", kind.kind_name())
                                .aspect("topology.source_identity", expected_source)
                                .aspect("topology.target_identity", expected_target);
                            if let Some(path) = topology_relation_dependency_path(kind) {
                                assertion.aspect(path, kind.kind_name())
                            } else {
                                assertion
                            }
                        })?);
                    }
                }
            }
        }

        Ok(TopologyQueryAppliedIntent {
            receipt: ForgeQueryBatchWriteReceipt::from_write_receipts(receipts)?,
            mutation_evidence,
        })
    }
}

fn resolve_entity_reference(
    reference: &EntityReference,
    entities: &BTreeMap<EntityId, ImportedTopologyEntity>,
    created_entities: &BTreeMap<String, String>,
) -> Result<String, TopologyQueryApplyError> {
    match reference {
        EntityReference::Existing(entity_id) => entities
            .get(entity_id)
            .map(|entry| entry.query_identity.clone())
            .ok_or(TopologyQueryApplyError::MissingExistingEntityBinding(
                *entity_id,
            )),
        EntityReference::Created(create_key) => created_entities
            .get(create_key.as_str())
            .cloned()
            .ok_or_else(|| {
                TopologyQueryApplyError::MissingCreatedEntityReference(
                    create_key.as_str().to_string(),
                )
            }),
    }
}
