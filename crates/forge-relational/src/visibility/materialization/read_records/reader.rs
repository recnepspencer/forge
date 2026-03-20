use crate::capabilities::{SnapshotSource, VersionSource, VisibilityPolicySource};
use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::logic::runtime::{RelationalRuntime, VisibilityResidency};
use crate::publication::data::diff::AspectKey;
use crate::query::data::QueryWorkPacket;
use crate::snapshots::data::{SnapshotHandle, SnapshotInspectionSummary};
use crate::storage::data::{
    EntityReadRecord, PacketResult, RelationReadRecord, RelationalReadView,
};
use crate::storage::logic::state::{DenseSlotBitSet, PartitionAccess};
use crate::symbols::data::InternedString;
use crate::visibility::cache_state::{
    cached_state_for_version, reconstruct_state, residency_for_version,
};
use crate::visibility::snapshot_states::{build_visibility_state, read_view_from_snapshot_state};
use serde_json::json;

use super::materialization::{
    materialize_current_entity_record, materialize_current_relation_record,
    materialize_entity_record_at_version, materialize_relation_record_at_version,
};
use super::visibility::{slot_kind_matches, visible_slots_in_partition_from_state};

pub struct VisibilityReadContext<'runtime> {
    runtime: &'runtime RelationalRuntime,
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
        let read_view = self.read_version(version_id);
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
                snapshot_id: handle.snapshot_id,
                version_id,
                read_policy,
            };
            return Some(read_view);
        }
        let version_id = self
            .runtime
            .published_snapshot_version(handle.snapshot_id)?;
        let mut read_view = self.read_version(version_id);
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

    pub fn execute_read_packet(
        &self,
        handle: &SnapshotHandle,
        packet: &QueryWorkPacket,
    ) -> Option<PacketResult> {
        self.read_snapshot(handle)
            .map(|read_view| read_view.execute_packet(packet))
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
        Some(declared_aspects_for_entity_kind(self.runtime, record.kind.kind_id))
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
        let cached = cached_state_for_version(self.runtime, version_id).is_some();
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
