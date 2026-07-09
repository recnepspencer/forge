use crate::wal::DurableMutationId;
use worth_relational::facade::history::CommitId;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RecoveryQuarantineScope {
    ArtifactInstance,
    ArtifactFamily,
    Branch,
    Tenant,
    StoreWide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DurableRecoveryDegradedKind {
    RebuildRequired,
    QuarantineRequired,
    SalvageRequired,
    RetainedWithoutAcknowledgment,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DurableDegradedRecovery {
    pub durable_mutation_id: DurableMutationId,
    pub kind: DurableRecoveryDegradedKind,
    pub scope: RecoveryQuarantineScope,
    pub commit_id: Option<CommitId>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DegradedStateReport {
    quarantines: Vec<DurableDegradedRecovery>,
    salvages: Vec<DurableDegradedRecovery>,
    rebuilds: Vec<DurableDegradedRecovery>,
    retained_without_acknowledgment: Vec<DurableDegradedRecovery>,
}

impl DegradedStateReport {
    pub(crate) fn from_entries(entries: &[DurableDegradedRecovery]) -> Self {
        let mut quarantines = Vec::new();
        let mut salvages = Vec::new();
        let mut rebuilds = Vec::new();
        let mut retained_without_acknowledgment = Vec::new();

        for degraded in entries {
            match degraded.kind {
                DurableRecoveryDegradedKind::QuarantineRequired => {
                    quarantines.push(degraded.clone())
                }
                DurableRecoveryDegradedKind::SalvageRequired => salvages.push(degraded.clone()),
                DurableRecoveryDegradedKind::RebuildRequired => rebuilds.push(degraded.clone()),
                DurableRecoveryDegradedKind::RetainedWithoutAcknowledgment => {
                    retained_without_acknowledgment.push(degraded.clone())
                }
            }
        }

        Self {
            quarantines,
            salvages,
            rebuilds,
            retained_without_acknowledgment,
        }
    }

    pub fn quarantines(&self) -> &[DurableDegradedRecovery] {
        &self.quarantines
    }

    pub fn salvages(&self) -> &[DurableDegradedRecovery] {
        &self.salvages
    }

    pub fn rebuilds(&self) -> &[DurableDegradedRecovery] {
        &self.rebuilds
    }

    pub fn retained_without_acknowledgment(&self) -> &[DurableDegradedRecovery] {
        &self.retained_without_acknowledgment
    }
}
