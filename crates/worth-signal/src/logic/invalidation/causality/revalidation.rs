use crate::data::error::SignalError;
#[cfg(test)]
use crate::data::graph::storage::invalidation_causes::PendingCauseSetId;
use crate::data::handle::NodeId;
use crate::data::output::scopes_overlap;
use crate::data::proof::invalidation::binding::ResolvedDependencyCause;
use crate::data::proof::invalidation::output_commit::ProducedAspectDelta;
use crate::data::proof::PartitionScopeSet;

use crate::data::graph::SignalGraph;

enum CauseCommitAuthority<'a> {
    Published,
    Prepared(&'a ProducedAspectDelta),
}

impl SignalGraph {
    pub(crate) fn node_invalidation_input(
        &self,
        node: NodeId,
    ) -> Result<crate::data::proof::invalidation::revalidation::NodeInvalidationInput, SignalError>
    {
        use crate::data::proof::invalidation::revalidation::{
            CanonicalDependencyCauseSet, NodeInvalidationInput, ResolvedDependencyBasis,
        };
        self.ensure_cause_readmission_complete()?;
        self.validate_direct_invalidation_storage(node)?;
        let revision = self.dependency_revision(node)?;
        if let Some(pending) = self.pending_dependency_revalidation(node)? {
            if pending.dependency_revision() != revision {
                return Err(SignalError::invalid_input(
                    "pending dependency revalidation belongs to a stale dependency revision",
                ));
            }
            if !pending.is_resolved() {
                return Ok(NodeInvalidationInput::Pending(pending));
            }
            if pending.requires_structural_recompute() {
                let pending_causes = self.pending_causes(node)?.to_vec();
                let causes = if pending_causes.is_empty() {
                    CanonicalDependencyCauseSet::structural(revision)
                } else {
                    self.resolved_dependency_causes(node, pending_causes)?
                };
                debug_assert!(causes.is_bound_to_revision(revision));
                return Ok(NodeInvalidationInput::Resolved(causes));
            }
        }
        let causes = self.pending_causes(node)?.to_vec();
        let dirty_aspects = self.node_dirty_aspects(node)?;
        if let Some(direct) = self.get_entry(node)?.direct_invalidation_basis() {
            if !causes.is_empty() {
                return Err(SignalError::invalid_input(
                    "direct invalidation basis cannot coexist with dependency causes",
                ));
            }
            let cached_scoped_aspects = self
                .get_entry(node)?
                .dirty_partition_scope_payload()
                .to_vec();
            if dirty_aspects != direct.dirty_aspects()
                || cached_scoped_aspects.as_slice() != direct.scoped_aspects()
            {
                return Err(SignalError::invalid_input(
                    "dirty mask or scope cache drifted from direct invalidation basis",
                ));
            }
            let resolved = CanonicalDependencyCauseSet::from_source_recompute(
                revision,
                direct.dirty_aspects(),
                direct.scoped_aspects().to_vec(),
            );
            return Ok(NodeInvalidationInput::Resolved(resolved));
        }
        if causes.is_empty() && dirty_aspects.is_empty() {
            let basis = ResolvedDependencyBasis::new(revision);
            debug_assert!(basis.is_bound_to_revision(revision));
            return Ok(NodeInvalidationInput::ResolvedNoChange(basis));
        }
        if causes.is_empty() {
            return Err(SignalError::invalid_input(
                "dirty cache has no direct invalidation or dependency-cause basis",
            ));
        }
        let causes = self.resolved_dependency_causes(node, causes)?;
        debug_assert!(causes.is_bound_to_revision(revision));
        Ok(NodeInvalidationInput::Resolved(causes))
    }

    fn resolved_dependency_causes(
        &self,
        node: NodeId,
        causes: Vec<ResolvedDependencyCause>,
    ) -> Result<
        crate::data::proof::invalidation::revalidation::CanonicalDependencyCauseSet,
        SignalError,
    > {
        use crate::data::proof::invalidation::revalidation::CanonicalDependencyCauseSet;
        let resolved = CanonicalDependencyCauseSet::from_dependency_causes(causes);
        let cached_aspects = self.node_dirty_aspects(node)?;
        let cached_scoped_aspects = self
            .get_entry(node)?
            .dirty_partition_scope_payload()
            .to_vec();
        if cached_aspects != resolved.dirty_aspects()
            || cached_scoped_aspects.as_slice() != resolved.dirty_scoped_aspects()
        {
            return Err(SignalError::invalid_input(
                "dirty mask or scope cache drifted from canonical dependency causes",
            ));
        }
        Ok(resolved)
    }

