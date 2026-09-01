use crate::data::aspect::AspectMask;
use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::proof::invalidation::binding::ResolvedDependencyCause;
use crate::data::proof::invalidation::output_commit::ProducedAspectDelta;
use crate::data::telemetry::InvalidationPerformedCounter;

use super::{changed_scopes_for_edge, reconcile_edge_cause, CauseAdmissionContext};

#[derive(Debug)]
pub(crate) struct PreparedDirectCauseAdmission {
    producer: NodeId,
    commit: Option<ProducedAspectDelta>,
    replacements: Vec<PreparedConsumerCauseSet>,
    resolved_consumers: Vec<NodeId>,
    counter_deltas: PreparedDirectCounterDeltas,
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

enum DirectCandidateAdmission {
    Admitted(PreparedConsumerCauseSet, PreparedDirectCounterDeltas),
    ContractRejected(PreparedDirectCounterDeltas),
    CausalityRejected(PreparedDirectCounterDeltas),
}

#[derive(Debug, Default)]
struct PreparedDirectCounterDeltas {
    source_deltas: u64,
    edges_examined: u64,
    bucket_probes: u64,
    candidates_returned: u64,
    aspect_contract_rejections: u64,
    scope_rejections: u64,
    comparator_rejections: u64,
    settlements: u64,
}

impl PreparedDirectCounterDeltas {
    fn merge(&mut self, other: Self) {
        self.source_deltas += other.source_deltas;
        self.edges_examined += other.edges_examined;
        self.bucket_probes += other.bucket_probes;
        self.candidates_returned += other.candidates_returned;
        self.aspect_contract_rejections += other.aspect_contract_rejections;
        self.scope_rejections += other.scope_rejections;
        self.comparator_rejections += other.comparator_rejections;
        self.settlements += other.settlements;
    }
}

impl SignalGraph {
    pub(crate) fn prepare_direct_output_causes(
        &mut self,
        delta: &ProducedAspectDelta,
        comparator_resolver: &mut impl ComparatorPolicyResolver,
    ) -> Result<PreparedDirectCauseAdmission, SignalError> {
        let mut subscribers = Vec::new();
        let mut counter_deltas = PreparedDirectCounterDeltas {
            source_deltas: 1,
            ..Default::default()
        };
        for change in delta.changes.as_slice() {
            let query =
                self.query_reverse_subscriptions(delta.producer, change, delta.scope_precision)?;
            counter_deltas.bucket_probes += query.bucket_probes;
            counter_deltas.candidates_returned += query.candidates.len() as u64;
            let candidate_count = query.candidates.len() as u64;
            self.with_telemetry(|telemetry| {
                telemetry.invalidation.direct_subscriber_candidates_examined += candidate_count;
            });
            subscribers.extend(query.candidates);
        }
        subscribers.sort_unstable();
        subscribers.dedup();
        let mut replacements = Vec::with_capacity(subscribers.len());
        let resolved_consumers = self.pending_revalidation_waiters(delta.producer)?;
        for &consumer in &subscribers {
            match self.prepare_consumer_cause_set(consumer, delta, comparator_resolver)? {
                DirectCandidateAdmission::Admitted(replacement, counters) => {
                    counter_deltas.merge(counters);
                    replacements.push(replacement);
                }
                DirectCandidateAdmission::ContractRejected(counters) => {
                    counter_deltas.merge(counters);
                    self.with_telemetry(|telemetry| {
                        telemetry.invalidation.direct_contract_rejections += 1;
                    });
                }
                DirectCandidateAdmission::CausalityRejected(counters) => {
                    counter_deltas.merge(counters);
                    self.with_telemetry(|telemetry| {
                        telemetry.invalidation.direct_causality_rejections += 1;
                    });
                }
            }
        }
        Ok(PreparedDirectCauseAdmission {
            producer: delta.producer,
            commit: Some(delta.clone()),
            replacements,
            resolved_consumers,
            counter_deltas,
        })
    }

