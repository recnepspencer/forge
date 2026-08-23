use std::collections::BTreeMap;
use std::num::NonZeroU32;

use serde::Serialize;

use super::PendingCauseSetId;
use crate::data::error::SignalError;
use crate::data::proof::invalidation::binding::ResolvedDependencyCause;
use crate::data::proof::invalidation::output_commit::ProducedAspectDelta;

#[derive(Debug, Clone, Serialize, Default)]
pub(crate) struct CanonicalCauseSetStore {
    pub(super) generation: u32,
    pub(super) sets: Vec<Vec<ResolvedDependencyCause>>,
    #[serde(default)]
    pub(super) slot_generations: Vec<u32>,
    #[serde(default)]
    pub(super) free_indices: Vec<u32>,
    #[serde(default)]
    pub(super) next_output_commit_ordinal: u64,
    #[serde(default)]
    pub(super) published_output_commits: BTreeMap<u64, ProducedAspectDelta>,
    #[serde(skip)]
    pub(super) deserialized_quarantine: bool,
    #[cfg(test)]
    #[serde(skip)]
    pub(super) published_order_probe: Vec<(u64, crate::data::handle::NodeId)>,
}

impl CanonicalCauseSetStore {
    #[cfg(test)]
    pub(crate) const fn output_commit_ordinal_for_test(&self) -> u64 {
        self.next_output_commit_ordinal
    }

    pub(crate) fn reserve(&mut self, additional_sets: usize) {
        self.sets.reserve(additional_sets);
    }

    pub(crate) fn reserve_output_commit_ordinal(
        &self,
    ) -> crate::data::proof::invalidation::binding::OutputCommitOrdinal {
        let next = self
            .next_output_commit_ordinal
            .checked_add(1)
            .expect("output commit ordinal overflow");
        crate::data::proof::invalidation::binding::OutputCommitOrdinal(next)
    }

    fn publish_output_commit_ordinal(
        &mut self,
        ordinal: crate::data::proof::invalidation::binding::OutputCommitOrdinal,
    ) {
        debug_assert_eq!(ordinal, self.reserve_output_commit_ordinal());
        self.next_output_commit_ordinal = ordinal.0;
    }

    pub(crate) fn publish_output_commit(&mut self, delta: ProducedAspectDelta) {
        self.publish_output_commit_ordinal(delta.output_commit_ordinal);
        #[cfg(test)]
        self.published_order_probe
            .push((delta.output_commit_ordinal.0, delta.producer));
        self.published_output_commits
            .insert(delta.output_commit_ordinal.0, delta);
        self.prune_unreferenced_output_commits();
    }

    pub(crate) fn published_output_commit(
        &self,
        ordinal: crate::data::proof::invalidation::binding::OutputCommitOrdinal,
    ) -> Option<&ProducedAspectDelta> {
        self.published_output_commits.get(&ordinal.0)
    }

    pub(crate) fn insert(
        &mut self,
        causes: impl IntoIterator<Item = ResolvedDependencyCause>,
    ) -> PendingCauseSetId {
        let mut causes = causes.into_iter().collect::<Vec<_>>();
        causes.sort_by(|left, right| left.key.cmp(&right.key));
        causes.dedup_by(|left, right| left.key == right.key);
        if causes.is_empty() {
            return PendingCauseSetId::EMPTY;
        }
        self.normalize_slot_metadata();
        let index = if let Some(index) = self.free_indices.pop() {
            self.sets[index as usize] = causes;
            index as usize
        } else {
            self.sets.push(causes);
            self.slot_generations.push(self.generation);
            self.sets.len() - 1
        };
        PendingCauseSetId {
            index: NonZeroU32::new(index as u32 + 1),
            generation: self.slot_generations[index],
        }
    }

    pub(crate) fn get(
        &self,
        id: PendingCauseSetId,
    ) -> Result<&[ResolvedDependencyCause], SignalError> {
        let Some(index) = id.index else {
            return Ok(&[]);
        };
        let slot = index.get() as usize - 1;
        let slot_generation = self
            .slot_generations
            .get(slot)
            .copied()
            .unwrap_or(self.generation);
        if id.generation != slot_generation {
            return Err(SignalError::invalid_input("stale pending cause-set handle"));
        }
        self.sets
            .get(slot)
            .map(Vec::as_slice)
            .ok_or_else(|| SignalError::invalid_input("unknown pending cause-set handle"))
    }

    pub(crate) fn replace_set(
        &mut self,
        current: PendingCauseSetId,
        causes: impl IntoIterator<Item = ResolvedDependencyCause>,
    ) -> Result<PendingCauseSetId, SignalError> {
        let mut causes = causes.into_iter().collect::<Vec<_>>();
        causes.sort_by(|left, right| left.key.cmp(&right.key));
        causes.dedup_by(|left, right| left.key == right.key);
        if causes.is_empty() {
            self.release(current)?;
            return Ok(PendingCauseSetId::EMPTY);
        }
        let Some(index) = current.index else {
            return Ok(self.insert(causes));
        };
        self.get(current)?;
        self.sets[index.get() as usize - 1] = causes;
        self.prune_unreferenced_output_commits();
        Ok(current)
    }

    pub(crate) fn release(&mut self, current: PendingCauseSetId) -> Result<(), SignalError> {
        let Some(index) = current.index else {
            return Ok(());
        };
        self.get(current)?;
        self.normalize_slot_metadata();
        let index = index.get() as usize - 1;
        self.sets[index].clear();
        self.slot_generations[index] = self.slot_generations[index].wrapping_add(1);
        if !self.free_indices.contains(&(index as u32)) {
            self.free_indices.push(index as u32);
        }
        self.prune_unreferenced_output_commits();
        Ok(())
    }

    pub(crate) fn has_occupied_sets(&self) -> bool {
        self.sets.iter().any(|set| !set.is_empty())
    }

    #[cfg(test)]
    pub(crate) fn replace_published_change_scopes_for_test(
        &mut self,
        ordinal: crate::data::proof::invalidation::binding::OutputCommitOrdinal,
        scopes: crate::data::proof::PartitionScopeSet,
    ) {
        let delta = self
            .published_output_commits
            .get_mut(&ordinal.0)
            .expect("test commit ordinal must exist");
        delta.changes.first_mut_for_test().changed_scopes = scopes;
    }

    #[cfg(test)]
    pub(crate) fn replace_published_internal_ordinal_for_test(
        &mut self,
        key: crate::data::proof::invalidation::binding::OutputCommitOrdinal,
        internal: crate::data::proof::invalidation::binding::OutputCommitOrdinal,
    ) {
        self.published_output_commits
            .get_mut(&key.0)
            .expect("test commit ordinal must exist")
            .output_commit_ordinal = internal;
    }
}
