use crate::authority::commit::preparation::planning::strategy::{
    coarse_preparation_packet_count, strategy_for_parallel_packets, PreparationStrategySelection,
    TARGET_PREPARATION_ITEMS_PER_PACKET,
};
use crate::capabilities::{
    SchemaVersionSource, SnapshotSource, VersionSource, VisibilityPolicySource,
};
use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::history::data::AspectFilter;
use crate::indexes::logic::payload_field_key;
use crate::logic::runtime::{RelationalRuntime, VisibilityResidency};
use crate::publication::data::diff::AspectKey;
use crate::publication::patch::data::CanonicalAspectSet;
use crate::query::data::{
    reduce_query_fragments, PlannedQueryPacket, QueryComplexitySummary, QueryExecutionOutcome,
    QueryOrderingContract, QueryParallelLegality, QueryParallelProfitability, QueryPlanContextId,
    QueryPlanEvidenceBasis, QueryScope, QuerySerialReason, SnapshotPinnedQueryPlan,
};
use crate::schema::data::runtime_descriptor_semantics_policy;
use crate::snapshots::data::{SnapshotHandle, SnapshotInspectionSummary};
use crate::storage::data::{EntityReadRecord, RelationReadRecord, RelationalReadView};
use crate::storage::logic::state::{DenseSlotBitSet, PartitionAccess};
use crate::symbols::data::InternedString;
use crate::visibility::cache_state::{
    cached_state_for_version, reconstruct_state, residency_for_version,
};
use crate::visibility::snapshot_states::{build_visibility_state, read_view_from_snapshot_state};
use rayon::prelude::*;
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::materialization::{
    materialize_current_entity_record, materialize_current_relation_record,
    materialize_entity_record_at_version, materialize_relation_record_at_version,
};
use super::visibility::{slot_kind_matches, visible_slots_in_partition_from_state};

pub struct VisibilityReadContext<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

#[derive(Default)]
struct QueryFragmentScratch {
    entity_capacity_hint: usize,
    relation_capacity_hint: usize,
    entity_visit_key_capacity_hint: usize,
    relation_visit_key_capacity_hint: usize,
    visited_entities: BTreeSet<crate::identity::data::EntityId>,
    emitted_relations: BTreeSet<crate::identity::data::RelationId>,
    frontier: VecDeque<(
        crate::identity::data::EntityId,
        u32,
        crate::identity::data::EntityId,
        Option<crate::identity::data::RelationId>,
    )>,
}

impl QueryFragmentScratch {
    fn entity_buffer(&self) -> Vec<EntityReadRecord> {
        Vec::with_capacity(self.entity_capacity_hint)
    }

    fn relation_buffer(&self) -> Vec<RelationReadRecord> {
        Vec::with_capacity(self.relation_capacity_hint)
    }

    fn entity_visit_key_buffer(&self) -> Vec<crate::query::data::TraversalEntityVisitKey> {
        Vec::with_capacity(self.entity_visit_key_capacity_hint)
    }

    fn relation_visit_key_buffer(&self) -> Vec<crate::query::data::TraversalRelationVisitKey> {
        Vec::with_capacity(self.relation_visit_key_capacity_hint)
    }

    fn remember_entity_capacity(&mut self, len: usize) {
        self.entity_capacity_hint = self.entity_capacity_hint.max(len);
    }

    fn remember_relation_capacity(&mut self, len: usize) {
        self.relation_capacity_hint = self.relation_capacity_hint.max(len);
    }

    fn remember_entity_visit_key_capacity(&mut self, len: usize) {
        self.entity_visit_key_capacity_hint = self.entity_visit_key_capacity_hint.max(len);
    }

    fn remember_relation_visit_key_capacity(&mut self, len: usize) {
        self.relation_visit_key_capacity_hint = self.relation_visit_key_capacity_hint.max(len);
    }

    fn reset_traversal(&mut self) {
        self.visited_entities.clear();
        self.emitted_relations.clear();
        self.frontier.clear();
    }
}

