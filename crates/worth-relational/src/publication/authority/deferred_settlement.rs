use crate::authority::commit::pipeline::{
    assemble_commit_result, publish_commit_execution, CommitDurableAppendAdmission,
};
use crate::publication::data::{DeferredPublicationSettlement, DeferredPublicationSettlementError};
use crate::runtime::RelationalRuntime;
use crate::transactions::data::{CommitResult, TransactionCommitError};

impl RelationalRuntime {
    /// Complete durability and derived publication for a movement already
    /// performed through the independently borrowable publication port.
    pub fn settle_performed_publication(
        &mut self,
        performed: crate::mvcc::PerformedRelationalCommit,
    ) -> Result<CommitResult, TransactionCommitError> {
        let (positioned, _next_basis, completion) = performed.into_settlement_parts();
        let commit_id = positioned.envelope().commit.commit_id;
        let (published, durability_error) = publish_commit_execution(self, completion)?;
        let committed = assemble_commit_result(self, published);
        if let Some(error) = durability_error {
            let settlement = DeferredPublicationSettlement::new(
                self.runtime_instance_id(),
                std::sync::Arc::clone(&positioned),
                committed,
            );
            return Err(TransactionCommitError::performed_but_durability_deferred(
                settlement, error,
            ));
        }
        self.history.mark_publication_settled(commit_id);
        Ok(committed)
    }

    /// Retry the one missing durable append for an exact performed route.
    /// Calling this again after success is harmless and returns the same receipt.
    pub fn repair_deferred_publication_settlement(
        &mut self,
        settlement: &DeferredPublicationSettlement,
    ) -> Result<crate::history::data::RelationalCommitReceipt, DeferredPublicationSettlementError>
    {
        use DeferredPublicationSettlementError as RepairError;

        if settlement.runtime_instance_id() != self.runtime_instance_id() {
            return Err(RepairError::ForeignRuntime {
                expected_runtime_instance_id: settlement.runtime_instance_id(),
                actual_runtime_instance_id: self.runtime_instance_id(),
            });
        }
        let commit_id = settlement.commit().commit_id;
        let Some(positioned) = self.history.positioned_canonical_commit(commit_id) else {
            return Err(RepairError::PerformedRouteMissing);
        };
        if positioned.as_ref() != settlement.positioned().as_ref() {
            return Err(RepairError::PerformedRouteMismatch);
        }
        if !self.history.publication_requires_settlement(commit_id) {
            return Ok(positioned.envelope().commit.clone());
        }
        if let Some(durable) = self.durability.durable_log_envelope(commit_id) {
            if durable != positioned.as_ref() {
                return Err(RepairError::PerformedRouteMismatch);
            }
        } else {
            let admission = CommitDurableAppendAdmission::new(
                self,
                commit_id,
                &positioned.envelope().commit.branch_id,
            );
            let authority =
                crate::durability::authority::DurableAppendAuthority::from_commit(admission);
            self.durability_authority()
                .append_commit(authority, positioned.as_ref())
                .map_err(RepairError::DurableAppend)?;
        }
        self.history.mark_publication_settled(commit_id);
        Ok(positioned.envelope().commit.clone())
    }
}
