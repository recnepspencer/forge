use std::collections::BTreeSet;

use super::{CanonicalCauseSetStore, PendingCauseSetId};
use crate::data::proof::invalidation::binding::ResolvedDependencyCause;

impl CanonicalCauseSetStore {
    pub(crate) fn rebuild_generation(
        &mut self,
        live_sets: impl IntoIterator<Item = Vec<ResolvedDependencyCause>>,
    ) -> Vec<PendingCauseSetId> {
        self.generation = self.generation.wrapping_add(1);
        self.sets.clear();
        self.slot_generations.clear();
        self.free_indices.clear();
        let ids = live_sets.into_iter().map(|set| self.insert(set)).collect();
        self.prune_unreferenced_output_commits();
        ids
    }

    pub(crate) fn should_compact(&self) -> bool {
        let occupied = self.sets.iter().filter(|set| !set.is_empty()).count();
        self.sets.len().saturating_sub(occupied) > occupied
    }

    pub(super) fn normalize_slot_metadata(&mut self) {
        self.slot_generations
            .resize(self.sets.len(), self.generation);
    }

    pub(super) fn prune_unreferenced_output_commits(&mut self) {
        let referenced = self
            .sets
            .iter()
            .flatten()
            .map(|cause| cause.binding_axes.output_commit_ordinal.0)
            .collect::<BTreeSet<_>>();
        self.published_output_commits
            .retain(|ordinal, _| referenced.contains(ordinal));
    }

    #[cfg(test)]
    pub(crate) fn allocated_slot_count(&self) -> usize {
        self.sets.len()
    }

    #[cfg(test)]
    pub(crate) fn occupied_slot_count(&self) -> usize {
        self.sets.iter().filter(|set| !set.is_empty()).count()
    }
}
