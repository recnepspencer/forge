use crate::data::comparator::{ComparatorPolicyResolver, VersionComparatorPolicy};
use crate::data::core_profile::StableHashValue;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::{scope_touched_by_artifact_state, CanonicalChangedRegions, OutputChange};
use crate::data::proof::PartitionScopeSet;
use crate::data::trace::{RetainedDiagnosticArtifact, RuntimeArtifactState};
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
        defer_snapshot_commit: bool,
    ) -> Result<AppliedEffectReport, SignalError> {
        let comparison = self.compare_effect(&effect, comparator)?;
        let trace = self.build_effect_trace(&effect, comparison)?;
        self.transition_effect_state(&mut effect, trace)?;
        if !defer_snapshot_commit {
            self.commit_effect_snapshot(&mut effect)?;
        }
        let suppressed_downstream = self.apply_effect_suppression(
            effect.operational.node,
            &effect.operational.verdict,
            comparison.propagation_suppressed,
            comparator_resolver,
        )?;
        self.record_effect_telemetry(&effect, &comparison, suppressed_downstream);
        Ok(AppliedEffectReport {
            verdict: effect.operational.verdict,
            comparison,
            suppressed_downstream,
        })
    }

    fn compare_effect(
        &self,
        effect: &EvaluationEffect,
        comparator: VersionComparatorPolicy,
    ) -> Result<EffectComparison, SignalError> {
        let previous_trace = self
            .get_entry(effect.operational.node)?
            .get_runtime_artifact_state();
        let output_identity_unchanged = matches!(
            (
                previous_trace.and_then(|trace| trace.output_identity.as_ref()),
                effect.output_identity()
            ),
            (Some(previous), Some(current)) if previous == current
        );
        let continuity_token_unchanged = matches!(
            (
                previous_trace.and_then(|trace| trace.continuity_token.as_ref()),
                effect.continuity_token()
            ),
            (Some(previous), Some(current)) if previous == current
        );
        let propagation_suppressed =
            matches!(
                effect.operational.verdict,
                EvaluationVerdict::Suppressed { .. }
            ) && matches!(comparator, VersionComparatorPolicy::OutputIdentity)
                && output_identity_unchanged;

        Ok(EffectComparison {
            output_identity_unchanged,
            continuity_token_unchanged,
            propagation_suppressed,
            output_change: normalize_output_change(
                effect.operational.output_change,
                output_identity_unchanged,
                effect.output_identity().is_some(),
            ),
            changed_partition_count: count_changed_partitions(effect.changed_regions()),
        })
    }

    fn build_effect_trace(
        &self,
        effect: &EvaluationEffect,
        comparison: EffectComparison,
    ) -> Result<Option<(RuntimeArtifactState, Option<RetainedDiagnosticArtifact>)>, SignalError>
    {
        let trace = match effect.operational.verdict {
            EvaluationVerdict::Recomputed
            | EvaluationVerdict::Suppressed {
                reason:
                    SuppressionReason::OutputIdentityUnchanged
                    | SuppressionReason::ContinuityTokenUnchanged
                    | SuppressionReason::ComparatorMatch,
            } => Some((
                RuntimeArtifactState {
                    output_hash: effect
                        .output_identity()
                        .map(trace_identity_hash)
                        .unwrap_or_else(|| trace_output_hash(effect.operational.aspect_version)),
                    output_identity: effect.output_identity().cloned(),
                    continuity_token: effect.continuity_token().cloned(),
                    output_change: comparison.output_change,
                    recomputed: effect.recomputed(),
                    dependency_count: effect.operational.dependency_snapshot_update.entry_count()
                        as u32,
                    meaningful_input_changes: effect.operational.meaningful_input_changes,
                    changed_partition_count: comparison.changed_partition_count,
                    propagation_suppressed: comparison.propagation_suppressed,
                    changed_scopes: PartitionScopeSet::from_changed_regions(
                        &CanonicalChangedRegions::from_slice(effect.changed_regions()),
                    ),
                    memoized_origin: effect.memoized_origin(),
                    reuse_basis: effect.operational.reuse_basis,
                    reuse_boundary_context: Some(effect.operational.reuse_boundary_context.clone()),
                    execution_record_id: None,
                    semantic_segment_id: None,
                    lineage_artifact_id: None,
                },
                {
                    let retained = RetainedDiagnosticArtifact {
                        changed_regions: CanonicalChangedRegions::from_slice(
                            effect.changed_regions(),
                        ),
                        labels: effect.labels().to_vec(),
                        keyed_family: effect.keyed_context().and_then(|keyed| {
                            keyed
                                .family
                                .as_ref()
                                .map(|family| family.as_str().to_owned())
                        }),
                        keyed_key: effect.keyed_context().and_then(|keyed| {
                            keyed.key.as_ref().map(|key| key.as_str().to_owned())
                        }),
                        reuse_certification: effect.reuse_certification().cloned(),
                    };
                    if retained.changed_regions.is_empty()
                        && retained.labels.is_empty()
                        && retained.keyed_family.is_none()
                        && retained.keyed_key.is_none()
                        && retained.reuse_certification.is_none()
                    {
                        None
                    } else {
                        Some(retained)
                    }
                },
            )),
            EvaluationVerdict::Suppressed {
                reason:
                    SuppressionReason::ValidatedClean | SuppressionReason::ConditionRevertedClean,
            } => None,
            EvaluationVerdict::Deferred { .. } => None,
        };
        Ok(trace)
    }

    fn transition_effect_state(
        &mut self,
        effect: &mut EvaluationEffect,
        artifact: Option<(RuntimeArtifactState, Option<RetainedDiagnosticArtifact>)>,
    ) -> Result<(), SignalError> {
        let entry = self.get_entry_mut(effect.operational.node)?;
        if let Some(causality) = effect.take_causality() {
            entry.set_causality(Some(causality));
        }
        match effect.operational.verdict {
            EvaluationVerdict::Recomputed
            | EvaluationVerdict::Suppressed {
                reason:
                    SuppressionReason::OutputIdentityUnchanged
                    | SuppressionReason::ContinuityTokenUnchanged
                    | SuppressionReason::ComparatorMatch,
            } => {
                entry.apply_aspect_version(
                    effect.operational.aspect_version,
                    effect.changed_regions(),
                );
                if let Some((runtime_artifact_state, retained_artifact)) = artifact {
                    entry.set_runtime_artifact_state(Some(runtime_artifact_state));
                    entry.set_retained_diagnostic_artifact(retained_artifact);
                }
                entry.transition_clean();
            }
            EvaluationVerdict::Suppressed {
                reason:
                    SuppressionReason::ValidatedClean | SuppressionReason::ConditionRevertedClean,
            } => {
                entry.transition_clean();
            }
            EvaluationVerdict::Deferred {
                reason:
                    DeferralReason::ConditionNotMet
                    | DeferralReason::OnDemandNotRequested
                    | DeferralReason::DebounceWindow,
            } => {
                entry.set_state(NodeState::MaybeStale);
            }
        }
        Ok(())
    }

    fn commit_effect_snapshot(&mut self, effect: &mut EvaluationEffect) -> Result<(), SignalError> {
        if !effect.operational.snapshot_delta.changed() {
            return Ok(());
        }
        match effect.operational.verdict {
            EvaluationVerdict::Deferred { .. } => Ok(()),
            EvaluationVerdict::Recomputed => self
                .replace_dep_snapshot_shared(
                    effect.operational.node,
                    std::mem::replace(
                        &mut effect.operational.dependency_snapshot_update,
                        crate::data::dependency::DependencySnapshotUpdate::Replace(
                            crate::data::dependency::SharedDependencySnapshot::empty(),
                        ),
                    ),
                )
                .map(|_| ()),
            EvaluationVerdict::Suppressed {
                reason:
                    SuppressionReason::OutputIdentityUnchanged
                    | SuppressionReason::ContinuityTokenUnchanged
                    | SuppressionReason::ComparatorMatch,
            } => self
                .replace_dep_snapshot_shared(
                    effect.operational.node,
                    std::mem::replace(
                        &mut effect.operational.dependency_snapshot_update,
                        crate::data::dependency::DependencySnapshotUpdate::Replace(
                            crate::data::dependency::SharedDependencySnapshot::empty(),
                        ),
                    ),
                )
                .map(|_| ()),
            EvaluationVerdict::Suppressed {
                reason:
                    SuppressionReason::ValidatedClean | SuppressionReason::ConditionRevertedClean,
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
                    if !matches!(
                        self.get_entry(snapshot_entry.source)?.get_state(),
                        NodeState::Clean
                    ) {
                        return Ok(false);
                    }
                    if scope_touched_by_artifact_state(
                        self.get_entry(snapshot_entry.source)?
                            .get_runtime_artifact_state(),
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
            if !matches!(
                self.get_entry(snapshot_entry.source)?.get_state(),
                NodeState::Clean
            ) {
                return Ok(false);
            }
            let current_version = self
                .get_entry(snapshot_entry.source)?
                .version_for_scope(snapshot_entry.aspect, snapshot_entry.scope.as_ref());
            if let Some(scope) = &snapshot_entry.scope {
                if current_version == snapshot_entry.cached_version {
                    continue;
                }
                if !scope_touched_by_artifact_state(
                    self.get_entry(snapshot_entry.source)?
                        .get_runtime_artifact_state(),
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
        match effect.operational.verdict {
            EvaluationVerdict::Recomputed => {
                if effect.recomputed() {
                    self.telemetry_mut().evaluation.nodes_recomputed += 1;
                }
            }
            EvaluationVerdict::Suppressed { reason } => match reason {
                SuppressionReason::ValidatedClean => {
                    self.telemetry_mut().evaluation.skipped_by_comparator += 1;
                }
                SuppressionReason::OutputIdentityUnchanged
                | SuppressionReason::ContinuityTokenUnchanged
                | SuppressionReason::ComparatorMatch => {
                    self.telemetry_mut()
                        .evaluation
                        .output_identity_unchanged_count += 1;
                    self.telemetry_mut()
                        .evaluation
                        .suppressed_downstream_propagations += suppressed_downstream;
                }
                SuppressionReason::ConditionRevertedClean => {}
            },
            EvaluationVerdict::Deferred { .. } => {}
        }

        if !effect.changed_regions().is_empty() && effect.recomputed() {
            self.telemetry_mut()
                .invalidation
                .partition_aware_recomputations += 1;
        }

        let _ = comparison;
    }
}

fn count_changed_partitions(changed_regions: &[crate::data::output::ChangedRegion]) -> u32 {
    let mut partitions: SmallVec<[crate::data::output::PartitionToken; 4]> = SmallVec::new();
    for region in changed_regions {
        if partitions
            .iter()
            .any(|partition| partition == &region.partition)
        {
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
