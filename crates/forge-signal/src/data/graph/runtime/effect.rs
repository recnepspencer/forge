use crate::data::comparator::{ComparatorPolicyResolver, VersionComparatorPolicy};
use crate::data::core_profile::StableHashValue;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::{scope_touched_by_trace, OutputChange};
use crate::data::trace::TraceSummary;
use crate::logic::evaluation::{
    AppliedEffectReport, DeferralReason, EffectComparison, EvaluationEffect, EvaluationVerdict,
    SuppressionReason,
};
use smallvec::SmallVec;

use super::graph::SignalGraph;

impl SignalGraph {
    pub(crate) fn apply_effect(
        &mut self,
        mut effect: EvaluationEffect,
        comparator: VersionComparatorPolicy,
        comparator_resolver: &mut impl ComparatorPolicyResolver,
    ) -> Result<AppliedEffectReport, SignalError> {
        let comparison = self.compare_effect(&effect, comparator)?;
        let trace = self.build_effect_trace(&effect, comparison)?;
        self.transition_effect_state(&mut effect, trace)?;
        self.commit_effect_snapshot(&mut effect)?;
        let suppressed_downstream = self.apply_effect_suppression(
            effect.node,
            &effect.verdict,
            comparison.propagation_suppressed,
            comparator_resolver,
        )?;
        self.record_effect_telemetry(&effect, &comparison, suppressed_downstream);
        Ok(AppliedEffectReport {
            verdict: effect.verdict,
            comparison,
            suppressed_downstream,
        })
    }

    fn compare_effect(
        &self,
        effect: &EvaluationEffect,
        comparator: VersionComparatorPolicy,
    ) -> Result<EffectComparison, SignalError> {
        let previous_trace = self.get_entry(effect.node)?.get_trace_summary();
        let output_identity_unchanged = matches!(
            (
                previous_trace.and_then(|trace| trace.output_identity.as_ref()),
                effect.output_identity.as_ref()
            ),
            (Some(previous), Some(current)) if previous == current
        );
        let continuity_token_unchanged = matches!(
            (
                previous_trace.and_then(|trace| trace.continuity_token.as_ref()),
                effect.continuity_token.as_ref()
            ),
            (Some(previous), Some(current)) if previous == current
        );
        let propagation_suppressed = matches!(effect.verdict, EvaluationVerdict::Suppressed { .. })
            && matches!(comparator, VersionComparatorPolicy::OutputIdentity)
            && output_identity_unchanged;

        Ok(EffectComparison {
            output_identity_unchanged,
            continuity_token_unchanged,
            propagation_suppressed,
            output_change: normalize_output_change(
                effect.output_change,
                output_identity_unchanged,
                effect.output_identity.is_some(),
            ),
            changed_partition_count: count_changed_partitions(&effect.changed_regions),
        })
    }

    fn build_effect_trace(
        &self,
        effect: &EvaluationEffect,
        comparison: EffectComparison,
    ) -> Result<Option<TraceSummary>, SignalError> {
        let trace = match effect.verdict {
            EvaluationVerdict::Recomputed
            | EvaluationVerdict::Suppressed {
                reason:
                    SuppressionReason::OutputIdentityUnchanged
                    | SuppressionReason::ContinuityTokenUnchanged
                    | SuppressionReason::ComparatorMatch,
            } => Some(
                TraceSummary {
                    output_hash: effect
                        .output_identity
                        .as_ref()
                        .map(trace_identity_hash)
                        .unwrap_or_else(|| trace_output_hash(effect.aspect_version)),
                    output_identity: effect.output_identity.clone(),
                    continuity_token: effect.continuity_token.clone(),
                    output_change: comparison.output_change,
                    recomputed: effect.recomputed,
                    dependency_count: effect.dependency_snapshot.entries().len() as u32,
                    meaningful_input_changes: effect.meaningful_input_changes,
                    changed_partition_count: comparison.changed_partition_count,
                    propagation_suppressed: comparison.propagation_suppressed,
                    changed_regions: canonical_changed_regions(&effect.changed_regions),
                    keyed_family: effect
                        .keyed_context
                        .as_ref()
                        .and_then(|keyed| keyed.family.as_ref().map(|family| family.as_str().to_owned())),
                    keyed_key: effect
                        .keyed_context
                        .as_ref()
                        .and_then(|keyed| keyed.key.as_ref().map(|key| key.as_str().to_owned())),
                    memoized_origin: effect.memoized_origin,
                    labels: canonical_labels(&effect.labels),
                    execution_record_id: None,
                    semantic_segment_id: None,
                    lineage_artifact_id: None,
                },
            ),
            EvaluationVerdict::Suppressed {
                reason: SuppressionReason::ValidatedClean | SuppressionReason::ConditionRevertedClean,
            } => None,
            EvaluationVerdict::Deferred { .. } => None,
        };
        Ok(trace)
    }

