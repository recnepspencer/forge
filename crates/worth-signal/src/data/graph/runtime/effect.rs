use crate::clock::RuntimeInstant;
use crate::data::comparator::{ComparatorPolicyResolver, VersionComparatorPolicy};
use crate::data::core_profile::StableHashValue;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::{scope_touched_by_hot_artifact, CanonicalChangedRegions, OutputChange};
use crate::data::proof::PartitionScopeSet;
use crate::data::trace::{
    ArtifactMergeAuthority, ArtifactTransitionKey, ArtifactWriteDelta, ColdArtifactIntent,
    CompactChangedScopeProof, ContinuityAuthorityToken, HotArtifactWrite, ReuseOperationalBasis,
    RuntimeArtifactHot, RuntimeArtifactState, RuntimeArtifactWarm,
    COLD_ARTIFACT_INTENT_LABEL_LIMIT,
};
use crate::diagnostics::policy::ArtifactRetentionPolicy;
use crate::logic::evaluation::{
    AppliedEffectReport, DeferralReason, EffectComparison, EvaluationEffect, EvaluationVerdict,
    SuppressionReason,
};
use smallvec::SmallVec;

use super::graph::{RuntimeArtifactStructuralDelta, SignalGraph};

#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
#[derive(Debug)]
pub(crate) struct ApplyCommitPacket {
    pub(crate) effect: EvaluationEffect,
    pub(crate) comparison: EffectComparison,
    pub(crate) artifact_write: Option<HotArtifactWrite>,
    pub(crate) pending_snapshot: Option<crate::logic::evaluation::PendingDependencySnapshot>,
    pub(crate) defer_snapshot_commit: bool,
}

#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
#[derive(Debug)]
pub(crate) struct SuppressionFreeApplyCommitPacket(ApplyCommitPacket);

impl TryFrom<ApplyCommitPacket> for SuppressionFreeApplyCommitPacket {
    type Error = SignalError;

    fn try_from(packet: ApplyCommitPacket) -> Result<Self, Self::Error> {
        if packet.comparison.propagation_suppressed {
            return Err(SignalError::internal(
                "grouped concurrent commit packet unexpectedly required shared suppression",
            ));
        }
        Ok(Self(packet))
    }
}

impl SignalGraph {
    pub(crate) fn apply_effect(
        &mut self,
        mut effect: EvaluationEffect,
        comparator: VersionComparatorPolicy,
        comparator_resolver: &mut impl ComparatorPolicyResolver,
        defer_snapshot_commit: bool,
    ) -> Result<
        (
            AppliedEffectReport,
            Option<crate::logic::evaluation::PendingDependencySnapshot>,
        ),
        SignalError,
    > {
        let previous_warm = if let Some(snapshot) = effect.previous_artifact_warm().cloned() {
            Some(snapshot)
        } else {
            self.node_runtime_artifact_reuse_boundary_snapshot(effect.operational.node)?
                .map(
                    |trace| crate::logic::evaluation::PreviousArtifactWarmSnapshot {
                        output_identity: trace.output_identity,
                        continuity_token: trace.continuity_token,
                        reuse_boundary_authority: trace.reuse_boundary_authority,
                    },
                )
        };
        let comparison = self.compare_effect(&effect, previous_warm.as_ref(), comparator)?;
        let artifact_write =
            self.build_effect_artifact_write(&effect, previous_warm.as_ref(), comparison)?;
        let pending_snapshot = if defer_snapshot_commit {
            let snapshot_start = RuntimeInstant::now();
            let pending = crate::logic::evaluation::PendingDependencySnapshot {
                node: effect.operational.node,
                update: std::mem::replace(
                    &mut effect.operational.dependency_snapshot_update,
                    crate::data::dependency::CommittedSnapshotUpdate::Replace(
                        crate::data::dependency::ReplacementSnapshotUpdate::from_snapshot(
                            crate::data::dependency::DependencySnapshot::empty(),
                            &mut self.topology.dependency_snapshot_shapes,
                        ),
                    ),
                ),
                delta: effect.operational.snapshot_delta,
            };
            self.telemetry_mut()
                .execution
                .deferred_snapshot_packet_nanos += snapshot_start.elapsed().as_nanos();
            Some(pending)
        } else {
            None
        };
        self.transition_effect_state(&mut effect, artifact_write)?;
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
        Ok((
            AppliedEffectReport {
                verdict: effect.operational.verdict,
                comparison,
                suppressed_downstream,
                temporal_eligibility: None,
            },
            pending_snapshot,
        ))
    }

