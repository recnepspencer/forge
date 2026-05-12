use serde::Serialize;

use crate::boundary::errors::ForgeSignalJsError;
use crate::recipe::model::TransactionOp;
use crate::runtime::core::RuntimeCore;
use forge_signal::facade::history::RuntimeSnapshot;

use super::{
    committed_truth_digest_for_runtime, publish_definition_envelope_into_worker_runtime,
    WorkerBranchLifecycleTruthReport, WorkerBranchTruthEnvelope, WorkerPortableGraphPublication,
    WorkerRuntimeShell,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerBranchLifecycleParityProbeReport {
    pub main_branch_report: WorkerBranchLifecycleTruthReport,
    pub restored_branch_report: WorkerBranchLifecycleTruthReport,
}

impl WorkerBranchLifecycleParityProbeReport {
    pub fn new(
        main_branch_report: WorkerBranchLifecycleTruthReport,
        restored_branch_report: WorkerBranchLifecycleTruthReport,
    ) -> Self {
        Self {
            main_branch_report,
            restored_branch_report,
        }
    }
}

pub fn probe_worker_branch_lifecycle_parity(
    publication: WorkerPortableGraphPublication,
    feature_transaction_ops: Vec<TransactionOp>,
    main_transaction_ops: Vec<TransactionOp>,
) -> Result<WorkerBranchLifecycleParityProbeReport, ForgeSignalJsError> {
    let mut worker_shell = WorkerRuntimeShell::new(publication.policy.clone())?;
    worker_shell.publish_graph(publication.clone())?;
    let mut compatibility_runtime = RuntimeCore::new(publication.policy.clone())?;
    publish_definition_envelope_into_worker_runtime(
        &mut compatibility_runtime,
        publication.into_definition_envelope(),
    )?;

    let worker_main = worker_shell.branch_truth_envelope()?;
    let compatibility_main = compatibility_runtime.current_branch();
    let worker_feature = worker_shell.create_branch("worker-parity-feature".to_owned())?;
    let compatibility_feature =
        compatibility_runtime.create_branch("worker-parity-feature".to_owned())?;

    let feature_snapshot_pair = capture_feature_branch_snapshots(
        &mut worker_shell,
        &mut compatibility_runtime,
        worker_feature.id.0,
        compatibility_feature.id.0,
        feature_transaction_ops,
    )?;
    let main_branch_report = commit_main_branch_after_feature_snapshot(
        &mut worker_shell,
        &mut compatibility_runtime,
        worker_main.branch_id,
        compatibility_main.id.0,
        main_transaction_ops,
    )?;
    let restored_branch_report = restore_feature_branch_snapshots(
        &mut worker_shell,
        &mut compatibility_runtime,
        worker_feature.id.0,
        compatibility_feature.id.0,
        feature_snapshot_pair,
    )?;

    Ok(WorkerBranchLifecycleParityProbeReport::new(
        main_branch_report,
        restored_branch_report,
    ))
}

fn capture_feature_branch_snapshots(
    worker_shell: &mut WorkerRuntimeShell,
    compatibility_runtime: &mut RuntimeCore,
    worker_feature_branch_id: u64,
    compatibility_feature_branch_id: u64,
    feature_transaction_ops: Vec<TransactionOp>,
) -> Result<(RuntimeSnapshot, RuntimeSnapshot), ForgeSignalJsError> {
    worker_shell.switch_branch(worker_feature_branch_id)?;
    compatibility_runtime.switch_branch(compatibility_feature_branch_id)?;
    worker_shell.apply_committed_transaction(feature_transaction_ops.clone())?;
    compatibility_runtime.apply_transaction(feature_transaction_ops)?;

    Ok((
        worker_shell.branch_snapshot(worker_feature_branch_id)?,
        compatibility_runtime.branch_snapshot(compatibility_feature_branch_id)?,
    ))
}

fn commit_main_branch_after_feature_snapshot(
    worker_shell: &mut WorkerRuntimeShell,
    compatibility_runtime: &mut RuntimeCore,
    worker_main_branch_id: u64,
    compatibility_main_branch_id: u64,
    main_transaction_ops: Vec<TransactionOp>,
) -> Result<WorkerBranchLifecycleTruthReport, ForgeSignalJsError> {
    worker_shell.switch_branch(worker_main_branch_id)?;
    compatibility_runtime.switch_branch(compatibility_main_branch_id)?;
    worker_shell.apply_committed_transaction(main_transaction_ops.clone())?;
    compatibility_runtime.apply_transaction(main_transaction_ops)?;

    compare_worker_branch_to_compatibility_runtime(
        &worker_shell.branch_truth_envelope()?,
        compatibility_runtime,
    )
}

fn restore_feature_branch_snapshots(
    worker_shell: &mut WorkerRuntimeShell,
    compatibility_runtime: &mut RuntimeCore,
    worker_feature_branch_id: u64,
    compatibility_feature_branch_id: u64,
    feature_snapshot_pair: (RuntimeSnapshot, RuntimeSnapshot),
) -> Result<WorkerBranchLifecycleTruthReport, ForgeSignalJsError> {
    let (worker_feature_snapshot, compatibility_feature_snapshot) = feature_snapshot_pair;
    let restored_worker_branch =
        worker_shell.restore_branch_snapshot(worker_feature_branch_id, worker_feature_snapshot)?;
    compatibility_runtime.restore_branch_snapshot(
        compatibility_feature_branch_id,
        compatibility_feature_snapshot,
    )?;
    let restored_compatibility_digest = compatibility_runtime
        .branch_state_proof(compatibility_feature_branch_id)?
        .state_digest;

    Ok(WorkerBranchLifecycleTruthReport::compare(
        &restored_worker_branch,
        restored_compatibility_digest,
    ))
}

fn compare_worker_branch_to_compatibility_runtime(
    worker_branch: &WorkerBranchTruthEnvelope,
    compatibility_runtime: &RuntimeCore,
) -> Result<WorkerBranchLifecycleTruthReport, ForgeSignalJsError> {
    Ok(WorkerBranchLifecycleTruthReport::compare(
        worker_branch,
        committed_truth_digest_for_runtime(compatibility_runtime)?,
    ))
}
