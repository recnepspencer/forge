use std::path::Path;

use worth_store_certification::courtroom::operational_recovery::{
    required_s10_crash_reopen_yieldpoints, S10OperationalScenarioKind,
};
use worth_store_operations::certification_scenario::{
    execute_scenario_restore_staging, OwnerBackedBackupScenario,
};
use worth_store_physical_certification::{
    DrivenOperationalControlStore, OperationalRecoveryControlTransitionKind as Control,
    OperationalRecoveryProcessCrashConfig, OperationalRecoveryProductionDriver,
    OperationalRecoveryYieldpoint as Yieldpoint,
};

use super::{execute_forensics, repair, select_control};

pub fn execute_scenario_crash_probe(
    kind: S10OperationalScenarioKind,
    identity: &str,
    media_root: &Path,
    config: OperationalRecoveryProcessCrashConfig,
) {
    assert!(required_s10_crash_reopen_yieldpoints(kind).contains(&config.yieldpoint()));
    let scenario = OwnerBackedBackupScenario::materialize_at(identity, 1, media_root);
    let program = CrashProbeProgram::for_yieldpoint(config.yieldpoint());
    let driver = OperationalRecoveryProductionDriver::crash_once_at(config);
    let control = scenario.control_store();
    let driven = DrivenOperationalControlStore::new(&control, &driver);
    match program {
        CrashProbeProgram::BackupCut => {
            let _ = scenario.execute_named(identity, "restore-source", &driven);
        }
        CrashProbeProgram::BackupAbandonment => {
            let _ = scenario.abandon(identity, &driven);
        }
        CrashProbeProgram::RestorePublication => {
            let source = scenario
                .execute_named(identity, "restore-source", &driven)
                .into_restore_source();
            let staged = execute_scenario_restore_staging(
                &format!("{identity}/restore"),
                source,
                &scenario.workspace_root().join("restore"),
                scenario.authority(),
                &control,
                &driven,
            );
            super::super::recovery_publication::publish_restore(
                identity, staged, &scenario, &control, &driven,
            );
        }
        CrashProbeProgram::Repair => {
            let _ = repair::execute(identity, &scenario, &control, &driven, &mut Vec::new());
        }
        CrashProbeProgram::ReplicaLifecycle { rejoin } => {
            let _ = super::super::replication::execute_replica_lifecycle(
                identity, &scenario, &control, &driver, &driven, rejoin,
            );
        }
        CrashProbeProgram::ForensicAcquisition => {
            let _ = execute_forensics(identity, &scenario, &driver);
        }
        CrashProbeProgram::Audit => {
            let _ = scenario.execute_named(identity, "restore-source", &driven);
            let selected = select_control(&scenario, &control);
            let _ = super::audit::derive_audits(&driver, &selected);
        }
    }
}

#[derive(Clone, Copy)]
enum CrashProbeProgram {
    BackupCut,
    BackupAbandonment,
    RestorePublication,
    Repair,
    ReplicaLifecycle { rejoin: bool },
    ForensicAcquisition,
    Audit,
}

impl CrashProbeProgram {
    fn for_yieldpoint(point: Yieldpoint) -> Self {
        match point {
            Yieldpoint::BeforeDurableControlTransition(control)
            | Yieldpoint::AfterDurableControlTransition(control) => {
                Self::for_control_transition(control)
            }
            Yieldpoint::BeforeForensicSourceAcquisition
            | Yieldpoint::AfterForensicSourceRecord
            | Yieldpoint::BeforeForensicFinalization
            | Yieldpoint::AfterForensicFinalization => Self::ForensicAcquisition,
            Yieldpoint::BeforeBootstrapTransfer
            | Yieldpoint::AfterBootstrapTransfer
            | Yieldpoint::BeforeBootstrapControlRecord
            | Yieldpoint::AfterBootstrapControlRecord
            | Yieldpoint::BeforeBootstrapPostVerification
            | Yieldpoint::AfterBootstrapPostVerification
            | Yieldpoint::BeforeBootstrapCompletion
            | Yieldpoint::AfterBootstrapCompletion
            | Yieldpoint::BeforePromotionExternalFence
            | Yieldpoint::AfterPromotionExternalFence
            | Yieldpoint::BeforePromotionFenceRecord
            | Yieldpoint::AfterPromotionFenceRecord
            | Yieldpoint::BeforePromotionRecord
            | Yieldpoint::AfterPromotionRecord
            | Yieldpoint::BeforePromotionPostVerification
            | Yieldpoint::AfterPromotionPostVerification
            | Yieldpoint::BeforePromotionPublication
            | Yieldpoint::AfterPromotionPublication
            | Yieldpoint::BeforePromotionReadmission
            | Yieldpoint::AfterPromotionReadmission => Self::ReplicaLifecycle { rejoin: false },
            Yieldpoint::BeforeOldPrimaryRejoinPlan
            | Yieldpoint::AfterOldPrimaryRejoinPlan
            | Yieldpoint::BeforeOldPrimaryRejoinExecution
            | Yieldpoint::AfterOldPrimaryRejoinExecution
            | Yieldpoint::BeforeOldPrimaryRejoinCompletion
            | Yieldpoint::AfterOldPrimaryRejoinCompletion => {
                Self::ReplicaLifecycle { rejoin: true }
            }
            Yieldpoint::BeforeAuditDerivation
            | Yieldpoint::AfterAuditDerivation
            | Yieldpoint::BeforeAuditExport
            | Yieldpoint::AfterAuditExport => Self::Audit,
        }
    }

    fn for_control_transition(control: Control) -> Self {
        match control {
            Control::BackupSourceLease
            | Control::BackupMaterializationOpen
            | Control::BackupMaterializationCompletion
            | Control::IndependentBackupVerification => Self::BackupCut,
            Control::WorkflowAbandonment => Self::BackupAbandonment,
            Control::AuthorizationConsumption
            | Control::RecoveryOwnerReceipt
            | Control::RecoveryStagingCompletion
            | Control::RecoveryPublicationPreparation
            | Control::RecoveryPublicationPending
            | Control::RecoveryPublicationDisposition
            | Control::RecoveryPublicationFenceRelease => Self::RestorePublication,
            Control::RepairExecutionOpen
            | Control::RepairOwnerEffect
            | Control::RepairOwnerReceipt
            | Control::RepairDisposition => Self::Repair,
            Control::ReplicaBootstrapTransfer
            | Control::ReplicaBootstrapCompletion
            | Control::ReplicaPromotionFence
            | Control::ReplicaPromotionRecord
            | Control::ReplicaPromotionPublication
            | Control::ReplicaPromotionReadmission => Self::ReplicaLifecycle { rejoin: false },
            Control::OldPrimaryRejoinPlan | Control::OldPrimaryRejoinCompletion => {
                Self::ReplicaLifecycle { rejoin: true }
            }
        }
    }
}