    fn transition_effect_state(
        &mut self,
        effect: &mut EvaluationEffect,
        trace: Option<TraceSummary>,
    ) -> Result<(), SignalError> {
        let entry = self.get_entry_mut(effect.node)?;
        if let Some(causality) = effect.causality.take() {
            entry.set_causality(Some(causality));
        }
        match effect.verdict {
            EvaluationVerdict::Recomputed
            | EvaluationVerdict::Suppressed {
                reason:
                    SuppressionReason::OutputIdentityUnchanged
                    | SuppressionReason::ContinuityTokenUnchanged
                    | SuppressionReason::ComparatorMatch,
            } => {
                entry.apply_aspect_version(effect.aspect_version, &effect.changed_regions);
                if let Some(trace) = trace {
                    entry.set_trace_summary(Some(trace));
                }
                entry.transition_clean();
            }
            EvaluationVerdict::Suppressed {
                reason: SuppressionReason::ValidatedClean | SuppressionReason::ConditionRevertedClean,
            } => {
                entry.transition_clean();
            }
            EvaluationVerdict::Deferred {
                reason: DeferralReason::ConditionNotMet
                    | DeferralReason::OnDemandNotRequested
                    | DeferralReason::DebounceWindow,
            } => {
                entry.set_state(NodeState::MaybeStale);
            }
        }
        Ok(())
    }

    fn commit_effect_snapshot(&mut self, effect: &mut EvaluationEffect) -> Result<(), SignalError> {
        match effect.verdict {
            EvaluationVerdict::Deferred { .. } => Ok(()),
            EvaluationVerdict::Recomputed => {
                self.set_dep_snapshot(
                    effect.node,
                    std::mem::replace(
                        &mut effect.dependency_snapshot,
                        crate::data::dependency::DependencySnapshot::empty(),
                    ),
                )
            }
            EvaluationVerdict::Suppressed {
                reason:
                    SuppressionReason::OutputIdentityUnchanged
                    | SuppressionReason::ContinuityTokenUnchanged
                    | SuppressionReason::ComparatorMatch,
            } => {
                self.set_dep_snapshot(
                    effect.node,
                    std::mem::replace(
                        &mut effect.dependency_snapshot,
                        crate::data::dependency::DependencySnapshot::empty(),
                    ),
                )
            }
            EvaluationVerdict::Suppressed {
                reason: SuppressionReason::ValidatedClean | SuppressionReason::ConditionRevertedClean,
            } => Ok(()),
        }
    }

