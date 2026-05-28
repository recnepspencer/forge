use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use forge_relational::facade::payloads::RecordPayload;
use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use forge_relational::facade::symbols::InternedString;
use forge_relational::facade::transactions::{
    CommitResult, CreateIntent, CreatedEntityRef, DeleteEntityIntent, DeleteRelationIntent,
    EntityMutationIntent, EntityReference as RelationalEntityReference, EntitySpec,
    MutationIntent, RelationMutationIntent, RelationSpec, TransactionCommitError,
    TransactionOptions, WorkerIntentBatch,
};
use schema::facade::platform::aspects::{
    Aspect, DiagnosticsAspect, GeometryAspect, NamingAspect, TopologyAspect,
};
use schema::facade::platform::authority::{
    CanonicalTopologyMutationBatch, CreateKey, DerivedTopologyReadBasis, EntityReference,
    MutationOrigin, PersistedTopologyTruthBatch, RawTopologyIntent, TopologyMutation,
    TopologyMutationBatch,
};
use schema::facade::platform::entities::{DiagnosticsEntityKind, EntityKind};
use schema::facade::platform::relations::{
    DiagnosticsRelationKind, GeometryRelationKind, NamingRelationKind, RelationKind,
    TopologyRelationKind,
};
use serde_json::json;

use crate::committed_artifact::TopologyCommittedArtifact;

#[derive(Debug)]
pub(crate) enum TopologyIntentCommitError {
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

impl From<TransactionCommitError> for TopologyIntentCommitError {
    fn from(value: TransactionCommitError) -> Self {
        Self::Commit(value)
    }
}

impl fmt::Display for TopologyIntentCommitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateCreateKey(key) => write!(f, "duplicate create key `{}`", key.as_str()),
            Self::DuplicateLiveEntityLabel(key) => {
                write!(f, "duplicate live entity label `{}`", key.as_str())
            }
            Self::MissingCreatedEntity(key) => {
                write!(f, "missing created entity reference `{}`", key.as_str())
            }
            Self::UnsupportedIdentityEntityMutation(entity_id) => {
                write!(f, "unsupported identity entity mutation for `{entity_id:?}`")
            }
            Self::UnsupportedIdentityRelationMutation(relation_id) => {
                write!(f, "unsupported identity relation mutation for `{relation_id:?}`")
            }
            Self::MissingEntity(entity_id) => write!(f, "missing entity `{entity_id:?}`"),
            Self::MissingRelation(relation_id) => write!(f, "missing relation `{relation_id:?}`"),
            Self::EntityKindMismatch {
                entity_id,
                expected,
                found,
            } => write!(
                f,
                "entity kind mismatch for `{entity_id:?}`: expected `{expected:?}`, found `{found:?}`"
            ),
            Self::RelationShapeMismatch {
                relation_id,
                expected_kind,
                found_kind,
                expected_source,
                found_source,
                expected_target,
                found_target,
            } => write!(
                f,
                "relation shape mismatch for `{relation_id:?}`: expected kind `{expected_kind:?}` source `{expected_source:?}` target `{expected_target:?}`, found kind `{found_kind:?}` source `{found_source:?}` target `{found_target:?}`"
            ),
            Self::ReadSnapshot(message) => f.write_str(message),
            Self::Commit(error) => write!(f, "{error:?}"),
        }
    }
}

pub(crate) fn commit_topology_intent(
    runtime: &mut RelationalRuntime,
    intent: RawTopologyIntent,
) -> Result<TopologyCommittedArtifact, TopologyIntentCommitError> {
    commit_topology_intent_on_branch(runtime, intent, BranchId("main".to_string()))
}