    #[cfg_attr(not(feature = "parallel"), allow(dead_code))]
    pub(crate) fn build_apply_commit_packet(
        &self,
        effect: EvaluationEffect,
        comparator: VersionComparatorPolicy,
        defer_snapshot_commit: bool,
    ) -> Result<ApplyCommitPacket, SignalError> {
        let previous_warm = if let Some(snapshot) = effect.previous_artifact_warm().cloned() {
            Some(snapshot)
        } else {
            self.node_runtime_artifact_reuse_boundary_snapshot(effect.operational.node)?
                .map(
                    |trace| crate::logic::evaluation::PreviousArtifactWarmSnapshot {
                        output_identity: trace.output_identity,
                        continuity_token: trace.continuity_token,
                        reuse_boundary_authority: trace.reuse_boundary_authority,
                    },
                )
        };
        let comparison = self.compare_effect(&effect, previous_warm.as_ref(), comparator)?;
        let artifact_write =
            self.build_effect_artifact_write(&effect, previous_warm.as_ref(), comparison)?;
        let pending_snapshot = if defer_snapshot_commit {
            Some(crate::logic::evaluation::PendingDependencySnapshot {
                node: effect.operational.node,
                update: effect.operational.dependency_snapshot_update.clone(),
                delta: effect.operational.snapshot_delta,
            })
        } else {
            None
        };
        Ok(ApplyCommitPacket {
            effect,
            comparison,
            artifact_write,
            pending_snapshot,
            defer_snapshot_commit,
        })
    }

    #[cfg_attr(not(feature = "parallel"), allow(dead_code))]
    pub(crate) fn publish_suppression_free_apply_commit_packet(
        &mut self,
        packet: SuppressionFreeApplyCommitPacket,
    ) -> Result<
        (
            AppliedEffectReport,
            Option<crate::logic::evaluation::PendingDependencySnapshot>,
        ),
        SignalError,
    > {
        let ApplyCommitPacket {
            mut effect,
            comparison,
            artifact_write,
            pending_snapshot,
            defer_snapshot_commit,
        } = packet.0;
        self.transition_effect_state(&mut effect, artifact_write)?;
        if !defer_snapshot_commit {
            self.commit_effect_snapshot(&mut effect)?;
        }
        self.record_effect_telemetry(&effect, &comparison, 0);
        Ok((
            AppliedEffectReport {
                verdict: effect.operational.verdict,
                comparison,
                suppressed_downstream: 0,
                temporal_eligibility: None,
            },
            pending_snapshot,
        ))
    }