    pub(crate) fn pending_dependency_revalidation(
        &self,
        node: NodeId,
    ) -> Result<
        Option<crate::data::proof::invalidation::binding::PendingDependencyRevalidation>,
        SignalError,
    > {
        Ok(self
            .get_entry(node)?
            .pending_dependency_revalidation()
            .cloned())
    }

    pub(crate) fn ensure_cause_readmission_complete(&self) -> Result<(), SignalError> {
        if self.cause_readmission_required || self.cause_sets.requires_readmission() {
            return Err(SignalError::invalid_input(
                "checkpoint dependency authority is quarantined until restore readmission",
            ));
        }
        Ok(())
    }

    pub(crate) fn readmit_checkpoint_causes(&mut self) -> Result<(), SignalError> {
        for node in self.live_node_ids() {
            self.validate_direct_invalidation_storage(node)?;
        }
        if !self.cause_readmission_required && !self.cause_sets.requires_readmission() {
            return Ok(());
        }
        self.cause_sets
            .readmit_graph_instance(self.runtime_instance_id());
        let nodes = self.live_node_ids();
        for &node in &nodes {
            let id = self.get_entry(node)?.pending_cause_set_id();
            let causes = self.cause_sets.get(id)?.to_vec();
            self.validate_pending_causes(node, &causes)
                .map_err(|error| {
                    SignalError::incompatible_snapshot(format!(
                        "checkpoint cause readmission failed for {node}: {error}"
                    ))
                })?;
        }
        self.cause_readmission_required = false;
        self.cause_sets.complete_readmission();
        for node in nodes {
            self.rebuild_dirty_caches_from_pending_causes(node)?;
        }
        Ok(())
    }

    fn validate_direct_invalidation_storage(&self, node: NodeId) -> Result<(), SignalError> {
        let entry = self.get_entry(node)?;
        let causes = self.cause_sets.get(entry.pending_cause_set_id())?;
        let direct = entry.direct_invalidation_basis();
        if direct.is_some() && !causes.is_empty() {
            return Err(SignalError::invalid_input(
                "direct invalidation basis cannot coexist with dependency causes",
            ));
        }
        if !causes.is_empty() && matches!(entry.get_state(), crate::data::node::NodeState::Clean) {
            return Err(SignalError::invalid_input(
                "dependency causes require an unsettled consumer",
            ));
        }
        let dirty_aspects = entry.get_dirty_aspects();
        let dirty_scoped_aspects = entry.dirty_partition_scope_payload();
        match direct {
            Some(basis) => {
                if matches!(entry.get_state(), crate::data::node::NodeState::Clean) {
                    return Err(SignalError::invalid_input(
                        "direct invalidation basis requires an unsettled node",
                    ));
                }
                if let crate::data::proof::invalidation::source_seed::DirectInvalidationBasis::SourceRecompute {
                    dirty_aspects: basis_aspects,
                    scoped_aspects,
                } = basis
                {
                    if basis_aspects.is_empty()
                        || scoped_aspects.windows(2).any(|pair| pair[0] >= pair[1])
                        || scoped_aspects.iter().any(|(aspect, _)| {
                            !basis_aspects.contains(
                                crate::data::aspect::AspectMask::from_aspect(*aspect),
                            )
                        })
                    {
                        return Err(SignalError::invalid_input(
                            "source recompute basis is empty or non-canonical",
                        ));
                    }
                }
                if dirty_aspects != basis.dirty_aspects()
                    || dirty_scoped_aspects != basis.scoped_aspects()
                {
                    return Err(SignalError::invalid_input(
                        "direct invalidation cache drifted from its persisted basis",
                    ));
                }
            }
            None if causes.is_empty()
                && (!dirty_aspects.is_empty() || !dirty_scoped_aspects.is_empty()) =>
            {
                return Err(SignalError::invalid_input(
                    "dirty cache has no persisted direct or dependency-cause basis",
                ));
            }
            None => {}
        }
        Ok(())
    }

    pub(crate) fn validate_pending_causes(
        &self,
        consumer: NodeId,
        causes: &[ResolvedDependencyCause],
    ) -> Result<(), SignalError> {
        for cause in causes {
            self.validate_pending_cause(consumer, cause, CauseCommitAuthority::Published)?;
        }
        Ok(())
    }

    pub(crate) fn validate_prepared_pending_causes(
        &self,
        consumer: NodeId,
        causes: &[ResolvedDependencyCause],
        delta: &ProducedAspectDelta,
    ) -> Result<(), SignalError> {
        for cause in causes {
            self.validate_pending_cause(consumer, cause, CauseCommitAuthority::Prepared(delta))?;
        }
        Ok(())
    }

