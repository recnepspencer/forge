use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, RelationId};
use forge_relational::facade::payloads::RecordPayload;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::symbols::InternedString;
use forge_relational::facade::transactions::{
    CreateIntent, CreatedEntityRef, DeleteEntityIntent, DeleteRelationIntent, EntityReference,
    EntitySpec, MutationIntent, RelationMutationIntent, RelationSpec, TransactionCommitError,
    TransactionOptions, WorkerIntentBatch,
};
use serde_json::json;

use crate::data::aspects::{
    WorthAspect, WorthDiagnosticsAspect, WorthGeometryAspect, WorthNamingAspect,
    WorthTopologyAspect,
};
use crate::data::authority::{
    CanonicalTopologyMutationBatch, DerivedTopologyReadBasis, PersistedTopologyTruthBatch,
    RawWorthTopologyIntent, WorthCreateKey, WorthEntityReference, WorthTopologyMutation,
    WorthTopologyMutationBatch,
};
use crate::data::entities::{WorthDiagnosticsEntityKind, WorthEntityKind};
use crate::data::relations::{
    WorthDiagnosticsRelationKind, WorthGeometryRelationKind, WorthNamingRelationKind,
    WorthRelationKind, WorthTopologyRelationKind,
};
use crate::data::tracing::{
    WorthAuthorityTraceAnchor, WorthAuthorityTraceEvidence, WorthBoundaryEnvelope,
    WorthBoundaryFailure, WorthDecisionTrace, WorthIntegrityMarkers, WorthPerformanceAccounting,
};

#[derive(Debug)]
pub enum WorthTopologyAuthorityError {
    DuplicateCreateKey(WorthCreateKey),
    DuplicateLiveEntityLabel(WorthCreateKey),
    MissingCreatedEntity(WorthCreateKey),
    UnsupportedIdentityEntityMutation(EntityId),
    UnsupportedIdentityRelationMutation(RelationId),
    MissingEntity(EntityId),
    MissingRelation(RelationId),
    EntityKindMismatch {
        entity_id: EntityId,
        expected: WorthEntityKind,
        found: WorthEntityKind,
    },
    RelationShapeMismatch {
        relation_id: RelationId,
        expected_kind: WorthRelationKind,
        found_kind: WorthRelationKind,
        expected_source: EntityId,
        found_source: EntityId,
        expected_target: EntityId,
        found_target: EntityId,
    },
    ReadSnapshot(String),
    Commit(TransactionCommitError),
}

impl From<TransactionCommitError> for WorthTopologyAuthorityError {
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

pub type WorthTracedTopologyCommit = WorthBoundaryEnvelope<VerifiedTopologyCommit>;

pub struct WorthTopologyAuthority<'a> {
    runtime: &'a mut RelationalRuntime,
}

impl<'a> WorthTopologyAuthority<'a> {
    pub fn new(runtime: &'a mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn apply_topology_intent_traced(
        &mut self,
        intent: RawWorthTopologyIntent,
    ) -> Result<WorthTracedTopologyCommit, WorthBoundaryFailure<WorthTopologyAuthorityError>> {
        self.apply_topology_intent_on_branch_traced(intent, BranchId("main".to_string()))
    }

