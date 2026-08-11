#[cfg(test)]
use smallvec::SmallVec;

use crate::data::aspect::{Aspect, AspectMask};
#[cfg(test)]
use crate::data::core_profile::HOT_VEC_INLINE_CAPACITY;
use crate::data::output::PartitionSubscription;

use super::{NodeEntry, NodeState};

impl Default for NodeEntry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(not(test), allow(dead_code))]
impl NodeEntry {
    /// Create a new node entry in the `Dirty` state.
    pub fn new() -> Self {
        Self {
            hot: super::layout::NodeHotData {
                state: NodeState::Dirty,
                dirty_aspects: AspectMask::ALL,
                dirty_partition_scope_aspects: AspectMask::EMPTY,
                aspect_version_header: crate::data::aspect::AspectVersionHeader::zero(),
                dependencies_id: crate::data::graph::DependencySetId::EMPTY,
                subscribers_id: crate::data::graph::SubscriberSetId::EMPTY,
                dep_snapshot_id: crate::data::dependency::DependencySnapshotId::EMPTY,
            },
            warm: super::layout::NodeWarmData::default(),
            cold: None,
        }
    }

    /// The current state of this node.
    pub fn get_state(&self) -> &NodeState {
        &self.hot.state
    }

    /// Set the node state.
    pub fn set_state(&mut self, state: NodeState) {
        self.hot.state = state;
    }

    /// Transition to `Clean` and clear all dirty tracking.
    #[allow(dead_code)]
    pub fn transition_clean(&mut self) {
        self.set_state(NodeState::Clean);
        self.set_dirty_aspects(AspectMask::EMPTY);
        self.clear_dirty_partition_scopes();
    }

    /// Transition to `Dirty` for one aspect and merge any scoped dirty regions.
    pub fn transition_dirty(&mut self, aspect: Aspect, scopes: &[PartitionSubscription]) {
        let was_clean = matches!(self.hot.state, NodeState::Clean);
        let already_dirty_for_aspect = self
            .hot
            .dirty_aspects
            .contains(AspectMask::from_aspect(aspect));
        self.set_state(NodeState::Dirty);
        self.add_dirty_aspect(aspect);
        self.merge_dirty_partition_scopes(aspect, scopes, was_clean, already_dirty_for_aspect);
    }

    /// Transition to `MaybeStale` for one aspect.
    #[allow(dead_code)]
    pub fn transition_maybe_stale(&mut self, aspect: Aspect) {
        let was_clean = matches!(self.hot.state, NodeState::Clean);
        let already_dirty_for_aspect = self
            .hot
            .dirty_aspects
            .contains(AspectMask::from_aspect(aspect));
        self.set_state(NodeState::MaybeStale);
        self.add_dirty_aspect(aspect);
        if was_clean || !already_dirty_for_aspect {
            self.clear_dirty_partition_scopes_for(aspect);
        }
    }

    /// Dirty aspects currently pending recomputation for this node.
    pub fn get_dirty_aspects(&self) -> AspectMask {
        self.hot.dirty_aspects
    }

    /// Replace the dirty aspect mask.
    pub fn set_dirty_aspects(&mut self, dirty_aspects: AspectMask) {
        self.hot.dirty_aspects = dirty_aspects;
    }

    #[cfg(test)]
    pub fn get_dirty_partition_scopes(
        &self,
    ) -> SmallVec<[PartitionSubscription; HOT_VEC_INLINE_CAPACITY]> {
        self.dirty_partition_scopes().cloned().collect()
    }

    #[cfg(test)]
    pub fn dirty_partition_scopes(&self) -> impl Iterator<Item = &PartitionSubscription> {
        self.warm
            .dirty_partition_scope_payload
            .iter()
            .map(|(_, scope)| scope)
    }

    pub fn clear_dirty_partition_scopes(&mut self) {
        self.hot.dirty_partition_scope_aspects = AspectMask::EMPTY;
        self.warm.dirty_partition_scope_payload.clear();
    }

    pub fn get_dirty_partition_scopes_for(
        &self,
        aspect: Aspect,
    ) -> impl Iterator<Item = &PartitionSubscription> {
        self.warm
            .dirty_partition_scope_payload
            .iter()
            .filter(move |(candidate_aspect, _)| *candidate_aspect == aspect)
            .map(|(_, scope)| scope)
    }

    pub fn clear_dirty_partition_scopes_for(&mut self, aspect: Aspect) {
        self.warm
            .dirty_partition_scope_payload
            .retain(|(candidate_aspect, _)| *candidate_aspect != aspect);
        self.sync_dirty_partition_scope_flag(aspect);
    }

    pub fn add_dirty_partition_scope(&mut self, aspect: Aspect, scope: PartitionSubscription) {
        if !self.warm.dirty_partition_scope_payload.iter().any(
            |(candidate_aspect, candidate_scope)| {
                *candidate_aspect == aspect && *candidate_scope == scope
            },
        ) {
            self.warm
                .dirty_partition_scope_payload
                .push((aspect, scope));
            self.hot.dirty_partition_scope_aspects.insert(aspect);
        }
    }

    /// Add one dirty aspect to the current mask.
    pub fn add_dirty_aspect(&mut self, aspect: Aspect) {
        self.hot.dirty_aspects.insert(aspect);
    }

    fn merge_dirty_partition_scopes(
        &mut self,
        changed_aspect: Aspect,
        changed_scopes: &[PartitionSubscription],
        was_clean: bool,
        already_dirty_for_aspect: bool,
    ) {
        if changed_scopes.is_empty() {
            // Whole-aspect invalidation supersedes scoped dirtiness for this aspect only.
            self.clear_dirty_partition_scopes_for(changed_aspect);
            return;
        }
        if !was_clean
            && already_dirty_for_aspect
            && self
                .get_dirty_partition_scopes_for(changed_aspect)
                .next()
                .is_none()
        {
            // An existing whole-aspect dirty mark is already stronger than any scoped follow-up.
            return;
        }
        for scope in changed_scopes {
            self.add_dirty_partition_scope(changed_aspect, scope.clone());
        }
    }

    pub(super) fn sync_all_dirty_partition_scope_flags(&mut self) {
        self.hot.dirty_partition_scope_aspects = AspectMask::EMPTY;
        for (aspect, _) in &self.warm.dirty_partition_scope_payload {
            self.hot.dirty_partition_scope_aspects.insert(*aspect);
        }
    }

    fn sync_dirty_partition_scope_flag(&mut self, aspect: Aspect) {
        if self
            .warm
            .dirty_partition_scope_payload
            .iter()
            .any(|(candidate_aspect, _)| *candidate_aspect == aspect)
        {
            self.hot.dirty_partition_scope_aspects.insert(aspect);
        } else {
            self.hot.dirty_partition_scope_aspects = AspectMask::from_bits(
                self.hot.dirty_partition_scope_aspects.bits()
                    & !AspectMask::from_aspect(aspect).bits(),
            );
        }
    }
}