    fn validate_pending_cause(
        &self,
        consumer: NodeId,
        cause: &ResolvedDependencyCause,
        commit_authority: CauseCommitAuthority<'_>,
    ) -> Result<(), SignalError> {
        self.validate_cause_identity_axes(consumer, cause)?;
        let edge = self
            .current_runtime_dependencies_of(consumer)?
            .iter()
            .find(|edge| {
                edge.source() == cause.key.producer
                    && edge.aspect() == cause.key.aspect
                    && edge.scope_ref() == cause.key.edge_scope.as_ref()
            })
            .ok_or_else(|| {
                SignalError::invalid_input(
                    "pending dependency cause does not match a current dependency edge",
                )
            })?;
        let snapshot = self
            .get_dep_snapshot(consumer)?
            .entries()
            .iter()
            .find(|entry| {
                entry.source == cause.key.producer
                    && entry.aspect == cause.key.aspect
                    && entry.scope.as_ref() == edge.scope_ref()
            })
            .ok_or_else(|| {
                SignalError::invalid_input(
                    "pending dependency cause has no matching dependency snapshot",
                )
            })?;
        if snapshot.cached_version != cause.binding_axes.cached_version {
            return Err(SignalError::invalid_input(
                "pending dependency cause cached version drifted from its dependency snapshot",
            ));
        }
        let current_version = self.node_version_for_scope(
            cause.key.producer,
            cause.key.aspect,
            cause.key.edge_scope.as_ref(),
        )?;
        if current_version != cause.binding_axes.committed_version {
            return Err(SignalError::invalid_input(
                "pending dependency cause committed version drifted from producer authority",
            ));
        }
        if !self.commit_authority_matches(cause, commit_authority) {
            return Err(SignalError::invalid_input(
                "pending dependency cause has no matching performed output commit",
            ));
        }
        let expected_scopes = edge
            .scope_ref()
            .cloned()
            .map(|scope| PartitionScopeSet::new([scope]))
            .unwrap_or_default();
        if cause.changed_scopes.as_slice() != expected_scopes.as_slice() {
            return Err(SignalError::invalid_input(
                "pending dependency cause scope is not normalized to its dependency edge",
            ));
        }
        Ok(())
    }

    fn commit_authority_matches(
        &self,
        cause: &ResolvedDependencyCause,
        authority: CauseCommitAuthority<'_>,
    ) -> bool {
        let ordinal = cause.binding_axes.output_commit_ordinal;
        if ordinal.0 == 0 {
            return false;
        }
        let delta = match authority {
            CauseCommitAuthority::Published => self.cause_sets.published_output_commit(ordinal),
            CauseCommitAuthority::Prepared(delta) if delta.output_commit_ordinal == ordinal => {
                Some(delta)
            }
            CauseCommitAuthority::Prepared(_) => self.cause_sets.published_output_commit(ordinal),
        };
        delta.is_some_and(|delta| {
            delta.output_commit_ordinal == ordinal
                && delta.producer == cause.key.producer
                && delta.changes.as_slice().iter().any(|change| {
                    let scope_matches = cause.key.edge_scope.as_ref().is_none_or(|edge_scope| {
                        change.changed_scopes.is_empty()
                            || change
                                .changed_scopes
                                .iter()
                                .any(|changed| scopes_overlap(changed, edge_scope))
                    });
                    change.aspect == cause.key.aspect
                        && change.committed_version == cause.binding_axes.committed_version
                        && scope_matches
                })
        })
    }

    fn validate_cause_identity_axes(
        &self,
        consumer: NodeId,
        cause: &ResolvedDependencyCause,
    ) -> Result<(), SignalError> {
        let axes = &cause.binding_axes;
        let key = &cause.key;
        let key_matches_binding = key.graph_instance == axes.graph_instance
            && key.consumer == axes.consumer
            && key.dependency_revision == axes.dependency_revision
            && key.producer == axes.producer
            && key.aspect == axes.aspect
            && key.edge_scope == axes.edge_scope;
        let binding_matches_graph = axes.graph_instance == self.runtime_instance_id()
            && axes.consumer == consumer
            && axes.dependency_revision == self.dependency_revision(consumer)?
            && self.is_alive(axes.producer);
        if !key_matches_binding || !binding_matches_graph {
            return Err(SignalError::invalid_input(
                "pending dependency cause binding axes do not match current graph authority",
            ));
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn inject_pending_causes_unchecked_for_test(
        &mut self,
        node: NodeId,
        causes: impl IntoIterator<Item = ResolvedDependencyCause>,
    ) -> Result<PendingCauseSetId, SignalError> {
        let current = self.get_entry(node)?.pending_cause_set_id();
        let id = self.cause_sets.replace_set(current, causes)?;
        self.get_entry_mut(node)?.set_pending_cause_set_id(id);
        self.rebuild_dirty_caches_from_pending_causes(node)?;
        Ok(id)
    }
}
