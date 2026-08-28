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
        let diagnostic_capture = self.publication.diagnostics.begin_operation_capture();
        let late_interruption = performed.late_interruption();
        let (positioned, next_basis, completion, candidate_retention, control) =
            performed.into_settlement_parts();
        let commit_id = positioned.envelope().commit.commit_id;
        let published_snapshot_basis =
            crate::visibility::snapshot_states::VisibilitySnapshotBasis::from_observation(
                &next_basis.observation(),
            );
        let (mut published, durability_error) =
            publish_commit_execution(self, completion, published_snapshot_basis)?;
        published.append_diagnostics(diagnostic_capture.finish());
        let late_interruption = late_interruption.or_else(|| {
            let event = control.observe(crate::runtime::RelationalInterruptionBoundary::Settlement);
            if let Some(event) = event {
                candidate_retention.record_interruption(event);
            }
            event
        });
        let committed = assemble_commit_result(self, published, late_interruption);
        if let Some(error) = durability_error {
            let snapshot_closeout = self
                .visibility
                .published_snapshot_closeout(committed.snapshot.snapshot_id())
                .expect("deferred publication retains its exact published snapshot");
            let settlement = DeferredPublicationSettlement::new(
                self.runtime_instance_id(),
                std::sync::Arc::clone(&positioned),
                committed,
                snapshot_closeout,
            );
            self.publication_binding()
                .register_deferred_settlement(
                    settlement.clone(),
                    self.config
                        .publication
                        .policy
                        .max_published_snapshot_handles,
                )
                .expect("published snapshot capacity bounds runtime-owned settlement recovery");
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
            self.publication_binding()
                .release_deferred_settlement(commit_id);
            settlement.close_published_snapshot();
            return Ok(positioned.envelope().commit.clone());
        }
        if let Some(durable) = self.durability.durable_log_envelope(commit_id) {
            if durable != positioned.as_ref() {
                return Err(RepairError::PerformedRouteMismatch);
            }
        } else {
            let admission = CommitDurableAppendAdmission::new(
                self.runtime_instance_id(),
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
        self.publication_binding()
            .release_deferred_settlement(commit_id);
        settlement.close_published_snapshot();
        Ok(positioned.envelope().commit.clone())
    }

    /// Retry a performed publication retained by this runtime even when the
    /// caller that first received the external repair capability was lost.
    pub fn repair_pending_publication_settlement(
        &mut self,
        commit_id: crate::history::data::CommitId,
    ) -> Result<crate::history::data::RelationalCommitReceipt, DeferredPublicationSettlementError>
    {
        if let Some(settlement) = self.publication_binding().deferred_settlement(commit_id) {
            return self.repair_deferred_publication_settlement(&settlement);
        }
        let positioned = self
            .history
            .positioned_canonical_commit(commit_id)
            .ok_or(DeferredPublicationSettlementError::RecoveryUnavailable { commit_id })?;
        if self.history.publication_requires_settlement(commit_id) {
            return Err(DeferredPublicationSettlementError::RecoveryUnavailable { commit_id });
        }
        Ok(positioned.envelope().commit.clone())
    }
}