    pub fn apply_topology_intent_on_branch_traced(
        &mut self,
        intent: RawWorthTopologyIntent,
        branch_id: BranchId,
    ) -> Result<WorthTracedTopologyCommit, WorthBoundaryFailure<WorthTopologyAuthorityError>> {
        let snapshot = self.runtime.snapshots().snapshot();
        let read = self.runtime.read_truth().read_snapshot(&snapshot);

        let touched_aspects = touched_aspects_for_intent(read.as_ref(), &intent)
            .map_err(|error| authority_failure_for_intent(error, &branch_id, &intent))?;
        let canonical_batch = CanonicalTopologyMutationBatch {
            batch: WorthTopologyMutationBatch::from_raw_intent(intent, touched_aspects),
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
        let authority = WorthAuthorityTraceEvidence::from_commit_results(
            branch_id.clone(),
            &verified_commit.commits,
        );
        let authority_anchor = WorthAuthorityTraceAnchor::from_commit_results(
            branch_id.clone(),
            &verified_commit.commits,
        );
        let integrity_markers = integrity_markers_for_verified_commit(&verified_commit);
        let performance_accounting = authority.performance_accounting();
        Ok(WorthBoundaryEnvelope::success(
            verified_commit,
            Vec::new(),
            WorthDecisionTrace {
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
        batch: &WorthTopologyMutationBatch,
        branch_id: &BranchId,
    ) -> Result<
        Vec<forge_relational::facade::transactions::CommitResult>,
        WorthTopologyAuthorityError,
    > {
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
        batch: &WorthTopologyMutationBatch,
    ) -> Result<Vec<MutationIntent>, WorthTopologyAuthorityError> {
        let mut seen = BTreeSet::new();
        let mut created_entities = BTreeMap::new();

        for mutation in &batch.mutations {
            match mutation {
                WorthTopologyMutation::CreateEntity { create_key, kind } => {
                    if !seen.insert(create_key.clone()) {
                        return Err(WorthTopologyAuthorityError::DuplicateCreateKey(
                            create_key.clone(),
                        ));
                    }
                    if read.is_some_and(|snapshot| {
                        live_entity_label_exists(snapshot, create_key.as_str())
                    }) {
                        return Err(WorthTopologyAuthorityError::DuplicateLiveEntityLabel(
                            create_key.clone(),
                        ));
                    }
                    created_entities.insert(
                        create_key.clone(),
                        CreatedEntityRef {
                            partition_id: forge_relational::facade::identity::PartitionId::main(),
                            kind_id: kind.kind_id(),
                            client_key: InternedString::Raw(create_key.as_str().to_string()),
                        },
                    );
                }
                WorthTopologyMutation::CreateRelation { create_key, .. } => {
                    if !seen.insert(create_key.clone()) {
                        return Err(WorthTopologyAuthorityError::DuplicateCreateKey(
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
                WorthTopologyMutation::CreateEntity { create_key, kind } => {
                    lowered.push(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                        partition_id: forge_relational::facade::identity::PartitionId::main(),
                        kind_id: kind.kind_id(),
                        client_key: InternedString::Raw(create_key.as_str().to_string()),
                        payload: RecordPayload::StructuredJson(entity_create_payload(
                            *kind,
                            create_key.as_str(),
                        )),
                    })));
                }
                WorthTopologyMutation::CreateRelation {
                    create_key,
                    kind,
                    source,
                    target,
                } => {
                    lowered.push(MutationIntent::Create(CreateIntent::Relation(
                        RelationSpec {
                            partition_id: forge_relational::facade::identity::PartitionId::main(),
                            kind_id: kind.kind_id(),
                            client_key: InternedString::Raw(create_key.as_str().to_string()),
                            source: resolve_entity_reference(source, &created_entities)?,
                            target: resolve_entity_reference(target, &created_entities)?,
                            payload: None,
                        },
                    )));
                }
                _ => {
                    let read = read.ok_or_else(|| {
                        WorthTopologyAuthorityError::ReadSnapshot(
                            "worth authority requires a readable starting snapshot for existing truth mutations"
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

fn authority_failure_for_intent(
    error: WorthTopologyAuthorityError,
    branch_id: &BranchId,
    intent: &RawWorthTopologyIntent,
) -> WorthBoundaryFailure<WorthTopologyAuthorityError> {
    WorthBoundaryFailure::failure(
        error,
        Vec::new(),
        WorthDecisionTrace {
            authority_anchor: None,
            bridge_anchor: None,
            derived_anchor: None,
            signal_anchor: None,
            authority: None,
            bridge: None,
            derived: None,
            signal: None,
        },
        WorthIntegrityMarkers::new(
            Some(branch_id.clone()),
            BTreeSet::new(),
            Some(intent.mutation_origin),
            None,
            intent.precision_fallbacks.len(),
            intent.precision_budget_fallbacks.len(),
        ),
        WorthPerformanceAccounting::default(),
    )
}

fn authority_failure_for_batch(
    error: WorthTopologyAuthorityError,
    branch_id: &BranchId,
    batch: &WorthTopologyMutationBatch,
) -> WorthBoundaryFailure<WorthTopologyAuthorityError> {
    let authority = match &error {
        WorthTopologyAuthorityError::Commit(commit_error) => {
            Some(WorthAuthorityTraceEvidence::from_commit_logs(
                branch_id.clone(),
                vec![commit_error.commit_log().clone()],
            ))
        }
        _ => None,
    };
    let performance_accounting = authority
        .as_ref()
        .map(WorthAuthorityTraceEvidence::performance_accounting)
        .unwrap_or_default();
    WorthBoundaryFailure::failure(
        error,
        Vec::new(),
        WorthDecisionTrace {
            authority_anchor: None,
            bridge_anchor: None,
            derived_anchor: None,
            signal_anchor: None,
            authority,
            bridge: None,
            derived: None,
            signal: None,
        },
        WorthIntegrityMarkers::new(
            Some(branch_id.clone()),
            batch.touched_aspects.clone(),
            Some(batch.mutation_origin),
            None,
            batch.precision_fallbacks.len(),
            batch.precision_budget_fallbacks.len(),
        ),
        performance_accounting,
    )
}

fn integrity_markers_for_verified_commit(commit: &VerifiedTopologyCommit) -> WorthIntegrityMarkers {
    WorthIntegrityMarkers::new(
        Some(commit.branch_id.clone()),
        commit.canonical_batch.batch.touched_aspects.clone(),
        Some(commit.canonical_batch.batch.mutation_origin),
        Some(commit.read_basis.authority.truth_basis_identity.clone()),
        commit.canonical_batch.batch.precision_fallbacks.len(),
        commit
            .canonical_batch
            .batch
            .precision_budget_fallbacks
            .len(),
    )
}

fn entity_create_payload(kind: WorthEntityKind, label: &str) -> serde_json::Value {
    match kind {
        WorthEntityKind::Topology(_) => json!({
            "label": label,
            "structure": label,
            "topology": {
                "structure": label,
            }
        }),
        WorthEntityKind::Geometry(_) => json!({
            "label": label,
            "binding": label,
            "geometry": {
                "binding": label,
            }
        }),
        WorthEntityKind::Naming(_) => json!({
            "label": label,
            "persistent_name": label,
            "naming": {
                "persistent_name": label,
            }
        }),
        WorthEntityKind::Diagnostics(WorthDiagnosticsEntityKind::WireInterpretation)
        | WorthEntityKind::Diagnostics(WorthDiagnosticsEntityKind::ShellInterpretation) => json!({
            "label": label,
            "interpretations": label,
            "diagnostics": {
                "interpretations": label,
            }
        }),
    }
}

fn batch_name(origin: crate::data::authority::WorthMutationOrigin) -> &'static str {
    match origin {
        crate::data::authority::WorthMutationOrigin::Seed => "worth-topology-seed",
        crate::data::authority::WorthMutationOrigin::LocalEdit => "worth-topology-local-edit",
        crate::data::authority::WorthMutationOrigin::Replay => "worth-topology-replay",
        crate::data::authority::WorthMutationOrigin::BranchLocalApplication => {
            "worth-topology-branch-local"
        }
    }
}

fn lower_existing_mutation(
    read: &forge_relational::facade::runtime::RelationalReadView,
    mutation: &WorthTopologyMutation,
    lowered: &mut Vec<MutationIntent>,
) -> Result<(), WorthTopologyAuthorityError> {
    match mutation {
        WorthTopologyMutation::CreateEntity { .. }
        | WorthTopologyMutation::CreateRelation { .. } => {
            unreachable!("create mutations are handled before lowering existing mutations")
        }
        WorthTopologyMutation::UpsertEntity { entity_id, kind } => {
            let Some(existing) = read.get_entity(*entity_id) else {
                return Err(
                    WorthTopologyAuthorityError::UnsupportedIdentityEntityMutation(*entity_id),
                );
            };
            let found = WorthEntityKind::from_kind_id(existing.kind.kind_id).ok_or_else(|| {
                WorthTopologyAuthorityError::ReadSnapshot(format!(
                    "unknown worth entity kind id `{}` for entity `{:?}`",
                    existing.kind.kind_id.0, entity_id
                ))
            })?;
            if found != *kind {
                return Err(WorthTopologyAuthorityError::EntityKindMismatch {
                    entity_id: *entity_id,
                    expected: *kind,
                    found,
                });
            }
        }
        WorthTopologyMutation::UpsertRelation {
            relation_id,
            kind,
            source,
            target,
        } => {
            let Some(existing) = read.get_relation(*relation_id) else {
                return Err(
                    WorthTopologyAuthorityError::UnsupportedIdentityRelationMutation(*relation_id),
                );
            };
            let found_kind =
                WorthRelationKind::from_kind_id(existing.kind.kind_id).ok_or_else(|| {
                    WorthTopologyAuthorityError::ReadSnapshot(format!(
                        "unknown worth relation kind id `{}` for relation `{:?}`",
                        existing.kind.kind_id.0, relation_id
                    ))
                })?;
            if found_kind != *kind || existing.source != *source || existing.target != *target {
                return Err(WorthTopologyAuthorityError::RelationShapeMismatch {
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
        WorthTopologyMutation::RemoveEntity { entity_id } => {
            if read.get_entity(*entity_id).is_none() {
                return Err(WorthTopologyAuthorityError::MissingEntity(*entity_id));
            }
            lowered.push(MutationIntent::Entity(
                forge_relational::facade::transactions::EntityMutationIntent::Delete(
                    DeleteEntityIntent {
                        entity_id: *entity_id,
                    },
                ),
            ));
        }
        WorthTopologyMutation::RemoveRelation { relation_id } => {
            if read.get_relation(*relation_id).is_none() {
                return Err(WorthTopologyAuthorityError::MissingRelation(*relation_id));
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
    reference: &WorthEntityReference,
    created: &BTreeMap<WorthCreateKey, CreatedEntityRef>,
) -> Result<EntityReference, WorthTopologyAuthorityError> {
    match reference {
        WorthEntityReference::Existing(entity_id) => Ok(EntityReference::Existing(*entity_id)),
        WorthEntityReference::Created(create_key) => created
            .get(create_key)
            .cloned()
            .map(EntityReference::Created)
            .ok_or_else(|| WorthTopologyAuthorityError::MissingCreatedEntity(create_key.clone())),
    }
}

fn live_entity_label_exists(
    read: &forge_relational::facade::runtime::RelationalReadView,
    label: &str,
) -> bool {
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
    read: Option<&forge_relational::facade::runtime::RelationalReadView>,
    intent: &RawWorthTopologyIntent,
) -> Result<BTreeSet<WorthAspect>, WorthTopologyAuthorityError> {
    let mut aspects = BTreeSet::new();
    for mutation in &intent.mutations {
        match mutation {
            WorthTopologyMutation::CreateEntity { kind, .. } => {
                aspects.extend(entity_aspects(*kind));
            }
            WorthTopologyMutation::CreateRelation { kind, .. } => {
                aspects.extend(relation_aspects(*kind));
            }
            WorthTopologyMutation::UpsertEntity { kind, .. } => {
                aspects.extend(entity_aspects(*kind));
            }
            WorthTopologyMutation::UpsertRelation { kind, .. } => {
                aspects.extend(relation_aspects(*kind));
            }
            WorthTopologyMutation::RemoveEntity { entity_id } => {
                let read = read.ok_or_else(|| {
                    WorthTopologyAuthorityError::ReadSnapshot(
                        "worth authority requires a readable starting snapshot for entity removal"
                            .to_string(),
                    )
                })?;
                let Some(existing) = read.get_entity(*entity_id) else {
                    return Err(WorthTopologyAuthorityError::MissingEntity(*entity_id));
                };
                let kind =
                    WorthEntityKind::from_kind_id(existing.kind.kind_id).ok_or_else(|| {
                        WorthTopologyAuthorityError::ReadSnapshot(format!(
                            "unknown worth entity kind id `{}` for entity `{:?}`",
                            existing.kind.kind_id.0, entity_id
                        ))
                    })?;
                aspects.extend(entity_aspects(kind));
            }
            WorthTopologyMutation::RemoveRelation { relation_id } => {
                let read = read.ok_or_else(|| {
                    WorthTopologyAuthorityError::ReadSnapshot(
                        "worth authority requires a readable starting snapshot for relation removal"
                            .to_string(),
                    )
                })?;
                let Some(existing) = read.get_relation(*relation_id) else {
                    return Err(WorthTopologyAuthorityError::MissingRelation(*relation_id));
                };
                let kind =
                    WorthRelationKind::from_kind_id(existing.kind.kind_id).ok_or_else(|| {
                        WorthTopologyAuthorityError::ReadSnapshot(format!(
                            "unknown worth relation kind id `{}` for relation `{:?}`",
                            existing.kind.kind_id.0, relation_id
                        ))
                    })?;
                aspects.extend(relation_aspects(kind));
            }
        }
    }
    Ok(aspects)
}

fn entity_aspects(kind: WorthEntityKind) -> [WorthAspect; 2] {
    [
        match kind {
            WorthEntityKind::Topology(_) => WorthAspect::Topology(WorthTopologyAspect::Structure),
            WorthEntityKind::Geometry(_) => WorthAspect::Geometry(WorthGeometryAspect::Binding),
            WorthEntityKind::Naming(_) => WorthAspect::Naming(WorthNamingAspect::PersistentName),
            WorthEntityKind::Diagnostics(WorthDiagnosticsEntityKind::WireInterpretation)
            | WorthEntityKind::Diagnostics(WorthDiagnosticsEntityKind::ShellInterpretation) => {
                WorthAspect::Diagnostics(WorthDiagnosticsAspect::Interpretations)
            }
        },
        WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
    ]
}

fn relation_aspects(kind: WorthRelationKind) -> [WorthAspect; 2] {
    [
        match kind {
            WorthRelationKind::Topology(WorthTopologyRelationKind::ModelOwnsBody)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::BodyOwnsLump)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::LumpOwnsRegion)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::RegionOwnsShell)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::ShellOwnsFace)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::WireOwnsHalfEdge) => {
                WorthAspect::Topology(WorthTopologyAspect::Ownership)
            }
            WorthRelationKind::Topology(WorthTopologyRelationKind::FaceOuterLoop)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::FaceInnerLoop)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::LoopOwnsHalfEdge)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeNext)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgePrev)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeUsesEdge)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeStartsAtVertex)
            | WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeEndsAtVertex) => {
                WorthAspect::Topology(WorthTopologyAspect::Boundary)
            }
            WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeRadialNext) => {
                WorthAspect::Topology(WorthTopologyAspect::Radial)
            }
            WorthRelationKind::Geometry(
                WorthGeometryRelationKind::FaceUsesSurfaceBinding
                | WorthGeometryRelationKind::EdgeUsesCurveBinding
                | WorthGeometryRelationKind::HalfEdgeUsesCoedgeBinding
                | WorthGeometryRelationKind::VertexUsesGeometryBinding,
            ) => WorthAspect::Geometry(WorthGeometryAspect::Binding),
            WorthRelationKind::Naming(WorthNamingRelationKind::PersistentNameTargetsEntity) => {
                WorthAspect::Naming(WorthNamingAspect::PersistentName)
            }
            WorthRelationKind::Diagnostics(
                WorthDiagnosticsRelationKind::WireHasInterpretation
                | WorthDiagnosticsRelationKind::ShellHasInterpretation,
            ) => WorthAspect::Diagnostics(WorthDiagnosticsAspect::Interpretations),
        },
        WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::entities::{
        WorthDiagnosticsEntityKind, WorthEntityKind, WorthTopologyEntityKind,
    };
    use crate::data::seed::seed_minimal_topology;
    use forge_relational::facade::runtime::RelationalRuntimeApi;

    #[test]
    fn authority_rejects_identity_shaped_entity_mutations_without_existing_truth() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_setup(|schema| {
                schema.schema_registry(crate::facade::worth_bootstrap_schema_registry().unwrap());
            })
            .build();
        let _seeded = seed_minimal_topology(&mut runtime, "authority-create-reject").unwrap();

        let intent = RawWorthTopologyIntent::new(
            vec![WorthTopologyMutation::UpsertEntity {
                entity_id: EntityId::new(
                    forge_relational::facade::identity::PartitionId::main(),
                    999,
                    1,
                ),
                kind: WorthEntityKind::Topology(WorthTopologyEntityKind::Shell),
            }],
            crate::data::authority::WorthMutationOrigin::LocalEdit,
        );

        let error = WorthTopologyAuthority::new(&mut runtime)
            .apply_topology_intent_traced(intent)
            .unwrap_err()
            .into_error();

        assert!(matches!(
            error,
            WorthTopologyAuthorityError::UnsupportedIdentityEntityMutation(_)
        ));
    }

    #[test]
    fn authority_can_publish_same_commit_topology_graph_creates_with_symbolic_keys() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_setup(|schema| {
                schema.schema_registry(crate::facade::worth_bootstrap_schema_registry().unwrap());
            })
            .build();

        let intent = RawWorthTopologyIntent::new(
            vec![
                WorthTopologyMutation::CreateEntity {
                    create_key: WorthCreateKey::new("create.model"),
                    kind: WorthEntityKind::Topology(WorthTopologyEntityKind::Model),
                },
                WorthTopologyMutation::CreateEntity {
                    create_key: WorthCreateKey::new("create.body"),
                    kind: WorthEntityKind::Topology(WorthTopologyEntityKind::Body),
                },
                WorthTopologyMutation::CreateRelation {
                    create_key: WorthCreateKey::new("create.model.owns_body"),
                    kind: WorthRelationKind::Topology(WorthTopologyRelationKind::ModelOwnsBody),
                    source: WorthEntityReference::Created(WorthCreateKey::new("create.model")),
                    target: WorthEntityReference::Created(WorthCreateKey::new("create.body")),
                },
            ],
            crate::data::authority::WorthMutationOrigin::Seed,
        );

        let verified = WorthTopologyAuthority::new(&mut runtime)
            .apply_topology_intent_traced(intent)
            .expect("create batch should commit")
            .into_primary_result();

        assert_eq!(verified.commits.len(), 1);
        assert_eq!(verified.branch_id.0, "main");
        let read = runtime
            .read_truth()
            .read_snapshot(&verified.persisted_truth.snapshot)
            .expect("verified create snapshot should remain readable");
        assert_eq!(
            read.entities()
                .iter()
                .filter(|record| {
                    record
                        .payload
                        .as_json()
                        .and_then(|json| json.get("label"))
                        .and_then(|value| value.as_str())
                        .is_some_and(|label| label.starts_with("create."))
                })
                .count(),
            2
        );
        assert_eq!(
            read.relations()
                .iter()
                .filter(|record| record.kind.kind_id
                    == WorthRelationKind::Topology(WorthTopologyRelationKind::ModelOwnsBody)
                        .kind_id())
                .count(),
            1
        );
    }

    #[test]
    fn authority_traced_commit_surfaces_schema_owned_trace_envelope() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_setup(|schema| {
                schema.schema_registry(crate::facade::worth_bootstrap_schema_registry().unwrap());
            })
            .build();

        let traced = WorthTopologyAuthority::new(&mut runtime)
            .apply_topology_intent_traced(RawWorthTopologyIntent::new(
                vec![WorthTopologyMutation::CreateEntity {
                    create_key: WorthCreateKey::new("traced.model"),
                    kind: WorthEntityKind::Topology(WorthTopologyEntityKind::Model),
                }],
                crate::data::authority::WorthMutationOrigin::Seed,
            ))
            .expect("traced create batch should commit");

        assert_eq!(traced.primary_result().branch_id.0, "main");
        assert_eq!(
            traced
                .decision_trace()
                .authority
                .as_ref()
                .expect("authority trace evidence")
                .commit_count,
            1
        );
        assert_eq!(
            traced.integrity_markers().truth_basis_identity,
            Some(
                traced
                    .primary_result()
                    .read_basis
                    .authority
                    .truth_basis_identity
                    .clone()
            )
        );
        assert!(traced
            .performance_accounting()
            .counters
            .iter()
            .any(|counter| counter.name == "authority.total_phase_count"));
    }

    #[test]
    fn authority_can_publish_existing_topology_deletions_into_verified_commit_artifacts() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_setup(|schema| {
                schema.schema_registry(crate::facade::worth_bootstrap_schema_registry().unwrap());
            })
            .build();
        let seeded = seed_minimal_topology(&mut runtime, "authority-delete").unwrap();

        let intent = RawWorthTopologyIntent::new(
            vec![WorthTopologyMutation::RemoveEntity {
                entity_id: seeded.vertex,
            }],
            crate::data::authority::WorthMutationOrigin::LocalEdit,
        );

        let verified = WorthTopologyAuthority::new(&mut runtime)
            .apply_topology_intent_traced(intent)
            .expect("delete should commit")
            .into_primary_result();

        assert_eq!(verified.commits.len(), 1);
        let read = runtime
            .read_truth()
            .read_snapshot(&verified.persisted_truth.snapshot)
            .expect("verified snapshot should remain readable");
        assert!(read.get_entity(seeded.vertex).is_none());
        assert!(
            verified
                .read_basis
                .touched_aspects()
                .contains(&WorthAspect::Topology(WorthTopologyAspect::Structure))
                || verified
                    .read_basis
                    .touched_aspects()
                    .contains(&WorthAspect::Topology(WorthTopologyAspect::Boundary))
        );
    }

    #[test]
    fn authority_can_publish_branch_local_topology_commits_on_a_real_branch() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_setup(|schema| {
                schema.schema_registry(crate::facade::worth_bootstrap_schema_registry().unwrap());
            })
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "authority-branch").unwrap();
        runtime
            .history_authority()
            .create_branch(
                BranchId("feature".to_string()),
                &BranchId("main".to_string()),
            )
            .expect("feature branch should be creatable");

        let intent = RawWorthTopologyIntent::new(
            vec![WorthTopologyMutation::RemoveEntity {
                entity_id: seeded.vertex,
            }],
            crate::data::authority::WorthMutationOrigin::BranchLocalApplication,
        );

        let verified = WorthTopologyAuthority::new(&mut runtime)
            .apply_topology_intent_on_branch_traced(intent, BranchId("feature".to_string()))
            .expect("branch-local delete should commit")
            .into_primary_result();

        assert_eq!(verified.branch_id.0, "feature");
        assert_eq!(verified.persisted_truth.branch_id.0, "feature");
        assert_eq!(verified.read_basis.branch_id().0, "feature");
        let history = runtime.history();
        let feature_head = history
            .branch_head(&BranchId("feature".to_string()))
            .expect("feature branch head");
        let main_head = history
            .branch_head(&BranchId("main".to_string()))
            .expect("main branch head");
        assert_ne!(feature_head.commit_id, main_head.commit_id);
    }

    #[test]
    fn authority_can_publish_mixed_create_and_existing_mutations_in_one_commit() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_setup(|schema| {
                schema.schema_registry(crate::facade::worth_bootstrap_schema_registry().unwrap());
            })
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "authority-mixed").unwrap();

