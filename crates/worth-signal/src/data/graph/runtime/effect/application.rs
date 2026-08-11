use crate::data::error::SignalError;
use crate::data::trace::ColdArtifactIntent;
use crate::diagnostics::policy::ArtifactRetentionPolicy;
use crate::logic::evaluation::EvaluationEffect;

use super::vocabulary::verdict_commits_snapshot;
use super::SignalGraph;

impl SignalGraph {
    pub(super) fn materialize_retained_artifact(
        &mut self,
        cold_intent: Option<ColdArtifactIntent>,
    ) -> Option<crate::data::trace::ColdArtifactRecord> {
        let cold_intent = cold_intent?;
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

    pub(super) fn commit_effect_snapshot(
        &mut self,
        effect: &mut EvaluationEffect,
    ) -> Result<(), SignalError> {
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
}
