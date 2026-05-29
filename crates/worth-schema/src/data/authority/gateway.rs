mod failure_envelopes;
#[cfg(test)]
mod tests;
mod touched_aspect_scope;

use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, RelationId};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::symbols::ClientKey;
use forge_relational::facade::transactions::{
    CreateIntent, CreatedEntityRef, DeleteEntityIntent, DeleteRelationIntent,
    EntityReference as RelationalEntityReference, EntitySpec, MutationIntent,
    RelationMutationIntent, RelationSpec, TransactionCommitError, TransactionOptions,
    WorkerIntentBatch,
};

use crate::data::authority::aspect_field_patches::{
    entity_create_fields, entity_record_label, relation_create_fields,
};
use crate::data::authority::{
    CanonicalTopologyMutationBatch, CreateKey, DerivedTopologyReadBasis, EntityReference,
    PersistedTopologyTruthBatch, RawTopologyIntent, TopologyMutation, TopologyMutationBatch,
};
use crate::data::entities::EntityKind;
use crate::data::relations::RelationKind;
use crate::data::tracing::{
    AuthorityTraceAnchor, AuthorityTraceEvidence, BoundaryEnvelope, BoundaryFailure, DecisionTrace,
};

use failure_envelopes::{
    authority_failure_for_batch, authority_failure_for_intent,
    integrity_markers_for_verified_commit,
};
use touched_aspect_scope::touched_aspects_for_intent;

#[derive(Debug)]
pub enum TopologyAuthorityError {
    DuplicateCreateKey(CreateKey),
    DuplicateLiveEntityLabel(CreateKey),
    MissingCreatedEntity(CreateKey),
    UnsupportedIdentityEntityMutation(EntityId),
    UnsupportedIdentityRelationMutation(RelationId),
    MissingEntity(EntityId),
    MissingRelation(RelationId),
    EntityKindMismatch {
        entity_id: EntityId,
        expected: EntityKind,
        found: EntityKind,
    },
    RelationShapeMismatch {
        relation_id: RelationId,
        expected_kind: RelationKind,
        found_kind: RelationKind,
        expected_source: EntityId,
        found_source: EntityId,
        expected_target: EntityId,
        found_target: EntityId,
    },
    ReadSnapshot(String),
    Commit(TransactionCommitError),
}

impl From<TransactionCommitError> for TopologyAuthorityError {
    fn from(value: TransactionCommitError) -> Self {
        Self::Commit(value)
    }
}

#[derive(Debug, Clone)]
pub struct VerifiedTopologyCommit {
    pub canonical_batch: CanonicalTopologyMutationBatch,
    pub branch_id: BranchId,
    pub commits: Vec<forge_relational::facade::transactions::CommitResult>,
    pub persisted_truth: PersistedTopologyTruthBatch,
    pub read_basis: DerivedTopologyReadBasis,
}

pub type TracedTopologyCommit = BoundaryEnvelope<VerifiedTopologyCommit>;

pub struct TopologyAuthority<'a> {
    runtime: &'a mut RelationalRuntime,
}

impl<'a> TopologyAuthority<'a> {
    pub fn new(runtime: &'a mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn apply_topology_intent_traced(
        &mut self,
        intent: RawTopologyIntent,
    ) -> Result<TracedTopologyCommit, BoundaryFailure<TopologyAuthorityError>> {
        self.apply_topology_intent_on_branch_traced(intent, BranchId("main".to_string()))
    }

    pub fn apply_topology_intent_on_branch_traced(
        &mut self,
        intent: RawTopologyIntent,
        branch_id: BranchId,
    ) -> Result<TracedTopologyCommit, BoundaryFailure<TopologyAuthorityError>> {
        let snapshot = self.runtime.snapshots().snapshot();
        let read = self.runtime.read_truth().read_snapshot(&snapshot);

        let touched_aspects = touched_aspects_for_intent(read.as_ref(), &intent)
            .map_err(|error| authority_failure_for_intent(error, &branch_id, &intent))?;
        let canonical_batch = CanonicalTopologyMutationBatch {
            batch: TopologyMutationBatch::from_raw_intent(intent, touched_aspects),
        };

        let commits = self
            .execute_canonical_batch(read.as_ref(), &canonical_batch.batch, &branch_id)
            .map_err(|error| {
                authority_failure_for_batch(error, &branch_id, &canonical_batch.batch)
            })?;

        let persisted_snapshot = commits
            .last()
            .map(|commit| commit.snapshot.clone())
            .unwrap_or(snapshot);
        let persisted_truth = PersistedTopologyTruthBatch {
            batch: canonical_batch.batch.clone(),
            snapshot: persisted_snapshot,
            branch_id: branch_id.clone(),
            mutation_origin: canonical_batch.batch.mutation_origin,
        };
        let read_basis = DerivedTopologyReadBasis::from_persisted_truth(&persisted_truth);
        let verified_commit = VerifiedTopologyCommit {
            canonical_batch,
            branch_id: branch_id.clone(),
            commits,
            persisted_truth,
            read_basis,
        };
        let authority = AuthorityTraceEvidence::from_commit_results(
            branch_id.clone(),
            &verified_commit.commits,
        );
        let authority_anchor =
            AuthorityTraceAnchor::from_commit_results(branch_id.clone(), &verified_commit.commits);
        let integrity_markers = integrity_markers_for_verified_commit(&verified_commit);
        let performance_accounting = authority.performance_accounting();
        Ok(BoundaryEnvelope::success(
            verified_commit,
            Vec::new(),
            DecisionTrace {
                authority_anchor: Some(authority_anchor),
                bridge_anchor: None,
                derived_anchor: None,
                signal_anchor: None,
                authority: Some(authority),
                bridge: None,
                derived: None,
                signal: None,
            },
            integrity_markers,
            performance_accounting,
        ))
    }

