use forge_signal::facade::history::RuntimeSnapshot;
use serde::Serialize;

use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::summaries::ReplayFrameSummary;

use super::{
    canonical_worker_certification_digest, worker_replay_restore_capability, WorkerRuntimeShell,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerReplayCheckpointRetainedHistoryReport {
    pub envelope_family: &'static str,
    pub checkpoint_artifact: &'static str,
    pub retained_history_artifact: &'static str,
    pub exact_restore_artifact: &'static str,
    pub incompatibility_artifact: &'static str,
    pub branch_id: u64,
    pub snapshot_id: u64,
    pub checkpoint_replay_cursor: u64,
    pub full_replay_frame_count: u64,
    pub retained_replay_frame_count: u64,
    pub worker_first_truth_digest: String,
    pub checkpoint_digest: String,
    pub full_replay_digest: String,
    pub retained_history_digest: String,
    pub replay_restore_digest: String,
    pub capability_availability_digest: String,
    pub replay_import_compatibility_digest: String,
    pub placement_identity_digest: String,
    pub lowered_plan_identity_digest: String,
    pub fallback_count: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerReplayCheckpointRetainedHistoryCertificationPackage {
    pub certification_family: &'static str,
    pub covered_suite_count: u64,
    pub checkpoint_artifact: &'static str,
    pub retained_history_artifact: &'static str,
    pub exact_restore_artifact: &'static str,
    pub incompatibility_artifact: &'static str,
    pub branch_id: u64,
    pub snapshot_id: u64,
    pub checkpoint_replay_cursor: u64,
    pub full_replay_frame_count: u64,
    pub retained_replay_frame_count: u64,
    pub worker_first_truth_digest: String,
    pub checkpoint_digest: String,
    pub full_replay_digest: String,
    pub retained_history_digest: String,
    pub replay_restore_digest: String,
    pub capability_availability_digest: String,
    pub replay_import_compatibility_digest: String,
    pub placement_identity_digest: String,
    pub lowered_plan_identity_digest: String,
    pub fallback_count: u64,
    pub certification_digest: String,
}

impl WorkerReplayCheckpointRetainedHistoryReport {
    pub(in crate::runtime::worker_host) fn from_checkpoint(
        shell: &mut WorkerRuntimeShell,
        branch_id: u64,
        checkpoint: RuntimeSnapshot,
    ) -> Result<Self, ForgeSignalJsError> {
        if checkpoint.meta.branch_id.0 != branch_id {
            return Err(ForgeSignalJsError::invalid_input(
                "worker replay checkpoint certification requires a checkpoint from the certified branch",
            ));
        }
        let checkpoint_replay_cursor = checkpoint
            .meta
            .replay_head
            .map(|cursor| cursor.0)
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(
                    "worker replay checkpoint certification requires checkpoint replay cursor",
                )
            })?;
        let checkpoint_digest = canonical_worker_certification_digest(&checkpoint)?;
        let checkpoint_snapshot_id = checkpoint.meta.snapshot_id.0;
        let full_replay = shell.core.replay_for_branch(branch_id)?;
        let retained_replay_frames: Vec<ReplayFrameSummary> = full_replay
            .frames
            .iter()
            .filter(|frame| {
                frame.cursor > checkpoint_replay_cursor
                    && frame.snapshot_id != Some(checkpoint_snapshot_id)
            })
            .cloned()
            .collect();
        if retained_replay_frames.is_empty() {
            return Err(ForgeSignalJsError::invalid_input(
                "worker replay checkpoint certification requires retained history after checkpoint",
            ));
        }
        let full_replay_digest = canonical_worker_certification_digest(&full_replay)?;
        let retained_history_digest = canonical_worker_certification_digest(&(
            "workerCheckpointPlusRetainedHistory",
            branch_id,
            checkpoint_snapshot_id,
            checkpoint_replay_cursor,
            checkpoint_digest.as_str(),
            &retained_replay_frames,
        ))?;
        let current_branch = shell.branch_truth_envelope_for_branch(branch_id)?;
        let placement = shell.core.worker_callback_placement_eligibility()?;
        let lowered_plan_identity_digest =
            worker_replay_restore_capability::historical_lowered_plan_identity_digest(
                placement.placement_identity_digest.as_str(),
                placement.replay_import_compatibility_digest.as_str(),
            )?;
        let replay_restore_digest = canonical_worker_certification_digest(&(
            "workerReplayCheckpointRetainedHistory",
            "workerBranchCheckpointSnapshot",
            "checkpointPlusRetainedReplayHistory",
            "checkpointPlusRetainedReplayHistory",
            "none",
            checkpoint_digest.as_str(),
            full_replay_digest.as_str(),
            retained_history_digest.as_str(),
            current_branch.committed_truth_digest.as_str(),
            placement.capability_availability_digest.as_str(),
            placement.replay_import_compatibility_digest.as_str(),
            placement.placement_identity_digest.as_str(),
            lowered_plan_identity_digest.as_str(),
        ))?;

        Ok(Self {
            envelope_family: "replayCheckpointRetainedHistory",
            checkpoint_artifact: "workerBranchCheckpointSnapshot",
            retained_history_artifact: "checkpointPlusRetainedReplayHistory",
            exact_restore_artifact: "checkpointPlusRetainedReplayHistory",
            incompatibility_artifact: "none",
            branch_id,
            snapshot_id: checkpoint_snapshot_id,
            checkpoint_replay_cursor,
            full_replay_frame_count: full_replay.frames.len() as u64,
            retained_replay_frame_count: retained_replay_frames.len() as u64,
            worker_first_truth_digest: current_branch.committed_truth_digest,
            checkpoint_digest,
            full_replay_digest,
            retained_history_digest,
            replay_restore_digest,
            capability_availability_digest: placement.capability_availability_digest,
            replay_import_compatibility_digest: placement.replay_import_compatibility_digest,
            placement_identity_digest: placement.placement_identity_digest,
            lowered_plan_identity_digest,
            fallback_count: 0,
        })
    }
}

