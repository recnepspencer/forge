use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::proof::invalidation::binding::ResolvedDependencyCause;
use crate::data::proof::invalidation::output_commit::ProducedAspectDelta;

use super::{changed_scopes_for_edge, reconcile_edge_cause, CauseAdmissionContext};

#[derive(Debug)]
pub(crate) struct PreparedDirectCauseAdmission {
    producer: NodeId,
    commit: Option<ProducedAspectDelta>,
    replacements: Vec<PreparedConsumerCauseSet>,
    resolved_consumers: Vec<NodeId>,
}

impl PreparedDirectCauseAdmission {
    pub(crate) fn suppressed_downstream_count(&self) -> u64 {
        if self.commit.is_none() {
            return self.resolved_consumers.len() as u64;
        }
        self.replacements
            .iter()
            .filter(|replacement| replacement.causes.is_empty())
            .count() as u64
    }

    pub(crate) fn validate_packet(
        &self,
        producer: NodeId,
        delta: Option<&ProducedAspectDelta>,
    ) -> Result<(), SignalError> {
        if self.producer != producer || self.commit.as_ref() != delta {
            return Err(SignalError::internal(
                "prepared direct causes do not match their output commit",
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
struct PreparedConsumerCauseSet {
    consumer: NodeId,
    causes: Vec<ResolvedDependencyCause>,
}

impl SignalGraph {
    pub(crate) fn prepare_direct_output_causes(
        &mut self,
        delta: &ProducedAspectDelta,
        comparator_resolver: &mut impl ComparatorPolicyResolver,
    ) -> Result<PreparedDirectCauseAdmission, SignalError> {
        self.refresh_runtime_subscribers_of(delta.producer)?;
        let subscribers = self
            .current_runtime_subscribers_of(delta.producer)?
            .to_vec();
        let mut replacements = Vec::with_capacity(subscribers.len());
        for &consumer in &subscribers {
            if let Some(replacement) =
                self.prepare_consumer_cause_set(consumer, delta, comparator_resolver)?
            {
                replacements.push(replacement);
            }
        }
        self.cause_sets.reserve(replacements.len());
        Ok(PreparedDirectCauseAdmission {
            producer: delta.producer,
            commit: Some(delta.clone()),
            replacements,
            resolved_consumers: subscribers,
        })
    }

    pub(crate) fn prepare_stable_output_resolution(
        &mut self,
        producer: NodeId,
    ) -> Result<PreparedDirectCauseAdmission, SignalError> {
        self.refresh_runtime_subscribers_of(producer)?;
        Ok(PreparedDirectCauseAdmission {
            producer,
            commit: None,
            replacements: Vec::new(),
            resolved_consumers: self.current_runtime_subscribers_of(producer)?.to_vec(),
        })
    }

    fn prepare_consumer_cause_set(
        &self,
        consumer: NodeId,
        delta: &ProducedAspectDelta,
        comparator_resolver: &mut impl ComparatorPolicyResolver,
    ) -> Result<Option<PreparedConsumerCauseSet>, SignalError> {
        let relevant = self
            .current_runtime_dependencies_of(consumer)?
            .iter()
            .filter(|edge| edge.source() == delta.producer)
            .cloned()
            .collect::<Vec<_>>();
        if relevant.is_empty() {
            return Ok(None);
        }
        let snapshot = self.get_dep_snapshot(consumer)?;
        let revision = self.dependency_revision(consumer)?;
        let graph_instance = self.runtime_instance_id();
        let config = self.node_eval_config(consumer)?;
        let policy = comparator_resolver.policy_for_node(consumer, config.comparator.as_ref());
        let mut causes = self.pending_causes(consumer)?.to_vec();
        let mut affected = false;

        for edge in relevant {
            let Some(change) = delta
                .changes
                .as_slice()
                .iter()
                .find(|change| change.aspect == edge.aspect())
            else {
                continue;
            };
            let Some(changed_scopes) = changed_scopes_for_edge(change, edge.scope_ref()) else {
                continue;
            };
            affected = true;
            let Some(cached_version) = snapshot
                .entries()
                .iter()
                .find(|entry| {
                    entry.source == delta.producer
                        && entry.aspect == change.aspect
                        && entry.scope.as_ref() == edge.scope_ref()
                })
                .map(|entry| entry.cached_version)
            else {
                continue;
            };
            let meaningful = policy.has_meaningful_change(
                change.aspect,
                cached_version,
                change.committed_version,
                comparator_resolver,
            )?;
            reconcile_edge_cause(
                &mut causes,
                CauseAdmissionContext {
                    graph_instance,
                    consumer,
                    revision,
                    producer: delta.producer,
                    output_commit_ordinal: delta.output_commit_ordinal,
                },
                edge.aspect(),
                edge.scope_ref().cloned(),
                cached_version,
                change.committed_version,
                changed_scopes,
                meaningful,
            );
        }
        Ok(affected.then_some(PreparedConsumerCauseSet { consumer, causes }))
    }

    pub(crate) fn publish_direct_output_causes(
        &mut self,
        prepared: PreparedDirectCauseAdmission,
    ) -> Result<(), SignalError> {
        let PreparedDirectCauseAdmission {
            producer,
            commit,
            replacements,
            resolved_consumers,
        } = prepared;
        for replacement in replacements {
            let preserves_direct_dirty_obligation = self
                .get_entry(replacement.consumer)?
                .direct_invalidation_basis()
                .is_some();
            if preserves_direct_dirty_obligation {
                continue;
            }
            let has_causes = !replacement.causes.is_empty();
            if let Some(delta) = commit.as_ref() {
                self.replace_prepared_pending_causes(
                    replacement.consumer,
                    replacement.causes,
                    delta,
                )?;
            } else {
                self.replace_pending_causes(replacement.consumer, replacement.causes)?;
            }
            let state = if has_causes {
                crate::data::node::NodeState::Dirty
            } else {
                crate::data::node::NodeState::MaybeStale
            };
            self.set_node_state(replacement.consumer, state)?;
        }
        for consumer in resolved_consumers {
            self.resolve_node_pending_revalidation(consumer, producer)?;
        }
        Ok(())
    }
}
