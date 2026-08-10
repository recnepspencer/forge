use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::scope_touched_by_hot_artifact;
use crate::logic::evaluation::EvaluationVerdict;

use super::SignalGraph;

impl SignalGraph {
    pub(super) fn apply_effect_suppression(
        &mut self,
        node: NodeId,
        _verdict: &EvaluationVerdict,
        propagation_suppressed: bool,
        comparator_resolver: &mut impl ComparatorPolicyResolver,
    ) -> Result<u64, SignalError> {
        if !propagation_suppressed {
            return Ok(0);
        }

        let mut suppressed = 0_u64;
        let mut stack = std::mem::take(&mut self.traversal.topology_node_buffer);
        stack.clear();
        self.refresh_runtime_subscribers_of(node)?;
        stack.extend_from_slice(self.current_runtime_subscribers_of(node)?);
        self.traversal
            .suppression_marks
            .ensure_len(self.arena_capacity());
        self.traversal.suppression_marks.clear_all();
        while let Some(current) = stack.pop() {
            if !self.is_alive(current) {
                continue;
            }
            if !self
                .traversal
                .suppression_marks
                .mark(current.index() as usize)
            {
                continue;
            }
            if matches!(self.get_state(current)?, NodeState::Clean) {
                continue;
            }
            if self.check_upstream_unchanged_ignoring_source(current, node, comparator_resolver)? {
                self.transition_node_clean(current)?;
                suppressed += 1;
                self.refresh_runtime_subscribers_of(current)?;
                stack.extend_from_slice(self.current_runtime_subscribers_of(current)?);
            }
        }
        self.traversal.topology_node_buffer = stack;
        Ok(suppressed)
    }

    pub(super) fn check_upstream_unchanged_ignoring_source(
        &self,
        node: NodeId,
        ignored_source: NodeId,
        resolver: &mut impl ComparatorPolicyResolver,
    ) -> Result<bool, SignalError> {
        let snapshot = self.get_dep_snapshot(node)?;
        let node_cfg = self.node_eval_config(node)?;
        let comparator = resolver.policy_for_node(node, node_cfg.comparator.as_ref());

        for snapshot_entry in snapshot.entries() {
            if snapshot_entry.source == ignored_source {
                if let Some(scope) = &snapshot_entry.scope {
                    if !matches!(self.get_state(snapshot_entry.source)?, NodeState::Clean) {
                        return Ok(false);
                    }
                    if scope_touched_by_hot_artifact(
                        self.node_runtime_artifact_hot(snapshot_entry.source)?,
                        scope,
                    ) {
                        return Ok(false);
                    }
                }
                continue;
            }
            if !self.is_alive(snapshot_entry.source) {
                return Ok(false);
            }
            if !matches!(self.get_state(snapshot_entry.source)?, NodeState::Clean) {
                return Ok(false);
            }
            let current_version = self.node_version_for_scope(
                snapshot_entry.source,
                snapshot_entry.aspect,
                snapshot_entry.scope.as_ref(),
            )?;
            if let Some(scope) = &snapshot_entry.scope {
                if current_version == snapshot_entry.cached_version {
                    continue;
                }
                if !scope_touched_by_hot_artifact(
                    self.node_runtime_artifact_hot(snapshot_entry.source)?,
                    scope,
                ) {
                    continue;
                }
                return Ok(false);
            }
            if comparator.has_meaningful_change(
                snapshot_entry.aspect,
                snapshot_entry.cached_version,
                current_version,
                resolver,
            )? {
                return Ok(false);
            }
        }

        Ok(true)
    }
}
