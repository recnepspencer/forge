use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, RelationId};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::symbols::ClientKey;
use forge_relational::facade::transactions::{
    CreateIntent, CreatedEntityRef, DeleteEntityIntent, DeleteRelationIntent,
    EntityReference as RelationalEntityReference, EntitySpec, MutationIntent,
    RelationMutationIntent, RelationSpec, TransactionCommitError,
};

use crate::data::aspects::{
    Aspect, DiagnosticsAspect, GeometryAspect, NamingAspect, TopologyAspect,
};
use crate::data::authority::aspect_field_patches::{
    entity_create_fields, entity_record_label, relation_create_fields,
};
use crate::data::authority::{
    CreateKey, DerivedTopologyReadBasis, EntityReference, PersistedTopologyTruth,
    RawTopologyIntent, TopologyCommittedMutationSet, TopologyMutation,
};
use crate::data::entities::{DiagnosticsEntityKind, EntityKind};
use crate::data::mutation_commit::commit_topology_mutation_set_on_branch_internal;
use crate::data::relations::{
    DiagnosticsRelationKind, GeometryRelationKind, NamingRelationKind, RelationKind,
    TopologyRelationKind,
};
use crate::data::tracing::{
    AuthorityTraceAnchor, AuthorityTraceEvidence, BoundaryEnvelope, BoundaryFailure, DecisionTrace,
    IntegrityMarkers, PerformanceAccounting,
};

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
    pub committed_mutation_set: TopologyCommittedMutationSet,
    pub branch_id: BranchId,
    pub commits: Vec<forge_relational::facade::transactions::CommitResult>,
    pub persisted_truth: PersistedTopologyTruth,
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
        let committed_mutation_set =
            TopologyCommittedMutationSet::from_raw_intent(intent, touched_aspects);

        let commits = self
            .execute_committed_mutation_set(read.as_ref(), &committed_mutation_set, &branch_id)
            .map_err(|error| {
                authority_failure_for_mutation_set(error, &branch_id, &committed_mutation_set)
            })?;

        let persisted_snapshot = commits
            .last()
            .map(|commit| commit.snapshot.clone())
            .unwrap_or(snapshot);
        let persisted_truth = PersistedTopologyTruth {
            committed_mutation_set: committed_mutation_set.clone(),
            snapshot: persisted_snapshot,
            branch_id: branch_id.clone(),
        };
        let read_basis = DerivedTopologyReadBasis::from_persisted_truth(&persisted_truth);
        let verified_commit = VerifiedTopologyCommit {
            committed_mutation_set,
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

    fn execute_committed_mutation_set(
        &mut self,
        read: Option<&forge_relational::facade::runtime::RelationalReadView>,
        committed_mutation_set: &TopologyCommittedMutationSet,
        branch_id: &BranchId,
    ) -> Result<Vec<forge_relational::facade::transactions::CommitResult>, TopologyAuthorityError>
    {
        let lowered = self.lower_committed_mutation_set(read, committed_mutation_set)?;
        if lowered.is_empty() {
            return Ok(Vec::new());
        }

        Ok(vec![commit_topology_mutation_set_on_branch_internal(
            self.runtime,
            branch_id,
            mutation_set_transaction_label(committed_mutation_set.mutation_origin),
            lowered,
        )?])
    }

    fn lower_committed_mutation_set(
        &self,
        read: Option<&forge_relational::facade::runtime::RelationalReadView>,
        committed_mutation_set: &TopologyCommittedMutationSet,
    ) -> Result<Vec<MutationIntent>, TopologyAuthorityError> {
        let mut seen = BTreeSet::new();
        let mut created_entities = BTreeMap::new();

        for mutation in &committed_mutation_set.mutations {
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
        for mutation in &committed_mutation_set.mutations {
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

fn authority_failure_for_intent(
    error: TopologyAuthorityError,
    branch_id: &BranchId,
    intent: &RawTopologyIntent,
) -> BoundaryFailure<TopologyAuthorityError> {
    BoundaryFailure::failure(
        error,
        Vec::new(),
        DecisionTrace {
            authority_anchor: None,
            bridge_anchor: None,
            derived_anchor: None,
            signal_anchor: None,
            authority: None,
            bridge: None,
            derived: None,
            signal: None,
        },
        IntegrityMarkers::new(
            Some(branch_id.clone()),
            BTreeSet::new(),
            Some(intent.mutation_origin),
            None,
            intent.precision_fallbacks.len(),
            intent.precision_budget_fallbacks.len(),
        ),
        PerformanceAccounting::default(),
    )
}

fn authority_failure_for_mutation_set(
    error: TopologyAuthorityError,
    branch_id: &BranchId,
    committed_mutation_set: &TopologyCommittedMutationSet,
) -> BoundaryFailure<TopologyAuthorityError> {
    let authority = match &error {
        TopologyAuthorityError::Commit(commit_error) => {
            Some(AuthorityTraceEvidence::from_commit_logs(
                branch_id.clone(),
                vec![commit_error.commit_log().clone()],
            ))
        }
        _ => None,
    };
    let performance_accounting = authority
        .as_ref()
        .map(AuthorityTraceEvidence::performance_accounting)
        .unwrap_or_default();
    BoundaryFailure::failure(
        error,
        Vec::new(),
        DecisionTrace {
            authority_anchor: None,
            bridge_anchor: None,
            derived_anchor: None,
            signal_anchor: None,
            authority,
            bridge: None,
            derived: None,
            signal: None,
        },
        IntegrityMarkers::new(
            Some(branch_id.clone()),
            committed_mutation_set.touched_aspects.clone(),
            Some(committed_mutation_set.mutation_origin),
            None,
            committed_mutation_set.precision_fallbacks.len(),
            committed_mutation_set.precision_budget_fallbacks.len(),
        ),
        performance_accounting,
    )
}

fn integrity_markers_for_verified_commit(commit: &VerifiedTopologyCommit) -> IntegrityMarkers {
    IntegrityMarkers::new(
        Some(commit.branch_id.clone()),
        commit.committed_mutation_set.touched_aspects.clone(),
        Some(commit.committed_mutation_set.mutation_origin),
        Some(commit.read_basis.authority.truth_basis_identity.clone()),
        commit.committed_mutation_set.precision_fallbacks.len(),
        commit
            .committed_mutation_set
            .precision_budget_fallbacks
            .len(),
    )
}

fn mutation_set_transaction_label(origin: crate::data::authority::MutationOrigin) -> &'static str {
    match origin {
        crate::data::authority::MutationOrigin::Seed => "topology-mutation-seed",
        crate::data::authority::MutationOrigin::LocalEdit => "topology-mutation-local-edit",
        crate::data::authority::MutationOrigin::Replay => "topology-mutation-replay",
        crate::data::authority::MutationOrigin::BranchLocalApplication => {
            "topology-mutation-branch-local"
        }
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

fn touched_aspects_for_intent(
    read: Option<&forge_relational::facade::runtime::RelationalReadView>,
    intent: &RawTopologyIntent,
) -> Result<BTreeSet<Aspect>, TopologyAuthorityError> {
    let mut aspects = BTreeSet::new();
    for mutation in &intent.mutations {
        match mutation {
            TopologyMutation::CreateEntity { kind, .. } => {
                aspects.extend(entity_aspects(*kind));
            }
            TopologyMutation::CreateRelation { kind, .. } => {
                aspects.extend(relation_aspects(*kind));
            }
            TopologyMutation::UpsertEntity { kind, .. } => {
                aspects.extend(entity_aspects(*kind));
            }
            TopologyMutation::UpsertRelation { kind, .. } => {
                aspects.extend(relation_aspects(*kind));
            }
            TopologyMutation::RemoveEntity { entity_id } => {
                let read = read.ok_or_else(|| {
                    TopologyAuthorityError::ReadSnapshot(
                        " authority requires a readable starting snapshot for entity removal"
                            .to_string(),
                    )
                })?;
                let Some(existing) = read.get_entity(*entity_id) else {
                    return Err(TopologyAuthorityError::MissingEntity(*entity_id));
                };
                let kind = EntityKind::from_kind_id(existing.kind.kind_id).ok_or_else(|| {
                    TopologyAuthorityError::ReadSnapshot(format!(
                        "unknown  entity kind id `{}` for entity `{:?}`",
                        existing.kind.kind_id.0, entity_id
                    ))
                })?;
                aspects.extend(entity_aspects(kind));
            }
            TopologyMutation::RemoveRelation { relation_id } => {
                let read = read.ok_or_else(|| {
                    TopologyAuthorityError::ReadSnapshot(
                        " authority requires a readable starting snapshot for relation removal"
                            .to_string(),
                    )
                })?;
                let Some(existing) = read.get_relation(*relation_id) else {
                    return Err(TopologyAuthorityError::MissingRelation(*relation_id));
                };
                let kind = RelationKind::from_kind_id(existing.kind.kind_id).ok_or_else(|| {
                    TopologyAuthorityError::ReadSnapshot(format!(
                        "unknown  relation kind id `{}` for relation `{:?}`",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::entities::{DiagnosticsEntityKind, EntityKind, TopologyEntityKind};
    use crate::data::seed::seed_minimal_topology;
    use forge_relational::facade::runtime::RelationalRuntimeApi;

    #[test]
    fn authority_rejects_identity_shaped_entity_mutations_without_existing_truth() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_setup(|schema| {
                schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
            })
            .build();
        let _seeded = seed_minimal_topology(&mut runtime, "authority-create-reject").unwrap();

        let intent = RawTopologyIntent::new(
            vec![TopologyMutation::UpsertEntity {
                entity_id: EntityId::new(
                    forge_relational::facade::identity::PartitionId::main(),
                    999,
                    1,
                ),
                kind: EntityKind::Topology(TopologyEntityKind::Shell),
            }],
            crate::data::authority::MutationOrigin::LocalEdit,
        );

        let error = TopologyAuthority::new(&mut runtime)
            .apply_topology_intent_traced(intent)
            .unwrap_err()
            .into_error();

        assert!(matches!(
            error,
            TopologyAuthorityError::UnsupportedIdentityEntityMutation(_)
        ));
    }

    #[test]
    fn authority_can_publish_same_commit_topology_graph_creates_with_symbolic_keys() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_setup(|schema| {
                schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
            })
            .build();

        let intent = RawTopologyIntent::new(
            vec![
                TopologyMutation::CreateEntity {
                    create_key: CreateKey::new("create.model"),
                    kind: EntityKind::Topology(TopologyEntityKind::Model),
                },
                TopologyMutation::CreateEntity {
                    create_key: CreateKey::new("create.body"),
                    kind: EntityKind::Topology(TopologyEntityKind::Body),
                },
                TopologyMutation::CreateRelation {
                    create_key: CreateKey::new("create.model.owns_body"),
                    kind: RelationKind::Topology(TopologyRelationKind::ModelOwnsBody),
                    source: EntityReference::Created(CreateKey::new("create.model")),
                    target: EntityReference::Created(CreateKey::new("create.body")),
                },
            ],
            crate::data::authority::MutationOrigin::Seed,
        );

        let verified = TopologyAuthority::new(&mut runtime)
            .apply_topology_intent_traced(intent)
            .expect("create mutation set should commit")
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
                    EntityKind::from_kind_id(record.kind.kind_id)
                        .and_then(|kind| entity_record_label(record, kind))
                        .is_some_and(|label| label.starts_with("create."))
                })
                .count(),
            2
        );
        assert_eq!(
            read.relations()
                .iter()
                .filter(|record| record.kind.kind_id
                    == RelationKind::Topology(TopologyRelationKind::ModelOwnsBody).kind_id())
                .count(),
            1
        );
    }

    #[test]
    fn authority_traced_commit_surfaces_schema_owned_trace_envelope() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_setup(|schema| {
                schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
            })
            .build();

        let traced = TopologyAuthority::new(&mut runtime)
            .apply_topology_intent_traced(RawTopologyIntent::new(
                vec![TopologyMutation::CreateEntity {
                    create_key: CreateKey::new("traced.model"),
                    kind: EntityKind::Topology(TopologyEntityKind::Model),
                }],
                crate::data::authority::MutationOrigin::Seed,
            ))
            .expect("traced create mutation set should commit");

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
                schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
            })
            .build();
        let seeded = seed_minimal_topology(&mut runtime, "authority-delete").unwrap();

        let intent = RawTopologyIntent::new(
            vec![TopologyMutation::RemoveEntity {
                entity_id: seeded.vertex,
            }],
            crate::data::authority::MutationOrigin::LocalEdit,
        );

        let verified = TopologyAuthority::new(&mut runtime)
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
                .contains(&Aspect::Topology(TopologyAspect::Structure))
                || verified
                    .read_basis
                    .touched_aspects()
                    .contains(&Aspect::Topology(TopologyAspect::Boundary))
        );
    }

    #[test]
    fn authority_can_publish_branch_local_topology_commits_on_a_real_branch() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_setup(|schema| {
                schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
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

        let intent = RawTopologyIntent::new(
            vec![TopologyMutation::RemoveEntity {
                entity_id: seeded.vertex,
            }],
            crate::data::authority::MutationOrigin::BranchLocalApplication,
        );

        let verified = TopologyAuthority::new(&mut runtime)
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
                schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
            })
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "authority-mixed").unwrap();

        let intent = RawTopologyIntent::new(
            vec![
                TopologyMutation::UpsertEntity {
                    entity_id: seeded.shell,
                    kind: EntityKind::Topology(TopologyEntityKind::Shell),
                },
                TopologyMutation::CreateEntity {
                    create_key: CreateKey::new("authority-mixed.diagnostics.wire"),
                    kind: EntityKind::Diagnostics(DiagnosticsEntityKind::WireInterpretation),
                },
            ],
            crate::data::authority::MutationOrigin::LocalEdit,
        );

        let verified = TopologyAuthority::new(&mut runtime)
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
                == EntityKind::Diagnostics(DiagnosticsEntityKind::WireInterpretation).kind_id()
        }));
    }

    #[test]
    fn authority_create_after_seed_preserves_existing_topology_and_names() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_setup(|schema| {
                schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
            })
            .build();

        let _seeded = seed_minimal_topology(&mut runtime, "authority-create-after-seed").unwrap();
        let intent = RawTopologyIntent::new(
            vec![
                TopologyMutation::CreateEntity {
                    create_key: CreateKey::new("authority-create-after-seed.added_vertex"),
                    kind: EntityKind::Topology(TopologyEntityKind::Vertex),
                },
                TopologyMutation::CreateEntity {
                    create_key: CreateKey::new(
                        "authority-create-after-seed.added_vertex.persistent_name",
                    ),
                    kind: EntityKind::Naming(
                        crate::data::entities::NamingEntityKind::PersistentName,
                    ),
                },
                TopologyMutation::CreateRelation {
                    create_key: CreateKey::new(
                        "authority-create-after-seed.added_vertex.persistent_name.targets",
                    ),
                    kind: RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity),
                    source: EntityReference::Created(CreateKey::new(
                        "authority-create-after-seed.added_vertex.persistent_name",
                    )),
                    target: EntityReference::Created(CreateKey::new(
                        "authority-create-after-seed.added_vertex",
                    )),
                },
            ],
            crate::data::authority::MutationOrigin::LocalEdit,
        );

        let verified = TopologyAuthority::new(&mut runtime)
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
                EntityKind::from_kind_id(record.kind.kind_id)
                    .and_then(|kind| entity_record_label(record, kind))
                    .is_some_and(|entity_label| entity_label == label)
            }));
        }

        let naming_targets = read
            .relations()
            .iter()
            .filter(|record| {
                record.kind.kind_id
                    == RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity)
                        .kind_id()
            })
            .count();
        assert_eq!(naming_targets, 12);
    }

    #[test]
    fn authority_rejects_create_key_that_collides_with_live_entity_label() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_setup(|schema| {
                schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
            })
            .build();

        let _seeded = seed_minimal_topology(&mut runtime, "authority-live-label").unwrap();
        let error = TopologyAuthority::new(&mut runtime)
            .apply_topology_intent_traced(RawTopologyIntent::new(
                vec![TopologyMutation::CreateEntity {
                    create_key: CreateKey::new("authority-live-label.vertex"),
                    kind: EntityKind::Topology(TopologyEntityKind::Vertex),
                }],
                crate::data::authority::MutationOrigin::LocalEdit,
            ))
            .expect_err("duplicate live entity label should be rejected")
            .into_error();

        assert!(matches!(
            error,
            TopologyAuthorityError::DuplicateLiveEntityLabel(_)
        ));
    }
}
