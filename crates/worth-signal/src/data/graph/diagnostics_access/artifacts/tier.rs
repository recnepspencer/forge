use crate::data::graph::signal_graph::SignalGraph;
use crate::diagnostics::policy::{ArtifactRetentionPolicy, DiagnosticsAvailability};
use crate::diagnostics::state::DiagnosticsState;
use crate::state::{SignalSnapshotV1, SnapshotArtifactRestoreMode, SnapshotRestoreIntent};

impl SignalGraph {
    pub(super) fn explanation_reconstruction_availability(&self) -> DiagnosticsAvailability {
        let policy = self.installed_runtime_policy();
        if policy.observation_activation()
            == worth_foundational::ObservationActivationProfile::OnDemand
            && !self.diagnostics_state().has_observation_activation(
                crate::logic::transaction::SignalObservationSurface::DescriptiveFacts.bit(),
            )
        {
            return DiagnosticsAvailability::ObservationNotActivated;
        }
        if matches!(
            policy.retention_budget().explanation_retention,
            ArtifactRetentionPolicy::Omit
        ) {
            DiagnosticsAvailability::OmittedByTier
        } else if policy
            .reconstruction_budget()
            .allow_explanation_reconstruction
        {
            DiagnosticsAvailability::ReconstructedAvailable
        } else {
            DiagnosticsAvailability::DeniedByBudget
        }
    }

    pub(super) fn provenance_reconstruction_availability(&self) -> DiagnosticsAvailability {
        let policy = self.installed_runtime_policy();
        if policy.observation_activation()
            == worth_foundational::ObservationActivationProfile::OnDemand
            && !self.diagnostics_state().has_observation_activation(
                crate::logic::transaction::SignalObservationSurface::DescriptiveFacts.bit(),
            )
        {
            return DiagnosticsAvailability::ObservationNotActivated;
        }
        if matches!(
            policy.retention_budget().provenance_retention,
            ArtifactRetentionPolicy::Omit
        ) {
            DiagnosticsAvailability::OmittedByTier
        } else if policy
            .reconstruction_budget()
            .allow_provenance_reconstruction
        {
            DiagnosticsAvailability::ReconstructedAvailable
        } else {
            DiagnosticsAvailability::DeniedByBudget
        }
    }

    pub(super) fn apply_snapshot_diagnostic_policy_richness(
        restored: &mut SignalGraph,
        snapshot: &SignalSnapshotV1,
        current_diagnostics: &DiagnosticsState,
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
            let installed = restored.installed_runtime_policy();
            restored
                .diagnostics_state_mut()
                .set_installed_policy(installed);
        }
        let diagnostics_breadth = snapshot.diagnostics.recent_history.len() as u64
            + snapshot.diagnostics.replay_frames.len() as u64
            + snapshot.diagnostics.explanation_facts.len() as u64
            + snapshot.diagnostics.provenance_facts.len() as u64
            + snapshot.diagnostics.lineage_records.len() as u64;
        restored.with_telemetry(|telemetry| {
            telemetry.checkpoint.restore_diagnostic_richness_breadth += diagnostics_breadth;
        });
    }
}