impl WorkerReplayCheckpointRetainedHistoryCertificationPackage {
    pub(in crate::runtime::worker_host) fn from_retained_report(
        shell: &WorkerRuntimeShell,
    ) -> Result<Self, ForgeSignalJsError> {
        let report = shell.latest_worker_replay_checkpoint_retained_history_report()?;
        let placement = shell.core.worker_callback_placement_eligibility()?;
        if placement.capability_availability_digest != report.capability_availability_digest
            || placement.replay_import_compatibility_digest
                != report.replay_import_compatibility_digest
            || placement.placement_identity_digest != report.placement_identity_digest
        {
            return Err(ForgeSignalJsError::invalid_input(
                "worker replay checkpoint certification requires current capability posture",
            ));
        }
        let lowered_plan_identity_digest =
            worker_replay_restore_capability::historical_lowered_plan_identity_digest(
                placement.placement_identity_digest.as_str(),
                placement.replay_import_compatibility_digest.as_str(),
            )?;
        if lowered_plan_identity_digest != report.lowered_plan_identity_digest {
            return Err(ForgeSignalJsError::invalid_input(
                "worker replay checkpoint certification requires current lowered plan identity",
            ));
        }
        let current_branch_truth = shell.core.branch_state_proof(report.branch_id)?;
        if current_branch_truth.state_digest != report.worker_first_truth_digest {
            return Err(ForgeSignalJsError::invalid_input(
                "worker replay checkpoint certification requires current branch truth",
            ));
        }
        let worker_first_truth_digest = current_branch_truth.state_digest;
        let certification_digest = canonical_worker_certification_digest(&(
            "workerReplayCheckpointRetainedHistoryCertification",
            report.checkpoint_artifact,
            report.retained_history_artifact,
            report.exact_restore_artifact,
            report.incompatibility_artifact,
            report.checkpoint_digest.as_str(),
            report.full_replay_digest.as_str(),
            report.retained_history_digest.as_str(),
            report.replay_restore_digest.as_str(),
            lowered_plan_identity_digest.as_str(),
            worker_first_truth_digest.as_str(),
        ))?;

        Ok(Self {
            certification_family: "workerReplayCheckpointRetainedHistoryCertification",
            covered_suite_count: 1,
            checkpoint_artifact: report.checkpoint_artifact,
            retained_history_artifact: report.retained_history_artifact,
            exact_restore_artifact: report.exact_restore_artifact,
            incompatibility_artifact: report.incompatibility_artifact,
            branch_id: report.branch_id,
            snapshot_id: report.snapshot_id,
            checkpoint_replay_cursor: report.checkpoint_replay_cursor,
            full_replay_frame_count: report.full_replay_frame_count,
            retained_replay_frame_count: report.retained_replay_frame_count,
            worker_first_truth_digest,
            checkpoint_digest: report.checkpoint_digest.clone(),
            full_replay_digest: report.full_replay_digest.clone(),
            retained_history_digest: report.retained_history_digest.clone(),
            replay_restore_digest: report.replay_restore_digest.clone(),
            capability_availability_digest: report.capability_availability_digest.clone(),
            replay_import_compatibility_digest: report.replay_import_compatibility_digest.clone(),
            placement_identity_digest: report.placement_identity_digest.clone(),
            lowered_plan_identity_digest,
            fallback_count: report.fallback_count,
            certification_digest,
        })
    }
}

impl WorkerRuntimeShell {
    pub fn record_worker_replay_checkpoint_retained_history(
        &mut self,
        branch_id: u64,
        checkpoint: RuntimeSnapshot,
    ) -> Result<WorkerReplayCheckpointRetainedHistoryReport, ForgeSignalJsError> {
        let report = WorkerReplayCheckpointRetainedHistoryReport::from_checkpoint(
            self, branch_id, checkpoint,
        )?;
        self.latest_worker_replay_checkpoint_retained_history_report = Some(report.clone());
        Ok(report)
    }

    pub fn certify_worker_replay_checkpoint_retained_history(
        &mut self,
    ) -> Result<WorkerReplayCheckpointRetainedHistoryCertificationPackage, ForgeSignalJsError> {
        let package =
            WorkerReplayCheckpointRetainedHistoryCertificationPackage::from_retained_report(self)?;
        self.latest_worker_replay_checkpoint_retained_history_certification = Some(package.clone());
        Ok(package)
    }

    pub(in crate::runtime::worker_host) fn latest_worker_replay_checkpoint_retained_history_report(
        &self,
    ) -> Result<&WorkerReplayCheckpointRetainedHistoryReport, ForgeSignalJsError> {
        self.latest_worker_replay_checkpoint_retained_history_report
            .as_ref()
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(
                    "worker replay checkpoint certification requires checkpoint retained-history evidence",
                )
            })
    }
}