impl<'runtime> VisibilityReadContext<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub(crate) const fn runtime(&self) -> &'runtime RelationalRuntime {
        self.runtime
    }

    pub fn inspect_snapshot(&self, handle: &SnapshotHandle) -> Option<SnapshotInspectionSummary> {
        if let Some((version_id, _read_policy)) =
            self.runtime.active_snapshot_binding(handle.snapshot_id)
        {
            let state = reconstruct_state(
                self.runtime,
                version_id,
                !self.runtime.protect_active_snapshots(),
            )?;
            return Some(SnapshotInspectionSummary {
                version_id,
                entity_count: state.pinned_entity_count,
                relation_count: state.pinned_relation_count,
                pinned_entity_count: state.pinned_entity_count,
                pinned_relation_count: state.pinned_relation_count,
            });
        }
        let version_id = self
            .runtime
            .published_snapshot_version(handle.snapshot_id)?;
        let state = self
            .runtime
            .visibility
            .published_snapshot_state(handle.snapshot_id)?;
        let read_view = read_view_from_snapshot_state(self.runtime, &state);
        Some(SnapshotInspectionSummary {
            version_id,
            entity_count: read_view.entities.len(),
            relation_count: read_view.relations.len(),
            pinned_entity_count: 0,
            pinned_relation_count: 0,
        })
    }

    pub fn read_snapshot(&self, handle: &SnapshotHandle) -> Option<RelationalReadView> {
        if let Some((version_id, read_policy)) =
            self.runtime.active_snapshot_binding(handle.snapshot_id)
        {
            let state = reconstruct_state(
                self.runtime,
                version_id,
                !self.runtime.protect_active_snapshots(),
            )?;
            let mut read_view = read_view_from_snapshot_state(self.runtime, &state);
            read_view.snapshot = SnapshotHandle {
                runtime_instance_id: self.runtime.runtime_instance_id(),
                snapshot_id: handle.snapshot_id,
                version_id,
                read_policy,
            };
            return Some(read_view);
        }
        self.runtime
            .published_snapshot_version(handle.snapshot_id)?;
        let state = self
            .runtime
            .visibility
            .published_snapshot_state(handle.snapshot_id)?;
        let mut read_view = read_view_from_snapshot_state(self.runtime, &state);
        read_view.snapshot = handle.clone();
        Some(read_view)
    }

    pub fn read_version(&self, version_id: crate::identity::data::VersionId) -> RelationalReadView {
        let state = reconstruct_state(self.runtime, version_id, true).unwrap_or_else(|| {
            build_visibility_state(
                self.runtime,
                version_id,
                crate::snapshots::data::SnapshotId(0),
                crate::snapshots::data::SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
            )
        });
        read_view_from_snapshot_state(self.runtime, &state)
    }

    pub fn query_plan_context(&self, handle: &SnapshotHandle) -> Option<QueryPlanContextId> {
        let snapshot = self.resolved_snapshot_handle(handle)?;
        let (schema_version, descriptor_semantics_version, evidence_basis) =
            self.query_schema_context(snapshot.version_id)?;
        Some(QueryPlanContextId {
            runtime_instance_id: self.runtime.runtime_instance_id(),
            snapshot_id: snapshot.snapshot_id,
            version_id: snapshot.version_id,
            schema_version,
            descriptor_semantics_version,
            evidence_basis,
        })
    }

    pub fn plan_query_packet(
        &self,
        handle: &SnapshotHandle,
        packet: PlannedQueryPacket,
    ) -> Option<SnapshotPinnedQueryPlan> {
        let snapshot = self.resolved_snapshot_handle(handle)?;
        if packet.context_id != self.query_plan_context(&snapshot)? {
            return None;
        }
        let legality = if packet.requires_serial_reduction() {
            QueryParallelLegality::RequiresSerialReduction
        } else {
            QueryParallelLegality::LegalReadOnlySnapshot
        };
        let profitability = self.query_profitability(&snapshot, &packet);
        Some(SnapshotPinnedQueryPlan {
            packet,
            snapshot,
            legality,
            profitability,
        })
    }

    pub fn execute_query_plan(
        &self,
        plan: SnapshotPinnedQueryPlan,
    ) -> Option<QueryExecutionOutcome> {
        if plan.packet.context_id != self.query_plan_context(&plan.snapshot)? {
            return None;
        }

        let read_view = self.read_snapshot(&plan.snapshot)?;
        let packets = packetized_query_work(&plan.packet, &read_view)?;
        let packet_count = packets.len();
        let target_count = packetized_query_item_count(&packets);
        let peak_width = packetized_query_peak_width(&packets);
        let scope_units = query_scope_units(&packets);
        let touched_partitions = partition_count_for_targets(&packets);
        let strategy = self.query_execution_strategy(&plan, packet_count);
        let scratch_reuse_count = packetized_fragment_scratch_reuse_count(&packets);
        self.runtime.performance_access().count_query_packet_shape(
            packet_count,
            target_count,
            peak_width,
            scope_units,
        );
        if scratch_reuse_count > 0 {
            self.runtime
                .performance_access()
                .count_query_fragment_scratch_reuse_by(scratch_reuse_count);
        }

        if matches!(plan.legality, QueryParallelLegality::LegalReadOnlySnapshot) {
            self.runtime
                .performance_access()
                .count_query_parallel_legal();
        }
        if matches!(plan.profitability, QueryParallelProfitability::Profitable) {
            self.runtime
                .performance_access()
                .count_query_parallel_profitable();
        }

        let fragments = match strategy {
            PreparationStrategySelection::Serial => {
                self.runtime
                    .performance_access()
                    .count_query_serial_strategy();
                let mut scratch = QueryFragmentScratch::default();
                packets
                    .iter()
                    .enumerate()
                    .map(|(ordinal, packet)| {
                        execute_query_fragment(
                            self.runtime,
                            &read_view,
                            &plan.packet,
                            packet,
                            ordinal as u64,
                            &mut scratch,
                        )
                    })
                    .collect::<Option<Vec<_>>>()?
            }
            PreparationStrategySelection::StagedParallel => {
                self.runtime
                    .performance_access()
                    .count_query_staged_parallel_strategy();
                let bucket_count = packets.len().min(rayon::current_num_threads()).max(1);
                let mut buckets = vec![Vec::new(); bucket_count];
                for (ordinal, packet) in packets.iter().enumerate() {
                    buckets[ordinal % bucket_count].push((ordinal as u64, packet));
                }
                let bucketed_fragments = buckets
                    .into_par_iter()
                    .map(|bucket| {
                        let mut scratch = QueryFragmentScratch::default();
                        bucket
                            .into_iter()
                            .map(|(ordinal, packet)| {
                                execute_query_fragment(
                                    self.runtime,
                                    &read_view,
                                    &plan.packet,
                                    packet,
                                    ordinal,
                                    &mut scratch,
                                )
                            })
                            .collect::<Option<Vec<_>>>()
                    })
                    .collect::<Option<Vec<Vec<_>>>>()?;
                bucketed_fragments.into_iter().flatten().collect()
            }
        };

        let entity_records_emitted = fragments
            .iter()
            .map(|fragment| fragment.counters.entity_records_emitted)
            .sum();
        let relation_records_emitted = fragments
            .iter()
            .map(|fragment| fragment.counters.relation_records_emitted)
            .sum();
        let complexity = QueryComplexitySummary {
            packet_count,
            fragment_count: packet_count,
            touched_partitions,
            target_count,
            entity_records_emitted,
            relation_records_emitted,
        };
        let result =
            reduce_query_fragments(plan.packet.execution_shape, plan.packet.ordering, fragments);
        self.runtime
            .performance_access()
            .count_query_emissions(result.entities.len(), result.relations.len());

        Some(QueryExecutionOutcome {
            plan,
            result,
            complexity,
        })
    }

    pub fn visible_entities_of_kind(
        &self,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<EntityReadRecord> {
        let state = self.runtime.storage_access().current_state();
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            records.extend(self.visible_entities_of_kind_in_partition_from_state(
                &state,
                partition_id,
                kind_id,
                version_id,
            ));
        }
        debug_assert!(entity_records_are_canonical(&records));
        records
    }

    pub fn visible_entities_of_kind_in_partition(
        &self,
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<EntityReadRecord> {
        let state = self.runtime.storage_access().current_state();
        let records = self.visible_entities_of_kind_in_partition_from_state(
            &state,
            partition_id,
            kind_id,
            version_id,
        );
        debug_assert!(entity_records_are_canonical(&records));
        records
    }

    pub fn visible_relations_of_kind(
        &self,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<RelationReadRecord> {
        let state = self.runtime.storage_access().current_state();
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            records.extend(self.visible_relations_of_kind_in_partition_from_state(
                &state,
                partition_id,
                kind_id,
                version_id,
            ));
        }
        sort_relation_records(&mut records);
        records
    }

    pub fn visible_relations_of_kind_in_partition(
        &self,
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<RelationReadRecord> {
        let state = self.runtime.storage_access().current_state();
        let mut records = self.visible_relations_of_kind_in_partition_from_state(
            &state,
            partition_id,
            kind_id,
            version_id,
        );
        sort_relation_records(&mut records);
        records
    }

    pub fn entity_aspect_versions(
        &self,
        entity_id: crate::identity::data::EntityId,
    ) -> Option<Vec<(AspectKey, u64)>> {
        let partition = self
            .runtime
            .storage_access()
            .partition_state(entity_id.partition_id)?;
        let slot = entity_id.local_slot.0 as usize;
        let slot_view = partition.entity_arena.get_slot(slot)?;
        if slot_view.generation() != entity_id.generation.0
            || slot_view.partition_id() != entity_id.partition_id
        {
            return None;
        }
        let versions = partition.entity_arena.aspect_versions_at(slot)?;
        let mut resolved: Vec<_> = versions
            .iter()
            .filter_map(|(symbol, version)| {
                self.runtime
                    .services
                    .symbols
                    .resolve(*symbol)
                    .map(|name| (AspectKey(InternedString::Raw(name.to_string())), *version))
            })
            .collect();
        resolved.sort();
        debug_assert!(aspect_versions_are_canonical(&resolved));
        Some(resolved)
    }

    pub fn relation_aspect_versions(
        &self,
        relation_id: crate::identity::data::RelationId,
    ) -> Option<Vec<(AspectKey, u64)>> {
        let partition = self
            .runtime
            .storage_access()
            .partition_state(relation_id.partition_id)?;
        let slot = relation_id.local_slot.0 as usize;
        let slot_view = partition.relation_arena.get_slot(slot)?;
        if slot_view.generation() != relation_id.generation.0
            || slot_view.partition_id() != relation_id.partition_id
        {
            return None;
        }
        let versions = partition.relation_arena.aspect_versions_at(slot)?;
        let mut resolved: Vec<_> = versions
            .iter()
            .filter_map(|(symbol, version)| {
                self.runtime
                    .services
                    .symbols
                    .resolve(*symbol)
                    .map(|name| (AspectKey(InternedString::Raw(name.to_string())), *version))
            })
            .collect();
        resolved.sort();
        debug_assert!(aspect_versions_are_canonical(&resolved));
        Some(resolved)
    }

    pub fn entity_aspects_at_version(
        &self,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<Vec<AspectKey>> {
        let state = self.runtime.storage_access().current_state();
        let record = self.entity_record_for_id_at_version(&state, entity_id, version_id)?;
        Some(declared_aspects_for_entity_kind(
            self.runtime,
            record.kind.kind_id,
        ))
    }

    pub fn relation_aspects_at_version(
        &self,
        relation_id: crate::identity::data::RelationId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<Vec<AspectKey>> {
        let state = self.runtime.storage_access().current_state();
        let record = self.relation_record_for_id_at_version(&state, relation_id, version_id)?;
        Some(declared_aspects_for_relation_kind(
            self.runtime,
            record.kind.kind_id,
        ))
    }

    pub fn inspect_version_read_path(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<RelationalDiagnosticArtifact> {
        if version_id.0 == 0 || version_id.0 > self.runtime.current_version_id().0 {
            return None;
        }

        let residency = residency_for_version(self.runtime, version_id);
        let cached = cached_state_for_version(self.runtime, version_id).is_some();
        let recent_window = self.runtime.recent_visibility_window();
        let protected = is_protected(&residency);
        let recent_candidate =
            self.runtime.visibility_cache_enabled() && recent_window > 0 && !protected;

        let mut entries = Vec::new();
        if !cached {
            entries.push(snapshot_miss_entry(false));
        }
        entries.push(snapshot_decision_entry(
            cached,
            protected,
            recent_candidate,
            false,
        ));
        Some(snapshot_read_path_artifact(
            version_id,
            cached,
            recent_candidate,
            recent_window,
            residency,
            entries,
        ))
    }

    pub fn inspect_snapshot_read_path(
        &self,
        handle: &SnapshotHandle,
    ) -> Option<RelationalDiagnosticArtifact> {
        if let Some((version_id, _read_policy)) =
            self.runtime.active_snapshot_binding(handle.snapshot_id)
        {
            let residency = residency_for_version(self.runtime, version_id);
            let cached = cached_state_for_version(self.runtime, version_id).is_some();
            let recent_window = self.runtime.recent_visibility_window();
            let recent_candidate = !self.runtime.protect_active_snapshots()
                && self.runtime.visibility_cache_enabled()
                && recent_window > 0
                && !is_protected(&residency);
            let mut entries = Vec::new();
            if !cached {
                entries.push(snapshot_miss_entry(false));
            }
            entries.push(snapshot_decision_entry(
                cached,
                is_protected(&residency),
                recent_candidate,
                false,
            ));
            return Some(snapshot_read_path_artifact(
                version_id,
                cached,
                recent_candidate,
                recent_window,
                residency,
                entries,
            ));
        }

        let version_id = self
            .runtime
            .published_snapshot_version(handle.snapshot_id)?;
        let residency = residency_for_version(self.runtime, version_id);
        let cached = self
            .runtime
            .visibility
            .published_snapshot_state(handle.snapshot_id)
            .is_some();
        let recent_window = self.runtime.recent_visibility_window();
        let recent_candidate = self.runtime.visibility_cache_enabled()
            && recent_window > 0
            && !is_protected(&residency);
        let mut entries = vec![RelationalDiagnosticsEntry {
            code: DiagnosticCode::PublishedSnapshotHandleRead,
            message: "snapshot read will resolve through a published handle".to_string(),
            fields: json!({
                "snapshot_id": handle.snapshot_id.0,
                "version_id": version_id.0,
            }),
        }];
        if !cached {
            entries.push(snapshot_miss_entry(true));
        }
        entries.push(snapshot_decision_entry(
            cached,
            is_protected(&residency),
            recent_candidate,
            true,
        ));
        Some(snapshot_read_path_artifact(
            version_id,
            cached,
            recent_candidate,
            recent_window,
            residency,
            entries,
        ))
    }

    pub(crate) fn entity_record_for_id_at_version(
        &self,
        state: &impl PartitionAccess,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<EntityReadRecord> {
        let partition = state.get_partition(entity_id.partition_id)?;
        let slot = entity_id.local_slot.0 as usize;
        if version_id == self.runtime.current_version_id() {
            materialize_current_entity_record(self.runtime, partition, entity_id.partition_id, slot)
                .filter(|record| {
                    entity_id.generation.0 == 0
                        || record.entity_id.generation == entity_id.generation
                })
        } else {
            materialize_entity_record_at_version(
                self.runtime,
                partition,
                entity_id.partition_id,
                slot,
                version_id,
            )
            .filter(|record| {
                entity_id.generation.0 == 0 || record.entity_id.generation == entity_id.generation
            })
        }
    }

    pub(crate) fn relation_record_for_id_at_version(
        &self,
        state: &impl PartitionAccess,
        relation_id: crate::identity::data::RelationId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<RelationReadRecord> {
        let partition = state.get_partition(relation_id.partition_id)?;
        let slot = relation_id.local_slot.0 as usize;
        if version_id == self.runtime.current_version_id() {
            materialize_current_relation_record(
                self.runtime,
                partition,
                relation_id.partition_id,
                slot,
            )
            .filter(|record| {
                relation_id.generation.0 == 0
                    || record.relation_id.generation == relation_id.generation
            })
        } else {
            materialize_relation_record_at_version(
                self.runtime,
                partition,
                relation_id.partition_id,
                slot,
                version_id,
            )
            .filter(|record| {
                relation_id.generation.0 == 0
                    || record.relation_id.generation == relation_id.generation
            })
        }
    }

    pub(crate) fn visible_entities_of_kind_in_partition_from_state(
        &self,
        state: &impl PartitionAccess,
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<EntityReadRecord> {
        let mut records = Vec::new();
        let current_version = VersionSource::current_version_id(self.runtime);
        let Some(partition) = state.get_partition(partition_id) else {
            return records;
        };
        if version_id == current_version {
            for slot in partition.entity_arena.live_bitset.iter_set_slots() {
                if !slot_kind_matches(&partition.entity_arena, slot, kind_id) {
                    continue;
                }
                if let Some(record) =
                    materialize_current_entity_record(self.runtime, partition, partition_id, slot)
                {
                    records.push(record);
                }
            }
        } else {
            self.runtime.services.instrumentation.count(|counters| {
                counters.visibility_entity_slot_scans += partition.entity_arena.slot_count();
            });
            for slot in 0..partition.entity_arena.slot_count() {
                if !slot_kind_matches(&partition.entity_arena, slot, kind_id) {
                    continue;
                }
                if let Some(record) = materialize_entity_record_at_version(
                    self.runtime,
                    partition,
                    partition_id,
                    slot,
                    version_id,
                ) {
                    records.push(record);
                }
            }
        }
        records
    }

    pub(crate) fn visible_relations_of_kind_in_partition_from_state(
        &self,
        state: &impl PartitionAccess,
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<RelationReadRecord> {
        let mut records = Vec::new();
        let current_version = VersionSource::current_version_id(self.runtime);
        let Some(partition) = state.get_partition(partition_id) else {
            return records;
        };
        if version_id == current_version {
            for slot in partition.relation_arena.live_bitset.iter_set_slots() {
                if !slot_kind_matches(&partition.relation_arena, slot, kind_id) {
                    continue;
                }
                if let Some(record) =
                    materialize_current_relation_record(self.runtime, partition, partition_id, slot)
                {
                    records.push(record);
                }
            }
        } else {
            self.runtime.services.instrumentation.count(|counters| {
                counters.visibility_relation_slot_scans += partition.relation_arena.slot_count();
            });
            for slot in 0..partition.relation_arena.slot_count() {
                if !slot_kind_matches(&partition.relation_arena, slot, kind_id) {
                    continue;
                }
                if let Some(record) = materialize_relation_record_at_version(
                    self.runtime,
                    partition,
                    partition_id,
                    slot,
                    version_id,
                ) {
                    records.push(record);
                }
            }
        }
        records
    }

    pub(crate) fn visible_entity_slots_from_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<(crate::identity::data::PartitionId, DenseSlotBitSet)> {
        let mut partitions = Vec::new();
        for partition_id in state.partition_ids() {
            if let Some(entity_slots) =
                self.visible_entity_slots_in_partition_from_state(state, partition_id, version_id)
            {
                partitions.push((partition_id, entity_slots));
            }
        }
        partitions
    }

    pub(crate) fn visible_entity_slots_in_partition_from_state(
        &self,
        state: &impl PartitionAccess,
        partition_id: crate::identity::data::PartitionId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<DenseSlotBitSet> {
        visible_slots_in_partition_from_state::<crate::storage::logic::state::EntityRecordKind>(
            self.runtime,
            state,
            partition_id,
            version_id,
            |runtime, scanned| {
                runtime.services.instrumentation.count(|counters| {
                    counters.visibility_entity_slot_scans += scanned;
                });
            },
        )
    }

    pub(crate) fn visible_relation_slots_from_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<(crate::identity::data::PartitionId, DenseSlotBitSet)> {
        let mut partitions = Vec::new();
        for partition_id in state.partition_ids() {
            if let Some(relation_slots) =
                self.visible_relation_slots_in_partition_from_state(state, partition_id, version_id)
            {
                partitions.push((partition_id, relation_slots));
            }
        }
        partitions
    }

    pub(crate) fn visible_relation_slots_in_partition_from_state(
        &self,
        state: &impl PartitionAccess,
        partition_id: crate::identity::data::PartitionId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<DenseSlotBitSet> {
        visible_slots_in_partition_from_state::<crate::storage::logic::state::RelationRecordKind>(
            self.runtime,
            state,
            partition_id,
            version_id,
            |runtime, scanned| {
                runtime.services.instrumentation.count(|counters| {
                    counters.visibility_relation_slot_scans += scanned;
                });
            },
        )
    }

    pub(crate) fn relation_visible_at_version(
        &self,
        relation_id: crate::identity::data::RelationId,
        version_id: crate::identity::data::VersionId,
    ) -> bool {
        let current_state = self.runtime.storage_access().current_state();
        self.relation_record_for_id_at_version(&current_state, relation_id, version_id)
            .is_some()
    }

    fn resolved_snapshot_handle(&self, handle: &SnapshotHandle) -> Option<SnapshotHandle> {
        if let Some((version_id, read_policy)) =
            self.runtime.active_snapshot_binding(handle.snapshot_id)
        {
            return Some(SnapshotHandle {
                runtime_instance_id: self.runtime.runtime_instance_id(),
                snapshot_id: handle.snapshot_id,
                version_id,
                read_policy,
            });
        }
        self.runtime
            .published_snapshot_version(handle.snapshot_id)
            .map(|version_id| SnapshotHandle {
                runtime_instance_id: self.runtime.runtime_instance_id(),
                snapshot_id: handle.snapshot_id,
                version_id,
                read_policy: handle.read_policy,
            })
    }

    fn query_schema_context(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<(
        crate::schema::data::SchemaVersionId,
        crate::schema::data::DescriptorSemanticsVersion,
        QueryPlanEvidenceBasis,
    )> {
        if let Some(envelope) = self
            .runtime
            .history_access()
            .commit_envelope_for_version(version_id)
        {
            return Some((
                envelope.schema_version,
                envelope.descriptor_semantics_version,
                QueryPlanEvidenceBasis::CanonicalCommitEnvelope {
                    commit_id: envelope.commit.commit_id,
                },
            ));
        }

        if version_id == self.runtime.current_version_id()
            && self.runtime.history_access().latest_commit().is_none()
        {
            return Some((
                self.runtime.primary_schema_version_id(),
                runtime_descriptor_semantics_policy().current_write_version(),
                QueryPlanEvidenceBasis::GenesisRuntimeBootstrap,
            ));
        }

        None
    }

    fn query_profitability(
        &self,
        snapshot: &SnapshotHandle,
        packet: &PlannedQueryPacket,
    ) -> QueryParallelProfitability {
        if packet.target_count_hint <= 1 {
            return QueryParallelProfitability::SerialPreferred {
                reason: QuerySerialReason::TinyPacket,
            };
        }

        if let QueryScope::ExplicitTargets { targets } = &packet.scope {
            let touched_partitions = targets
                .iter()
                .map(|target| match target {
                    crate::transactions::data::RecordRef::Entity(entity_id) => {
                        entity_id.partition_id
                    }
                    crate::transactions::data::RecordRef::Relation(relation_id) => {
                        relation_id.partition_id
                    }
                })
                .collect::<std::collections::BTreeSet<_>>();
            if touched_partitions.len() > 1 {
                return QueryParallelProfitability::Profitable;
            }

            let read_packet = PlannedQueryPacket::explicit_targets(
                packet.label.clone(),
                packet.context_id.clone(),
                targets.to_vec(),
            );
            if let Some(read_plan) = self
                .runtime
                .storage_access()
                .plan_read_explicit_query_packet(snapshot, &read_packet)
            {
                let touched_chunk_count =
                    read_plan.entity_chunk_indexes.len() + read_plan.relation_chunk_indexes.len();
                if touched_chunk_count <= 1 {
                    return QueryParallelProfitability::SerialPreferred {
                        reason: QuerySerialReason::SingleChunkSurface,
                    };
                }
            }
        }

        if matches!(
            packet.locality,
            crate::query::data::QueryLocalityClass::CrossPartitionTraversal
        ) && packet.target_count_hint > 0
            && packet.target_count_hint <= 4
        {
            return QueryParallelProfitability::SerialPreferred {
                reason: QuerySerialReason::BroadCrossPartitionCoordination,
            };
        }

        QueryParallelProfitability::Profitable
    }

    fn query_execution_strategy(
        &self,
        plan: &SnapshotPinnedQueryPlan,
        packet_count: usize,
    ) -> PreparationStrategySelection {
        if !matches!(plan.legality, QueryParallelLegality::LegalReadOnlySnapshot) {
            return PreparationStrategySelection::Serial;
        }
        if !matches!(plan.profitability, QueryParallelProfitability::Profitable) {
            return PreparationStrategySelection::Serial;
        }

        strategy_for_parallel_packets(self.runtime.config.execution.execution_model, packet_count)
            .selected_mode
    }
}

#[derive(Debug, Clone)]
enum PacketizedQueryWork {
    ExplicitTargets(Vec<crate::transactions::data::RecordRef>),
    EntityKindScan {
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
    },
    RelationKindScan {
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
    },
    EntityPayloadFieldEquals {
        partition_id: crate::identity::data::PartitionId,
        field: String,
        value: String,
    },
    EntityPayloadFieldAnyOf {
        partition_id: crate::identity::data::PartitionId,
        field: String,
        values: Vec<String>,
    },
    RelationPayloadFieldEquals {
        partition_id: crate::identity::data::PartitionId,
        field: String,
        value: String,
    },
    RelationPayloadFieldAnyOf {
        partition_id: crate::identity::data::PartitionId,
        field: String,
        values: Vec<String>,
    },
    AspectFilteredEntities {
        partition_id: crate::identity::data::PartitionId,
        kind_id: Option<crate::identity::data::KindId>,
        aspect_filter: AspectFilter,
    },
    AspectFilteredRelations {
        partition_id: crate::identity::data::PartitionId,
        kind_id: Option<crate::identity::data::KindId>,
        aspect_filter: AspectFilter,
    },
    OutgoingNeighborhood {
        seeds: Vec<crate::identity::data::EntityId>,
        relation_kind_scope: Option<Vec<crate::identity::data::KindId>>,
    },
    IncomingNeighborhood {
        seeds: Vec<crate::identity::data::EntityId>,
        relation_kind_scope: Option<Vec<crate::identity::data::KindId>>,
    },
    ConnectivityTraversal {
        seeds: Vec<crate::identity::data::EntityId>,
        relation_kind_scope: Option<Vec<crate::identity::data::KindId>>,
        max_depth: Option<u32>,
    },
}

fn packetized_query_work(
    packet: &PlannedQueryPacket,
    read_view: &RelationalReadView,
) -> Option<Vec<PacketizedQueryWork>> {
    match &packet.scope {
        QueryScope::ExplicitTargets { targets } => {
            let mut by_partition: BTreeMap<
                crate::identity::data::PartitionId,
                Vec<crate::transactions::data::RecordRef>,
            > = BTreeMap::new();

            for target in targets.iter() {
                let partition_id = match target {
                    crate::transactions::data::RecordRef::Entity(entity_id) => {
                        entity_id.partition_id
                    }
                    crate::transactions::data::RecordRef::Relation(relation_id) => {
                        relation_id.partition_id
                    }
                };
                by_partition
                    .entry(partition_id)
                    .or_default()
                    .push(target.clone());
            }

            let mut packets = Vec::new();
            for (_partition_id, partition_targets) in by_partition {
                let packet_count = coarse_preparation_packet_count(
                    partition_targets.len(),
                    TARGET_PREPARATION_ITEMS_PER_PACKET,
                );
                if packet_count <= 1 {
                    packets.push(PacketizedQueryWork::ExplicitTargets(partition_targets));
                    continue;
                }

                for chunk in partition_targets.chunks(TARGET_PREPARATION_ITEMS_PER_PACKET) {
                    packets.push(PacketizedQueryWork::ExplicitTargets(chunk.to_vec()));
                }
            }

            Some(packets)
        }
        QueryScope::EntityKindScan {
            kind_id,
            partition_scope,
        } => Some(
            entity_scan_partitions(partition_scope, read_view)
                .into_iter()
                .map(|partition_id| PacketizedQueryWork::EntityKindScan {
                    partition_id,
                    kind_id: *kind_id,
                })
                .collect(),
        ),
        QueryScope::RelationKindScan {
            kind_id,
            partition_scope,
        } => Some(
            relation_scan_partitions(partition_scope, read_view)
                .into_iter()
                .map(|partition_id| PacketizedQueryWork::RelationKindScan {
                    partition_id,
                    kind_id: *kind_id,
                })
                .collect(),
        ),
        QueryScope::EntityPayloadFieldEquals {
            field,
            value,
            partition_scope,
        } => Some(
            entity_scan_partitions(partition_scope, read_view)
                .into_iter()
                .map(
                    |partition_id| PacketizedQueryWork::EntityPayloadFieldEquals {
                        partition_id,
                        field: field.clone(),
                        value: value.clone(),
                    },
                )
                .collect(),
        ),
        QueryScope::EntityPayloadFieldAnyOf {
            field,
            values,
            partition_scope,
        } => {
            let values = QueryScope::canonical_value_scope(values.as_ref())
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            Some(
                entity_scan_partitions(partition_scope, read_view)
                    .into_iter()
                    .map(
                        |partition_id| PacketizedQueryWork::EntityPayloadFieldAnyOf {
                            partition_id,
                            field: field.clone(),
                            values: values.clone(),
                        },
                    )
                    .collect(),
            )
        }
        QueryScope::RelationPayloadFieldEquals {
            field,
            value,
            partition_scope,
        } => Some(
            relation_scan_partitions(partition_scope, read_view)
                .into_iter()
                .map(
                    |partition_id| PacketizedQueryWork::RelationPayloadFieldEquals {
                        partition_id,
                        field: field.clone(),
                        value: value.clone(),
                    },
                )
                .collect(),
        ),
        QueryScope::RelationPayloadFieldAnyOf {
            field,
            values,
            partition_scope,
        } => {
            let values = QueryScope::canonical_value_scope(values.as_ref())
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            Some(
                relation_scan_partitions(partition_scope, read_view)
                    .into_iter()
                    .map(
                        |partition_id| PacketizedQueryWork::RelationPayloadFieldAnyOf {
                            partition_id,
                            field: field.clone(),
                            values: values.clone(),
                        },
                    )
                    .collect(),
            )
        }
        QueryScope::AspectFilteredEntities {
            kind_id,
            aspect_filter,
            partition_scope,
        } => Some(
            entity_scan_partitions(partition_scope, read_view)
                .into_iter()
                .map(|partition_id| PacketizedQueryWork::AspectFilteredEntities {
                    partition_id,
                    kind_id: *kind_id,
                    aspect_filter: aspect_filter.clone(),
                })
                .collect(),
        ),
        QueryScope::AspectFilteredRelations {
            kind_id,
            aspect_filter,
            partition_scope,
        } => Some(
            relation_scan_partitions(partition_scope, read_view)
                .into_iter()
                .map(
                    |partition_id| PacketizedQueryWork::AspectFilteredRelations {
                        partition_id,
                        kind_id: *kind_id,
                        aspect_filter: aspect_filter.clone(),
                    },
                )
                .collect(),
        ),
        QueryScope::OutgoingNeighborhood {
            seeds,
            relation_kind_scope,
        } => Some(
            canonical_seed_ids(seeds)
                .into_iter()
                .map(|seed| PacketizedQueryWork::OutgoingNeighborhood {
                    seeds: vec![seed],
                    relation_kind_scope: relation_kind_scope.as_ref().map(canonical_kind_scope),
                })
                .collect(),
        ),
        QueryScope::IncomingNeighborhood {
            seeds,
            relation_kind_scope,
        } => Some(
            canonical_seed_ids(seeds)
                .into_iter()
                .map(|seed| PacketizedQueryWork::IncomingNeighborhood {
                    seeds: vec![seed],
                    relation_kind_scope: relation_kind_scope.as_ref().map(canonical_kind_scope),
                })
                .collect(),
        ),
        QueryScope::ConnectivityTraversal {
            seeds,
            relation_kind_scope,
            max_depth,
        } => Some(
            canonical_seed_ids(seeds)
                .into_iter()
                .map(|seed| PacketizedQueryWork::ConnectivityTraversal {
                    seeds: vec![seed],
                    relation_kind_scope: relation_kind_scope.as_ref().map(canonical_kind_scope),
                    max_depth: *max_depth,
                })
                .collect(),
        ),
    }
}

fn execute_query_fragment(
    runtime: &RelationalRuntime,
    read_view: &RelationalReadView,
    packet: &PlannedQueryPacket,
    work: &PacketizedQueryWork,
    ordinal: u64,
    scratch: &mut QueryFragmentScratch,
) -> Option<crate::query::data::QueryWorkerFragment> {
    match work {
        PacketizedQueryWork::ExplicitTargets(targets) => read_view.execute_planned_packet_fragment(
            packet.plan_key,
            packet.ordering,
            targets,
            ordinal,
        ),
        PacketizedQueryWork::EntityKindScan {
            partition_id,
            kind_id,
        } => Some(entity_fragment(
            read_view,
            packet,
            ordinal,
            0,
            1,
            scratch,
            |record| {
                record.entity_id.partition_id == *partition_id && record.kind.kind_id == *kind_id
            },
        )),
        PacketizedQueryWork::RelationKindScan {
            partition_id,
            kind_id,
        } => Some(relation_fragment(
            read_view,
            packet,
            ordinal,
            0,
            1,
            scratch,
            |record| {
                record.relation_id.partition_id == *partition_id && record.kind.kind_id == *kind_id
            },
        )),
        PacketizedQueryWork::EntityPayloadFieldEquals {
            partition_id,
            field,
            value,
        } => Some(entity_fragment(
            read_view,
            packet,
            ordinal,
            0,
            1,
            scratch,
            |record| {
                record.entity_id.partition_id == *partition_id
                    && payload_field_key(&record.payload, field).as_deref() == Some(value.as_str())
            },
        )),
        PacketizedQueryWork::EntityPayloadFieldAnyOf {
            partition_id,
            field,
            values,
        } => {
            let values = values.iter().map(String::as_str).collect::<BTreeSet<_>>();
            Some(entity_fragment(
                read_view,
                packet,
                ordinal,
                values.len(),
                1,
                scratch,
                |record| {
                    record.entity_id.partition_id == *partition_id
                        && payload_field_key(&record.payload, field)
                            .as_deref()
                            .is_some_and(|value| values.contains(value))
                },
            ))
        }
        PacketizedQueryWork::RelationPayloadFieldEquals {
            partition_id,
            field,
            value,
        } => Some(relation_fragment(
            read_view,
            packet,
            ordinal,
            0,
            1,
            scratch,
            |record| {
                record.relation_id.partition_id == *partition_id
                    && optional_payload_field_key(&record.payload, field).as_deref()
                        == Some(value.as_str())
            },
        )),
        PacketizedQueryWork::RelationPayloadFieldAnyOf {
            partition_id,
            field,
            values,
        } => {
            let values = values.iter().map(String::as_str).collect::<BTreeSet<_>>();
            Some(relation_fragment(
                read_view,
                packet,
                ordinal,
                values.len(),
                1,
                scratch,
                |record| {
                    record.relation_id.partition_id == *partition_id
                        && optional_payload_field_key(&record.payload, field)
                            .as_deref()
                            .is_some_and(|value| values.contains(value))
                },
            ))
        }
        PacketizedQueryWork::AspectFilteredEntities {
            partition_id,
            kind_id,
            aspect_filter,
        } => Some(entity_fragment(
            read_view,
            packet,
            ordinal,
            0,
            1,
            scratch,
            |record| {
                record.entity_id.partition_id == *partition_id
                    && kind_id.is_none_or(|kind_id| record.kind.kind_id == kind_id)
                    && aspect_filter_matches_entity(runtime, record, aspect_filter)
            },
        )),
        PacketizedQueryWork::AspectFilteredRelations {
            partition_id,
            kind_id,
            aspect_filter,
        } => Some(relation_fragment(
            read_view,
            packet,
            ordinal,
            0,
            1,
            scratch,
            |record| {
                record.relation_id.partition_id == *partition_id
                    && kind_id.is_none_or(|kind_id| record.kind.kind_id == kind_id)
                    && aspect_filter_matches_relation(runtime, record, aspect_filter)
            },
        )),
        PacketizedQueryWork::OutgoingNeighborhood {
            seeds,
            relation_kind_scope,
        } => traversal_fragment(
            runtime,
            read_view,
            packet,
            seeds,
            relation_kind_scope.as_deref(),
            ordinal,
            TraversalMode::OutgoingNeighborhood,
            scratch,
        ),
        PacketizedQueryWork::IncomingNeighborhood {
            seeds,
            relation_kind_scope,
        } => traversal_fragment(
            runtime,
            read_view,
            packet,
            seeds,
            relation_kind_scope.as_deref(),
            ordinal,
            TraversalMode::IncomingNeighborhood,
            scratch,
        ),
        PacketizedQueryWork::ConnectivityTraversal {
            seeds,
            relation_kind_scope,
            max_depth,
        } => traversal_fragment(
            runtime,
            read_view,
            packet,
            seeds,
            relation_kind_scope.as_deref(),
            ordinal,
            TraversalMode::ConnectivityTraversal {
                max_depth: *max_depth,
            },
            scratch,
        ),
    }
}

fn entity_fragment(
    read_view: &RelationalReadView,
    packet: &PlannedQueryPacket,
    ordinal: u64,
    target_count: usize,
    touched_partitions: usize,
    scratch: &mut QueryFragmentScratch,
    include: impl Fn(&EntityReadRecord) -> bool,
) -> crate::query::data::QueryWorkerFragment {
    let mut entities = scratch.entity_buffer();
    for record in read_view.entities() {
        if include(record) {
            entities.push(record.clone());
        }
    }
    let entity_records_emitted = entities.len();
    scratch.remember_entity_capacity(entity_records_emitted);
    crate::query::data::QueryWorkerFragment {
        plan_key: packet.plan_key,
        fragment_key: crate::query::data::deterministic_query_fragment_key(
            packet.plan_key,
            ordinal,
        ),
        ordering: packet.ordering,
        counters: crate::query::data::QueryFragmentCounters {
            target_count,
            entity_records_emitted,
            relation_records_emitted: 0,
            touched_partitions: usize::from(entity_records_emitted > 0) * touched_partitions,
        },
        entities,
        relations: Vec::new(),
        traversal_basis: None,
    }
}

fn relation_fragment(
    read_view: &RelationalReadView,
    packet: &PlannedQueryPacket,
    ordinal: u64,
    target_count: usize,
    touched_partitions: usize,
    scratch: &mut QueryFragmentScratch,
    include: impl Fn(&RelationReadRecord) -> bool,
) -> crate::query::data::QueryWorkerFragment {
    let mut relations = scratch.relation_buffer();
    for record in read_view.relations() {
        if include(record) {
            relations.push(record.clone());
        }
    }
    let relation_records_emitted = relations.len();
    scratch.remember_relation_capacity(relation_records_emitted);
    crate::query::data::QueryWorkerFragment {
        plan_key: packet.plan_key,
        fragment_key: crate::query::data::deterministic_query_fragment_key(
            packet.plan_key,
            ordinal,
        ),
        ordering: packet.ordering,
        counters: crate::query::data::QueryFragmentCounters {
            target_count,
            entity_records_emitted: 0,
            relation_records_emitted,
            touched_partitions: usize::from(relation_records_emitted > 0) * touched_partitions,
        },
        entities: Vec::new(),
        relations,
        traversal_basis: None,
    }
}

fn partition_count_for_targets(packets: &[PacketizedQueryWork]) -> usize {
    let mut partitions = std::collections::BTreeSet::new();
    for packet in packets {
        match packet {
            PacketizedQueryWork::ExplicitTargets(targets) => {
                for target in targets {
                    match target {
                        crate::transactions::data::RecordRef::Entity(entity_id) => {
                            partitions.insert(entity_id.partition_id);
                        }
                        crate::transactions::data::RecordRef::Relation(relation_id) => {
                            partitions.insert(relation_id.partition_id);
                        }
                    }
                }
            }
            PacketizedQueryWork::EntityKindScan { partition_id, .. }
            | PacketizedQueryWork::RelationKindScan { partition_id, .. }
            | PacketizedQueryWork::EntityPayloadFieldEquals { partition_id, .. }
            | PacketizedQueryWork::EntityPayloadFieldAnyOf { partition_id, .. }
            | PacketizedQueryWork::RelationPayloadFieldEquals { partition_id, .. }
            | PacketizedQueryWork::RelationPayloadFieldAnyOf { partition_id, .. }
            | PacketizedQueryWork::AspectFilteredEntities { partition_id, .. }
            | PacketizedQueryWork::AspectFilteredRelations { partition_id, .. } => {
                partitions.insert(*partition_id);
            }
            PacketizedQueryWork::OutgoingNeighborhood { seeds, .. }
            | PacketizedQueryWork::IncomingNeighborhood { seeds, .. }
            | PacketizedQueryWork::ConnectivityTraversal { seeds, .. } => {
                partitions.extend(seeds.iter().map(|entity_id| entity_id.partition_id));
            }
        }
    }
    partitions.len()
}

fn packetized_query_item_count(packets: &[PacketizedQueryWork]) -> usize {
    packets
        .iter()
        .map(|packet| match packet {
            PacketizedQueryWork::ExplicitTargets(targets) => targets.len(),
            PacketizedQueryWork::EntityKindScan { .. }
            | PacketizedQueryWork::RelationKindScan { .. }
            | PacketizedQueryWork::EntityPayloadFieldEquals { .. }
            | PacketizedQueryWork::RelationPayloadFieldEquals { .. }
            | PacketizedQueryWork::AspectFilteredEntities { .. }
            | PacketizedQueryWork::AspectFilteredRelations { .. } => 1,
            PacketizedQueryWork::EntityPayloadFieldAnyOf { values, .. }
            | PacketizedQueryWork::RelationPayloadFieldAnyOf { values, .. } => values.len(),
            PacketizedQueryWork::OutgoingNeighborhood { seeds, .. }
            | PacketizedQueryWork::IncomingNeighborhood { seeds, .. }
            | PacketizedQueryWork::ConnectivityTraversal { seeds, .. } => seeds.len(),
        })
        .sum()
}

fn packetized_fragment_scratch_reuse_count(packets: &[PacketizedQueryWork]) -> usize {
    packets
        .iter()
        .filter(|packet| packet_uses_fragment_scratch(packet))
        .count()
        .saturating_sub(1)
}

fn packet_uses_fragment_scratch(packet: &PacketizedQueryWork) -> bool {
    !matches!(packet, PacketizedQueryWork::ExplicitTargets(_))
}

fn packetized_query_peak_width(packets: &[PacketizedQueryWork]) -> usize {
    packets
        .iter()
        .map(|packet| match packet {
            PacketizedQueryWork::ExplicitTargets(targets) => targets.len(),
            PacketizedQueryWork::EntityKindScan { .. }
            | PacketizedQueryWork::RelationKindScan { .. }
            | PacketizedQueryWork::EntityPayloadFieldEquals { .. }
            | PacketizedQueryWork::RelationPayloadFieldEquals { .. }
            | PacketizedQueryWork::AspectFilteredEntities { .. }
            | PacketizedQueryWork::AspectFilteredRelations { .. } => 1,
            PacketizedQueryWork::EntityPayloadFieldAnyOf { values, .. }
            | PacketizedQueryWork::RelationPayloadFieldAnyOf { values, .. } => values.len(),
            PacketizedQueryWork::OutgoingNeighborhood { seeds, .. }
            | PacketizedQueryWork::IncomingNeighborhood { seeds, .. }
            | PacketizedQueryWork::ConnectivityTraversal { seeds, .. } => seeds.len(),
        })
        .max()
        .unwrap_or(0)
}

fn query_scope_units(packets: &[PacketizedQueryWork]) -> usize {
    partition_count_for_targets(packets).max(usize::from(!packets.is_empty()))
}

fn entity_scan_partitions(
    partition_scope: &Option<std::sync::Arc<[crate::identity::data::PartitionId]>>,
    read_view: &RelationalReadView,
) -> Vec<crate::identity::data::PartitionId> {
    if let Some(partitions) = partition_scope {
        return partitions.iter().copied().collect();
    }

    read_view
        .entities()
        .iter()
        .map(|record| record.entity_id.partition_id)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn relation_scan_partitions(
    partition_scope: &Option<std::sync::Arc<[crate::identity::data::PartitionId]>>,
    read_view: &RelationalReadView,
) -> Vec<crate::identity::data::PartitionId> {
    if let Some(partitions) = partition_scope {
        return partitions.iter().copied().collect();
    }

    read_view
        .relations()
        .iter()
        .map(|record| record.relation_id.partition_id)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum TraversalMode {
    OutgoingNeighborhood,
    IncomingNeighborhood,
    ConnectivityTraversal { max_depth: Option<u32> },
}

fn canonical_seed_ids(
    seeds: &std::sync::Arc<[crate::identity::data::EntityId]>,
) -> Vec<crate::identity::data::EntityId> {
    let mut seeds = seeds.iter().copied().collect::<Vec<_>>();
    seeds.sort();
    seeds.dedup();
    seeds
}

fn canonical_kind_scope(
    relation_kind_scope: &std::sync::Arc<[crate::identity::data::KindId]>,
) -> Vec<crate::identity::data::KindId> {
    let mut kinds = relation_kind_scope.iter().copied().collect::<Vec<_>>();
    kinds.sort();
    kinds.dedup();
    kinds
}

fn aspect_filter_matches_entity(
    runtime: &RelationalRuntime,
    record: &EntityReadRecord,
    aspect_filter: &AspectFilter,
) -> bool {
    let aspects = CanonicalAspectSet::new(declared_aspects_for_entity_kind(
        runtime,
        record.kind.kind_id,
    ));
    aspect_filter.matches(&aspects)
}

fn aspect_filter_matches_relation(
    runtime: &RelationalRuntime,
    record: &RelationReadRecord,
    aspect_filter: &AspectFilter,
) -> bool {
    let aspects = CanonicalAspectSet::new(declared_aspects_for_relation_kind(
        runtime,
        record.kind.kind_id,
    ));
    aspect_filter.matches(&aspects)
}

fn optional_payload_field_key(
    payload: &Option<crate::payloads::data::RecordPayload>,
    field: &str,
) -> Option<String> {
    payload
        .as_ref()
        .and_then(|payload| payload_field_key(payload, field))
}

fn traversal_fragment(
    runtime: &RelationalRuntime,
    read_view: &RelationalReadView,
    packet: &PlannedQueryPacket,
    seeds: &[crate::identity::data::EntityId],
    relation_kind_scope: Option<&[crate::identity::data::KindId]>,
    ordinal: u64,
    mode: TraversalMode,
    scratch: &mut QueryFragmentScratch,
) -> Option<crate::query::data::QueryWorkerFragment> {
    if packet.ordering != QueryOrderingContract::CanonicalTraversalOrder {
        return None;
    }

    let storage = runtime.storage_access();
    let state = storage.current_state();
    let relation_kind_scope =
        relation_kind_scope.map(|scope| scope.iter().copied().collect::<BTreeSet<_>>());
    scratch.reset_traversal();
    let mut entities = scratch.entity_buffer();
    let mut relations = scratch.relation_buffer();
    let mut entity_visit_keys = scratch.entity_visit_key_buffer();
    let mut relation_visit_keys = scratch.relation_visit_key_buffer();

    for seed in seeds.iter().copied() {
        if scratch.visited_entities.insert(seed) {
            scratch.frontier.push_back((seed, 0u32, seed, None));
        }
    }

    while let Some((entity_id, depth, root_seed, via_relation)) = scratch.frontier.pop_front() {
        let Some(entity_record) = runtime.visibility_reads().entity_record_for_id_at_version(
            &state,
            entity_id,
            read_view.snapshot.version_id,
        ) else {
            continue;
        };
        entity_visit_keys.push(crate::query::data::TraversalEntityVisitKey {
            depth,
            root_seed,
            via_relation,
            entity_id,
        });
        entities.push(entity_record);

        let relation_ids = relation_ids_for_traversal(
            runtime,
            read_view,
            entity_id,
            &mode,
            relation_kind_scope.as_ref(),
        );
        let allow_expansion = match mode {
            TraversalMode::OutgoingNeighborhood | TraversalMode::IncomingNeighborhood => depth == 0,
            TraversalMode::ConnectivityTraversal { max_depth } => {
                max_depth.is_none_or(|max_depth| depth < max_depth)
            }
        };
        if !allow_expansion {
            continue;
        }

        for relation_id in relation_ids {
            let Some(relation_record) = runtime
                .visibility_reads()
                .relation_record_for_id_at_version(
                    &state,
                    relation_id,
                    read_view.snapshot.version_id,
                )
            else {
                continue;
            };
            if scratch
                .emitted_relations
                .insert(relation_record.relation_id)
            {
                relation_visit_keys.push(crate::query::data::TraversalRelationVisitKey {
                    depth,
                    root_seed,
                    relation_id: relation_record.relation_id,
                });
                relations.push(relation_record.clone());
            }

            let neighbor = match mode {
                TraversalMode::OutgoingNeighborhood
                | TraversalMode::ConnectivityTraversal { .. } => relation_record.target,
                TraversalMode::IncomingNeighborhood => relation_record.source,
            };
            if scratch.visited_entities.insert(neighbor) {
                scratch.frontier.push_back((
                    neighbor,
                    depth + 1,
                    root_seed,
                    Some(relation_record.relation_id),
                ));
            }
        }
    }
    scratch.remember_entity_capacity(entities.len());
    scratch.remember_relation_capacity(relations.len());
    scratch.remember_entity_visit_key_capacity(entity_visit_keys.len());
    scratch.remember_relation_visit_key_capacity(relation_visit_keys.len());
    let touched_partitions = entities
        .iter()
        .map(|record| record.entity_id.partition_id)
        .collect::<BTreeSet<_>>()
        .len();

    Some(crate::query::data::QueryWorkerFragment {
        plan_key: packet.plan_key,
        fragment_key: crate::query::data::deterministic_query_fragment_key(
            packet.plan_key,
            ordinal,
        ),
        ordering: packet.ordering,
        counters: crate::query::data::QueryFragmentCounters {
            target_count: seeds.len(),
            entity_records_emitted: entities.len(),
            relation_records_emitted: relations.len(),
            touched_partitions,
        },
        entities,
        relations,
        traversal_basis: Some(crate::query::data::TraversalReductionBasis {
            entity_visit_keys,
            relation_visit_keys,
        }),
    })
}

fn relation_ids_for_traversal(
    runtime: &RelationalRuntime,
    read_view: &RelationalReadView,
    entity_id: crate::identity::data::EntityId,
    mode: &TraversalMode,
    relation_kind_scope: Option<&BTreeSet<crate::identity::data::KindId>>,
) -> Vec<crate::identity::data::RelationId> {
    let storage = runtime.storage_access();
    let state = storage.current_state();
    let mut relation_ids = match mode {
        TraversalMode::OutgoingNeighborhood | TraversalMode::ConnectivityTraversal { .. } => {
            storage.outgoing_relations_for_entity(entity_id, read_view.snapshot.version_id)
        }
        TraversalMode::IncomingNeighborhood => {
            storage.incoming_relations_for_entity(entity_id, read_view.snapshot.version_id)
        }
    };
    relation_ids.sort();
    relation_ids.retain(|relation_id| {
        let Some(relation_record) = runtime
            .visibility_reads()
            .relation_record_for_id_at_version(&state, *relation_id, read_view.snapshot.version_id)
        else {
            return false;
        };
        relation_kind_scope.is_none_or(|scope| scope.contains(&relation_record.kind.kind_id))
    });
    relation_ids
}

fn sort_relation_records(records: &mut [RelationReadRecord]) {
    records.sort_by_key(|record| {
        (
            record.source.partition_id.0,
            record.source.local_slot.0,
            record.target.partition_id.0,
            record.target.local_slot.0,
            record.relation_id.partition_id.0,
            record.relation_id.local_slot.0,
        )
    });
}

fn declared_aspects_for_entity_kind(
    runtime: &RelationalRuntime,
    kind_id: crate::identity::data::KindId,
) -> Vec<AspectKey> {
    runtime
        .aspect_semantics
        .plans
        .entity_plans
        .get(&kind_id)
        .map(plan_aspect_keys)
        .unwrap_or_default()
}

fn declared_aspects_for_relation_kind(
    runtime: &RelationalRuntime,
    kind_id: crate::identity::data::KindId,
) -> Vec<AspectKey> {
    runtime
        .aspect_semantics
        .plans
        .relation_plans
        .get(&kind_id)
        .map(plan_aspect_keys)
        .unwrap_or_default()
}

fn plan_aspect_keys(plan: &crate::schema::data::LoweredAspectPlan) -> Vec<AspectKey> {
    let mut aspects = plan
        .executable_bindings
        .iter()
        .map(|binding| binding.aspect_key.clone())
        .collect::<Vec<_>>();
    if !aspects.windows(2).all(|window| window[0] < window[1]) {
        aspects.sort();
        aspects.dedup();
    }
    aspects
}

fn entity_records_are_canonical(records: &[EntityReadRecord]) -> bool {
    records.windows(2).all(|window| {
        let left = &window[0];
        let right = &window[1];
        (
            left.entity_id.partition_id.0,
            left.entity_id.local_slot.0,
            left.entity_id.generation,
        ) <= (
            right.entity_id.partition_id.0,
            right.entity_id.local_slot.0,
            right.entity_id.generation,
        )
    })
}

fn aspect_versions_are_canonical(versions: &[(AspectKey, u64)]) -> bool {
    versions.windows(2).all(|window| window[0] <= window[1])
}

fn snapshot_read_path_artifact(
    version_id: crate::identity::data::VersionId,
    cached: bool,
    recent_candidate: bool,
    recent_window: usize,
    residency: VisibilityResidency,
    mut extra_entries: Vec<RelationalDiagnosticsEntry>,
) -> RelationalDiagnosticArtifact {
    let mut entries = vec![RelationalDiagnosticsEntry {
        code: DiagnosticCode::SnapshotReadPathInspected,
        message: "snapshot/version read path inspected".to_string(),
        fields: json!({
            "version_id": version_id.0,
            "cached_visibility_state": cached,
            "recent_candidate": recent_candidate,
            "recent_window": recent_window,
            "recent_resident": residency.recent_resident,
            "branch_head_refs": residency.branch_head_refs,
            "replay_refs": residency.replay_refs,
            "active_snapshot_refs": residency.active_snapshot_refs,
        }),
    }];
    entries.append(&mut extra_entries);
    RelationalDiagnosticArtifact {
        scope: DiagnosticsScope::Snapshot,
        kind: DiagnosticsArtifactKind::DetailedTrace,
        determinism: DeterminismExpectation::Required,
        entries,
    }
}

fn snapshot_decision_entry(
    cached: bool,
    protected: bool,
    recent_candidate: bool,
    published_handle: bool,
) -> RelationalDiagnosticsEntry {
    let (code, message) = if cached {
        (
            DiagnosticCode::VisibilityCacheHit,
            "read will reuse cached visibility state",
        )
    } else if protected {
        (
            DiagnosticCode::VisibilityCacheProtectedRead,
            "read will reconstruct and keep a protected visibility state",
        )
    } else if recent_candidate {
        (
            DiagnosticCode::VisibilityCacheRecentAdmissionCandidate,
            "read will reconstruct and may admit visibility state into the recent cache",
        )
    } else {
        (
            DiagnosticCode::VisibilityCacheTransientRead,
            "read will reconstruct transient visibility state without cache residency",
        )
    };
    RelationalDiagnosticsEntry {
        code,
        message: message.to_string(),
        fields: json!({
            "cached_visibility_state": cached,
            "protected_visibility_state": protected,
            "recent_admission_candidate": recent_candidate,
            "published_handle": published_handle,
        }),
    }
}

fn snapshot_miss_entry(published_handle: bool) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry {
        code: DiagnosticCode::VisibilityCacheMissReconstructed,
        message: "read will reconstruct visibility state from committed history".to_string(),
        fields: json!({
            "published_handle": published_handle,
        }),
    }
}

fn is_protected(residency: &VisibilityResidency) -> bool {
    residency.branch_head_refs > 0
        || residency.replay_refs > 0
        || residency.active_snapshot_refs > 0
}