pub(crate) fn commit_topology_intent_on_branch(
    runtime: &mut RelationalRuntime,
    intent: RawTopologyIntent,
    branch_id: BranchId,
) -> Result<TopologyCommittedArtifact, TopologyIntentCommitError> {
    let snapshot = runtime.snapshots().snapshot();
    let read = runtime.read_truth().read_snapshot(&snapshot);
    let touched_aspects = touched_aspects_for_intent(read.as_ref(), &intent)?;
    let canonical_batch = CanonicalTopologyMutationBatch {
        batch: TopologyMutationBatch::from_raw_intent(intent, touched_aspects),
    };
    let commits = execute_canonical_batch(runtime, read.as_ref(), &canonical_batch.batch, &branch_id)?;
    if commits.is_empty() {
        return Ok(TopologyCommittedArtifact::empty_from_intent(
            snapshot,
            branch_id,
            RawTopologyIntent::new(
                canonical_batch.batch.mutations.clone(),
                canonical_batch.batch.mutation_origin,
            ),
        ));
    }
    let persisted_snapshot = commits
        .last()
        .map(|commit| commit.snapshot.clone())
        .expect("non-empty commits");
    let persisted_truth = PersistedTopologyTruthBatch {
        batch: canonical_batch.batch.clone(),
        snapshot: persisted_snapshot,
        branch_id: branch_id.clone(),
        mutation_origin: canonical_batch.batch.mutation_origin,
    };
    let read_basis = DerivedTopologyReadBasis::from_persisted_truth(&persisted_truth);
    Ok(TopologyCommittedArtifact::from_parts(
        canonical_batch,
        branch_id,
        commits,
        persisted_truth,
        read_basis,
    ))
}

fn execute_canonical_batch(
    runtime: &mut RelationalRuntime,
    read: Option<&RelationalReadView>,
    batch: &TopologyMutationBatch,
    branch_id: &BranchId,
) -> Result<Vec<CommitResult>, TopologyIntentCommitError> {
    let lowered = lower_canonical_batch(read, batch)?;
    if lowered.is_empty() {
        return Ok(Vec::new());
    }
    let lowered_batch = lowered.into_iter().fold(
        WorkerIntentBatch::new(batch_name(batch.mutation_origin)),
        |batch, mutation| batch.push(mutation),
    );
    let mut tx = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id.clone()),
        ..TransactionOptions::default()
    });
    tx.push_batch(lowered_batch);
    Ok(vec![tx.commit()?])
}

fn lower_canonical_batch(
    read: Option<&RelationalReadView>,
    batch: &TopologyMutationBatch,
) -> Result<Vec<MutationIntent>, TopologyIntentCommitError> {
    let mut seen = BTreeSet::new();
    let mut created_entities = BTreeMap::new();

    for mutation in &batch.mutations {
        match mutation {
            TopologyMutation::CreateEntity { create_key, kind } => {
                if !seen.insert(create_key.clone()) {
                    return Err(TopologyIntentCommitError::DuplicateCreateKey(create_key.clone()));
                }
                if read.is_some_and(|snapshot| live_entity_label_exists(snapshot, create_key.as_str())) {
                    return Err(TopologyIntentCommitError::DuplicateLiveEntityLabel(create_key.clone()));
                }
                created_entities.insert(
                    create_key.clone(),
                    CreatedEntityRef {
                        partition_id: PartitionId::main(),
                        kind_id: kind.kind_id(),
                        client_key: InternedString::Raw(create_key.as_str().to_string()),
                    },
                );
            }
            TopologyMutation::CreateRelation { create_key, .. } => {
                if !seen.insert(create_key.clone()) {
                    return Err(TopologyIntentCommitError::DuplicateCreateKey(create_key.clone()));
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
                    partition_id: PartitionId::main(),
                    kind_id: kind.kind_id(),
                    client_key: InternedString::Raw(create_key.as_str().to_string()),
                    payload: RecordPayload::StructuredJson(entity_create_payload(*kind, create_key.as_str())),
                })));
            }
            TopologyMutation::CreateRelation {
                create_key,
                kind,
                source,
                target,
            } => {
                lowered.push(MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: kind.kind_id(),
                    client_key: InternedString::Raw(create_key.as_str().to_string()),
                    source: resolve_entity_reference(source, &created_entities)?,
                    target: resolve_entity_reference(target, &created_entities)?,
                    payload: None,
                })));
            }
            _ => {
                let read = read.ok_or_else(|| {
                    TopologyIntentCommitError::ReadSnapshot(
                        "topology intent commit requires a readable starting snapshot for existing truth mutations"
                            .to_string(),
                    )
                })?;
                lower_existing_mutation(read, mutation, &mut lowered)?;
            }
        }
    }
    Ok(lowered)
}

