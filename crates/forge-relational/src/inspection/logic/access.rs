use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::history::data::{AspectFilter, BranchId, CommitId};
use crate::inspection::data::{
    CommitInspection, ConnectivityComponentSummary, ConnectivityInspectionRequest,
    ConnectivityInspectionSummary, GraphInspectionRequest, GraphInspectionSummary,
    HistoricalAspectObservation, HistoricalAvailabilityObservation, HistoricalInspectionMode,
    HistoricalOpenResult, HistoricalRecordInspection, HistoricalRecordObservation,
    HistoricalRecordValue, HistoricalSnapshotView, InspectionAccessPath, InspectionAvailability,
    InspectionDegradation, InspectionOrigin, InspectionRecordClass, InspectionResolutionContext,
    InspectionScope, KindInspectionRequest, KindInspectionSummary, NeighborInspectionResult,
    PinStateObservation, RecentCommitInspectionRequest, RecentCommitInspectionWindow,
    ReclaimEligibility, RecordRetentionInspection, RetentionInspectionSummary,
    RetentionStateObservation, SnapshotPinInspection, StructuralIdentityComparison,
    StructuralIdentityComparisonVerdict, StructuralIdentityEvidence,
    StructuralIdentityQueryRequest,
};
use crate::logic::runtime::RelationalRuntime;
use crate::publication::patch::data::CanonicalAspectSet;
use crate::storage::data::{RecordLifecycleState, RelationalReadView, RetentionPlan};
use crate::transactions::data::RecordRef;
use crate::visibility::cache_state::cached_state_for_version;

pub struct InspectionAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl RelationalRuntime {
    pub fn inspection_access(&self) -> InspectionAccess<'_> {
        InspectionAccess::new(self)
    }
}

