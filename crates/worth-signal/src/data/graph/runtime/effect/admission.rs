use crate::data::error::SignalError;
use crate::data::output::CanonicalChangedRegions;
use crate::data::output_equivalence::OutputEquivalencePolicy;
use crate::data::proof::PartitionScopeSet;
use crate::data::trace::{
    ArtifactMergeAuthority, ArtifactTransitionKey, CompactChangedScopeProof,
    ContinuityAuthorityToken, HotArtifactWrite, ReuseOperationalBasis, RuntimeArtifactHot,
    RuntimeArtifactState, RuntimeArtifactWarm,
};
use crate::logic::evaluation::{
    EffectComparison, EvaluationEffect, EvaluationVerdict, SuppressionReason,
};

use super::vocabulary::{
    build_cold_artifact_intent, count_changed_partitions, normalize_output_change,
    trace_identity_hash, trace_output_hash, verdict_retains_runtime_artifact,
};
use super::{ApplyCommitPacket, SignalGraph};

impl SignalGraph {
    pub(super) fn compare_effect(
        &self,
        effect: &EvaluationEffect,
        previous_trace: Option<&crate::logic::evaluation::PreviousArtifactWarmSnapshot>,
        output_equivalence: OutputEquivalencePolicy,
    ) -> Result<EffectComparison, SignalError> {
        crate::data::proof::invalidation::output_commit::SemanticOutputCommitDecision::validate_declared_change(
            self.node_aspect_version(effect.operational.node)?,
            effect.operational.aspect_version,
            self.get_contract(effect.operational.node)?.semantics.produces,
            effect.operational.output_change == crate::data::output::OutputChange::Unchanged,
        )
        .map_err(|violation| {
            SignalError::invalid_input(format!("output commit contract violation: {violation:?}"))
        })?;
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
            matches!(output_equivalence, OutputEquivalencePolicy::OutputIdentity)
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

    pub(super) fn build_effect_artifact_write(
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
            build_cold_artifact_intent(effect, &self.installed_runtime_policy().retention_budget());
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

    #[cfg_attr(not(feature = "parallel"), allow(dead_code))]
    pub(crate) fn build_apply_commit_packet(
        &self,
        effect: EvaluationEffect,
        output_equivalence: OutputEquivalencePolicy,
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
        let comparison =
            self.compare_effect(&effect, previous_warm.as_ref(), output_equivalence)?;
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
}