fn lower_existing_mutation(
    read: &RelationalReadView,
    mutation: &TopologyMutation,
    lowered: &mut Vec<MutationIntent>,
) -> Result<(), TopologyIntentCommitError> {
    match mutation {
        TopologyMutation::CreateEntity { .. } | TopologyMutation::CreateRelation { .. } => {
            unreachable!("create mutations are lowered before existing-truth mutations")
        }
        TopologyMutation::UpsertEntity { entity_id, kind } => {
            let Some(existing) = read.get_entity(*entity_id) else {
                return Err(TopologyIntentCommitError::UnsupportedIdentityEntityMutation(*entity_id));
            };
            let found = EntityKind::from_kind_id(existing.kind.kind_id).ok_or_else(|| {
                TopologyIntentCommitError::ReadSnapshot(format!(
                    "unknown entity kind id `{}` for entity `{:?}`",
                    existing.kind.kind_id.0, entity_id
                ))
            })?;
            if found != *kind {
                return Err(TopologyIntentCommitError::EntityKindMismatch {
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
                return Err(TopologyIntentCommitError::UnsupportedIdentityRelationMutation(*relation_id));
            };
            let found_kind = RelationKind::from_kind_id(existing.kind.kind_id).ok_or_else(|| {
                TopologyIntentCommitError::ReadSnapshot(format!(
                    "unknown relation kind id `{}` for relation `{:?}`",
                    existing.kind.kind_id.0, relation_id
                ))
            })?;
            if found_kind != *kind || existing.source != *source || existing.target != *target {
                return Err(TopologyIntentCommitError::RelationShapeMismatch {
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
                return Err(TopologyIntentCommitError::MissingEntity(*entity_id));
            }
            lowered.push(MutationIntent::Entity(EntityMutationIntent::Delete(DeleteEntityIntent {
                entity_id: *entity_id,
            })));
        }
        TopologyMutation::RemoveRelation { relation_id } => {
            if read.get_relation(*relation_id).is_none() {
                return Err(TopologyIntentCommitError::MissingRelation(*relation_id));
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
) -> Result<RelationalEntityReference, TopologyIntentCommitError> {
    match reference {
        EntityReference::Existing(entity_id) => Ok(RelationalEntityReference::Existing(*entity_id)),
        EntityReference::Created(create_key) => created
            .get(create_key)
            .cloned()
            .map(RelationalEntityReference::Created)
            .ok_or_else(|| TopologyIntentCommitError::MissingCreatedEntity(create_key.clone())),
    }
}

fn live_entity_label_exists(read: &RelationalReadView, label: &str) -> bool {
    read.entities().iter().any(|record| {
        record
            .payload
            .as_json()
            .and_then(|json| json.get("label"))
            .and_then(|value| value.as_str())
            .is_some_and(|existing| existing == label)
    })
}

fn touched_aspects_for_intent(
    read: Option<&RelationalReadView>,
    intent: &RawTopologyIntent,
) -> Result<BTreeSet<Aspect>, TopologyIntentCommitError> {
    let mut aspects = BTreeSet::new();
    for mutation in &intent.mutations {
        match mutation {
            TopologyMutation::CreateEntity { kind, .. } | TopologyMutation::UpsertEntity { kind, .. } => {
                aspects.extend(entity_aspects(*kind));
            }
            TopologyMutation::CreateRelation { kind, .. }
            | TopologyMutation::UpsertRelation { kind, .. } => {
                aspects.extend(relation_aspects(*kind));
            }
            TopologyMutation::RemoveEntity { entity_id } => {
                let read = read.ok_or_else(|| {
                    TopologyIntentCommitError::ReadSnapshot(
                        "topology intent commit requires a readable starting snapshot for entity removal"
                            .to_string(),
                    )
                })?;
                let Some(existing) = read.get_entity(*entity_id) else {
                    return Err(TopologyIntentCommitError::MissingEntity(*entity_id));
                };
                let kind = EntityKind::from_kind_id(existing.kind.kind_id).ok_or_else(|| {
                    TopologyIntentCommitError::ReadSnapshot(format!(
                        "unknown entity kind id `{}` for entity `{:?}`",
                        existing.kind.kind_id.0, entity_id
                    ))
                })?;
                aspects.extend(entity_aspects(kind));
            }
            TopologyMutation::RemoveRelation { relation_id } => {
                let read = read.ok_or_else(|| {
                    TopologyIntentCommitError::ReadSnapshot(
                        "topology intent commit requires a readable starting snapshot for relation removal"
                            .to_string(),
                    )
                })?;
                let Some(existing) = read.get_relation(*relation_id) else {
                    return Err(TopologyIntentCommitError::MissingRelation(*relation_id));
                };
                let kind = RelationKind::from_kind_id(existing.kind.kind_id).ok_or_else(|| {
                    TopologyIntentCommitError::ReadSnapshot(format!(
                        "unknown relation kind id `{}` for relation `{:?}`",
                        existing.kind.kind_id.0, relation_id
                    ))
                })?;
                aspects.extend(relation_aspects(kind));
            }
        }
    }
    Ok(aspects)
}

fn entity_aspects(kind: EntityKind) -> [Aspect; 2] {
    [
        match kind {
            EntityKind::Topology(_) => Aspect::Topology(TopologyAspect::Structure),
            EntityKind::Geometry(_) => Aspect::Geometry(GeometryAspect::Binding),
            EntityKind::Naming(_) => Aspect::Naming(NamingAspect::PersistentName),
            EntityKind::Diagnostics(DiagnosticsEntityKind::WireInterpretation)
            | EntityKind::Diagnostics(DiagnosticsEntityKind::ShellInterpretation) => {
                Aspect::Diagnostics(DiagnosticsAspect::Interpretations)
            }
        },
        Aspect::Diagnostics(DiagnosticsAspect::Decisions),
    ]
}

fn relation_aspects(kind: RelationKind) -> [Aspect; 2] {
    [
        match kind {
            RelationKind::Topology(TopologyRelationKind::ModelOwnsBody)
            | RelationKind::Topology(TopologyRelationKind::BodyOwnsLump)
            | RelationKind::Topology(TopologyRelationKind::LumpOwnsRegion)
            | RelationKind::Topology(TopologyRelationKind::RegionOwnsShell)
            | RelationKind::Topology(TopologyRelationKind::ShellOwnsFace)
            | RelationKind::Topology(TopologyRelationKind::WireOwnsHalfEdge) => {
                Aspect::Topology(TopologyAspect::Ownership)
            }
            RelationKind::Topology(TopologyRelationKind::FaceOuterLoop)
            | RelationKind::Topology(TopologyRelationKind::FaceInnerLoop)
            | RelationKind::Topology(TopologyRelationKind::LoopOwnsHalfEdge)
            | RelationKind::Topology(TopologyRelationKind::HalfEdgeNext)
            | RelationKind::Topology(TopologyRelationKind::HalfEdgePrev)
            | RelationKind::Topology(TopologyRelationKind::HalfEdgeUsesEdge)
            | RelationKind::Topology(TopologyRelationKind::HalfEdgeStartsAtVertex)
            | RelationKind::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex) => {
                Aspect::Topology(TopologyAspect::Boundary)
            }
            RelationKind::Topology(TopologyRelationKind::HalfEdgeRadialNext) => {
                Aspect::Topology(TopologyAspect::Radial)
            }
            RelationKind::Geometry(
                GeometryRelationKind::FaceUsesSurfaceBinding
                | GeometryRelationKind::EdgeUsesCurveBinding
                | GeometryRelationKind::HalfEdgeUsesCoedgeBinding
                | GeometryRelationKind::VertexUsesGeometryBinding,
            ) => Aspect::Geometry(GeometryAspect::Binding),
            RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity) => {
                Aspect::Naming(NamingAspect::PersistentName)
            }
            RelationKind::Diagnostics(
                DiagnosticsRelationKind::WireHasInterpretation
                | DiagnosticsRelationKind::ShellHasInterpretation,
            ) => Aspect::Diagnostics(DiagnosticsAspect::Interpretations),
        },
        Aspect::Diagnostics(DiagnosticsAspect::Decisions),
    ]
}

fn entity_create_payload(kind: EntityKind, label: &str) -> serde_json::Value {
    match kind {
        EntityKind::Topology(_) => json!({
            "label": label,
            "structure": label,
            "topology": { "structure": label }
        }),
        EntityKind::Geometry(_) => json!({
            "label": label,
            "binding": label,
            "geometry": { "binding": label }
        }),
        EntityKind::Naming(_) => json!({
            "label": label,
            "persistent_name": label,
            "naming": { "persistent_name": label }
        }),
        EntityKind::Diagnostics(DiagnosticsEntityKind::WireInterpretation)
        | EntityKind::Diagnostics(DiagnosticsEntityKind::ShellInterpretation) => json!({
            "label": label,
            "interpretations": label,
            "diagnostics": { "interpretations": label }
        }),
    }
}

fn batch_name(origin: MutationOrigin) -> &'static str {
    match origin {
        MutationOrigin::Seed => "topology-seed",
        MutationOrigin::LocalEdit => "topology-local-edit",
        MutationOrigin::Replay => "topology-replay",
        MutationOrigin::BranchLocalApplication => "topology-branch-local",
    }
}
