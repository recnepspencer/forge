use super::{
    WorkloadEvidenceCounters, WorkloadEvidenceGuard, WorkloadEvidenceGuardError,
    WorkloadEvidenceRow, WorkloadEvidenceStage,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadEvidenceLedger {
    rows: Vec<WorkloadEvidenceRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompleteWorkloadEvidenceLedger {
    ledger: WorkloadEvidenceLedger,
}

impl WorkloadEvidenceLedger {
    pub fn from_rows(rows: Vec<WorkloadEvidenceRow>) -> Result<Self, WorkloadEvidenceLedgerError> {
        if rows.is_empty() {
            return Err(WorkloadEvidenceLedgerError::EmptyLedger);
        }
        if rows
            .iter()
            .any(|row| row.evidence_identity().trim().is_empty())
        {
            return Err(WorkloadEvidenceLedgerError::MissingEvidenceIdentity);
        }
        if let Some(duplicate_stage) = duplicate_stage(&rows) {
            return Err(WorkloadEvidenceLedgerError::DuplicateEvidenceStage(
                duplicate_stage,
            ));
        }
        Ok(Self { rows })
    }

    pub fn rows(&self) -> &[WorkloadEvidenceRow] {
        &self.rows
    }

    pub fn certify_complete(
        self,
    ) -> Result<CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedgerError> {
        if let Some(stage) = self.missing_authority_stage() {
            return Err(WorkloadEvidenceLedgerError::MissingAuthorityStage(stage));
        }
        if let Some(stage) = self.first_manual_authority_stage() {
            return Err(WorkloadEvidenceLedgerError::ManualAuthorityStage(stage));
        }
        if let Some(stage) = self.first_unadmitted_authority_stage() {
            return Err(WorkloadEvidenceLedgerError::UnadmittedAuthorityStage(stage));
        }
        Ok(CompleteWorkloadEvidenceLedger { ledger: self })
    }

    pub fn counters(&self) -> WorkloadEvidenceCounters {
        WorkloadEvidenceCounters::new(self.rows.len())
    }

    pub fn guards(&self) -> WorkloadEvidenceGuard<'_> {
        WorkloadEvidenceGuard::new(self)
    }

    pub fn evidence_for_stage(&self, stage: WorkloadEvidenceStage) -> Option<&str> {
        self.rows
            .iter()
            .find(|row| row.stage() == stage)
            .map(WorkloadEvidenceRow::evidence_identity)
    }

    pub fn missing_authority_stage(&self) -> Option<WorkloadEvidenceStage> {
        WorkloadEvidenceStage::AUTHORITY_STAGES
            .iter()
            .copied()
            .find(|stage| self.evidence_for_stage(*stage).is_none())
    }

    pub fn covers_authority_stages(&self) -> bool {
        self.missing_authority_stage().is_none()
    }

    pub fn row_for_stage(&self, stage: WorkloadEvidenceStage) -> Option<&WorkloadEvidenceRow> {
        self.rows.iter().find(|row| row.stage() == stage)
    }

    fn first_manual_authority_stage(&self) -> Option<WorkloadEvidenceStage> {
        WorkloadEvidenceStage::AUTHORITY_STAGES
            .iter()
            .copied()
            .find(|stage| {
                self.row_for_stage(*stage)
                    .is_some_and(|row| !row.is_receipt_backed())
            })
    }

    fn first_unadmitted_authority_stage(&self) -> Option<WorkloadEvidenceStage> {
        WorkloadEvidenceStage::AUTHORITY_STAGES
            .iter()
            .copied()
            .find(|stage| {
                self.row_for_stage(*stage)
                    .is_some_and(|row| row.is_receipt_backed() && !row.is_admitted())
            })
    }
}

impl CompleteWorkloadEvidenceLedger {
    pub fn rows(&self) -> &[WorkloadEvidenceRow] {
        self.ledger.rows()
    }

    pub fn counters(&self) -> WorkloadEvidenceCounters {
        self.ledger.counters()
    }

    pub fn guards(&self) -> WorkloadEvidenceGuard<'_> {
        self.ledger.guards()
    }

    pub fn evidence_for_stage(&self, stage: WorkloadEvidenceStage) -> Option<&str> {
        self.ledger.evidence_for_stage(stage)
    }

    pub fn row_for_stage(&self, stage: WorkloadEvidenceStage) -> Option<&WorkloadEvidenceRow> {
        self.ledger.row_for_stage(stage)
    }

    pub fn into_ledger(self) -> WorkloadEvidenceLedger {
        self.ledger
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadEvidenceLedgerError {
    EmptyLedger,
    MissingEvidenceIdentity,
    DuplicateEvidenceStage(WorkloadEvidenceStage),
    MissingAuthorityStage(WorkloadEvidenceStage),
    ManualAuthorityStage(WorkloadEvidenceStage),
    UnadmittedAuthorityStage(WorkloadEvidenceStage),
    GuardFailed(WorkloadEvidenceGuardError),
}

impl WorkloadEvidenceLedgerError {
    pub fn human_reason(self) -> String {
        match self {
            Self::EmptyLedger => {
                "workload evidence ledger requires at least one evidence row".to_string()
            }
            Self::MissingEvidenceIdentity => {
                "workload evidence rows require a readable identity".to_string()
            }
            Self::DuplicateEvidenceStage(stage) => match stage {
                WorkloadEvidenceStage::Topology => {
                    "workload evidence ledger has duplicate topology evidence"
                }
                WorkloadEvidenceStage::GeometryBinding => {
                    "workload evidence ledger has duplicate geometry binding evidence"
                }
                WorkloadEvidenceStage::SurfaceSupport => {
                    "workload evidence ledger has duplicate surface support evidence"
                }
                WorkloadEvidenceStage::Projection => {
                    "workload evidence ledger has duplicate projection evidence"
                }
                WorkloadEvidenceStage::Transform => {
                    "workload evidence ledger has duplicate transform evidence"
                }
                WorkloadEvidenceStage::RetainedReplay => {
                    "workload evidence ledger has duplicate retained replay evidence"
                }
                WorkloadEvidenceStage::Diagnostics => {
                    "workload evidence ledger has duplicate diagnostic evidence"
                }
                WorkloadEvidenceStage::Response => {
                    "workload evidence ledger has duplicate response evidence"
                }
                WorkloadEvidenceStage::Operator => {
                    "workload evidence ledger has duplicate operator evidence"
                }
            }
            .to_string(),
            Self::MissingAuthorityStage(stage) => {
                format!("workload evidence ledger is missing {}", stage.human_name())
            }
            Self::ManualAuthorityStage(stage) => {
                format!(
                    "workload evidence ledger has hand-filled {} instead of a source receipt",
                    stage.human_name()
                )
            }
            Self::UnadmittedAuthorityStage(stage) => {
                format!(
                    "workload evidence ledger cannot complete because {} is not admitted",
                    stage.human_name()
                )
            }
            Self::GuardFailed(error) => error.human_reason().to_string(),
        }
    }
}

impl From<WorkloadEvidenceGuardError> for WorkloadEvidenceLedgerError {
    fn from(error: WorkloadEvidenceGuardError) -> Self {
        Self::GuardFailed(error)
    }
}

fn duplicate_stage(rows: &[WorkloadEvidenceRow]) -> Option<WorkloadEvidenceStage> {
    rows.iter().enumerate().find_map(|(row_index, row)| {
        rows.iter()
            .skip(row_index + 1)
            .any(|candidate| candidate.stage() == row.stage())
            .then_some(row.stage())
    })
}
