use std::sync::Arc;

use super::{
    PerformedRelationalCommit, PreparedBranchPublicationPreflight, RelationalPublicationDeferred,
    RelationalPublicationDenial, RelationalPublicationFailure, RelationalPublicationFailureKind,
    RelationalPublicationOutcome, StaleRelationalBranchObservation,
};

impl PreparedBranchPublicationPreflight {
    pub(super) fn perform_cutover(mut self) -> RelationalPublicationOutcome {
        let _publication_lifecycle = self
            .movement
            .canonical_publication_route
            .enter_publication();
        if std::time::Instant::now() >= self.expires_at {
            return RelationalPublicationOutcome::deferred(
                RelationalPublicationDeferred::CandidateLifetimeExpired {
                    maximum_lifetime_millis: self.maximum_lifetime_millis,
                },
            );
        }
        match self
            .control
            .observe(crate::mvcc::RelationalInterruptionBoundary::BeforeCriticalSection)
        {
            Some(event)
                if event.interruption()
                    == crate::mvcc::RelationalOperationInterruption::Cancelled =>
            {
                self.next_basis
                    .inner
                    .retention_binding
                    .record_interruption(event);
                return RelationalPublicationOutcome::interrupted(event);
            }
            Some(event) => {
                self.next_basis
                    .inner
                    .retention_binding
                    .record_interruption(event);
                return RelationalPublicationOutcome::interrupted(event);
            }
            None => {}
        }
        let _critical_section = self.publication_cell.coordination().enter();
        if std::time::Instant::now() >= self.expires_at {
            return RelationalPublicationOutcome::deferred(
                RelationalPublicationDeferred::CandidateLifetimeExpired {
                    maximum_lifetime_millis: self.maximum_lifetime_millis,
                },
            );
        }
        if let Some(event) = self
            .control
            .observe(crate::mvcc::RelationalInterruptionBoundary::BeforeCriticalSection)
        {
            self.next_basis
                .inner
                .retention_binding
                .record_interruption(event);
            return RelationalPublicationOutcome::interrupted(event);
        }
        #[cfg(any(test, feature = "test-operation-control"))]
        self.control.pause_inside_critical_section();
        let mut publication_state = self.publication_cell.enter_state();
        match publication_state.lifecycle_posture() {
            crate::branch::RelationalBranchLifecyclePosture::Live => {}
            crate::branch::RelationalBranchLifecyclePosture::Archived => {
                return RelationalPublicationOutcome::denied(RelationalPublicationDenial::Archived);
            }
            crate::branch::RelationalBranchLifecyclePosture::Deleting => {
                return RelationalPublicationOutcome::denied(RelationalPublicationDenial::Deleting);
            }
        }
        let observed_cell = publication_state.snapshot_cell();
        let Some(observed_root) = observed_cell.root() else {
            return RelationalPublicationOutcome::failed(RelationalPublicationFailure::new(
                RelationalPublicationFailureKind::SelectedRootUnavailable,
                "selected branch root is unavailable before publication movement",
            ));
        };
        let observed = match crate::branch::descriptor_for_cell(&observed_cell, &observed_root) {
            Ok(descriptor) => descriptor,
            Err(denial) => {
                return RelationalPublicationOutcome::failed(RelationalPublicationFailure::new(
                    RelationalPublicationFailureKind::BranchObservation(denial.clone()),
                    format!("branch observation failed before movement: {denial:?}"),
                ));
            }
        };
        if observed != self.expected {
            return RelationalPublicationOutcome::stale(StaleRelationalBranchObservation::new(
                self.expected,
                observed,
            ));
        }

        let previous_head_version = observed_root
            .canonical_envelope()
            .map(|envelope| envelope.commit.version_id);
        let next_head_version = self
            .movement
            .root
            .canonical_envelope()
            .map(|envelope| envelope.commit.version_id);

        let retired_root = std::cell::RefCell::new(None);
        let head_retirement = &mut self.head_retirement;
        let next_root = &self.movement.root;
        let positioned_commit = match self
            .movement
            .canonical_publication_route
            .record_performed_with_cutover(self.publication_cell.clone(), || {
                let previous_root = publication_state.replace_with(self.next_state);
                head_retirement.transfer_head(&previous_root, next_root);
                self.branch_head_versions
                    .move_head(previous_head_version, next_head_version);
                *retired_root.borrow_mut() = Some(previous_root);
                drop(publication_state);
            }) {
            Ok(positioned) => positioned,
            Err(crate::runtime::CanonicalPublicationRecordError::ReservationContended) => {
                return RelationalPublicationOutcome::deferred(
                    RelationalPublicationDeferred::PatchPositionReservationContended,
                );
            }
            Err(crate::runtime::CanonicalPublicationRecordError::PositionCapacityExhausted) => {
                return RelationalPublicationOutcome::failed(RelationalPublicationFailure::new(
                    RelationalPublicationFailureKind::PatchPositionCapacityExhausted,
                    "performed publication stream position capacity exhausted",
                ));
            }
        };
        let retired_root = retired_root
            .into_inner()
            .expect("performed cutover returns the exact previous head root");
        #[cfg(any(test, feature = "test-operation-control"))]
        self.control.pause_after_linearization();
        drop(_critical_section);
        self.head_retirement.replace_head(retired_root);
        drop(_publication_lifecycle);
        let settlement_retention = self
            .candidate_retention
            .into_performed_settlement(Arc::clone(&self.movement.root));
        self.movement.record_allocations.commit();
        let mut performed = PerformedRelationalCommit::record(
            positioned_commit,
            self.next_basis,
            self.completion,
            settlement_retention,
            self.control.clone(),
        );
        if let Some(interruption) = self
            .control
            .observe(crate::runtime::RelationalInterruptionBoundary::AfterLinearization)
        {
            performed
                .next_basis()
                .inner
                .retention_binding
                .record_interruption(interruption);
            performed.record_late_interruption(interruption);
        }
        RelationalPublicationOutcome::performed(performed)
    }
}
