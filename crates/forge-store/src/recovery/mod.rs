mod backup;
mod degraded;
mod execution;
mod maintenance;
mod planning;
mod precedence;
mod report;
mod support;

use crate::bulk::BulkPlanKind;
use crate::wal::{DurableMutationId, RecoveryDecisionClass};
use forge_relational::facade::history::CommitId;
use serde::Serialize;

pub(crate) use backup::build_backup_restore_compatibility_report;
pub use backup::{
    BackupRestoreCompatibilityReport, BackupRestoreIncompatibility,
    BackupRestoreIncompatibilityKind, ObservedSnapshotVersionTuple,
};
pub use degraded::{
    DegradedStateReport, DurableDegradedRecovery, DurableRecoveryDegradedKind,
    RecoveryQuarantineScope,
};
pub(crate) use execution::{evaluate_recovery_for_mutation, RecoveryAction};
pub(crate) use maintenance::{
    build_maintenance_recovery_report, classify_snapshot_maintenance_recovery,
};
pub use maintenance::{
    MaintenanceArtifactFamily, MaintenanceRecoveryDisposition, MaintenanceRecoveryEntry,
    MaintenanceRecoveryReport,
};
pub use maintenance::{SnapshotMaintenanceRecoveryAction, SnapshotMaintenanceRecoveryReport};
pub(crate) use planning::build_recovery_plan;
pub(crate) use precedence::{build_recovery_source_set, select_recovery_source};
pub use precedence::{RecoverySourceKind, RecoverySourceReport};
pub use report::{
    BulkRecoveryDisposition, BulkRecoverySummary, DurableRecoverySourceSummary, RecoveredBulkChunk,
    RecoveryOperatorAction, RecoveryOperatorActionKind, RecoveryOperatorDisposition,
    RecoveryStatusReport, ResumeEligibleRecoveredBulkChunk,
};
pub(crate) use support::build_support_artifact_recovery_report;
pub use support::{
    SupportArtifactFamily, SupportArtifactRecoveryDisposition, SupportArtifactRecoveryEntry,
    SupportArtifactRecoveryReport,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DurableRecoveryPlan {
    pub pending_durable_mutation_ids: Vec<DurableMutationId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DurableRecoveryDecision {
    pub durable_mutation_id: DurableMutationId,
    pub decision: RecoveryDecisionClass,
    pub commit_id: Option<CommitId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DurableRecoveryOutcome {
    pub decisions: Vec<DurableRecoveryDecision>,
    pub degraded: Vec<DurableDegradedRecovery>,
    pub source_reports: Vec<RecoverySourceReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum DurableMutationIdentity {
    GenericOperation {
        operation_name: String,
    },
    BulkChunk {
        plan_kind: BulkPlanKind,
        program_id: String,
        plan_id: String,
        chunk_ordinal: u64,
    },
}

impl DurableRecoveryOutcome {
    pub(crate) fn mutation_identity(
        &self,
        durable_mutation_id: DurableMutationId,
    ) -> Option<&DurableMutationIdentity> {
        self.source_reports
            .iter()
            .find(|report| report.durable_mutation_id() == durable_mutation_id)
            .map(|report| report.mutation_identity())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum DurableRetryResolution {
    PreviouslyAcknowledgedEquivalentCommit {
        commit_id: CommitId,
    },
    NotPreviouslyPublished,
    RetryRequiresOperatorOrHigherLevelPolicy {
        durable_mutation_id: DurableMutationId,
    },
}

impl DurableRecoveryOutcome {
    pub fn degraded_state_report(&self) -> DegradedStateReport {
        DegradedStateReport::from_entries(&self.degraded)
    }
}
