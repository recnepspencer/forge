use forge_store_certification::{RecoveryPhysicsCrashLane, RecoveryPhysicsCrashMatrix};
use forge_store_recovery_physics::{
    BoundedRecoveryReceipt, FreshRuntimeCrashRecoveryEvidence, FreshRuntimeRecoveryExecution,
    PersistedRecoveryArtifacts, RecoveryPhysicsCloseoutDenial, RecoveryRuntimeClassification,
    RuntimeRecoveryReport, RuntimeRecoveryReportDenial, S4CrashFaultSchedulerEvidence,
    S4CrashHarnessTranscriptSource, S4LoweredCrashHarnessEvidence, S4RecoveryCrashSeam,
    WalCheckpointLsnRecoveryPhysicsSuite,
};
use forge_store_test_support::{ExecutedS4CrashHarnessTranscript, FreshRuntimeRecoveryDriver};

use super::executed_recovery::{
    executed_reopened_recovery_from_admission,
    executed_reopened_recovery_from_admission_with_redo_lsn_and_operation_digest, CloseoutFixture,
};
use super::foundational_evidence::{
    verified_fresh_runtime_driver, verified_fresh_runtime_driver_for_artifacts,
    verified_reopened_artifact_admission, verified_reopened_artifact_admission_for_artifacts,
    verified_report_for_artifacts,
};

pub fn required_crash_seams() -> [S4RecoveryCrashSeam; 8] {
    [
        S4RecoveryCrashSeam::WalAppend,
        S4RecoveryCrashSeam::PageFlush,
        S4RecoveryCrashSeam::CheckpointManifestWrite,
        S4RecoveryCrashSeam::CheckpointCutover,
        S4RecoveryCrashSeam::CompactionCutover,
        S4RecoveryCrashSeam::Acknowledgment,
        S4RecoveryCrashSeam::DirectorySync,
        S4RecoveryCrashSeam::RenameDurability,
    ]
}

pub fn crash_scheduler_evidence(
    seam: S4RecoveryCrashSeam,
) -> Result<S4CrashFaultSchedulerEvidence, RecoveryPhysicsCloseoutDenial> {
    let fixture = super::executed_recovery::executed_recovery_fixture();
    crash_scheduler_evidence_for_fixture(seam, &fixture)
}

pub fn crash_scheduler_evidence_for_fixture(
    seam: S4RecoveryCrashSeam,
    fixture: &CloseoutFixture,
) -> Result<S4CrashFaultSchedulerEvidence, RecoveryPhysicsCloseoutDenial> {
    let admission = verified_reopened_artifact_admission_for_artifacts(&fixture.artifacts);
    let driver = verified_fresh_runtime_driver_for_artifacts(&admission, &fixture.artifacts);
    let (_crash_receipt, execution) =
        executed_reopened_recovery_from_admission_with_redo_lsn_and_operation_digest(
            &driver,
            fixture.replay_redo_lsn,
            &fixture.operation_digest,
        )
        .unwrap();
    let fresh_runtime_recovery = fresh_runtime_evidence_for_artifacts(
        fixture.receipt.clone(),
        execution,
        &fixture.artifacts,
    )?;
    S4CrashFaultSchedulerEvidence::from_lowered_crash_plan(
        lowered_harness_evidence_for_seam(seam, &fixture.receipt)?,
        fresh_runtime_recovery,
    )
}

pub fn missing_crash_scheduler_evidence_denial() -> RecoveryPhysicsCloseoutDenial {
    RecoveryPhysicsCloseoutDenial::MissingCrashFaultSchedulerEvidence
}

pub fn same_process_runtime_report_denial() -> RuntimeRecoveryReportDenial {
    let admission = verified_reopened_artifact_admission();
    let driver = verified_fresh_runtime_driver(&admission);
    let (_receipt, execution) = executed_reopened_recovery_from_admission(&driver).unwrap();
    FreshRuntimeRecoveryDriver::same_process_live_state_reuse()
        .witness_from_reopened_execution(execution)
        .unwrap_err()
}

