use crate::identity::data::VersionId;
use crate::storage::data::RecordLifecycleState;

use super::{RecordArena, RecordKind, SlotInit};

impl<K: RecordKind> RecordArena<K> {
    pub(crate) fn apply_extra_update(
        &mut self,
        slot: usize,
        extra: K::Extra,
        version_id: VersionId,
    ) {
        self.extra[slot] = extra.clone();
        if let Some(current) = self.metadata_history[slot].last_mut() {
            K::retire_metadata(current, version_id);
        }
        let kind_id = self.kind_ids[slot].expect("record extra update requires retained kind id");
        self.metadata_history[slot].push(K::metadata_for_create(
            kind_id,
            self.generations[slot],
            version_id,
            &extra,
        ));
    }

    pub(crate) fn retire(&mut self, slot: usize, version_id: VersionId) {
        self.retired_at[slot] = Some(version_id);
        self.lifecycle[slot] = RecordLifecycleState::DeletedRetained;
        self.live_bitset.set(slot, false);
        self.reclaimable_bitset.set(slot, true);
        if let Some(current) = self.metadata_history[slot].last_mut() {
            K::retire_metadata(current, version_id);
        }
    }

    pub(crate) fn push_slot(&mut self, init: SlotInit<K>) -> (usize, u32, bool) {
        let SlotInit {
            partition_id,
            kind_id,
            version_id,
            extra,
        } = init;
        if let Some(slot) = self.free_list.pop() {
            let idx = slot as usize;
            if let Some(current) = self.metadata_history[idx].last_mut() {
                K::retire_metadata(current, version_id);
            }
            self.partition_ids[idx] = partition_id;
            self.generations[idx] += 1;
            self.lifecycle[idx] = RecordLifecycleState::Live;
            self.kind_ids[idx] = Some(kind_id);
            self.metadata_history[idx].push(K::metadata_for_create(
                kind_id,
                self.generations[idx],
                version_id,
                &extra,
            ));
            self.created_at[idx] = version_id;
            self.retired_at[idx] = None;
            self.extra[idx] = extra;
            self.aspect_versions[idx].clear();
            self.diagnostics_enrichment[idx].clear();
            self.branch_pins[idx] = 0;
            self.replay_pins[idx] = 0;
            self.snapshot_pins[idx] = 0;
            self.live_bitset.set(idx, true);
            self.reclaimable_bitset.set(idx, false);
            return (idx, self.generations[idx], true);
        }

        let slot = self.generations.len();
        self.partition_ids.push(partition_id);
        self.generations.push(1);
        self.lifecycle.push(RecordLifecycleState::Live);
        self.kind_ids.push(Some(kind_id));
        self.metadata_history
            .push(vec![K::metadata_for_create(kind_id, 1, version_id, &extra)]);
        self.created_at.push(version_id);
        self.retired_at.push(None);
        self.extra.push(extra);
        self.aspect_versions.push(std::collections::BTreeMap::new());
        self.diagnostics_enrichment
            .push(std::collections::BTreeMap::new());
        self.branch_pins.push(0);
        self.replay_pins.push(0);
        self.snapshot_pins.push(0);
        self.live_bitset.set(slot, true);
        self.reclaimable_bitset.set(slot, false);
        (slot, 1, false)
    }

    pub(crate) fn reset_slot(&mut self, slot: usize) {
        self.kind_ids[slot] = None;
        self.extra[slot] = K::empty_extra();
        self.aspect_versions[slot].clear();
        self.diagnostics_enrichment[slot].clear();
        self.branch_pins[slot] = 0;
        self.replay_pins[slot] = 0;
        self.snapshot_pins[slot] = 0;
        self.retired_at[slot] = None;
        self.free_list.push(slot as u64);
    }

    pub(crate) fn set_lifecycle_for_slot(
        &mut self,
        slot: usize,
        lifecycle: RecordLifecycleState,
    ) -> bool {
        let Some(current) = self.lifecycle.get_mut(slot) else {
            return false;
        };
        *current = lifecycle;
        true
    }
}
