use super::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority,
    WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadEvidenceCounters, WorkloadEvidenceGuard,
    WorkloadEvidenceGuardError, WorkloadEvidenceRow, WorkloadEvidenceStage,
    WorkloadEvidenceStageIndexProduct, WorkloadEvidenceStageLinkSet,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadEvidenceLedger {
    stage_index: WorkloadEvidenceStageIndexProduct,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompleteWorkloadEvidenceLedger {
    ledger: WorkloadEvidenceLedger,
}

impl WorkloadEvidenceLedger {
    pub fn from_rows(rows: Vec<WorkloadEvidenceRow>) -> Result<Self, WorkloadEvidenceLedgerError> {
        Ok(Self {
            stage_index: WorkloadEvidenceStageIndexProduct::new(rows)?,
        })
    }

    pub(crate) fn rows(&self) -> &[WorkloadEvidenceRow] {
        self.stage_index.rows()
    }

    pub fn stage_index(&self) -> &WorkloadEvidenceStageIndexProduct {
        &self.stage_index
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
        let stage_index_counters = self.stage_index.counters();
        WorkloadEvidenceCounters::new(
            stage_index_counters.row_count(),
            stage_index_counters
                .boolean_row_count()
                .saturating_sub(stage_index_counters.counterless_boolean_row_count()),
        )
    }

    pub fn guards(&self) -> WorkloadEvidenceGuard<'_> {
        WorkloadEvidenceGuard::new(self.stage_index())
    }

    pub(crate) fn evidence_for_stage(&self, stage: WorkloadEvidenceStage) -> Option<&str> {
        self.stage_index.evidence_for_stage(stage)
    }

    pub fn missing_authority_stage(&self) -> Option<WorkloadEvidenceStage> {
        self.stage_index.missing_authority_stage()
    }

    pub fn covers_authority_stages(&self) -> bool {
        self.missing_authority_stage().is_none()
    }

    pub(crate) fn row_for_stage(
        &self,
        stage: WorkloadEvidenceStage,
    ) -> Option<&WorkloadEvidenceRow> {
        self.stage_index.row_for_stage(stage)
    }

    pub fn require_boolean_receipt_lookup<T: BooleanEvidenceReceipt + 'static>(
        &self,
        receipt: &T,
    ) -> Result<WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadEvidenceLedgerError> {
        self.stage_index.require_boolean_receipt_lookup(receipt)
    }

    pub fn link_required_stages(
        &self,
        stages: &[WorkloadEvidenceStage],
    ) -> Result<WorkloadEvidenceStageLinkSet, WorkloadEvidenceLedgerError> {
        self.stage_index.link_required_stages(stages)
    }

    fn first_manual_authority_stage(&self) -> Option<WorkloadEvidenceStage> {
        self.stage_index.first_manual_authority_stage()
    }

    fn first_unadmitted_authority_stage(&self) -> Option<WorkloadEvidenceStage> {
        self.stage_index.first_unadmitted_authority_stage()
    }
}

impl CompleteWorkloadEvidenceLedger {
    pub(crate) fn rows(&self) -> &[WorkloadEvidenceRow] {
        self.ledger.rows()
    }

    pub fn counters(&self) -> WorkloadEvidenceCounters {
        self.ledger.counters()
    }

    pub fn guards(&self) -> WorkloadEvidenceGuard<'_> {
        self.ledger.guards()
    }

    pub fn stage_index(&self) -> &WorkloadEvidenceStageIndexProduct {
        self.ledger.stage_index()
    }

    pub(crate) fn evidence_for_stage(&self, stage: WorkloadEvidenceStage) -> Option<&str> {
        self.ledger.evidence_for_stage(stage)
    }

    pub(crate) fn row_for_stage(
        &self,
        stage: WorkloadEvidenceStage,
    ) -> Option<&WorkloadEvidenceRow> {
        self.ledger.row_for_stage(stage)
    }

    pub fn require_boolean_receipt<T: BooleanEvidenceReceipt + 'static>(
        &self,
        receipt: &T,
    ) -> Result<(), WorkloadEvidenceLedgerError> {
        self.stage_index().require_boolean_receipt(receipt)
    }

    pub fn require_boolean_receipt_lookup<T: BooleanEvidenceReceipt + 'static>(
        &self,
        receipt: &T,
    ) -> Result<WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadEvidenceLedgerError> {
        self.stage_index().require_boolean_receipt_lookup(receipt)
    }

    pub(crate) fn require_boolean_row_lookup(
        &self,
        stage: WorkloadEvidenceStage,
        evidence_identity: &str,
        support: super::WorkloadEvidenceSupport,
        counters: super::WorkloadEvidenceStageCounters,
    ) -> Result<WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadEvidenceLedgerError> {
        self.stage_index()
            .require_boolean_row_lookup(stage, evidence_identity, support, counters)
    }

    pub fn link_required_stages(
        &self,
        stages: &[WorkloadEvidenceStage],
    ) -> Result<WorkloadEvidenceStageLinkSet, WorkloadEvidenceLedgerError> {
        self.stage_index().link_required_stages(stages)
    }

    pub fn with_boolean_evidence_receipt<T: BooleanEvidenceRowAuthority + 'static>(
        &self,
        receipt: &T,
    ) -> Result<Self, WorkloadEvidenceLedgerError> {
        let mut rows = self.rows().to_vec();
        rows.push(WorkloadEvidenceRow::from_boolean_evidence_receipt(receipt));
        WorkloadEvidenceLedger::from_rows(rows)?.certify_complete()
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
    MismatchedAuthorityStageBinding(WorkloadEvidenceStage, WorkloadEvidenceStage),
    MissingBooleanStage(WorkloadEvidenceStage),
    ManualBooleanStage(WorkloadEvidenceStage),
    CounterlessBooleanStage(WorkloadEvidenceStage),
    SelectedLookupSliceExceedsScope(WorkloadEvidenceStage),
    MismatchedBooleanStageCounters(WorkloadEvidenceStage),
    MismatchedBooleanStage(WorkloadEvidenceStage),
    UnsupportedBooleanStage(WorkloadEvidenceStage),
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
            Self::DuplicateEvidenceStage(stage) => {
                format!(
                    "workload evidence ledger has duplicate {}",
                    stage.human_name()
                )
            }
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
            Self::MismatchedAuthorityStageBinding(stage, upstream_stage) => {
                format!(
                    "workload evidence ledger has {} that does not bind to the indexed {}",
                    stage.human_name(),
                    upstream_stage.human_name()
                )
            }
            Self::MissingBooleanStage(stage) => {
                format!("workload evidence ledger is missing {}", stage.human_name())
            }
            Self::ManualBooleanStage(stage) => {
                format!(
                    "workload evidence ledger has hand-filled {} instead of a source receipt",
                    stage.human_name()
                )
            }
            Self::CounterlessBooleanStage(stage) => {
                format!(
                    "workload evidence ledger cannot count {} without receipt-backed counters",
                    stage.human_name()
                )
            }
            Self::SelectedLookupSliceExceedsScope(stage) => {
                format!(
                    "selected lookup slice cannot include unrelated {} evidence",
                    stage.human_name()
                )
            }
            Self::MismatchedBooleanStageCounters(stage) => {
                format!(
                    "workload evidence ledger counter identity does not match the {} receipt",
                    stage.human_name()
                )
            }
            Self::MismatchedBooleanStage(stage) => {
                format!(
                    "workload evidence ledger does not match the {} receipt",
                    stage.human_name()
                )
            }
            Self::UnsupportedBooleanStage(stage) => {
                format!(
                    "workload evidence ledger records {} with the wrong support posture",
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