        let intent = RawWorthTopologyIntent::new(
            vec![
                WorthTopologyMutation::UpsertEntity {
                    entity_id: seeded.shell,
                    kind: WorthEntityKind::Topology(WorthTopologyEntityKind::Shell),
                },
                WorthTopologyMutation::CreateEntity {
                    create_key: WorthCreateKey::new("authority-mixed.diagnostics.wire"),
                    kind: WorthEntityKind::Diagnostics(
                        WorthDiagnosticsEntityKind::WireInterpretation,
                    ),
                },
            ],
            crate::data::authority::WorthMutationOrigin::LocalEdit,
        );

        let verified = WorthTopologyAuthority::new(&mut runtime)
            .apply_topology_intent_traced(intent)
            .expect("mixed create and existing mutations should commit together")
            .into_primary_result();

        assert_eq!(verified.commits.len(), 1);
        let read = runtime
            .read_truth()
            .read_snapshot(&verified.persisted_truth.snapshot)
            .expect("verified snapshot should remain readable");
        assert!(read.entities().iter().any(|record| {
            record.kind.kind_id
                == WorthEntityKind::Diagnostics(WorthDiagnosticsEntityKind::WireInterpretation)
                    .kind_id()
        }));
    }

    #[test]
    fn authority_create_after_seed_preserves_existing_topology_and_names() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_setup(|schema| {
                schema.schema_registry(crate::facade::worth_bootstrap_schema_registry().unwrap());
            })
            .build();

        let _seeded = seed_minimal_topology(&mut runtime, "authority-create-after-seed").unwrap();
        let intent = RawWorthTopologyIntent::new(
            vec![
                WorthTopologyMutation::CreateEntity {
                    create_key: WorthCreateKey::new("authority-create-after-seed.added_vertex"),
                    kind: WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
                },
                WorthTopologyMutation::CreateEntity {
                    create_key: WorthCreateKey::new(
                        "authority-create-after-seed.added_vertex.persistent_name",
                    ),
                    kind: WorthEntityKind::Naming(
                        crate::data::entities::WorthNamingEntityKind::PersistentName,
                    ),
                },
                WorthTopologyMutation::CreateRelation {
                    create_key: WorthCreateKey::new(
                        "authority-create-after-seed.added_vertex.persistent_name.targets",
                    ),
                    kind: WorthRelationKind::Naming(
                        WorthNamingRelationKind::PersistentNameTargetsEntity,
                    ),
                    source: WorthEntityReference::Created(WorthCreateKey::new(
                        "authority-create-after-seed.added_vertex.persistent_name",
                    )),
                    target: WorthEntityReference::Created(WorthCreateKey::new(
                        "authority-create-after-seed.added_vertex",
                    )),
                },
            ],
            crate::data::authority::WorthMutationOrigin::LocalEdit,
        );

        let verified = WorthTopologyAuthority::new(&mut runtime)
            .apply_topology_intent_traced(intent)
            .expect("post-seed create should commit")
            .into_primary_result();

        let read = runtime
            .read_truth()
            .read_snapshot(&verified.persisted_truth.snapshot)
            .expect("verified snapshot should remain readable");

        for label in [
            "authority-create-after-seed.model",
            "authority-create-after-seed.body",
            "authority-create-after-seed.vertex",
            "authority-create-after-seed.added_vertex",
        ] {
            assert!(read.entities().iter().any(|record| {
                record
                    .payload
                    .as_json()
                    .and_then(|json| json.get("label"))
                    .and_then(|value| value.as_str())
                    .is_some_and(|entity_label| entity_label == label)
            }));
        }

        let naming_targets = read
            .relations()
            .iter()
            .filter(|record| {
                record.kind.kind_id
                    == WorthRelationKind::Naming(
                        WorthNamingRelationKind::PersistentNameTargetsEntity,
                    )
                    .kind_id()
            })
            .count();
        assert_eq!(naming_targets, 12);
    }

    #[test]
    fn authority_rejects_create_key_that_collides_with_live_entity_label() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_setup(|schema| {
                schema.schema_registry(crate::facade::worth_bootstrap_schema_registry().unwrap());
            })
            .build();

        let _seeded = seed_minimal_topology(&mut runtime, "authority-live-label").unwrap();
        let error = WorthTopologyAuthority::new(&mut runtime)
            .apply_topology_intent_traced(RawWorthTopologyIntent::new(
                vec![WorthTopologyMutation::CreateEntity {
                    create_key: WorthCreateKey::new("authority-live-label.vertex"),
                    kind: WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
                }],
                crate::data::authority::WorthMutationOrigin::LocalEdit,
            ))
            .expect_err("duplicate live entity label should be rejected")
            .into_error();

        assert!(matches!(
            error,
            WorthTopologyAuthorityError::DuplicateLiveEntityLabel(_)
        ));
    }
}
