use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryExistingEntityTarget, ForgeQueryExistingRelationTarget,
    ForgeQueryRuntimeError, ForgeQueryWorkspace, ForgeQueryWorkspaceError,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use worth_schema::facade::{
    admit_worth_query_mutation_batch, DerivedTopologyReadBasis, RawWorthTopologyIntent,
    WorthEntityKind, WorthEntityReference, WorthQueryMutationAdmission,
    WorthQueryMutationAdmissionReport, WorthRelationKind, WorthTopologyMutation,
};

use crate::query::materialized::topology_relation_dependency_path;
use crate::query::WorthTopologyQueryMutationEvidence;

use super::authority_support::{
    index_imported_entities, index_imported_relations, mutation_evidence_for_intent,
    relation_touched_aspects,
};
use super::WorthTopologyQueryAssembly;

#[derive(Debug, Clone)]
pub struct WorthTopologyQueryAppliedIntent {
    pub receipt: ForgeQueryBatchWriteReceipt,
    pub mutation_evidence: WorthTopologyQueryMutationEvidence,
}

#[derive(Debug)]
pub enum WorthTopologyQueryApplyError {
    AdmissionBlocked(Vec<WorthQueryMutationAdmissionReport>),
    MissingCreatedEntityReference(String),
    MissingExistingEntityBinding(EntityId),
    MissingExistingRelationBinding(RelationId),
    MissingExistingEntityKind(EntityId),
    MissingExistingRelationKind(RelationId),
    UnsupportedMutation(String),
    Query(ForgeQueryRuntimeError),
}

