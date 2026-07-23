use super::*;

pub(super) struct BridgeWritebackAuthorityExecutionArtifacts {
    loop_prevention: BridgeWritebackLoopPreventionReport,
    outcome: BridgeWritebackAuthorityOutcome,
    receipt: Option<TruthWritebackReceipt>,
    execution_record: BridgeWritebackExecutionRecord,
}

impl BridgeWritebackAuthorityExecutionArtifacts {
    pub(super) fn new(
        loop_prevention: BridgeWritebackLoopPreventionReport,
        outcome: BridgeWritebackAuthorityOutcome,
        receipt: Option<TruthWritebackReceipt>,
        execution_record: BridgeWritebackExecutionRecord,
    ) -> Self {
        Self {
            loop_prevention,
            outcome,
            receipt,
            execution_record,
        }
    }

    pub(super) fn outcome(&self) -> &BridgeWritebackAuthorityOutcome {
        &self.outcome
    }

    pub(super) fn execution_record(&self) -> &BridgeWritebackExecutionRecord {
        &self.execution_record
    }

    pub(super) fn into_public_result(
        self,
    ) -> (
        BridgeWritebackLoopPreventionReport,
        BridgeWritebackAuthorityOutcome,
        Option<TruthWritebackReceipt>,
    ) {
        (self.loop_prevention, self.outcome, self.receipt)
    }
}
