use std::num::NonZeroU32;

use super::{CanonicalCauseSetStore, PendingCauseSetId};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::proof::invalidation::binding::ResolvedDependencyCause;

pub(super) struct CauseSetHandleRemap {
    pub(super) consumer: NodeId,
    pub(super) previous: PendingCauseSetId,
    pub(super) current: PendingCauseSetId,
}

impl CanonicalCauseSetStore {
    pub(super) fn rebuild_occupied_generation(
        &mut self,
    ) -> Result<Vec<CauseSetHandleRemap>, SignalError> {
        for set in self.sets.iter().filter(|set| !set.is_empty()) {
            let consumer = set[0].key.consumer;
            if set.iter().any(|cause| cause.key.consumer != consumer) {
                return Err(SignalError::invalid_input(
                    "canonical cause set contains multiple consumers",
                ));
            }
        }

        let occupied_set_count = self.occupied_set_count;
        let previous_sets = std::mem::take(&mut self.sets);
        let previous_generations = std::mem::take(&mut self.slot_generations);
        #[cfg(test)]
        {
            self.last_compaction_slot_visits = previous_sets.len();
        }
        self.generation = self.generation.wrapping_add(1);
        self.free_indices.clear();
        self.occupied_set_count = 0;
        self.output_commit_reference_counts.clear();
        self.sets.reserve(occupied_set_count);
        let mut remaps = Vec::with_capacity(occupied_set_count);
        for (index, set) in previous_sets.into_iter().enumerate() {
            if set.is_empty() {
                continue;
            }
            let consumer = set[0].key.consumer;
            let previous = PendingCauseSetId {
                index: NonZeroU32::new(index as u32 + 1),
                generation: previous_generations
                    .get(index)
                    .copied()
                    .unwrap_or(self.generation.wrapping_sub(1)),
            };
            let current = self.insert(set);
            remaps.push(CauseSetHandleRemap {
                consumer,
                previous,
                current,
            });
        }
        self.prune_unreferenced_output_commits();
        Ok(remaps)
    }

    pub(crate) fn should_compact(&self) -> bool {
        self.sets.len().saturating_sub(self.occupied_set_count) > self.occupied_set_count
    }

    pub(super) fn normalize_slot_metadata(&mut self) {
        self.slot_generations
            .resize(self.sets.len(), self.generation);
    }

    pub(super) fn prune_unreferenced_output_commits(&mut self) {
        self.published_output_commits
            .retain(|ordinal, _| self.output_commit_reference_counts.contains_key(ordinal));
    }

    pub(super) fn rebuild_derived_metadata(&mut self) {
        self.occupied_set_count = self.sets.iter().filter(|set| !set.is_empty()).count();
        self.output_commit_reference_counts.clear();
        for set in &self.sets {
            for cause in set {
                *self
                    .output_commit_reference_counts
                    .entry(cause.binding_axes.output_commit_ordinal.0)
                    .or_default() += 1;
            }
        }
        self.prune_unreferenced_output_commits();
    }

    pub(super) fn add_output_commit_references(&mut self, set_index: usize) {
        let ordinals = self.sets[set_index]
            .iter()
            .map(|cause| cause.binding_axes.output_commit_ordinal.0)
            .collect::<Vec<_>>();
        for ordinal in ordinals {
            *self
                .output_commit_reference_counts
                .entry(ordinal)
                .or_default() += 1;
        }
    }

    pub(super) fn add_output_commit_references_from(&mut self, causes: &[ResolvedDependencyCause]) {
        for cause in causes {
            *self
                .output_commit_reference_counts
                .entry(cause.binding_axes.output_commit_ordinal.0)
                .or_default() += 1;
        }
    }

    pub(super) fn remove_output_commit_references_from(
        &mut self,
        causes: &[ResolvedDependencyCause],
    ) {
        for cause in causes {
            let ordinal = cause.binding_axes.output_commit_ordinal.0;
            let count = self
                .output_commit_reference_counts
                .get_mut(&ordinal)
                .expect("stored cause commit ordinal must be reference-counted");
            *count -= 1;
            if *count == 0 {
                self.output_commit_reference_counts.remove(&ordinal);
                self.published_output_commits.remove(&ordinal);
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn allocated_slot_count(&self) -> usize {
        self.sets.len()
    }

    #[cfg(test)]
    pub(crate) fn occupied_slot_count(&self) -> usize {
        self.occupied_set_count
    }

    #[cfg(test)]
    pub(crate) fn last_compaction_slot_visits(&self) -> usize {
        self.last_compaction_slot_visits
    }
}