impl std::fmt::Display for WorthTopologyQueryApplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AdmissionBlocked(rows) => write!(
                f,
                "worth query apply is blocked for this raw intent: {}",
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

impl std::error::Error for WorthTopologyQueryApplyError {}

impl From<ForgeQueryRuntimeError> for WorthTopologyQueryApplyError {
    fn from(value: ForgeQueryRuntimeError) -> Self {
        Self::Query(value)
    }
}

impl From<ForgeQueryWorkspaceError> for WorthTopologyQueryApplyError {
    fn from(value: ForgeQueryWorkspaceError) -> Self {
        Self::Query(ForgeQueryRuntimeError::Workspace(value))
    }
}

#[derive(Debug, Clone)]
pub(super) struct ImportedTopologyEntity {
    pub(super) query_identity: String,
    pub(super) kind: WorthEntityKind,
}

#[derive(Debug, Clone)]
pub(super) struct ImportedTopologyRelation {
    pub(super) query_identity: String,
    pub(super) kind: WorthRelationKind,
    pub(super) source_query_identity: String,
    pub(super) target_query_identity: String,
}

impl WorthTopologyQueryAssembly {
    pub fn apply_raw_intent(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        intent: RawWorthTopologyIntent,
        read_basis: &DerivedTopologyReadBasis,
    ) -> Result<WorthTopologyQueryAppliedIntent, WorthTopologyQueryApplyError> {
        let admission = admit_worth_query_mutation_batch(&intent);
        if let WorthQueryMutationAdmission::Blocked(rows) = admission {
            return Err(WorthTopologyQueryApplyError::AdmissionBlocked(rows));
        }

        let entities = index_imported_entities(workspace.read(self.entities()))?;
        let relations = index_imported_relations(workspace.read(self.relations()))?;
        let mutation_evidence =
            mutation_evidence_for_intent(read_basis, &intent, &entities, &relations)?;
        let mut created_entities = BTreeMap::<String, String>::new();
        let mut receipts = Vec::new();

        for mutation in intent.mutations {
            match mutation {
                WorthTopologyMutation::CreateEntity { create_key, kind } => {
                    let WorthEntityKind::Topology(_) = kind else {
                        return Err(WorthTopologyQueryApplyError::UnsupportedMutation(format!(
                            "{kind:?}"
                        )));
                    };
                    let receipt = workspace.insert("WorthTopologyEntity", |builder| {
                        builder
                            .metadata(
                                WorthTopologyQueryMutationEvidence::metadata_key(),
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
                WorthTopologyMutation::CreateRelation {
                    kind,
                    source,
                    target,
                    ..
                } => {
                    let source_identity =
                        resolve_entity_reference(&source, &entities, &created_entities)?;
                    let target_identity =
                        resolve_entity_reference(&target, &entities, &created_entities)?;
                    receipts.push(workspace.insert("WorthTopologyRelation", |builder| {
                        let builder = builder
                            .metadata(
                                WorthTopologyQueryMutationEvidence::metadata_key(),
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
                WorthTopologyMutation::RemoveEntity { entity_id } => {
                    let imported = entities.get(&entity_id).ok_or(
                        WorthTopologyQueryApplyError::MissingExistingEntityBinding(entity_id),
                    )?;
                    let binding = workspace.bind_existing_entity(
                        ForgeQueryExistingEntityTarget::new(
                            format!("{entity_id:?}"),
                            imported.query_identity.clone(),
                        )?
                        .in_target_collection("WorthTopologyEntity")?,
                    )?;
                    receipts.push(workspace.delete_existing_with(binding, |delete| {
                        delete
                            .target_collection("WorthTopologyEntity")
                            .touches(mutation_evidence.touched_aspect_paths.clone())
                            .metadata(
                                WorthTopologyQueryMutationEvidence::metadata_key(),
                                &mutation_evidence,
                            )
                    })?);
                }
                WorthTopologyMutation::RemoveRelation { relation_id } => {
                    let imported = relations.get(&relation_id).ok_or(
                        WorthTopologyQueryApplyError::MissingExistingRelationBinding(relation_id),
                    )?;
                    let binding = workspace.bind_existing_relation(
                        ForgeQueryExistingRelationTarget::new(
                            format!("{relation_id:?}"),
                            imported.query_identity.clone(),
                        )?
                        .in_target_collection("WorthTopologyRelation")?,
                    )?;
                    receipts.push(workspace.delete_existing_with(binding, |delete| {
                        delete
                            .target_collection("WorthTopologyRelation")
                            .touches(worth_schema::facade::worth_query_aspect_path_strings(
                                relation_touched_aspects(imported.kind),
                            ))
                            .metadata(
                                WorthTopologyQueryMutationEvidence::metadata_key(),
                                &mutation_evidence,
                            )
                    })?);
                }
                WorthTopologyMutation::UpsertEntity { entity_id, kind } => {
                    let imported = entities.get(&entity_id).ok_or(
                        WorthTopologyQueryApplyError::MissingExistingEntityBinding(entity_id),
                    )?;
                    let binding = workspace.bind_existing_entity(
                        ForgeQueryExistingEntityTarget::new(
                            format!("{entity_id:?}"),
                            imported.query_identity.clone(),
                        )?
                        .in_target_collection("WorthTopologyEntity")?,
                    )?;
                    receipts.push(workspace.verify_existing(binding, |assertion| {
                        assertion
                            .metadata(
                                WorthTopologyQueryMutationEvidence::metadata_key(),
                                &mutation_evidence,
                            )
                            .aspect("topology.kind", kind.kind_name())
                    })?);
                }
                WorthTopologyMutation::UpsertRelation {
                    relation_id,
                    kind,
                    source,
                    target,
                } => {
                    let imported = relations.get(&relation_id).ok_or(
                        WorthTopologyQueryApplyError::MissingExistingRelationBinding(relation_id),
                    )?;
                    let expected_source = entities
                        .get(&source)
                        .ok_or(WorthTopologyQueryApplyError::MissingExistingEntityBinding(
                            source,
                        ))?
                        .query_identity
                        .clone();
                    let expected_target = entities
                        .get(&target)
                        .ok_or(WorthTopologyQueryApplyError::MissingExistingEntityBinding(
                            target,
                        ))?
                        .query_identity
                        .clone();
                    let binding = workspace.bind_existing_relation(
                        ForgeQueryExistingRelationTarget::new(
                            format!("{relation_id:?}"),
                            imported.query_identity.clone(),
                        )?
                        .in_target_collection("WorthTopologyRelation")?,
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
                                        WorthTopologyQueryMutationEvidence::metadata_key(),
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
                                    WorthTopologyQueryMutationEvidence::metadata_key(),
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

        Ok(WorthTopologyQueryAppliedIntent {
            receipt: ForgeQueryBatchWriteReceipt::from_write_receipts(receipts)?,
            mutation_evidence,
        })
    }
}

fn resolve_entity_reference(
    reference: &WorthEntityReference,
    entities: &BTreeMap<EntityId, ImportedTopologyEntity>,
    created_entities: &BTreeMap<String, String>,
) -> Result<String, WorthTopologyQueryApplyError> {
    match reference {
        WorthEntityReference::Existing(entity_id) => entities
            .get(entity_id)
            .map(|entry| entry.query_identity.clone())
            .ok_or(WorthTopologyQueryApplyError::MissingExistingEntityBinding(
                *entity_id,
            )),
        WorthEntityReference::Created(create_key) => created_entities
            .get(create_key.as_str())
            .cloned()
            .ok_or_else(|| {
                WorthTopologyQueryApplyError::MissingCreatedEntityReference(
                    create_key.as_str().to_string(),
                )
            }),
    }
}
