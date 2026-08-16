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

    pub(crate) fn node_partition_version_map(
        &self,
        node: NodeId,
    ) -> Result<crate::data::aspect::PartitionVersionMap, SignalError> {
        let hot = self.hot_ref(node)?;
        let warm = self.warm_ref(node)?;
        Ok(
            crate::data::aspect::PartitionVersionMap::from_storage_parts(
                hot.aspect_version_header,
                warm.aspect_version_overrides.clone(),
            ),
        )
    }

    pub(crate) fn replace_node_partition_version_map(
        &mut self,
        node: NodeId,
        versions: crate::data::aspect::PartitionVersionMap,
    ) -> Result<(), SignalError> {
        let (header, overrides) = versions.into_storage_parts();
        self.hot_mut(node)?.aspect_version_header = header;
        self.warm_mut(node)?.aspect_version_overrides = overrides;
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
        self.release_pending_causes(node)?;
        let hot = self.hot_mut(node)?;
        hot.state = NodeState::Clean;
        hot.dirty_aspects = crate::data::aspect::AspectMask::EMPTY;
        hot.dirty_partition_scope_aspects = crate::data::aspect::AspectMask::EMPTY;
        hot.pending_cause_set_id =
            crate::data::graph::storage::invalidation_causes::PendingCauseSetId::EMPTY;
        let warm = self.warm_mut(node)?;
        warm.dirty_partition_scope_payload.clear();
        warm.pending_dependency_revalidation = None;
        warm.direct_invalidation_basis = None;
        Ok(())
    }

    pub(crate) fn transition_node_dirty(
        &mut self,
        node: NodeId,
        aspect: crate::data::aspect::Aspect,
        scopes: &[PartitionSubscription],
    ) -> Result<(), SignalError> {
        let hot = self.hot_ref(node)?;
        let invalidates_dependency_causes = hot.pending_cause_set_id
            != crate::data::graph::storage::invalidation_causes::PendingCauseSetId::EMPTY;
        if invalidates_dependency_causes {
            self.release_pending_causes(node)?;
        }
        {
            let warm = self.warm_mut(node)?;
            warm.direct_invalidation_generation = warm
                .direct_invalidation_generation
                .checked_add(1)
                .expect("direct invalidation generation overflow");
            let generation = warm.direct_invalidation_generation;
            match warm.direct_invalidation_basis.as_mut() {
                Some(basis) => basis.merge_seed(generation, aspect, scopes.iter().cloned()),
                None => {
                    warm.direct_invalidation_basis = Some(
                        crate::data::proof::invalidation::source_seed::DirectInvalidationBasis::from_seed(
                            generation,
                            aspect,
                            scopes.iter().cloned(),
                        ),
                    );
                }
            }
        }
        let hot = self.hot_ref(node)?;
        let was_clean = matches!(hot.state, NodeState::Clean);
        let already_dirty_for_aspect = !invalidates_dependency_causes
            && hot
                .dirty_aspects
                .contains(crate::data::aspect::AspectMask::from_aspect(aspect));
        let has_scoped_payload = {
            let warm = self.warm_mut(node)?;
            if invalidates_dependency_causes {
                warm.dirty_partition_scope_payload.clear();
            }
            merge_dirty_partition_scopes(warm, aspect, scopes, was_clean, already_dirty_for_aspect)
        };
        let hot = self.hot_mut(node)?;
        if invalidates_dependency_causes {
            hot.pending_cause_set_id =
                crate::data::graph::storage::invalidation_causes::PendingCauseSetId::EMPTY;
            hot.dirty_aspects = crate::data::aspect::AspectMask::EMPTY;
            hot.dirty_partition_scope_aspects = crate::data::aspect::AspectMask::EMPTY;
        }
        hot.state = NodeState::Dirty;
        hot.dirty_aspects.insert(aspect);
        if has_scoped_payload {
            hot.dirty_partition_scope_aspects.insert(aspect);
        } else {
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

    pub(crate) fn transition_node_pending_revalidation(
        &mut self,
        node: NodeId,
    ) -> Result<(), SignalError> {
        self.transition_node_revalidation(node, false)
    }

    pub(crate) fn transition_node_structural_revalidation(
        &mut self,
        node: NodeId,
    ) -> Result<(), SignalError> {
        self.transition_node_revalidation(node, true)
    }

    fn transition_node_revalidation(
        &mut self,
        node: NodeId,
        requires_structural_recompute: bool,
    ) -> Result<(), SignalError> {
        let previous = self
            .pending_dependency_revalidation(node)?
            .map(|pending| pending.unresolved_producers().to_vec())
            .unwrap_or_default();
        let producers = self
            .current_runtime_dependencies_of(node)?
            .iter()
            .filter_map(|edge| {
                (!matches!(self.get_state(edge.source()), Ok(NodeState::Clean)))
                    .then_some(edge.source())
            })
            .collect::<Vec<_>>();
        let mut entry = self.get_entry_mut(node)?;
        if matches!(entry.get_state(), NodeState::Clean) {
            entry.set_state(NodeState::MaybeStale);
        }
        if requires_structural_recompute {
            entry.mark_pending_structural_revalidation(producers);
        } else {
            entry.mark_pending_dependency_revalidation(producers);
        }
        drop(entry);
        let current = self
            .pending_dependency_revalidation(node)?
            .map(|pending| pending.unresolved_producers().to_vec())
            .unwrap_or_default();
        self.replace_pending_revalidation_waiters(node, &previous, &current);
        Ok(())
    }

    pub(crate) fn resolve_node_pending_revalidation(
        &mut self,
        node: NodeId,
        producer: NodeId,
    ) -> Result<(), SignalError> {
        let mut resolutions = vec![(node, producer)];
        while let Some((consumer, resolved_producer)) = resolutions.pop() {
            self.replace_pending_revalidation_waiters(
                consumer,
                std::slice::from_ref(&resolved_producer),
                &[],
            );
            let resolved = self
                .get_entry_mut(consumer)?
                .resolve_pending_dependency_producer(resolved_producer);
            let requires_structural_recompute = self
                .pending_dependency_revalidation(consumer)?
                .is_some_and(|pending| pending.requires_structural_recompute());
            let became_stable = resolved
                && !requires_structural_recompute
                && matches!(self.get_state(consumer)?, NodeState::MaybeStale)
                && self.pending_causes(consumer)?.is_empty()
                && self.node_dirty_aspects(consumer)?.is_empty();
            if !became_stable {
                continue;
            }
            self.set_node_state(consumer, NodeState::Clean)?;
            let subscribers = self.pending_revalidation_waiters(consumer)?;
            resolutions.extend(
                subscribers
                    .into_iter()
                    .map(|subscriber| (subscriber, consumer)),
            );
        }
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
            warm.dirty_partition_scope_payload.sort_unstable();
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