    fn apply_effect_suppression(
        &mut self,
        node: NodeId,
        verdict: &EvaluationVerdict,
        propagation_suppressed: bool,
        comparator_resolver: &mut impl ComparatorPolicyResolver,
    ) -> Result<u64, SignalError> {
        if !propagation_suppressed || !matches!(verdict, EvaluationVerdict::Suppressed { .. }) {
            return Ok(0);
        }

        let mut suppressed = 0_u64;
        let mut stack: Vec<NodeId> = self.runtime_subscribers_of(node)?.to_vec();
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
            if matches!(self.get_entry(current)?.get_state(), NodeState::Clean) {
                continue;
            }
            if self.check_upstream_unchanged_ignoring_source(current, node, comparator_resolver)? {
                self.get_entry_mut(current)?.transition_clean();
                suppressed += 1;
                stack.extend_from_slice(self.runtime_subscribers_of(current)?);
            }
        }
        Ok(suppressed)
    }

    fn check_upstream_unchanged_ignoring_source(
        &self,
        node: NodeId,
        ignored_source: NodeId,
        resolver: &mut impl ComparatorPolicyResolver,
    ) -> Result<bool, SignalError> {
        let entry = self.get_entry(node)?;
        let snapshot = self.get_dep_snapshot(node)?;
        let node_cfg = entry.get_eval_config();
        let comparator = resolver.policy_for_node(node, node_cfg.comparator.as_ref());

        for snapshot_entry in snapshot.entries() {
            if snapshot_entry.source == ignored_source {
                if let Some(scope) = &snapshot_entry.scope {
                    if !matches!(self.get_entry(snapshot_entry.source)?.get_state(), NodeState::Clean) {
                        return Ok(false);
                    }
                    if scope_touched_by_trace(
                        self.get_entry(snapshot_entry.source)?.get_trace_summary(),
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
            if !matches!(self.get_entry(snapshot_entry.source)?.get_state(), NodeState::Clean) {
                return Ok(false);
            }
            let current_version = self.get_entry(snapshot_entry.source)?.version_for_scope(
                snapshot_entry.aspect,
                snapshot_entry.scope.as_ref(),
            );
            if let Some(scope) = &snapshot_entry.scope {
                if current_version == snapshot_entry.cached_version {
                    continue;
                }
                if !scope_touched_by_trace(
                    self.get_entry(snapshot_entry.source)?.get_trace_summary(),
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

    fn record_effect_telemetry(
        &mut self,
        effect: &EvaluationEffect,
        comparison: &EffectComparison,
        suppressed_downstream: u64,
    ) {
        match effect.verdict {
            EvaluationVerdict::Recomputed => {
                if effect.recomputed {
                    self.telemetry_mut().evaluation.nodes_recomputed += 1;
                }
            }
            EvaluationVerdict::Suppressed { reason } => {
                match reason {
                    SuppressionReason::ValidatedClean => {
                        self.telemetry_mut().evaluation.skipped_by_comparator += 1;
                    }
                    SuppressionReason::OutputIdentityUnchanged
                    | SuppressionReason::ContinuityTokenUnchanged
                    | SuppressionReason::ComparatorMatch => {
                        self.telemetry_mut().evaluation.output_identity_unchanged_count += 1;
                        self.telemetry_mut().evaluation.suppressed_downstream_propagations +=
                            suppressed_downstream;
                    }
                    SuppressionReason::ConditionRevertedClean => {}
                }
            }
            EvaluationVerdict::Deferred { .. } => {}
        }

        if !effect.changed_regions.is_empty() && effect.recomputed {
            self.telemetry_mut().invalidation.partition_aware_recomputations += 1;
        }

        let _ = comparison;
    }
}

fn canonical_labels(labels: &[String]) -> Vec<String> {
    if labels.len() <= 1 || labels.windows(2).all(|window| window[0] < window[1]) {
        return labels.to_vec();
    }

    let mut canonical = labels.to_vec();
    canonical.sort_unstable();
    canonical.dedup();
    canonical
}

fn canonical_changed_regions(
    changed_regions: &[crate::data::output::ChangedRegion],
) -> Vec<crate::data::output::ChangedRegion> {
    if changed_regions.len() <= 1
        || changed_regions
            .windows(2)
            .all(|window| window[0] < window[1])
    {
        return changed_regions.to_vec();
    }

    let mut canonical = changed_regions.to_vec();
    canonical.sort_unstable();
    canonical.dedup();
    canonical
}

fn count_changed_partitions(changed_regions: &[crate::data::output::ChangedRegion]) -> u32 {
    let mut partitions: SmallVec<[crate::data::output::PartitionToken; 4]> = SmallVec::new();
    for region in changed_regions {
        if partitions.iter().any(|partition| partition == &region.partition) {
            continue;
        }
        partitions.push(region.partition.clone());
    }
    partitions.len() as u32
}

fn normalize_output_change(
    declared: OutputChange,
    output_identity_unchanged: bool,
    has_output_identity: bool,
) -> OutputChange {
    if has_output_identity && output_identity_unchanged {
        OutputChange::Unchanged
    } else {
        declared
    }
}

fn trace_identity_hash(identity: &crate::data::output::OutputIdentity) -> StableHashValue {
    identity.stable_hash()
}

fn trace_output_hash(version: crate::data::aspect::AspectVersion) -> StableHashValue {
    let mut hash = 0xcbf29ce484222325_u128;
    for slot in version.slots() {
        hash ^= *slot as u128;
        hash = hash.wrapping_mul(0x100000001b3_u128);
    }
    hash as StableHashValue
}
