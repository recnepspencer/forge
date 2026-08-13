use crate::data::proof::invalidation::output_commit::CommittedProducedAspectDelta;
use crate::diagnostics::policy::ArtifactRetentionPolicy;
use crate::logic::evaluation::{
    EffectComparison, EvaluationEffect, EvaluationVerdict, SuppressionReason,
};

use super::vocabulary::record_reuse_telemetry;
use super::SignalGraph;

impl SignalGraph {
    pub(super) fn record_effect_telemetry(
        &mut self,
        performed: Option<&CommittedProducedAspectDelta>,
        effect: &EvaluationEffect,
        comparison: &EffectComparison,
        suppressed_downstream: u64,
    ) {
        debug_assert!(
            performed.is_none()
                || performed
                    .is_some_and(|commit| commit.delta().producer == effect.operational.node),
            "performed output commit must govern producer observation"
        );
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

    pub(super) fn retains_runtime_cold_artifacts(&self) -> bool {
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