    fn execute_canonical_batch(
        &mut self,
        read: Option<&forge_relational::facade::runtime::RelationalReadView>,
        batch: &TopologyMutationBatch,
        branch_id: &BranchId,
    ) -> Result<Vec<forge_relational::facade::transactions::CommitResult>, TopologyAuthorityError>
    {
        let lowered = self.lower_canonical_batch(read, batch)?;
        if lowered.is_empty() {
            return Ok(Vec::new());
        }

        let mut tx = self.runtime.begin_transaction(TransactionOptions {
            target_branch: Some(branch_id.clone()),
            ..TransactionOptions::default()
        });
        let batch = lowered.into_iter().fold(
            WorkerIntentBatch::new(batch_name(batch.mutation_origin)),
            |batch, mutation| batch.push(mutation),
        );
        tx.push_batch(batch);
        Ok(vec![tx.commit()?])
    }

    fn lower_canonical_batch(
        &self,
        read: Option<&forge_relational::facade::runtime::RelationalReadView>,
        batch: &TopologyMutationBatch,
    ) -> Result<Vec<MutationIntent>, TopologyAuthorityError> {
        let mut seen = BTreeSet::new();
        let mut created_entities = BTreeMap::new();

        for mutation in &batch.mutations {
            match mutation {
                TopologyMutation::CreateEntity { create_key, kind } => {
                    if !seen.insert(create_key.clone()) {
                        return Err(TopologyAuthorityError::DuplicateCreateKey(
                            create_key.clone(),
                        ));
                    }
                    if read.is_some_and(|snapshot| {
                        live_entity_label_exists(snapshot, create_key.as_str())
                    }) {
                        return Err(TopologyAuthorityError::DuplicateLiveEntityLabel(
                            create_key.clone(),
                        ));
                    }
                    created_entities.insert(
                        create_key.clone(),
                        CreatedEntityRef {
                            partition_id: forge_relational::facade::identity::PartitionId::main(),
                            kind_id: kind.kind_id(),
                            client_key: ClientKey::raw(create_key.as_str()),
                        },
                    );
                }
                TopologyMutation::CreateRelation { create_key, .. } => {
                    if !seen.insert(create_key.clone()) {
                        return Err(TopologyAuthorityError::DuplicateCreateKey(
                            create_key.clone(),
                        ));
                    }
                }
                _ => {}
            }
        }

        let mut lowered = Vec::new();
        for mutation in &batch.mutations {
            match mutation {
                TopologyMutation::CreateEntity { create_key, kind } => {
                    lowered.push(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                        partition_id: forge_relational::facade::identity::PartitionId::main(),
                        kind_id: kind.kind_id(),
                        client_key: ClientKey::raw(create_key.as_str()),
                        fields: entity_create_fields(*kind, create_key.as_str()),
                    })));
                }
                TopologyMutation::CreateRelation {
                    create_key,
                    kind,
                    source,
                    target,
                } => {
                    lowered.push(MutationIntent::Create(CreateIntent::Relation(
                        RelationSpec {
                            partition_id: forge_relational::facade::identity::PartitionId::main(),
                            kind_id: kind.kind_id(),
                            client_key: ClientKey::raw(create_key.as_str()),
                            source: resolve_entity_reference(source, &created_entities)?,
                            target: resolve_entity_reference(target, &created_entities)?,
                            fields: relation_create_fields(),
                        },
                    )));
                }
                _ => {
                    let read = read.ok_or_else(|| {
                        TopologyAuthorityError::ReadSnapshot(
                            " authority requires a readable starting snapshot for existing truth mutations"
                                .to_string(),
                        )
                    })?;
                    lower_existing_mutation(read, mutation, &mut lowered)?;
                }
            }
        }

        Ok(lowered)
    }
}

