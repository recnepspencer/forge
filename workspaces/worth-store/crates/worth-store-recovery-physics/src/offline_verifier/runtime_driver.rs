use super::{
    FreshRuntimeRecoveryExecution, FreshRuntimeRecoveryWitness, OfflineRecoveryVerificationReport,
    PersistedRecoveryArtifacts, ReopenedRecoveryArtifactAdmission,
    ReopenedRecoveryArtifactAdmissionDenial, ReopenedRuntimeRecoverySession,
    RuntimeRecoveryReportDenial,
};
use crate::{BoundedRecoveryPlan, BoundedRecoveryReceipt, ReopenedRecoveryDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryRuntimePosture {
    FreshRuntimeFromPersistedBytes,
    SameProcessLiveStateReuse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreshRuntimeRecoveryDriver {
    posture: RecoveryRuntimePosture,
    admission: Option<ReopenedRecoveryArtifactAdmission>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreshRuntimeReopenHarnessEvidence {
    admission: ReopenedRecoveryArtifactAdmission,
}

impl FreshRuntimeReopenHarnessEvidence {
    pub(crate) fn from_persisted_artifact_reopen(
        report: OfflineRecoveryVerificationReport,
        artifacts: &PersistedRecoveryArtifacts,
    ) -> Result<Self, ReopenedRecoveryArtifactAdmissionDenial> {
        let admission = ReopenedRecoveryArtifactAdmission::admit(report, artifacts)?;
        Ok(Self { admission })
    }
}

impl FreshRuntimeRecoveryDriver {
    pub fn from_reopen_harness_evidence(evidence: FreshRuntimeReopenHarnessEvidence) -> Self {
        Self {
            posture: RecoveryRuntimePosture::FreshRuntimeFromPersistedBytes,
            admission: Some(evidence.admission),
        }
    }

    pub const fn same_process_live_state_reuse() -> Self {
        Self {
            posture: RecoveryRuntimePosture::SameProcessLiveStateReuse,
            admission: None,
        }
    }

    pub const fn posture(&self) -> RecoveryRuntimePosture {
        self.posture
    }

    pub const fn is_fresh_runtime(&self) -> bool {
        matches!(
            self.posture,
            RecoveryRuntimePosture::FreshRuntimeFromPersistedBytes
        )
    }

    pub fn execute_reopened_runtime_recovery(
        &self,
        plan: &BoundedRecoveryPlan<'_>,
    ) -> Result<(BoundedRecoveryReceipt, FreshRuntimeRecoveryExecution), ReopenedRecoveryDenial>
    {
        let session = self
            .reopen_admitted_artifacts()
            .map_err(ReopenedRecoveryDenial::Runtime)?;
        plan.execute_reopened_runtime_recovery(&session)
    }

    pub(crate) fn reopen_admitted_artifacts(
        &self,
    ) -> Result<ReopenedRuntimeRecoverySession, RuntimeRecoveryReportDenial> {
        match self.posture {
            RecoveryRuntimePosture::FreshRuntimeFromPersistedBytes => self
                .admission
                .as_ref()
                .ok_or(RuntimeRecoveryReportDenial::MissingReopenedRuntimeBoundary)
                .and_then(ReopenedRuntimeRecoverySession::from_fresh_runtime_driver),
            RecoveryRuntimePosture::SameProcessLiveStateReuse => {
                Err(RuntimeRecoveryReportDenial::SameProcessLiveStateReuse)
            }
        }
    }

    pub fn witness_from_reopened_execution(
        &self,
        execution: FreshRuntimeRecoveryExecution,
    ) -> Result<FreshRuntimeRecoveryWitness, RuntimeRecoveryReportDenial> {
        match self.posture {
            RecoveryRuntimePosture::FreshRuntimeFromPersistedBytes => self
                .admission
                .as_ref()
                .ok_or(RuntimeRecoveryReportDenial::MissingReopenedRuntimeBoundary)
                .and_then(|admission| {
                    require_execution_matches_admitted_artifacts(admission, &execution)
                })
                .map(|()| FreshRuntimeRecoveryWitness::from_fresh_runtime_execution(execution)),
            RecoveryRuntimePosture::SameProcessLiveStateReuse => {
                Err(RuntimeRecoveryReportDenial::SameProcessLiveStateReuse)
            }
        }
    }
}

fn require_execution_matches_admitted_artifacts(
    admission: &ReopenedRecoveryArtifactAdmission,
    execution: &FreshRuntimeRecoveryExecution,
) -> Result<(), RuntimeRecoveryReportDenial> {
    if execution.artifact_digest() == admission.artifact_digest()
        && execution.recovery_profile() == admission.recovery_profile()
    {
        return Ok(());
    }
    Err(RuntimeRecoveryReportDenial::FreshRuntimeWitnessMismatch)
}
