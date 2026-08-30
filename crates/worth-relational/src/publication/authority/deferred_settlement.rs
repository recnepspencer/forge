use std::sync::Arc;

use crate::publication::data::{DeferredPublicationSettlement, DeferredPublicationSettlementError};
use crate::runtime::RelationalRuntime;
use crate::transactions::data::{CommitResult, TransactionCommitError};

use super::pending_settlement::{
    RelationalSettlementCompletion, RelationalSettlementResultOwner, RelationalSettlementStop,
};

impl RelationalRuntime {
    /// Complete durability and derived publication for a movement already
    /// performed through the independently borrowable publication port.
    ///
    /// The work itself was installed in the runtime's pending settlement
    /// registry before that movement, so this is the witness holder taking its
    /// turn at the one executor gate, not the sole owner of the work.
    pub fn settle_performed_publication(
        &self,
        performed: crate::mvcc::PerformedRelationalCommit,
    ) -> Result<CommitResult, TransactionCommitError> {
        let (positioned, record) = performed.into_settlement_parts();
        debug_assert_eq!(positioned.envelope().commit.commit_id, record.commit_id());
        if record.runtime_instance_id() != self.runtime_instance_id() {
            return Err(TransactionCommitError::publication_denied(
                crate::mvcc::RelationalPublicationDenial::ForeignRuntime {
                    expected_runtime_instance_id: self.runtime_instance_id(),
                    actual_runtime_instance_id: record.runtime_instance_id(),
                },
            ));
        }
        match self.execute_pending_settlement(&record, RelationalSettlementResultOwner::Witness) {
            Ok(RelationalSettlementCompletion::Performed { committed, .. }) => Ok(committed),
            Ok(RelationalSettlementCompletion::Repeated { committed, .. }) => {
                let retained = committed.expect(
                    "a receipt-only executor retains the commit result for its live witness",
                );
                Ok(Arc::try_unwrap(retained).unwrap_or_else(|shared| shared.as_ref().clone()))
            }
            Err(RelationalSettlementStop::DurabilityDeferred { carrier, error }) => Err(
                TransactionCommitError::performed_but_durability_deferred(carrier, error),
            ),
            Err(RelationalSettlementStop::Unrecoverable(Some(error))) => Err(error),
            Err(stop) => Err(TransactionCommitError::publication(
                crate::publication::data::PublicationError::new(
                    crate::publication::bundle::PublicationStage::DurableAppend,
                    format!("{:?}", stop.into_repair_error(record.commit_id())),
                ),
            )),
        }
    }

    /// Retry the one missing durable append for an exact performed route.
    /// Calling this again after success is harmless and returns the same
    /// receipt, because the carrier only names a record the runtime owns.
    pub fn repair_deferred_publication_settlement(
        &self,
        settlement: &DeferredPublicationSettlement,
    ) -> Result<crate::history::data::RelationalCommitReceipt, DeferredPublicationSettlementError>
    {
        if settlement.runtime_instance_id() != self.runtime_instance_id() {
            return Err(DeferredPublicationSettlementError::ForeignRuntime {
                expected_runtime_instance_id: self.runtime_instance_id(),
                actual_runtime_instance_id: settlement.runtime_instance_id(),
            });
        }
        let commit_id = settlement.commit().commit_id;
        let Some(record) = self.publication_binding().pending_settlement(commit_id) else {
            return self.settled_receipt_without_record(commit_id);
        };
        if record
            .deferred_route()
            .is_some_and(|retained| retained.as_ref() != settlement.positioned().as_ref())
        {
            return Err(DeferredPublicationSettlementError::PerformedRouteMismatch);
        }
        self.execute_pending_settlement(&record, RelationalSettlementResultOwner::Runtime)
            .map(|completion| completion.receipt().clone())
            .map_err(|stop| stop.into_repair_error(commit_id))
    }

    /// Retry a performed publication retained by this runtime even when the
    /// caller that first received the external repair capability was lost.
    pub fn repair_pending_publication_settlement(
        &self,
        commit_id: crate::history::data::CommitId,
    ) -> Result<crate::history::data::RelationalCommitReceipt, DeferredPublicationSettlementError>
    {
        let Some(record) = self.publication_binding().pending_settlement(commit_id) else {
            if self.publication_binding().settlement_admission_is_closed() {
                return Err(DeferredPublicationSettlementError::OwnerUnavailable {
                    runtime_instance_id: self.runtime_instance_id(),
                });
            }
            return self.settled_receipt_without_record(commit_id);
        };
        if record.runtime_instance_id() != self.runtime_instance_id() {
            return Err(DeferredPublicationSettlementError::ForeignRuntime {
                expected_runtime_instance_id: self.runtime_instance_id(),
                actual_runtime_instance_id: record.runtime_instance_id(),
            });
        }
        self.execute_pending_settlement(&record, RelationalSettlementResultOwner::Runtime)
            .map(|completion| completion.receipt().clone())
            .map_err(|stop| stop.into_repair_error(commit_id))
    }
}
