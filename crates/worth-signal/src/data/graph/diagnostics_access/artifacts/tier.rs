use crate::data::graph::signal_graph::SignalGraph;
use crate::diagnostics::policy::{
    ArtifactRetentionPolicy, DiagnosticsAvailability, SignalRuntimePolicy,
};
use crate::diagnostics::state::DiagnosticsState;
use crate::state::{SignalSnapshotV1, SnapshotArtifactRestoreMode, SnapshotRestoreIntent};

impl SignalGraph {
    pub(super) fn explanation_reconstruction_availability(&self) -> DiagnosticsAvailability {
        let policy = self.runtime_policy();
        if matches!(
            policy.retention_budget.explanation_retention,
            ArtifactRetentionPolicy::Omit
        ) {
            DiagnosticsAvailability::OmittedByTier
        } else if policy.can_reconstruct_explanation() {
            DiagnosticsAvailability::ReconstructedAvailable
        } else {
            DiagnosticsAvailability::DeniedByBudget
        }
    }

    pub(super) fn provenance_reconstruction_availability(&self) -> DiagnosticsAvailability {
        let policy = self.runtime_policy();
        if matches!(
            policy.retention_budget.provenance_retention,
            ArtifactRetentionPolicy::Omit
        ) {
            DiagnosticsAvailability::OmittedByTier
        } else if policy.can_reconstruct_provenance() {
            DiagnosticsAvailability::ReconstructedAvailable
        } else {
            DiagnosticsAvailability::DeniedByBudget
        }
    }

    pub(super) fn apply_snapshot_diagnostic_policy_richness(
        restored: &mut SignalGraph,
        snapshot: &SignalSnapshotV1,
        current_diagnostics: &DiagnosticsState,
        current_policy: SignalRuntimePolicy,
        intent: SnapshotRestoreIntent,
    ) {
        restored
            .observation
            .diagnostics
            .restore_snapshot_payload_preserving_history_from(
                snapshot.diagnostics.clone(),
                current_diagnostics,
            );
        if matches!(
            intent.artifacts,
            SnapshotArtifactRestoreMode::ApplyActiveRuntimePolicy
        ) {
            restored.observation.diagnostics.set_policy(current_policy);
        }
        let diagnostics_breadth = snapshot.diagnostics.recent_history.len() as u64
            + snapshot.diagnostics.replay_frames.len() as u64
            + snapshot.diagnostics.explanation_facts.len() as u64
            + snapshot.diagnostics.provenance_facts.len() as u64
            + snapshot.diagnostics.lineage_records.len() as u64;
        restored
            .telemetry_mut()
            .checkpoint
            .restore_diagnostic_richness_breadth += diagnostics_breadth;
    }
}
