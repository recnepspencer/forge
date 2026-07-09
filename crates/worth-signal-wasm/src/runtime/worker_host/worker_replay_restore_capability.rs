use worth_signal::facade::history::RuntimeSnapshot;
use serde::Serialize;

use crate::boundary::errors::WorthSignalJsError;

use super::{canonical_worker_certification_digest, WorkerBranchTruthEnvelope, WorkerRuntimeShell};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerReplayRestoreCapabilityReport {
    pub envelope_family: &'static str,
    pub restore_outcome: &'static str,
    pub exact_restore_artifact: &'static str,
    pub incompatibility_artifact: &'static str,
    pub branch_id: u64,
    pub snapshot_id: u64,
    pub replay_frame_count: u64,
    pub worker_first_truth_digest: String,
    pub snapshot_digest: String,
    pub replay_restore_digest: String,
    pub capability_availability_digest: String,
    pub replay_import_compatibility_digest: String,
    pub placement_identity_digest: String,
    pub lowered_plan_identity_digest: String,
    pub branch_restore_digest: String,
    pub fallback_count: u64,
    pub restored_branch: WorkerBranchTruthEnvelope,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerReplayRestoreCapabilityCertificationPackage {
    pub certification_family: &'static str,
    pub covered_suite_count: u64,
    pub restore_outcome: &'static str,
    pub exact_restore_artifact: &'static str,
    pub incompatibility_artifact: &'static str,
    pub branch_id: u64,
    pub snapshot_id: u64,
    pub replay_frame_count: u64,
    pub worker_first_truth_digest: String,
    pub snapshot_digest: String,
    pub replay_restore_digest: String,
    pub capability_availability_digest: String,
    pub replay_import_compatibility_digest: String,
    pub placement_identity_digest: String,
    pub lowered_plan_identity_digest: String,
    pub branch_restore_digest: String,
    pub fallback_count: u64,
    pub certification_digest: String,
}

impl WorkerReplayRestoreCapabilityReport {
    pub(in crate::runtime::worker_host) fn same_runtime_exact_restore(
        shell: &mut WorkerRuntimeShell,
        branch_id: u64,
        snapshot: RuntimeSnapshot,
    ) -> Result<Self, WorthSignalJsError> {
        let snapshot_id = snapshot.meta.snapshot_id.0;
        let snapshot_digest = canonical_worker_certification_digest(&snapshot)?;
        let replay_before_restore = shell.core.replay_for_branch(branch_id)?;
        let replay_before_restore_digest =
            canonical_worker_certification_digest(&replay_before_restore)?;

        shell.core.restore_branch_snapshot(branch_id, snapshot)?;
        shell.clear_worker_boundary_certification_evidence();

        let restored_branch = shell.branch_truth_envelope_for_branch(branch_id)?;
        let replay_after_restore = shell.core.replay_for_branch(branch_id)?;
        let replay_after_restore_digest =
            canonical_worker_certification_digest(&replay_after_restore)?;
        let placement = shell.core.worker_callback_placement_eligibility()?;
        let replay_frame_count = replay_after_restore.frames.len() as u64;
        let lowered_plan_identity_digest = historical_lowered_plan_identity_digest(
            placement.placement_identity_digest.as_str(),
            placement.replay_import_compatibility_digest.as_str(),
        )?;
        let replay_restore_digest = canonical_worker_certification_digest(&(
            "workerReplayRestoreCapability",
            "SameRuntimeExactRestore",
            snapshot_digest.as_str(),
            replay_before_restore_digest.as_str(),
            replay_after_restore_digest.as_str(),
            restored_branch.committed_truth_digest.as_str(),
            placement.placement_identity_digest.as_str(),
            placement.replay_import_compatibility_digest.as_str(),
            lowered_plan_identity_digest.as_str(),
            placement.capability_availability_digest.as_str(),
        ))?;
        let branch_restore_digest = canonical_worker_certification_digest(&(
            "workerBranchRestoreCapability",
            branch_id,
            snapshot_id,
            restored_branch.committed_truth_digest.as_str(),
            replay_restore_digest.as_str(),
        ))?;

        Ok(Self {
            envelope_family: "replayRestoreCapability",
            restore_outcome: "SameRuntimeExactRestore",
            exact_restore_artifact: "sameRuntimeBranchSnapshotStore",
            incompatibility_artifact: "none",
            branch_id,
            snapshot_id,
            replay_frame_count,
            worker_first_truth_digest: restored_branch.committed_truth_digest.clone(),
            snapshot_digest,
            replay_restore_digest,
            capability_availability_digest: placement.capability_availability_digest,
            replay_import_compatibility_digest: placement.replay_import_compatibility_digest,
            placement_identity_digest: placement.placement_identity_digest,
            lowered_plan_identity_digest,
            branch_restore_digest,
            fallback_count: 0,
            restored_branch,
        })
    }
}

impl WorkerReplayRestoreCapabilityCertificationPackage {
    pub(in crate::runtime::worker_host) fn from_retained_report(
        shell: &WorkerRuntimeShell,
    ) -> Result<Self, WorthSignalJsError> {
        let report = shell.latest_worker_replay_restore_capability_report()?;
        let current_branch_truth = shell.core.branch_state_proof(report.branch_id)?;
        if current_branch_truth.state_digest != report.worker_first_truth_digest {
            return Err(WorthSignalJsError::invalid_input(
                "worker replay restore certification requires current restored branch truth",
            ));
        }
        let placement = shell.core.worker_callback_placement_eligibility()?;
        if placement.capability_availability_digest != report.capability_availability_digest
            || placement.replay_import_compatibility_digest
                != report.replay_import_compatibility_digest
            || placement.placement_identity_digest != report.placement_identity_digest
        {
            return Err(WorthSignalJsError::invalid_input(
                "worker replay restore certification requires current capability posture",
            ));
        }
        let lowered_plan_identity_digest = historical_lowered_plan_identity_digest(
            placement.placement_identity_digest.as_str(),
            placement.replay_import_compatibility_digest.as_str(),
        )?;
        if lowered_plan_identity_digest != report.lowered_plan_identity_digest {
            return Err(WorthSignalJsError::invalid_input(
                "worker replay restore certification requires current lowered plan identity",
            ));
        }
        let worker_first_truth_digest = current_branch_truth.state_digest;
        let certification_digest = canonical_worker_certification_digest(&(
            "workerReplayRestoreCapabilityCertification",
            report.restore_outcome,
            report.exact_restore_artifact,
            report.incompatibility_artifact,
            report.snapshot_digest.as_str(),
            report.replay_restore_digest.as_str(),
            report.capability_availability_digest.as_str(),
            report.replay_import_compatibility_digest.as_str(),
            report.placement_identity_digest.as_str(),
            report.lowered_plan_identity_digest.as_str(),
            report.branch_restore_digest.as_str(),
            worker_first_truth_digest.as_str(),
        ))?;

        Ok(Self {
            certification_family: "workerReplayRestoreCapabilityCertification",
            covered_suite_count: 1,
            restore_outcome: report.restore_outcome,
            exact_restore_artifact: report.exact_restore_artifact,
            incompatibility_artifact: report.incompatibility_artifact,
            branch_id: report.branch_id,
            snapshot_id: report.snapshot_id,
            replay_frame_count: report.replay_frame_count,
            worker_first_truth_digest,
            snapshot_digest: report.snapshot_digest.clone(),
            replay_restore_digest: report.replay_restore_digest.clone(),
            capability_availability_digest: report.capability_availability_digest.clone(),
            replay_import_compatibility_digest: report.replay_import_compatibility_digest.clone(),
            placement_identity_digest: report.placement_identity_digest.clone(),
            lowered_plan_identity_digest,
            branch_restore_digest: report.branch_restore_digest.clone(),
            fallback_count: report.fallback_count,
            certification_digest,
        })
    }
}

pub(in crate::runtime::worker_host) fn historical_lowered_plan_identity_digest(
    placement_identity_digest: &str,
    replay_import_compatibility_digest: &str,
) -> Result<String, WorthSignalJsError> {
    canonical_worker_certification_digest(&(
        "workerHistoricalLoweredPlanIdentity",
        placement_identity_digest,
        replay_import_compatibility_digest,
    ))
}

impl WorkerRuntimeShell {
    pub fn restore_branch_snapshot_with_capability_report(
        &mut self,
        branch_id: u64,
        snapshot: RuntimeSnapshot,
    ) -> Result<WorkerReplayRestoreCapabilityReport, WorthSignalJsError> {
        let report = WorkerReplayRestoreCapabilityReport::same_runtime_exact_restore(
            self, branch_id, snapshot,
        )?;
        self.latest_worker_replay_restore_capability_report = Some(report.clone());
        Ok(report)
    }

    pub fn certify_worker_replay_restore_capability(
        &mut self,
    ) -> Result<WorkerReplayRestoreCapabilityCertificationPackage, WorthSignalJsError> {
        let package =
            WorkerReplayRestoreCapabilityCertificationPackage::from_retained_report(self)?;
        self.latest_worker_replay_restore_capability_certification = Some(package.clone());
        Ok(package)
    }

    pub(in crate::runtime::worker_host) fn latest_worker_replay_restore_capability_report(
        &self,
    ) -> Result<&WorkerReplayRestoreCapabilityReport, WorthSignalJsError> {
        self.latest_worker_replay_restore_capability_report
            .as_ref()
            .ok_or_else(|| {
                WorthSignalJsError::invalid_input(
                    "worker replay restore certification requires replay/restore evidence",
                )
            })
    }
}