fn fresh_runtime_evidence(
    crash_receipt: BoundedRecoveryReceipt,
    execution: FreshRuntimeRecoveryExecution,
) -> Result<FreshRuntimeCrashRecoveryEvidence, RecoveryPhysicsCloseoutDenial> {
    let artifacts = forge_store_test_support::deterministic_s4_recovery_artifacts();
    fresh_runtime_evidence_for_artifacts(crash_receipt, execution, &artifacts)
}

fn fresh_runtime_evidence_for_artifacts(
    crash_receipt: BoundedRecoveryReceipt,
    execution: FreshRuntimeRecoveryExecution,
    artifacts: &PersistedRecoveryArtifacts,
) -> Result<FreshRuntimeCrashRecoveryEvidence, RecoveryPhysicsCloseoutDenial> {
    let runtime_report = runtime_report_for_receipt(&crash_receipt, execution, artifacts);
    FreshRuntimeCrashRecoveryEvidence::from_runtime_report(crash_receipt, runtime_report)
}

fn runtime_report_for_receipt(
    receipt: &BoundedRecoveryReceipt,
    execution: FreshRuntimeRecoveryExecution,
    artifacts: &PersistedRecoveryArtifacts,
) -> RuntimeRecoveryReport {
    let verified_report = verified_report_for_artifacts(artifacts);
    let admission = verified_reopened_artifact_admission_for_artifacts(artifacts);
    let witness = verified_fresh_runtime_driver_for_artifacts(&admission, artifacts)
        .witness_from_reopened_execution(execution)
        .unwrap();
    RuntimeRecoveryReport::from_verified_bounded_recovery(
        witness,
        &verified_report,
        RecoveryRuntimeClassification::Recovered,
        receipt,
        Vec::new(),
    )
    .unwrap()
}

fn lowered_harness_evidence_for_seam(
    seam: S4RecoveryCrashSeam,
    receipt: &BoundedRecoveryReceipt,
) -> Result<S4LoweredCrashHarnessEvidence, RecoveryPhysicsCloseoutDenial> {
    let matrix = RecoveryPhysicsCrashMatrix::roadmap_2_s4()
        .lower()
        .map_err(|_| RecoveryPhysicsCloseoutDenial::MissingCrashFaultSchedulerEvidence)?;
    let lane = lane_for_seam(seam);
    let plan = matrix
        .plan_for_lane(lane)
        .ok_or(RecoveryPhysicsCloseoutDenial::MissingCrashSeam)?;
    let lowered_plan_id = format!("lowered-plan:{:?}:{}", plan.lane(), plan.seed());
    let executed = ExecutedS4CrashHarnessTranscript::execute(seam, lowered_plan_id, receipt)
        .map_err(|_| RecoveryPhysicsCloseoutDenial::MissingCrashFaultSchedulerEvidence)?;
    let source = S4CrashHarnessTranscriptSource::from_roadmap2_transcript(
        executed.seam(),
        executed.lowered_plan_id(),
        executed.storage_boundary_id(),
        executed.observer_transcript_id(),
        executed.proof_oracle_id(),
        executed.seed(),
        executed.backend_profile(),
        executed.fault_ordinal(),
    )?;
    WalCheckpointLsnRecoveryPhysicsSuite::from_required_s4_lanes()
        .admit_lowered_crash_harness_transcript(source)
}

fn lane_for_seam(seam: S4RecoveryCrashSeam) -> RecoveryPhysicsCrashLane {
    match seam {
        S4RecoveryCrashSeam::WalAppend => RecoveryPhysicsCrashLane::WalAppend,
        S4RecoveryCrashSeam::PageFlush => RecoveryPhysicsCrashLane::PageFlush,
        S4RecoveryCrashSeam::CheckpointManifestWrite => RecoveryPhysicsCrashLane::CheckpointWrite,
        S4RecoveryCrashSeam::CheckpointCutover => RecoveryPhysicsCrashLane::CheckpointCutover,
        S4RecoveryCrashSeam::CompactionCutover => RecoveryPhysicsCrashLane::CompactionCutover,
        S4RecoveryCrashSeam::Acknowledgment => RecoveryPhysicsCrashLane::Acknowledgment,
        S4RecoveryCrashSeam::DirectorySync => RecoveryPhysicsCrashLane::DirectorySync,
        S4RecoveryCrashSeam::RenameDurability => RecoveryPhysicsCrashLane::RenameDurability,
    }
}