impl<'runtime> InspectionAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn structural_identity(
        &self,
        scope: InspectionScope,
        target: RecordRef,
    ) -> Option<StructuralIdentityEvidence> {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.inspection_structural_identity_lookups += 1);
        let version_id = self.scope_version_id(&scope)?;
        match target {
            RecordRef::Entity(entity_id) => {
                let read = self.runtime.visibility_reads().entity_record_for_id_at_version(
                    &self.runtime.storage_access().current_state(),
                    entity_id,
                    version_id,
                )?;
                let partition = self.runtime.storage_access().partition_state(entity_id.partition_id)?;
                let slot = entity_id.local_slot.0 as usize;
                let slot_view = partition.entity_arena.get_slot(slot)?;
                let extra = slot_view.extra();
                let mut degradations = Vec::new();
                if extra.structural_fingerprint.is_none() {
                    degradations.push(InspectionDegradation::MissingStructuralFingerprint);
                }
                if extra.lineage_id.is_none() {
                    degradations.push(InspectionDegradation::MissingLineageIdentity);
                }
                Some(StructuralIdentityEvidence {
                    target,
                    record_class: InspectionRecordClass::Entity,
                    kind_id: read.kind.kind_id,
                    storage_identity: RecordRef::Entity(entity_id),
                    lineage_id: extra.lineage_id,
                    structural_fingerprint: extra.structural_fingerprint,
                    observed_version: version_id,
                    lifecycle: read.lifecycle,
                    origin: InspectionOrigin::CurrentTruth,
                    access_path: self.scope_access_path(&scope, version_id),
                    availability: self.scope_availability(&scope, version_id),
                    degradations,
                })
            }
            RecordRef::Relation(relation_id) => {
                let read = self.runtime.visibility_reads().relation_record_for_id_at_version(
                    &self.runtime.storage_access().current_state(),
                    relation_id,
                    version_id,
                )?;
                Some(StructuralIdentityEvidence {
                    target,
                    record_class: InspectionRecordClass::Relation,
                    kind_id: read.kind.kind_id,
                    storage_identity: RecordRef::Relation(relation_id),
                    lineage_id: None,
                    structural_fingerprint: None,
                    observed_version: version_id,
                    lifecycle: read.lifecycle,
                    origin: InspectionOrigin::CurrentTruth,
                    access_path: self.scope_access_path(&scope, version_id),
                    availability: self.scope_availability(&scope, version_id),
                    degradations: vec![
                        InspectionDegradation::MissingStructuralFingerprint,
                        InspectionDegradation::MissingLineageIdentity,
                    ],
                })
            }
        }
    }

    pub fn compare_structural_identity(
        &self,
        scope: InspectionScope,
        left: RecordRef,
        right: RecordRef,
    ) -> StructuralIdentityComparison {
        let left_evidence = self.structural_identity(scope.clone(), left);
        let right_evidence = self.structural_identity(scope, right);
        let verdict = match (&left_evidence, &right_evidence) {
            (Some(left), Some(right)) => match (left.structural_fingerprint, right.structural_fingerprint) {
                (Some(left), Some(right)) if left.family == right.family && left.value == right.value => {
                    StructuralIdentityComparisonVerdict::EqualByFingerprint
                }
                (Some(left), Some(right)) if left.family == right.family => {
                    StructuralIdentityComparisonVerdict::NotEqualByFingerprint
                }
                (Some(_), Some(_)) => {
                    StructuralIdentityComparisonVerdict::IncomparableFingerprintFamilyMismatch
                }
                _ => StructuralIdentityComparisonVerdict::IncomparableMissingFingerprint,
            },
            _ => StructuralIdentityComparisonVerdict::IncomparableMissingFingerprint,
        };
        StructuralIdentityComparison {
            left: left_evidence,
            right: right_evidence,
            verdict,
        }
    }

    pub fn query_structural_identity(
        &self,
        request: &StructuralIdentityQueryRequest,
    ) -> Vec<StructuralIdentityEvidence> {
        self.runtime.services.instrumentation.count(|counters| {
            counters.inspection_structural_identity_query_scans += 1;
        });
        let Some(version_id) = self.scope_version_id(&request.scope) else {
            return Vec::new();
        };
        let Some(read_view) = self.read_view_for_scope(&request.scope) else {
            return Vec::new();
        };
        let partition_scope = request
            .partition_scope
            .as_ref()
            .map(|scope| scope.iter().copied().collect::<BTreeSet<_>>());
        read_view
            .entities()
            .iter()
            .filter(|record| {
                partition_scope
                    .as_ref()
                    .is_none_or(|scope| scope.contains(&record.entity_id.partition_id))
            })
            .filter_map(|record| {
                let evidence = self.structural_identity(
                    InspectionScope::Version(version_id),
                    RecordRef::Entity(record.entity_id),
                )?;
                evidence
                    .structural_fingerprint
                    .is_some_and(|fingerprint| fingerprint.family == request.fingerprint_family)
                    .then_some(evidence)
            })
            .collect()
    }

    pub fn graph_summary(&self, request: &GraphInspectionRequest) -> GraphInspectionSummary {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.inspection_graph_summary_requests += 1);
        let version_id = self
            .scope_version_id(&request.scope)
            .unwrap_or_else(|| self.runtime.current_version_id());
        let read_view = self
            .read_view_for_scope(&request.scope)
            .unwrap_or_else(|| self.runtime.visibility_reads().read_version(version_id));
        let partition_scope = request
            .partition_scope
            .as_ref()
            .map(|scope| scope.iter().copied().collect::<BTreeSet<_>>());

        let entity_records = read_view
            .entities()
            .iter()
            .filter(|record| {
                partition_scope
                    .as_ref()
                    .is_none_or(|scope| scope.contains(&record.entity_id.partition_id))
            })
            .collect::<Vec<_>>();
        let relation_records = read_view
            .relations()
            .iter()
            .filter(|record| {
                partition_scope
                    .as_ref()
                    .is_none_or(|scope| scope.contains(&record.relation_id.partition_id))
            })
            .filter(|record| {
                request
                    .relation_kind_scope
                    .as_ref()
                    .is_none_or(|scope| scope.contains(&record.kind.kind_id))
            })
            .collect::<Vec<_>>();

        let mut entity_kinds = BTreeMap::<_, usize>::new();
        let mut relation_kinds = BTreeMap::<_, usize>::new();
        let mut partition_ids = BTreeSet::new();
        for record in &entity_records {
            *entity_kinds.entry(record.kind.kind_id).or_default() += 1;
            partition_ids.insert(record.entity_id.partition_id);
        }
        for record in &relation_records {
            *relation_kinds.entry(record.kind.kind_id).or_default() += 1;
            partition_ids.insert(record.relation_id.partition_id);
        }

        GraphInspectionSummary {
            scope: request.scope.clone(),
            version_id,
            partition_count: partition_ids.len(),
            entity_count: entity_records.len(),
            relation_count: relation_records.len(),
            entity_kinds: entity_kinds.into_iter().collect(),
            relation_kinds: relation_kinds.into_iter().collect(),
            origin: InspectionOrigin::VisibilitySnapshot,
            access_path: self.scope_access_path(&request.scope, version_id),
            availability: self.scope_availability(&request.scope, version_id),
            degradations: if request.summary_only {
                vec![InspectionDegradation::SummaryOnly]
            } else {
                Vec::new()
            },
        }
    }

    pub fn kind_summary(&self, request: &KindInspectionRequest) -> KindInspectionSummary {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.inspection_kind_summary_requests += 1);
        let version_id = self
            .scope_version_id(&request.scope)
            .unwrap_or_else(|| self.runtime.current_version_id());
        let read_view = self
            .read_view_for_scope(&request.scope)
            .unwrap_or_else(|| self.runtime.visibility_reads().read_version(version_id));
        let partition_scope = request
            .partition_scope
            .as_ref()
            .map(|scope| scope.iter().copied().collect::<BTreeSet<_>>());
        let mut touched_partitions = BTreeSet::new();
        let count = match request.record_class {
            InspectionRecordClass::Entity => read_view
                .entities()
                .iter()
                .filter(|record| record.kind.kind_id == request.kind_id)
                .filter(|record| {
                    partition_scope
                        .as_ref()
                        .is_none_or(|scope| scope.contains(&record.entity_id.partition_id))
                })
                .inspect(|record| {
                    touched_partitions.insert(record.entity_id.partition_id);
                })
                .count(),
            InspectionRecordClass::Relation => read_view
                .relations()
                .iter()
                .filter(|record| record.kind.kind_id == request.kind_id)
                .filter(|record| {
                    partition_scope
                        .as_ref()
                        .is_none_or(|scope| scope.contains(&record.relation_id.partition_id))
                })
                .inspect(|record| {
                    touched_partitions.insert(record.relation_id.partition_id);
                })
                .count(),
        };
        KindInspectionSummary {
            scope: request.scope.clone(),
            version_id,
            kind_id: request.kind_id,
            record_class: request.record_class,
            count,
            touched_partitions: touched_partitions.into_iter().collect(),
            origin: InspectionOrigin::VisibilitySnapshot,
            access_path: self.scope_access_path(&request.scope, version_id),
            availability: self.scope_availability(&request.scope, version_id),
        }
    }

    pub fn connectivity_summary(
        &self,
        request: &ConnectivityInspectionRequest,
    ) -> ConnectivityInspectionSummary {
        self.runtime.services.instrumentation.count(|counters| {
            counters.inspection_connectivity_summary_requests += 1;
        });
        let version_id = self
            .scope_version_id(&request.scope)
            .unwrap_or_else(|| self.runtime.current_version_id());
        let read_view = self
            .read_view_for_scope(&request.scope)
            .unwrap_or_else(|| self.runtime.visibility_reads().read_version(version_id));
        let partition_scope = request
            .partition_scope
            .as_ref()
            .map(|scope| scope.iter().copied().collect::<BTreeSet<_>>());
        let entities = read_view
            .entities()
            .iter()
            .filter(|record| {
                partition_scope
                    .as_ref()
                    .is_none_or(|scope| scope.contains(&record.entity_id.partition_id))
            })
            .map(|record| record.entity_id)
            .collect::<Vec<_>>();
        let entity_set = entities.iter().copied().collect::<BTreeSet<_>>();
        let relations = read_view
            .relations()
            .iter()
            .filter(|record| entity_set.contains(&record.source) && entity_set.contains(&record.target))
            .filter(|record| {
                request
                    .relation_kind_scope
                    .as_ref()
                    .is_none_or(|scope| scope.contains(&record.kind.kind_id))
            })
            .collect::<Vec<_>>();

        let mut adjacency = BTreeMap::<_, BTreeSet<_>>::new();
        for entity in &entities {
            adjacency.entry(*entity).or_default();
        }
        for relation in relations {
            adjacency.entry(relation.source).or_default().insert(relation.target);
            adjacency.entry(relation.target).or_default().insert(relation.source);
        }

        let mut visited = BTreeSet::new();
        let mut components = Vec::new();
        for entity in &entities {
            if !visited.insert(*entity) {
                continue;
            }
            let mut queue = VecDeque::from([*entity]);
            let mut members = vec![*entity];
            while let Some(current) = queue.pop_front() {
                for neighbor in adjacency.get(&current).into_iter().flatten() {
                    if visited.insert(*neighbor) {
                        members.push(*neighbor);
                        queue.push_back(*neighbor);
                    }
                }
            }
            members.sort();
            components.push(ConnectivityComponentSummary {
                member_count: members.len(),
                members: request.include_members.then_some(members),
            });
        }

        ConnectivityInspectionSummary {
            scope: request.scope.clone(),
            version_id,
            component_count: components.len(),
            largest_component_size: components
                .iter()
                .map(|component| component.member_count)
                .max()
                .unwrap_or(0),
            enumerated_entity_count: entities.len(),
            components,
            origin: InspectionOrigin::VisibilitySnapshot,
            access_path: self.scope_access_path(&request.scope, version_id),
            resolution_context: InspectionResolutionContext::ConnectivityTraversal,
            availability: self.scope_availability(&request.scope, version_id),
            degradations: if request.include_members {
                Vec::new()
            } else {
                vec![InspectionDegradation::SummaryOnly]
            },
        }
    }

    pub fn neighbors(
        &self,
        scope: InspectionScope,
        entity_id: crate::identity::data::EntityId,
    ) -> NeighborInspectionResult {
        self.runtime.services.instrumentation.count(|counters| {
            counters.inspection_neighbor_requests += 1;
        });
        let version_id = self
            .scope_version_id(&scope)
            .unwrap_or_else(|| self.runtime.current_version_id());
        let read_view = self
            .read_view_for_scope(&scope)
            .unwrap_or_else(|| self.runtime.visibility_reads().read_version(version_id));
        NeighborInspectionResult {
            entity_id,
            version_id,
            outgoing_relation_ids: read_view
                .relations()
                .iter()
                .filter(|record| record.source == entity_id)
                .map(|record| record.relation_id)
                .collect(),
            incoming_relation_ids: read_view
                .relations()
                .iter()
                .filter(|record| record.target == entity_id)
                .map(|record| record.relation_id)
                .collect(),
            origin: InspectionOrigin::VisibilitySnapshot,
            access_path: self.scope_access_path(&scope, version_id),
            resolution_context: InspectionResolutionContext::RelationNeighborhood,
            availability: self.scope_availability(&scope, version_id),
        }
    }

    pub fn open_historical_view(
        &self,
        version_id: crate::identity::data::VersionId,
        mode: HistoricalInspectionMode,
    ) -> HistoricalOpenResult {
        self.runtime.services.instrumentation.count(|counters| {
            counters.inspection_historical_view_opens += 1;
        });
        let direct_available = version_id == self.runtime.current_version_id()
            || cached_state_for_version(self.runtime, version_id).is_some();
        match mode {
            HistoricalInspectionMode::RetainedOnly if !direct_available => HistoricalOpenResult {
                view: None,
                origin: InspectionOrigin::VisibilitySnapshot,
                access_path: InspectionAccessPath::HistoricalRetainedRead,
                availability: InspectionAvailability::UnavailableByRetention,
                degradations: vec![InspectionDegradation::ReconstructionOmittedByMode],
            },
            HistoricalInspectionMode::RetainedOnly | HistoricalInspectionMode::AllowCanonicalReconstruction => {
                let read_view = self.runtime.visibility_reads().read_version(version_id);
                let availability = if direct_available {
                    InspectionAvailability::Direct
                } else {
                    InspectionAvailability::Reconstructed
                };
                let access_path = if direct_available {
                    InspectionAccessPath::HistoricalRetainedRead
                } else {
                    InspectionAccessPath::HistoricalReconstructedRead
                };
                HistoricalOpenResult {
                    view: Some(HistoricalSnapshotView {
                        snapshot: read_view.snapshot().clone(),
                        read_view,
                        origin: InspectionOrigin::VisibilitySnapshot,
                        access_path,
                        availability,
                    }),
                    origin: InspectionOrigin::VisibilitySnapshot,
                    access_path,
                    availability,
                    degradations: Vec::new(),
                }
            }
        }
    }

    pub fn inspect_historical_record(
        &self,
        branch_id: &BranchId,
        version_id: crate::identity::data::VersionId,
        target: RecordRef,
        mode: HistoricalInspectionMode,
    ) -> HistoricalRecordInspection {
        let open_result = self.open_historical_view(version_id, mode);
        let target_for_observation = target.clone();
        let record_observation = match (&open_result.view, target_for_observation) {
            (Some(view), RecordRef::Entity(entity_id)) => HistoricalRecordObservation {
                target: target.clone(),
                version_id,
                value: view
                    .read_view
                    .get_entity(entity_id)
                    .cloned()
                    .map(HistoricalRecordValue::Entity),
                origin: InspectionOrigin::VisibilitySnapshot,
                access_path: open_result.access_path,
                availability: open_result.availability,
            },
            (Some(view), RecordRef::Relation(relation_id)) => HistoricalRecordObservation {
                target: target.clone(),
                version_id,
                value: view
                    .read_view
                    .get_relation(relation_id)
                    .cloned()
                    .map(HistoricalRecordValue::Relation),
                origin: InspectionOrigin::VisibilitySnapshot,
                access_path: open_result.access_path,
                availability: open_result.availability,
            },
            (None, _) => HistoricalRecordObservation {
                target: target.clone(),
                version_id,
                value: None,
                origin: InspectionOrigin::VisibilitySnapshot,
                access_path: open_result.access_path,
                availability: open_result.availability,
            },
        };
        let lineage_resolution_context = match target {
            RecordRef::Entity(entity_id) => {
                self.runtime.lineage_access().resolve_record_history(branch_id, entity_id)
            }
            RecordRef::Relation(_) => None,
        };
        let aspect_history_observation = match target.clone() {
            RecordRef::Entity(entity_id) => Some(HistoricalAspectObservation {
                query_result: self
                    .runtime
                    .history_access()
                    .entity_aspect_history_with_trace(branch_id, entity_id, None::<&AspectFilter>),
                origin: InspectionOrigin::CanonicalCommitStorage,
                access_path: InspectionAccessPath::CommitIndexRead,
                availability: InspectionAvailability::Direct,
            }),
            RecordRef::Relation(relation_id) => Some(HistoricalAspectObservation {
                query_result: self.runtime.history_access().relation_aspect_history_with_trace(
                    branch_id,
                    relation_id,
                    None::<&AspectFilter>,
                ),
                origin: InspectionOrigin::CanonicalCommitStorage,
                access_path: InspectionAccessPath::CommitIndexRead,
                availability: InspectionAvailability::Direct,
            }),
        };
        let structural_identity_evidence = open_result.view.as_ref().and_then(|_| {
            self.structural_identity(InspectionScope::Version(version_id), target.clone())
        });
        HistoricalRecordInspection {
            branch_id: branch_id.clone(),
            record_observation,
            lineage_resolution_context,
            aspect_history_observation,
            structural_identity_evidence,
            retention_availability_observation: Some(HistoricalAvailabilityObservation {
                version_id,
                availability: open_result.availability,
                retained_directly: open_result.availability == InspectionAvailability::Direct,
            }),
        }
    }

    pub fn retention_summary(&self) -> RetentionInspectionSummary {
        let plan = self.inspect_retention_plan();
        RetentionInspectionSummary {
            current_version_id: self.runtime.current_version_id(),
            active_snapshot_count: plan.active_snapshot_count,
            branch_pinned_entities: plan.branch_pinned_entities,
            replay_pinned_entities: plan.replay_pinned_entities,
            snapshot_pinned_entities: plan.snapshot_pinned_entities,
            branch_pinned_relations: plan.branch_pinned_relations,
            replay_pinned_relations: plan.replay_pinned_relations,
            snapshot_pinned_relations: plan.snapshot_pinned_relations,
            reclaimable_entities: plan.reclaimable_entities,
            reclaimable_relations: plan.reclaimable_relations,
            origin: InspectionOrigin::RetentionState,
            access_path: InspectionAccessPath::DirectLookup,
            availability: InspectionAvailability::Direct,
        }
    }

    pub fn inspect_record_retention(&self, target: RecordRef) -> Option<RecordRetentionInspection> {
        let version_id = self.runtime.current_version_id();
        match target {
            RecordRef::Entity(entity_id) => {
                let surface = self
                    .runtime
                    .storage_access()
                    .record_slot_surface::<crate::storage::logic::state::EntityRecordKind>(
                        entity_id.partition_id,
                        entity_id.local_slot.0 as usize,
                    )?;
                Some(self.record_retention_inspection(
                    target,
                    surface.lifecycle,
                    surface.snapshot_pins,
                    surface.branch_pins,
                    surface.replay_pins,
                    version_id,
                ))
            }
            RecordRef::Relation(relation_id) => {
                let surface = self
                    .runtime
                    .storage_access()
                    .record_slot_surface::<crate::storage::logic::state::RelationRecordKind>(
                        relation_id.partition_id,
                        relation_id.local_slot.0 as usize,
                    )?;
                Some(self.record_retention_inspection(
                    target,
                    surface.lifecycle,
                    surface.snapshot_pins,
                    surface.branch_pins,
                    surface.replay_pins,
                    version_id,
                ))
            }
        }
    }

    pub fn inspect_snapshot_pinning(
        &self,
        handle: &crate::snapshots::data::SnapshotHandle,
    ) -> Option<SnapshotPinInspection> {
        Some(SnapshotPinInspection {
            snapshot: self.runtime.visibility_reads().inspect_snapshot(handle)?,
            origin: InspectionOrigin::VisibilitySnapshot,
            access_path: InspectionAccessPath::SnapshotRead,
            availability: InspectionAvailability::Direct,
        })
    }

    pub fn inspect_commit(&self, commit_id: CommitId) -> Option<CommitInspection> {
        self.runtime.services.instrumentation.count(|counters| {
            counters.inspection_commit_reads += 1;
        });
        let history_access = self.runtime.history_access();
        let envelope = history_access.commit_envelope(commit_id)?;
        Some(CommitInspection {
            commit: envelope.commit.clone(),
            changed_records: envelope
                .patch
                .records
                .iter()
                .map(|record| record.target.clone())
                .collect(),
            lineage_event_ids: envelope.lineage_event_ids.clone(),
            changed_aspects: CanonicalAspectSet::new(
                envelope
                    .patch
                    .records
                    .iter()
                    .flat_map(|record| record.aspects.iter().cloned()),
            ),
            origin: InspectionOrigin::CanonicalCommitStorage,
            access_path: InspectionAccessPath::CommitIndexRead,
        })
    }

    pub fn inspect_recent_commits(
        &self,
        request: &RecentCommitInspectionRequest,
    ) -> RecentCommitInspectionWindow {
        let commits = self
            .runtime
            .history
            .commit_envelopes
            .keys()
            .rev()
            .filter_map(|commit_id| self.inspect_commit(*commit_id))
            .filter(|inspection| {
                request
                    .branch_id
                    .as_ref()
                    .is_none_or(|branch_id| inspection.commit.branch_id == *branch_id)
            })
            .take(request.limit)
            .collect();
        let branch_head = request
            .branch_id
            .as_ref()
            .and_then(|branch_id| self.runtime.history_access().branch_head(branch_id).cloned());
        RecentCommitInspectionWindow {
            branch_head,
            commits,
            origin: InspectionOrigin::CanonicalCommitStorage,
            access_path: InspectionAccessPath::CommitIndexRead,
        }
    }

    pub fn inspect_branch_head(&self, branch_id: &BranchId) -> Option<CommitInspection> {
        let history = self.runtime.history_access();
        let head = history.branch_head(branch_id)?;
        self.inspect_commit(head.commit_id)
    }

    fn record_retention_inspection(
        &self,
        target: RecordRef,
        lifecycle: RecordLifecycleState,
        snapshot_pins: u32,
        branch_pins: u32,
        replay_pins: u32,
        version_id: crate::identity::data::VersionId,
    ) -> RecordRetentionInspection {
        let reclaim_eligibility = if !self.runtime.config.storage.mvcc.auto_reclaim_deleted_records {
            ReclaimEligibility::BlockedByPolicy
        } else if snapshot_pins > 0 {
            ReclaimEligibility::BlockedBySnapshotPins
        } else if branch_pins > 0 {
            ReclaimEligibility::BlockedByBranchPins
        } else if replay_pins > 0 {
            ReclaimEligibility::BlockedByReplayPins
        } else if lifecycle == RecordLifecycleState::Reclaimable {
            ReclaimEligibility::EligibleNow
        } else {
            ReclaimEligibility::BlockedByRetentionFence
        };
        RecordRetentionInspection {
            state: RetentionStateObservation {
                target: target.clone(),
                lifecycle,
            },
            pins: PinStateObservation {
                target,
                snapshot_pins,
                branch_pins,
                replay_pins,
            },
            reclaim_eligibility,
            historical_availability: HistoricalAvailabilityObservation {
                version_id,
                availability: if lifecycle == RecordLifecycleState::Reusable {
                    InspectionAvailability::UnavailableByRetention
                } else {
                    InspectionAvailability::Direct
                },
                retained_directly: lifecycle != RecordLifecycleState::Reusable,
            },
        }
    }

    fn inspect_retention_plan(&self) -> RetentionPlan {
        let retention_fence = self
            .runtime
            .visibility
            .retention_fence_version(self.runtime.current_version_id());
        let mut branch_pinned_entities = 0;
        let mut replay_pinned_entities = 0;
        let mut snapshot_pinned_entities = 0;
        let mut branch_pinned_relations = 0;
        let mut replay_pinned_relations = 0;
        let mut snapshot_pinned_relations = 0;
        let mut reclaimable_entities = 0;
        let mut reclaimable_relations = 0;
        for partition_id in self.runtime.storage_access().partition_ids() {
            for slot in 0..self
                .runtime
                .storage_access()
                .record_slot_count::<crate::storage::logic::state::EntityRecordKind>(partition_id)
            {
                if let Some(surface) = self
                    .runtime
                    .storage_access()
                    .record_slot_surface::<crate::storage::logic::state::EntityRecordKind>(
                        partition_id, slot,
                    )
                {
                    if surface.branch_pins > 0 {
                        branch_pinned_entities += 1;
                    }
                    if surface.replay_pins > 0 {
                        replay_pinned_entities += 1;
                    }
                    if surface.snapshot_pins > 0 {
                        snapshot_pinned_entities += 1;
                    }
                    if surface.lifecycle == RecordLifecycleState::Reclaimable {
                        reclaimable_entities += 1;
                    }
                }
            }
            for slot in 0..self
                .runtime
                .storage_access()
                .record_slot_count::<crate::storage::logic::state::RelationRecordKind>(partition_id)
            {
                if let Some(surface) = self
                    .runtime
                    .storage_access()
                    .record_slot_surface::<crate::storage::logic::state::RelationRecordKind>(
                        partition_id, slot,
                    )
                {
                    if surface.branch_pins > 0 {
                        branch_pinned_relations += 1;
                    }
                    if surface.replay_pins > 0 {
                        replay_pinned_relations += 1;
                    }
                    if surface.snapshot_pins > 0 {
                        snapshot_pinned_relations += 1;
                    }
                    if surface.lifecycle == RecordLifecycleState::Reclaimable {
                        reclaimable_relations += 1;
                    }
                }
            }
        }
        RetentionPlan {
            retention_fence_version: retention_fence,
            active_snapshot_count: self.runtime.visibility.active_snapshot_count(),
            branch_pinned_entities,
            replay_pinned_entities,
            snapshot_pinned_entities,
            branch_pinned_relations,
            replay_pinned_relations,
            snapshot_pinned_relations,
            reclaimable_entities,
            reclaimable_relations,
        }
    }

    fn read_view_for_scope(&self, scope: &InspectionScope) -> Option<RelationalReadView> {
        match scope {
            InspectionScope::Current => Some(
                self.runtime
                    .visibility_reads()
                    .read_version(self.runtime.current_version_id()),
            ),
            InspectionScope::Version(version_id) => {
                Some(self.runtime.visibility_reads().read_version(*version_id))
            }
            InspectionScope::Snapshot(handle) => self.runtime.visibility_reads().read_snapshot(handle),
        }
    }

    fn scope_version_id(&self, scope: &InspectionScope) -> Option<crate::identity::data::VersionId> {
        match scope {
            InspectionScope::Current => Some(self.runtime.current_version_id()),
            InspectionScope::Version(version_id) => Some(*version_id),
            InspectionScope::Snapshot(handle) => Some(handle.version_id),
        }
    }

    fn scope_access_path(
        &self,
        scope: &InspectionScope,
        version_id: crate::identity::data::VersionId,
    ) -> InspectionAccessPath {
        match scope {
            InspectionScope::Current => InspectionAccessPath::DirectLookup,
            InspectionScope::Version(_)
                if cached_state_for_version(self.runtime, version_id).is_some() =>
            {
                InspectionAccessPath::HistoricalRetainedRead
            }
            InspectionScope::Version(_) => InspectionAccessPath::HistoricalReconstructedRead,
            InspectionScope::Snapshot(_) => InspectionAccessPath::SnapshotRead,
        }
    }

    fn scope_availability(
        &self,
        scope: &InspectionScope,
        version_id: crate::identity::data::VersionId,
    ) -> InspectionAvailability {
        match scope {
            InspectionScope::Current => InspectionAvailability::Direct,
            InspectionScope::Version(_)
                if cached_state_for_version(self.runtime, version_id).is_some() =>
            {
                InspectionAvailability::Direct
            }
            InspectionScope::Version(_) => InspectionAvailability::Reconstructed,
            InspectionScope::Snapshot(_) => InspectionAvailability::Direct,
        }
    }
}
