use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::{CheckpointNodeImage, NodeEntry, NodeHotData, NodeState, NodeWarmData};
use crate::data::output::PartitionSubscription;
use crate::data::reuse::ReuseBasis;
use crate::data::{aspect::AspectVersion, core_profile::StableHashValue, output::ChangedRegion};

impl SignalGraph {
    pub(crate) fn replace_entry(
        &mut self,
        id: NodeId,
        entry: NodeEntry,
    ) -> Result<(), SignalError> {
        let mut target = self.get_entry_mut(id)?;
        *target = entry;
        drop(target);
        self.record_branch_mutation_state(id);
        Ok(())
    }

    pub(crate) fn replace_entry_from_checkpoint_image(
        &mut self,
        id: NodeId,
        image: CheckpointNodeImage,
    ) -> Result<(), SignalError> {
        self.replace_entry(id, NodeEntry::from_checkpoint_image(image))
    }

    pub(crate) fn node_runtime_artifact_structural_state(
        &self,
        node: NodeId,
    ) -> Result<
        (
            Option<crate::diagnostics::lineage::LineageArtifactId>,
            Option<StableHashValue>,
            Option<ReuseBasis>,
        ),
        SignalError,
    > {
        let runtime = self.warm_ref(node)?.runtime_artifact_state.as_ref();
        Ok((
            runtime.and_then(|state| state.lineage_artifact_id().get()),
            runtime.map(crate::data::trace::RuntimeArtifactState::output_hash),
            runtime.map(|state| state.reuse_basis().clone_inner()),
        ))
    }

    pub(crate) fn apply_node_aspect_version(
        &mut self,
        node: NodeId,
        version: AspectVersion,
        changed_regions: &[ChangedRegion],
    ) -> Result<(), SignalError> {
        let has_partition_overrides = {
            let warm = self.warm_mut(node)?;
            warm.aspect_version_overrides
                .apply_evaluation(version, changed_regions);
            warm.aspect_version_overrides.has_overrides()
        };
        let hot = self.hot_mut(node)?;
        hot.aspect_version_header.set_global(version);
        hot.aspect_version_header
            .set_has_partition_overrides(has_partition_overrides);
        Ok(())
    }

    pub(crate) fn apply_node_artifact_write_delta(
        &mut self,
        node: NodeId,
        delta: crate::data::trace::ArtifactWriteDelta,
    ) -> Result<bool, SignalError> {
        self.warm_mut(node)?.runtime_artifact_state = delta.runtime;
        let retained_present = delta.retained.is_some();
        if retained_present {
            self.cold_mut(node)?.retained_artifact = delta.retained;
        } else if let Some(cold) = self.arena.cold[node.index() as usize].as_mut() {
            cold.retained_artifact = None;
        }
        self.trim_cold_if_empty(node);
        Ok(retained_present)
    }

    pub(crate) fn transition_node_clean(&mut self, node: NodeId) -> Result<(), SignalError> {
        let hot = self.hot_mut(node)?;
        hot.state = NodeState::Clean;
        hot.dirty_aspects = crate::data::aspect::AspectMask::EMPTY;
        hot.dirty_partition_scope_aspects = crate::data::aspect::AspectMask::EMPTY;
        self.warm_mut(node)?.dirty_partition_scope_payload.clear();
        Ok(())
    }

    pub(crate) fn transition_node_dirty(
        &mut self,
        node: NodeId,
        aspect: crate::data::aspect::Aspect,
        scopes: &[PartitionSubscription],
    ) -> Result<(), SignalError> {
        let hot = self.hot_ref(node)?;
        let was_clean = matches!(hot.state, NodeState::Clean);
        let already_dirty_for_aspect = hot
            .dirty_aspects
            .contains(crate::data::aspect::AspectMask::from_aspect(aspect));
        let has_scoped_payload = {
            let warm = self.warm_mut(node)?;
            merge_dirty_partition_scopes(warm, aspect, scopes, was_clean, already_dirty_for_aspect)
        };
        let hot = self.hot_mut(node)?;
        hot.state = NodeState::Dirty;
        hot.dirty_aspects.insert(aspect);
        if has_scoped_payload {
            hot.dirty_partition_scope_aspects.insert(aspect);
        } else {
            sync_dirty_partition_scope_flag(hot, aspect);
        }
        Ok(())
    }

    pub(crate) fn transition_node_maybe_stale(
        &mut self,
        node: NodeId,
        aspect: crate::data::aspect::Aspect,
    ) -> Result<(), SignalError> {
        let hot = self.hot_ref(node)?;
        let was_clean = matches!(hot.state, NodeState::Clean);
        let already_dirty_for_aspect = hot
            .dirty_aspects
            .contains(crate::data::aspect::AspectMask::from_aspect(aspect));
        let should_clear_scopes = was_clean || !already_dirty_for_aspect;
        if should_clear_scopes {
            self.warm_mut(node)?
                .dirty_partition_scope_payload
                .retain(|(candidate_aspect, _)| *candidate_aspect != aspect);
        }
        let hot = self.hot_mut(node)?;
        hot.state = NodeState::MaybeStale;
        hot.dirty_aspects.insert(aspect);
        if should_clear_scopes {
            sync_dirty_partition_scope_flag(hot, aspect);
        }
        Ok(())
    }

    pub(crate) fn set_node_state(
        &mut self,
        node: NodeId,
        state: NodeState,
    ) -> Result<(), SignalError> {
        self.hot_mut(node)?.state = state;
        Ok(())
    }
}

fn merge_dirty_partition_scopes(
    warm: &mut NodeWarmData,
    changed_aspect: crate::data::aspect::Aspect,
    changed_scopes: &[PartitionSubscription],
    was_clean: bool,
    already_dirty_for_aspect: bool,
) -> bool {
    if changed_scopes.is_empty() {
        warm.dirty_partition_scope_payload
            .retain(|(candidate_aspect, _)| *candidate_aspect != changed_aspect);
        return false;
    }
    if !was_clean
        && already_dirty_for_aspect
        && warm
            .dirty_partition_scope_payload
            .iter()
            .find(|(candidate_aspect, _)| *candidate_aspect == changed_aspect)
            .is_none()
    {
        return false;
    }
    for scope in changed_scopes {
        if !warm
            .dirty_partition_scope_payload
            .iter()
            .any(|(candidate_aspect, candidate_scope)| {
                *candidate_aspect == changed_aspect && *candidate_scope == *scope
            })
        {
            warm.dirty_partition_scope_payload
                .push((changed_aspect, scope.clone()));
        }
    }
    true
}

fn sync_dirty_partition_scope_flag(hot: &mut NodeHotData, aspect: crate::data::aspect::Aspect) {
    hot.dirty_partition_scope_aspects = crate::data::aspect::AspectMask::from_bits(
        hot.dirty_partition_scope_aspects.bits()
            & !crate::data::aspect::AspectMask::from_aspect(aspect).bits(),
    );
}
