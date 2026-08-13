//! Crate-private execution evidence retained only for governed Bank recovery.

use bank_domain::schema::BankSchema;
use worth_query_host::facade::primary_graph::{
    WorthQueryAftermathDerivationFailure, WorthQueryApplicationCommitReceipt,
    WorthQueryApplicationHistoricalRead, WorthQueryCommittedDispatchOutboxObservation,
    WorthQueryCommittedDispatchOutboxReadDenial, WorthQueryPrimaryGraphApplicationRuntime,
    WorthQueryRecoveryHandle, WorthQueryRecoveryHandleDenial,
};
use worth_query_host::facade::provisional_aftermath::{
    WorthQueryRedoRecovery, WorthQueryUndoProgressionHandoff,
};

#[derive(Clone)]
pub(crate) struct BankCommitRecoveryEvidence {
    execution: WorthQueryApplicationCommitReceipt,
}

impl BankCommitRecoveryEvidence {
    pub(super) const fn from_execution(execution: WorthQueryApplicationCommitReceipt) -> Self {
        Self { execution }
    }

    pub(crate) fn historical_read(&self) -> WorthQueryApplicationHistoricalRead {
        WorthQueryApplicationHistoricalRead::at_application_commit(&self.execution)
    }

    pub(crate) fn observe_dispatch_outbox(
        &self,
        runtime: &WorthQueryPrimaryGraphApplicationRuntime<BankSchema>,
    ) -> Result<
        Option<WorthQueryCommittedDispatchOutboxObservation>,
        WorthQueryCommittedDispatchOutboxReadDenial,
    > {
        runtime.observe_committed_dispatch_outbox(&self.execution)
    }

    pub(crate) fn mint_recovery_handle(
        &self,
        runtime: &WorthQueryPrimaryGraphApplicationRuntime<BankSchema>,
    ) -> Result<WorthQueryRecoveryHandle, WorthQueryRecoveryHandleDenial> {
        runtime.mint_recovery_handle(&self.execution)
    }

    pub(crate) fn seal_redo_recovery(
        &self,
        handoff: WorthQueryUndoProgressionHandoff,
    ) -> Result<WorthQueryRedoRecovery, WorthQueryAftermathDerivationFailure> {
        WorthQueryRedoRecovery::from_completed_undo(handoff, &self.execution)
    }
}