    fn compare_effect(
        &self,
        effect: &EvaluationEffect,
        previous_trace: Option<&crate::logic::evaluation::PreviousArtifactWarmSnapshot>,
        comparator: VersionComparatorPolicy,
    ) -> Result<EffectComparison, SignalError> {
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
        let propagation_suppressed = matches!(comparator, VersionComparatorPolicy::OutputIdentity)
            && output_identity_unchanged
            && !matches!(
                effect.operational.verdict,
                EvaluationVerdict::Deferred { .. }
                    | EvaluationVerdict::Suppressed {
                        reason: SuppressionReason::ValidatedClean
                            | SuppressionReason::ConditionRevertedClean,
                    }
            );

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

    fn build_effect_artifact_write(
        &self,
        effect: &EvaluationEffect,
        previous_warm: Option<&crate::logic::evaluation::PreviousArtifactWarmSnapshot>,
        comparison: EffectComparison,
    ) -> Result<Option<HotArtifactWrite>, SignalError> {
        if !verdict_retains_runtime_artifact(&effect.operational.verdict) {
            return Ok(None);
        }
        let previous_hot = self.node_runtime_artifact_hot(effect.operational.node)?;
        let cold_intent =
            build_cold_artifact_intent(effect, &self.runtime_policy().retention_budget);
        let write = Some(HotArtifactWrite {
            runtime: Some(RuntimeArtifactState::new(
                RuntimeArtifactHot {
                    output_hash: if matches!(
                        effect.operational.verdict,
                        EvaluationVerdict::Recomputed
                    ) {
                        effect
                            .output_identity()
                            .map(trace_identity_hash)
                            .unwrap_or_else(|| trace_output_hash(effect.operational.aspect_version))
                    } else {
                        previous_hot
                            .map(|trace| trace.output_hash)
                            .unwrap_or_else(|| trace_output_hash(effect.operational.aspect_version))
                    },
                    output_change: comparison.output_change,
                    recomputed: effect.recomputed(),
                    dependency_count: effect.operational.dependency_snapshot_update.entry_count()
                        as u32,
                    meaningful_input_changes: effect.operational.meaningful_input_changes,
                    changed_partition_count: comparison.changed_partition_count,
                    propagation_suppressed: comparison.propagation_suppressed,
                    changed_scopes: CompactChangedScopeProof::new(
                        PartitionScopeSet::from_changed_regions(
                            &CanonicalChangedRegions::from_slice(effect.changed_regions()),
                        ),
                    ),
                },
                RuntimeArtifactWarm {
                    output_identity: if matches!(
                        effect.operational.verdict,
                        EvaluationVerdict::Recomputed
                    ) {
                        effect.output_identity().cloned()
                    } else {
                        previous_warm
                            .and_then(|trace| trace.output_identity.clone())
                            .or_else(|| effect.output_identity().cloned())
                    },
                    continuity_token: ContinuityAuthorityToken::new(
                        if matches!(effect.operational.verdict, EvaluationVerdict::Recomputed) {
                            effect.continuity_token().cloned()
                        } else {
                            previous_warm
                                .and_then(|trace| trace.continuity_token.clone())
                                .or_else(|| effect.continuity_token().cloned())
                        },
                    ),
                    memoized_origin: effect.memoized_origin(),
                    reuse_basis: ReuseOperationalBasis::new(effect.operational.reuse_basis.clone()),
                    reuse_origin: effect.operational.reuse_origin,
                    reuse_boundary_authority: Some(
                        effect.operational.reuse_boundary_authority.clone(),
                    ),
                    lineage_artifact_id: ArtifactTransitionKey::default(),
                    merge_authority: ArtifactMergeAuthority::default(),
                },
            )),
            cold_intent,
        });
        Ok(write)
    }

    fn transition_effect_state(
        &mut self,
        effect: &mut EvaluationEffect,
        artifact_write: Option<HotArtifactWrite>,
    ) -> Result<(), SignalError> {
        let node = effect.operational.node;
        let mut runtime_artifact_delta = None;
        let mut retained_artifact_changed = false;
        let mut state_changed = false;
        let mut retained_artifact = None;
        let mut runtime_artifact_state = None;
        if let Some(write) = artifact_write {
            self.telemetry_mut()
                .storage
                .hot_write_runtime_artifact_count += u64::from(write.runtime.is_some());
            runtime_artifact_state = write.runtime;
            if write.cold_intent.is_none() && runtime_policy_omits_cold_artifacts(self) {
                self.telemetry_mut().storage.hot_write_cold_bypass_count += 1;
                self.telemetry_mut()
                    .storage
                    .deferred_cold_artifact_bypass_count += 1;
            } else {
                retained_artifact = self.materialize_retained_artifact(write.cold_intent);
            }
        }
        {
            let (previous_artifact_id, previous_output_hash, previous_reuse_basis) =
                self.node_runtime_artifact_structural_state(node)?;
            let previous_state = self.node_state(node)?;
            if let Some(causality) = effect.take_causality() {
                self.set_causality(node, Some(causality))?;
            }
            if matches!(effect.operational.verdict, EvaluationVerdict::Recomputed) {
                self.apply_node_aspect_version(
                    node,
                    effect.operational.aspect_version,
                    effect.changed_regions(),
                )?;
            }
            if verdict_retains_runtime_artifact(&effect.operational.verdict) {
                if let Some(runtime_artifact_state) = runtime_artifact_state {
                    runtime_artifact_delta = Some(RuntimeArtifactStructuralDelta {
                        previous_artifact_id,
                        next_artifact_id: runtime_artifact_state.lineage_artifact_id().get(),
                        previous_output_hash,
                        next_output_hash: Some(runtime_artifact_state.output_hash()),
                        previous_reuse_basis: if matches!(
                            effect.operational.verdict,
                            EvaluationVerdict::Recomputed
                        ) {
                            previous_reuse_basis.clone()
                        } else {
                            previous_reuse_basis
                        },
                        next_reuse_basis: Some(runtime_artifact_state.reuse_basis().clone_inner()),
                    });
                    retained_artifact_changed = self.apply_node_artifact_write_delta(
                        node,
                        ArtifactWriteDelta {
                            runtime: Some(runtime_artifact_state),
                            retained: retained_artifact,
                        },
                    )?;
                }
            }
            if verdict_transitions_clean(&effect.operational.verdict) {
                self.transition_node_clean(node)?;
                state_changed = !matches!(previous_state, NodeState::Clean);
            } else {
                match effect.operational.verdict {
                    EvaluationVerdict::Deferred {
                        reason:
                            DeferralReason::ConditionNotMet
                            | DeferralReason::OnDemandNotRequested
                            | DeferralReason::DebounceWindow
                            | DeferralReason::TemporalConditionNotMet,
                    } => {
                        self.set_node_state(node, NodeState::MaybeStale)?;
                        state_changed = !matches!(previous_state, NodeState::MaybeStale);
                    }
                    _ => {}
                }
            }
        }
        if let Some(delta) = runtime_artifact_delta {
            self.record_branch_mutation_runtime_artifact(node, delta);
        }
        if retained_artifact_changed {
            self.record_branch_mutation_retained_artifact(node);
        }
        if state_changed {
            self.record_branch_mutation_state(node);
        }
        Ok(())
    }

    fn materialize_retained_artifact(
        &mut self,
        cold_intent: Option<ColdArtifactIntent>,
    ) -> Option<crate::data::trace::ColdArtifactRecord> {
        let Some(cold_intent) = cold_intent else {
            return None;
        };
        let policy = self.runtime_policy().retention_budget;
        if matches!(
            policy.explanation_retention,
            ArtifactRetentionPolicy::Retain
        ) || matches!(policy.provenance_retention, ArtifactRetentionPolicy::Retain)
        {
            let retained = cold_intent.materialize_record();
            if retained.is_some() {
                self.telemetry_mut()
                    .storage
                    .hot_write_cold_record_materialization_count += 1;
                self.telemetry_mut()
                    .storage
                    .eager_cold_artifact_materialization_count += 1;
            }
            retained
        } else {
            self.telemetry_mut().storage.hot_write_cold_bypass_count += 1;
            self.telemetry_mut()
                .storage
                .deferred_cold_artifact_bypass_count += 1;
            None
        }
    }

    fn commit_effect_snapshot(&mut self, effect: &mut EvaluationEffect) -> Result<(), SignalError> {
        if !effect.operational.snapshot_delta.changed() {
            return Ok(());
        }
        if verdict_commits_snapshot(&effect.operational.verdict) {
            let placeholder_update = crate::data::dependency::CommittedSnapshotUpdate::Replace(
                crate::data::dependency::ReplacementSnapshotUpdate::from_snapshot(
                    crate::data::dependency::DependencySnapshot::empty(),
                    &mut self.topology.dependency_snapshot_shapes,
                ),
            );
            return self
                .replace_dep_snapshot_committed(
                    effect.operational.node,
                    std::mem::replace(
                        &mut effect.operational.dependency_snapshot_update,
                        placeholder_update,
                    ),
                )
                .map(|_| ());
        }
        Ok(())
    }

    fn apply_effect_suppression(
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

    fn check_upstream_unchanged_ignoring_source(
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

    fn record_effect_telemetry(
        &mut self,
        effect: &EvaluationEffect,
        comparison: &EffectComparison,
        suppressed_downstream: u64,
    ) {
        let retains_cold_artifacts = self.retains_runtime_cold_artifacts();
        match effect.operational.verdict {
            EvaluationVerdict::Recomputed => {
                if effect.recomputed() {
                    self.telemetry_mut().evaluation.nodes_recomputed += 1;
                }
                if comparison.propagation_suppressed {
                    self.telemetry_mut()
                        .evaluation
                        .output_identity_unchanged_count += 1;
                    self.telemetry_mut()
                        .evaluation
                        .suppressed_downstream_propagations += suppressed_downstream;
                }
                record_reuse_telemetry(self.telemetry_mut(), effect);
                if retains_cold_artifacts {
                    self.telemetry_mut()
                        .storage
                        .hot_path_artifact_retention_count += 1;
                }
            }
            EvaluationVerdict::Suppressed { reason } => match reason {
                SuppressionReason::ValidatedClean => {
                    self.telemetry_mut().evaluation.skipped_by_comparator += 1;
                }
                SuppressionReason::ComparatorMatch => {
                    self.telemetry_mut().evaluation.skipped_by_comparator += 1;
                    if retains_cold_artifacts {
                        self.telemetry_mut()
                            .storage
                            .hot_path_artifact_retention_count += 1;
                    }
                    record_reuse_telemetry(self.telemetry_mut(), effect);
                }
                SuppressionReason::OutputIdentityUnchanged
                | SuppressionReason::ContinuityTokenUnchanged => {
                    if retains_cold_artifacts {
                        self.telemetry_mut()
                            .storage
                            .hot_path_artifact_retention_count += 1;
                    }
                    self.telemetry_mut()
                        .evaluation
                        .output_identity_unchanged_count += 1;
                    self.telemetry_mut()
                        .evaluation
                        .suppressed_downstream_propagations += suppressed_downstream;
                    record_reuse_telemetry(self.telemetry_mut(), effect);
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

    fn retains_runtime_cold_artifacts(&self) -> bool {
        let retention = self.runtime_policy().retention_budget;
        matches!(
            retention.explanation_retention,
            ArtifactRetentionPolicy::Retain
        ) || matches!(
            retention.provenance_retention,
            ArtifactRetentionPolicy::Retain
        )
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

fn verdict_retains_runtime_artifact(verdict: &EvaluationVerdict) -> bool {
    matches!(
        verdict,
        EvaluationVerdict::Recomputed
            | EvaluationVerdict::Suppressed {
                reason: SuppressionReason::OutputIdentityUnchanged
                    | SuppressionReason::ContinuityTokenUnchanged
                    | SuppressionReason::ComparatorMatch,
            }
    )
}

fn verdict_transitions_clean(verdict: &EvaluationVerdict) -> bool {
    matches!(
        verdict,
        EvaluationVerdict::Recomputed | EvaluationVerdict::Suppressed { .. }
    )
}

fn verdict_commits_snapshot(verdict: &EvaluationVerdict) -> bool {
    verdict_retains_runtime_artifact(verdict)
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

fn build_cold_artifact_intent(
    effect: &EvaluationEffect,
    retention: &crate::diagnostics::policy::RetentionBudget,
) -> Option<ColdArtifactIntent> {
    if matches!(
        retention.explanation_retention,
        ArtifactRetentionPolicy::Omit
    ) && matches!(
        retention.provenance_retention,
        ArtifactRetentionPolicy::Omit
    ) {
        return None;
    }
    let retain_reuse_boundary_detail = matches!(
        effect.operational.reuse_basis.strategy,
        Some(crate::data::reuse::ReuseStrategy::CrossIdentityPersistentMatch)
            | Some(crate::data::reuse::ReuseStrategy::PartialArtifactSplicing)
    );
    let labels = if matches!(
        retention.explanation_retention,
        ArtifactRetentionPolicy::Retain
    ) || matches!(
        retention.provenance_retention,
        ArtifactRetentionPolicy::Retain
    ) {
        effect
            .labels()
            .iter()
            .take(COLD_ARTIFACT_INTENT_LABEL_LIMIT)
            .cloned()
            .collect()
    } else {
        SmallVec::new()
    };
    let intent = ColdArtifactIntent {
        changed_regions: CanonicalChangedRegions::from_slice(effect.changed_regions()),
        labels,
        keyed_family: effect.keyed_context().and_then(|keyed| {
            keyed
                .family
                .as_ref()
                .map(|family| family.as_str().to_owned())
        }),
        keyed_key: effect
            .keyed_context()
            .and_then(|keyed| keyed.key.as_ref().map(|key| key.as_str().to_owned())),
        reuse_certification: effect.reuse_certification().cloned(),
        reuse_boundary_context: retain_reuse_boundary_detail
            .then(|| effect.reuse_boundary_detail().cloned())
            .flatten(),
    };
    (!intent.is_empty()).then_some(intent)
}

fn runtime_policy_omits_cold_artifacts(graph: &SignalGraph) -> bool {
    let retention = graph.runtime_policy().retention_budget;
    matches!(
        retention.explanation_retention,
        ArtifactRetentionPolicy::Omit
    ) && matches!(
        retention.provenance_retention,
        ArtifactRetentionPolicy::Omit
    )
}

fn record_reuse_telemetry(
    telemetry: &mut crate::data::telemetry::RuntimeTelemetry,
    effect: &EvaluationEffect,
) {
    telemetry.evaluation.reuse_eligibility_checks_attempted += 1;
    match effect.operational.reuse_origin {
        crate::data::reuse::ReuseOrigin::FreshCompute => {
            telemetry.evaluation.fresh_compute_count += 1
        }
        crate::data::reuse::ReuseOrigin::OutputSuppressed => {
            telemetry.evaluation.output_suppressed_count += 1
        }
        crate::data::reuse::ReuseOrigin::MemoizedArtifactReuse => {
            telemetry.evaluation.memoized_reuse_count += 1
        }
        crate::data::reuse::ReuseOrigin::SnapshotRestore => {
            telemetry.evaluation.snapshot_restore_reuse_count += 1
        }
        crate::data::reuse::ReuseOrigin::ReconciliationAdoption => {
            telemetry.evaluation.reconciliation_adoption_count += 1
        }
        crate::data::reuse::ReuseOrigin::CrossIdentityPersistentReuse => {
            telemetry.evaluation.cross_identity_reuse_count += 1
        }
        crate::data::reuse::ReuseOrigin::PartialArtifactSplice => {
            telemetry.evaluation.partial_artifact_splice_count += 1
        }
    }
    telemetry.evaluation.reuse_dependency_comparison_breadth +=
        u64::from(effect.operational.meaningful_input_changes);
    if effect.reuse_certification().is_some() {
        telemetry
            .evaluation
            .reuse_cold_certification_materialization_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{build_cold_artifact_intent, record_reuse_telemetry};
    use crate::data::aspect::AspectVersion;
    use crate::data::dependency::{
        CommittedSnapshotUpdate, DependencySnapshot, ReplacementSnapshotUpdate,
        SharedDependencySnapshot, SnapshotDeltaRecord,
    };
    use crate::data::handle::NodeId;
    use crate::data::output::ChangedRegion;
    use crate::data::output::{MemoizedResultOrigin, OutputChange};
    use crate::data::reuse::{
        ArtifactSemanticBoundary, ReuseBasis, ReuseBoundaryContext, ReuseBoundaryProof,
        ReuseCertificationRecord, ReuseCrossing, ReuseOrigin, ReuseSemanticRegionIdentity,
        ReuseSource, ReuseStrategy,
    };
    use crate::data::telemetry::RuntimeTelemetry;
    use crate::diagnostics::policy::{ArtifactRetentionPolicy, RetentionBudget};
    use crate::logic::evaluation::{
        DiagnosticEnvelope, EffectRuntimeMetadata, EvaluationEffect, EvaluationVerdict,
        OperationalEffect,
    };

    fn test_effect_with_labels(labels: Vec<String>) -> EvaluationEffect {
        let node = NodeId::new(0, 0);
        let mut shape_store = crate::data::dependency::DependencySnapshotShapeStore::default();
        EvaluationEffect {
            operational: OperationalEffect {
                node,
                verdict: EvaluationVerdict::Recomputed,
                aspect_version: AspectVersion::zero(),
                output_change: OutputChange::Replaced,
                reuse_basis: ReuseBasis::strategy(
                    ReuseStrategy::MemoizedArtifactReuse,
                    ReuseSource::MemoizedArtifact,
                    ReuseCrossing::None,
                ),
                reuse_origin: ReuseOrigin::MemoizedArtifactReuse,
                reuse_boundary_authority: ReuseBoundaryContext {
                    topology_regime: 1,
                    tolerance_regime: crate::data::comparator::VersionComparatorPolicy::Exact,
                    semantic_region: ReuseSemanticRegionIdentity::new(
                        node,
                        false,
                        Vec::new(),
                        crate::data::node::ContextRequirement::None,
                    ),
                    authority_policy:
                        crate::data::performance::AuthorityPolicy::SpeculativeThenReconcile,
                    artifact_family: None,
                    structural_dependency_basis:
                        crate::data::dependency::DependencySnapshotId::EMPTY,
                    partition_region_basis: Default::default(),
                    strategy_detail: crate::data::reuse::ReuseStrategyBoundaryContext::None,
                }
                .authority(),
                dependency_snapshot_update: CommittedSnapshotUpdate::Replace(
                    ReplacementSnapshotUpdate::from_snapshot(
                        DependencySnapshot::empty(),
                        &mut shape_store,
                    ),
                ),
                snapshot_delta: SnapshotDeltaRecord::between(
                    node,
                    &DependencySnapshot::empty(),
                    &SharedDependencySnapshot::empty(),
                ),
                meaningful_input_changes: 0,
            },
            diagnostics: DiagnosticEnvelope::from_parts(
                Some("artifact".into()),
                Some("continuity".into()),
                vec![ChangedRegion::new("wing").with_detail("rib-12")],
                labels,
            ),
            runtime_metadata: EffectRuntimeMetadata::default(),
        }
    }

    #[test]
    fn retained_reuse_certification_increments_cold_materialization_counter() {
        let mut telemetry = RuntimeTelemetry::default();
        let node = NodeId::new(0, 0);
        let mut shape_store = crate::data::dependency::DependencySnapshotShapeStore::default();
        let effect = EvaluationEffect {
            operational: OperationalEffect {
                node,
                verdict: EvaluationVerdict::Suppressed {
                    reason: crate::logic::evaluation::SuppressionReason::ComparatorMatch,
                },
                aspect_version: AspectVersion::zero(),
                output_change: OutputChange::Unchanged,
                reuse_basis: ReuseBasis::strategy(
                    ReuseStrategy::MemoizedArtifactReuse,
                    ReuseSource::MemoizedArtifact,
                    ReuseCrossing::None,
                ),
                reuse_origin: ReuseOrigin::MemoizedArtifactReuse,
                reuse_boundary_authority: ReuseBoundaryContext {
                    topology_regime: 1,
                    tolerance_regime: crate::data::comparator::VersionComparatorPolicy::Exact,
                    semantic_region: ReuseSemanticRegionIdentity::new(
                        node,
                        false,
                        Vec::new(),
                        crate::data::node::ContextRequirement::None,
                    ),
                    authority_policy:
                        crate::data::performance::AuthorityPolicy::SpeculativeThenReconcile,
                    artifact_family: None,
                    structural_dependency_basis:
                        crate::data::dependency::DependencySnapshotId::EMPTY,
                    partition_region_basis: Default::default(),
                    strategy_detail: crate::data::reuse::ReuseStrategyBoundaryContext::None,
                }
                .authority(),
                dependency_snapshot_update: CommittedSnapshotUpdate::Replace(
                    ReplacementSnapshotUpdate::from_snapshot(
                        DependencySnapshot::empty(),
                        &mut shape_store,
                    ),
                ),
                snapshot_delta: SnapshotDeltaRecord::between(
                    node,
                    &DependencySnapshot::empty(),
                    &SharedDependencySnapshot::empty(),
                ),
                meaningful_input_changes: 2,
            },
            diagnostics: None,
            runtime_metadata: EffectRuntimeMetadata {
                memoized_origin: MemoizedResultOrigin::MemoizedFromCache,
                recomputed: false,
                keyed_context: None,
                causality: None,
                reuse_certification: Some(ReuseCertificationRecord {
                    strategy: ReuseStrategy::MemoizedArtifactReuse,
                    origin: ReuseOrigin::MemoizedArtifactReuse,
                    source: ReuseSource::MemoizedArtifact,
                    crossing: ReuseCrossing::None,
                    proofs: vec![ReuseBoundaryProof {
                        boundary: ArtifactSemanticBoundary::TopologyRegime,
                        satisfied: true,
                    }],
                }),
                reuse_boundary_detail: None,
                previous_artifact_warm: None,
            },
        };

        record_reuse_telemetry(&mut telemetry, &effect);

        assert_eq!(telemetry.evaluation.memoized_reuse_count, 1);
        assert_eq!(
            telemetry
                .evaluation
                .reuse_cold_certification_materialization_count,
            1
        );
        assert_eq!(telemetry.evaluation.reuse_dependency_comparison_breadth, 2);
    }

    #[test]
    fn cold_artifact_intent_is_bypassed_under_omit_policy() {
        let effect = test_effect_with_labels(vec![
            "alpha".to_string(),
            "beta".to_string(),
            "gamma".to_string(),
        ]);
        let retention = RetentionBudget {
            explanation_retention: ArtifactRetentionPolicy::Omit,
            provenance_retention: ArtifactRetentionPolicy::Omit,
            ..RetentionBudget::operational()
        };

        assert!(build_cold_artifact_intent(&effect, &retention).is_none());
    }

    #[test]
    fn cold_artifact_intent_caps_label_count() {
        let effect = test_effect_with_labels(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
        ]);
        let retention = RetentionBudget {
            explanation_retention: ArtifactRetentionPolicy::Retain,
            provenance_retention: ArtifactRetentionPolicy::Retain,
            ..RetentionBudget::development()
        };

        let intent = build_cold_artifact_intent(&effect, &retention).expect("cold intent");
        assert_eq!(
            intent.labels.len(),
            crate::data::trace::COLD_ARTIFACT_INTENT_LABEL_LIMIT
        );
        assert_eq!(intent.labels.as_slice(), &["a", "b", "c", "d"]);
    }
}