    pub(crate) fn prepare_stable_output_resolution(
        &mut self,
        producer: NodeId,
    ) -> Result<PreparedDirectCauseAdmission, SignalError> {
        Ok(PreparedDirectCauseAdmission {
            producer,
            commit: None,
            replacements: Vec::new(),
            resolved_consumers: self.pending_revalidation_waiters(producer)?,
            counter_deltas: Default::default(),
        })
    }

    fn prepare_consumer_cause_set(
        &self,
        consumer: NodeId,
        delta: &ProducedAspectDelta,
        comparator_resolver: &mut impl ComparatorPolicyResolver,
    ) -> Result<DirectCandidateAdmission, SignalError> {
        let relevant = self
            .current_runtime_dependencies_of(consumer)?
            .iter()
            .filter(|edge| {
                edge.source() == delta.producer
                    && delta
                        .changes
                        .as_slice()
                        .iter()
                        .any(|change| change.aspect == edge.aspect())
            })
            .cloned()
            .collect::<Vec<_>>();
        let mut counters = PreparedDirectCounterDeltas {
            edges_examined: relevant.len() as u64,
            ..Default::default()
        };
        if relevant.is_empty() {
            return Ok(DirectCandidateAdmission::CausalityRejected(counters));
        }
        let snapshot = self.get_dep_snapshot(consumer)?;
        let revision = self.dependency_revision(consumer)?;
        let graph_instance = self.runtime_instance_id();
        let config = self.node_eval_config(consumer)?;
        let policy = comparator_resolver.policy_for_node(consumer, config.comparator.as_ref());
        let mut causes = self.pending_causes(consumer)?.to_vec();
        let mut affected = false;
        let mut contract_rejected = false;

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
                counters.scope_rejections += 1;
                continue;
            };
            let changed_aspect = AspectMask::from_aspect(change.aspect);
            let contract = self.get_contract(consumer)?;
            if !contract.cares_about_change(changed_aspect, changed_scopes.as_slice()) {
                contract_rejected = true;
                if contract.cares_about_change(changed_aspect, &[]) {
                    counters.scope_rejections += 1;
                } else {
                    counters.aspect_contract_rejections += 1;
                }
                continue;
            }
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
            if meaningful {
                counters.settlements += 1;
            } else {
                counters.comparator_rejections += 1;
            }
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
        Ok(if affected {
            DirectCandidateAdmission::Admitted(
                PreparedConsumerCauseSet { consumer, causes },
                counters,
            )
        } else if contract_rejected {
            DirectCandidateAdmission::ContractRejected(counters)
        } else {
            DirectCandidateAdmission::CausalityRejected(counters)
        })
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
            counter_deltas,
        } = prepared;
        for replacement in replacements {
            let preserves_direct_dirty_obligation = self
                .node_direct_invalidation_basis(replacement.consumer)?
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
        let performed = self.invalidation_performed_counter_state();
        performed.add(
            InvalidationPerformedCounter::SourceOutputDeltasConsumed,
            counter_deltas.source_deltas,
        );
        performed.add(
            InvalidationPerformedCounter::DirectSubscriberEdgesExamined,
            counter_deltas.edges_examined,
        );
        performed.add(
            InvalidationPerformedCounter::ReverseIndexBucketProbes,
            counter_deltas.bucket_probes,
        );
        performed.add(
            InvalidationPerformedCounter::ReverseIndexCandidatesReturned,
            counter_deltas.candidates_returned,
        );
        performed.add(
            InvalidationPerformedCounter::CandidatesRejectedByAspectContract,
            counter_deltas.aspect_contract_rejections,
        );
        performed.add(
            InvalidationPerformedCounter::CandidatesRejectedByScope,
            counter_deltas.scope_rejections,
        );
        performed.add(
            InvalidationPerformedCounter::CandidatesRejectedByComparator,
            counter_deltas.comparator_rejections,
        );
        performed.add(
            InvalidationPerformedCounter::DirectSettlementsProduced,
            counter_deltas.settlements,
        );
        Ok(())
    }
}
