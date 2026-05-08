use serde::Serialize;

use crate::boundary::errors::ForgeSignalJsError;

use super::{
    canonical_worker_certification_digest,
    WorkerImportExportCallbackUnavailabilityCertificationPackage,
    WorkerReplayCheckpointRetainedHistoryCertificationPackage,
    WorkerReplayRestoreCapabilityCertificationPackage, WorkerRuntimeShell,
    WorkerUnavailableCompatibilityCertificationPackage,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerPhase6CloseoutCertificationPackage {
    pub certification_family: &'static str,
    pub phase_closeout_mode: &'static str,
    pub covered_suite_count: u64,
    pub covered_phase6_artifact_count: u64,
    pub replay_restore_exact_artifact: &'static str,
    pub checkpoint_retained_history_artifact: &'static str,
    pub import_export_unavailability_artifact: &'static str,
    pub worker_unavailable_incompatibility_artifact: &'static str,
    pub fallback_count: u64,
    pub exported_callback_count: u64,
    pub unavailable_callback_count: u64,
    pub reattached_callback_count: u64,
    pub host_capability_transport_count: u64,
    pub replay_restore_certification_digest: String,
    pub checkpoint_retained_history_certification_digest: String,
    pub import_export_unavailability_certification_digest: String,
    pub worker_unavailable_compatibility_certification_digest: String,
    pub replay_restore_digest: String,
    pub retained_history_digest: String,
    pub callback_unavailability_digest: String,
    pub worker_unavailable_historical_capability_digest: String,
    pub capability_parity_digest: String,
    pub phase6_artifact_digest: String,
    pub certification_digest: String,
}

impl WorkerPhase6CloseoutCertificationPackage {
    pub(in crate::runtime::worker_host) fn from_certified_phase6_artifacts(
        shell: &WorkerRuntimeShell,
        worker_unavailable: WorkerUnavailableCompatibilityCertificationPackage,
    ) -> Result<Self, ForgeSignalJsError> {
        let replay_restore = latest_replay_restore(shell)?;
        let checkpoint = latest_checkpoint(shell)?;
        let import_export = latest_import_export(shell)?;
        reject_weak_replay_restore(replay_restore)?;
        reject_weak_checkpoint(checkpoint)?;
        reject_weak_import_export(import_export)?;
        reject_weak_worker_unavailable(&worker_unavailable)?;

        let fallback_count = replay_restore
            .fallback_count
            .saturating_add(checkpoint.fallback_count)
            .saturating_add(import_export.fallback_count)
            .saturating_add(worker_unavailable.fallback_count);
        if fallback_count != 0 {
            return Err(ForgeSignalJsError::invalid_input(
                "worker Phase 6 closeout certification requires zero fallback",
            ));
        }

        let capability_parity_digest = canonical_worker_certification_digest(&(
            "workerPhase6CapabilityParity",
            replay_restore.capability_availability_digest.as_str(),
            replay_restore.replay_import_compatibility_digest.as_str(),
            replay_restore.placement_identity_digest.as_str(),
            replay_restore.lowered_plan_identity_digest.as_str(),
            checkpoint.capability_availability_digest.as_str(),
            checkpoint.replay_import_compatibility_digest.as_str(),
            checkpoint.placement_identity_digest.as_str(),
            checkpoint.lowered_plan_identity_digest.as_str(),
            worker_unavailable.capability_availability_digest.as_str(),
            worker_unavailable
                .replay_import_compatibility_digest
                .as_str(),
            worker_unavailable.placement_identity_digest.as_str(),
            worker_unavailable.historical_capability_digest.as_str(),
        ))?;
        let phase6_artifact_digest = canonical_worker_certification_digest(&(
            "workerPhase6Artifacts",
            replay_restore.exact_restore_artifact,
            checkpoint.retained_history_artifact,
            import_export.callback_unavailability_artifact,
            worker_unavailable.incompatibility_artifact,
            import_export.unavailable_callback_count,
            worker_unavailable.unavailable_callback_count,
        ))?;
        let certification_digest = canonical_worker_certification_digest(&(
            "workerPhase6CloseoutCertification",
            replay_restore.certification_digest.as_str(),
            checkpoint.certification_digest.as_str(),
            import_export.certification_digest.as_str(),
            worker_unavailable.certification_digest.as_str(),
            capability_parity_digest.as_str(),
            phase6_artifact_digest.as_str(),
            fallback_count,
        ))?;

        Ok(Self {
            certification_family: "workerPhase6CloseoutCertification",
            phase_closeout_mode: "ReplayRestoreImportExportWorkerUnavailableParity",
            covered_suite_count: replay_restore
                .covered_suite_count
                .saturating_add(checkpoint.covered_suite_count)
                .saturating_add(import_export.covered_suite_count)
                .saturating_add(worker_unavailable.covered_suite_count),
            covered_phase6_artifact_count: 4,
            replay_restore_exact_artifact: replay_restore.exact_restore_artifact,
            checkpoint_retained_history_artifact: checkpoint.retained_history_artifact,
            import_export_unavailability_artifact: import_export.callback_unavailability_artifact,
            worker_unavailable_incompatibility_artifact: worker_unavailable
                .incompatibility_artifact,
            fallback_count,
            exported_callback_count: import_export.exported_callback_count,
            unavailable_callback_count: import_export
                .unavailable_callback_count
                .saturating_add(worker_unavailable.unavailable_callback_count),
            reattached_callback_count: import_export.reattached_callback_count,
            host_capability_transport_count: import_export.host_capability_transport_count,
            replay_restore_certification_digest: replay_restore.certification_digest.clone(),
            checkpoint_retained_history_certification_digest: checkpoint
                .certification_digest
                .clone(),
            import_export_unavailability_certification_digest: import_export
                .certification_digest
                .clone(),
            worker_unavailable_compatibility_certification_digest: worker_unavailable
                .certification_digest
                .clone(),
            replay_restore_digest: replay_restore.replay_restore_digest.clone(),
            retained_history_digest: checkpoint.retained_history_digest.clone(),
            callback_unavailability_digest: import_export.callback_unavailability_digest.clone(),
            worker_unavailable_historical_capability_digest: worker_unavailable
                .historical_capability_digest
                .clone(),
            capability_parity_digest,
            phase6_artifact_digest,
            certification_digest,
        })
    }
}

impl WorkerRuntimeShell {
    pub fn certify_worker_phase6_closeout(
        &self,
        worker_unavailable: WorkerUnavailableCompatibilityCertificationPackage,
    ) -> Result<WorkerPhase6CloseoutCertificationPackage, ForgeSignalJsError> {
        WorkerPhase6CloseoutCertificationPackage::from_certified_phase6_artifacts(
            self,
            worker_unavailable,
        )
    }
}

fn latest_replay_restore(
    shell: &WorkerRuntimeShell,
) -> Result<&WorkerReplayRestoreCapabilityCertificationPackage, ForgeSignalJsError> {
    shell
        .latest_worker_replay_restore_capability_certification
        .as_ref()
        .ok_or_else(|| {
            ForgeSignalJsError::invalid_input(
                "worker Phase 6 closeout certification requires replay restore certification",
            )
        })
}

fn latest_checkpoint(
    shell: &WorkerRuntimeShell,
) -> Result<&WorkerReplayCheckpointRetainedHistoryCertificationPackage, ForgeSignalJsError> {
    shell
        .latest_worker_replay_checkpoint_retained_history_certification
        .as_ref()
        .ok_or_else(|| {
            ForgeSignalJsError::invalid_input(
                "worker Phase 6 closeout certification requires checkpoint retained-history certification",
            )
        })
}

fn latest_import_export(
    shell: &WorkerRuntimeShell,
) -> Result<&WorkerImportExportCallbackUnavailabilityCertificationPackage, ForgeSignalJsError> {
    shell
        .latest_worker_import_export_callback_unavailability_certification
        .as_ref()
        .ok_or_else(|| {
            ForgeSignalJsError::invalid_input(
                "worker Phase 6 closeout certification requires import/export callback certification",
            )
        })
}

fn reject_weak_replay_restore(
    package: &WorkerReplayRestoreCapabilityCertificationPackage,
) -> Result<(), ForgeSignalJsError> {
    if package.exact_restore_artifact != "sameRuntimeBranchSnapshotStore"
        || package.incompatibility_artifact != "none"
        || package.fallback_count != 0
    {
        return Err(ForgeSignalJsError::invalid_input(
            "worker Phase 6 closeout certification requires exact same-runtime restore",
        ));
    }
    Ok(())
}

fn reject_weak_checkpoint(
    package: &WorkerReplayCheckpointRetainedHistoryCertificationPackage,
) -> Result<(), ForgeSignalJsError> {
    if package.retained_history_artifact != "checkpointPlusRetainedReplayHistory"
        || package.exact_restore_artifact != "checkpointPlusRetainedReplayHistory"
        || package.retained_replay_frame_count == 0
        || package.fallback_count != 0
    {
        return Err(ForgeSignalJsError::invalid_input(
            "worker Phase 6 closeout certification requires checkpoint plus retained history",
        ));
    }
    Ok(())
}

fn reject_weak_import_export(
    package: &WorkerImportExportCallbackUnavailabilityCertificationPackage,
) -> Result<(), ForgeSignalJsError> {
    if package.callback_unavailability_artifact != "computeCallbackUnavailableForPortableExport"
        || package.unavailable_callback_count == 0
        || package.reattached_callback_count != package.unavailable_callback_count
        || package.fallback_count != 0
    {
        return Err(ForgeSignalJsError::invalid_input(
            "worker Phase 6 closeout certification requires callback unavailability and reattachment",
        ));
    }
    Ok(())
}

fn reject_weak_worker_unavailable(
    package: &WorkerUnavailableCompatibilityCertificationPackage,
) -> Result<(), ForgeSignalJsError> {
    if package.worker_support_posture != "workerUnavailable"
        || package.incompatibility_artifact != "dedicatedWorkerUnavailable"
        || package.hidden_fallback_allowed
        || !package.denial_artifact_required
        || package.fallback_count != 0
    {
        return Err(ForgeSignalJsError::invalid_input(
            "worker Phase 6 closeout certification requires explicit worker-unavailable compatibility",
        ));
    }
    Ok(())
}