fn batch_name(origin: crate::data::authority::MutationOrigin) -> &'static str {
    match origin {
        crate::data::authority::MutationOrigin::Seed => "topology-seed",
        crate::data::authority::MutationOrigin::LocalEdit => "topology-local-edit",
        crate::data::authority::MutationOrigin::Replay => "topology-replay",
        crate::data::authority::MutationOrigin::BranchLocalApplication => "topology-branch-local",
    }
}

fn lower_existing_mutation(
    read: &forge_relational::facade::runtime::RelationalReadView,
    mutation: &TopologyMutation,
    lowered: &mut Vec<MutationIntent>,
) -> Result<(), TopologyAuthorityError> {
    match mutation {
        TopologyMutation::CreateEntity { .. } | TopologyMutation::CreateRelation { .. } => {
            unreachable!("create mutations are handled before lowering existing mutations")
        }
        TopologyMutation::UpsertEntity { entity_id, kind } => {
            let Some(existing) = read.get_entity(*entity_id) else {
                return Err(TopologyAuthorityError::UnsupportedIdentityEntityMutation(
                    *entity_id,
                ));
            };
            let found = EntityKind::from_kind_id(existing.kind.kind_id).ok_or_else(|| {
                TopologyAuthorityError::ReadSnapshot(format!(
                    "unknown  entity kind id `{}` for entity `{:?}`",
                    existing.kind.kind_id.0, entity_id
                ))
            })?;
            if found != *kind {
                return Err(TopologyAuthorityError::EntityKindMismatch {
                    entity_id: *entity_id,
                    expected: *kind,
                    found,
                });
            }
        }
        TopologyMutation::UpsertRelation {
            relation_id,
            kind,
            source,
            target,
        } => {
            let Some(existing) = read.get_relation(*relation_id) else {
                return Err(TopologyAuthorityError::UnsupportedIdentityRelationMutation(
                    *relation_id,
                ));
            };
            let found_kind =
                RelationKind::from_kind_id(existing.kind.kind_id).ok_or_else(|| {
                    TopologyAuthorityError::ReadSnapshot(format!(
                        "unknown  relation kind id `{}` for relation `{:?}`",
                        existing.kind.kind_id.0, relation_id
                    ))
                })?;
            if found_kind != *kind || existing.source != *source || existing.target != *target {
                return Err(TopologyAuthorityError::RelationShapeMismatch {
                    relation_id: *relation_id,
                    expected_kind: *kind,
                    found_kind,
                    expected_source: *source,
                    found_source: existing.source,
                    expected_target: *target,
                    found_target: existing.target,
                });
            }
        }
        TopologyMutation::RemoveEntity { entity_id } => {
            if read.get_entity(*entity_id).is_none() {
                return Err(TopologyAuthorityError::MissingEntity(*entity_id));
            }
            lowered.push(MutationIntent::Entity(
                forge_relational::facade::transactions::EntityMutationIntent::Delete(
                    DeleteEntityIntent {
                        entity_id: *entity_id,
                    },
                ),
            ));
        }
        TopologyMutation::RemoveRelation { relation_id } => {
            if read.get_relation(*relation_id).is_none() {
                return Err(TopologyAuthorityError::MissingRelation(*relation_id));
            }
            lowered.push(MutationIntent::Relation(RelationMutationIntent::Delete(
                DeleteRelationIntent {
                    relation_id: *relation_id,
                },
            )));
        }
    }

    Ok(())
}

fn resolve_entity_reference(
    reference: &EntityReference,
    created: &BTreeMap<CreateKey, CreatedEntityRef>,
) -> Result<RelationalEntityReference, TopologyAuthorityError> {
    match reference {
        EntityReference::Existing(entity_id) => Ok(RelationalEntityReference::Existing(*entity_id)),
        EntityReference::Created(create_key) => created
            .get(create_key)
            .cloned()
            .map(RelationalEntityReference::Created)
            .ok_or_else(|| TopologyAuthorityError::MissingCreatedEntity(create_key.clone())),
    }
}

fn live_entity_label_exists(
    read: &forge_relational::facade::runtime::RelationalReadView,
    label: &str,
) -> bool {
    read.entities().iter().any(|record| {
        EntityKind::from_kind_id(record.kind.kind_id)
            .and_then(|kind| entity_record_label(record, kind))
            .is_some_and(|existing| existing == label)
    })
}
