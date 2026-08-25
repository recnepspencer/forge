use std::sync::Arc;

use super::{
    PerformedRelationalCommit, PreparedRelationalCommitCandidate, RelationalPublicationDeferred,
    RelationalPublicationDenial, RelationalPublicationFailure, RelationalPublicationFailureKind,
    RelationalPublicationOutcome, StaleRelationalBranchObservation,
};
use crate::branch::{
    RelationalBranchBasisDescriptor, RelationalBranchPublicationCell,
    RelationalBranchReferenceCell, RelationalBranchRoot,
};

/// Independently borrowable Relational owner publication service.
#[derive(Debug, Clone)]
pub struct RelationalPublicationPort {
    runtime_instance_id: u64,
}

pub(crate) struct PreparedCanonicalBranchMovement {
    record_allocations: crate::runtime::PendingRecordAllocations,
    next_cell: RelationalBranchReferenceCell,
    root: Arc<RelationalBranchRoot>,
    canonical_publication_route: crate::runtime::PreparedCanonicalPublicationRoute,
}

struct PreparedBranchPublicationPreflight {
    movement: PreparedCanonicalBranchMovement,
    expected: RelationalBranchBasisDescriptor,
    expected_root: Arc<RelationalBranchRoot>,
    publication_cell: RelationalBranchPublicationCell,
    next_state: crate::branch::RelationalBranchReferenceMutableState,
    next_basis: crate::branch::AdmittedRelationalBranchBasis,
    completion: crate::authority::commit::pipeline::PreparedCommitPublicationCompletion,
}

impl RelationalPublicationPort {
    pub(crate) const fn new(runtime_instance_id: u64) -> Self {
        Self {
            runtime_instance_id,
        }
    }

    pub fn compare_and_publish(
        &self,
        candidate: PreparedRelationalCommitCandidate,
    ) -> RelationalPublicationOutcome {
        if candidate.runtime_instance_id() != self.runtime_instance_id {
            return worth_proof::TransitionOutcome::denied(
                RelationalPublicationDenial::ForeignRuntime {
                    expected_runtime_instance_id: self.runtime_instance_id,
                    actual_runtime_instance_id: candidate.runtime_instance_id(),
                },
            );
        }
        let (expected, expected_root, publication_cell, movement, completion) =
            candidate.into_publication_parts();
        movement.linearize(expected, expected_root, publication_cell, completion)
    }
}

impl PreparedCanonicalBranchMovement {
    pub(crate) fn new(
        record_allocations: crate::runtime::PendingRecordAllocations,
        mut next_cell: RelationalBranchReferenceCell,
        root: Arc<RelationalBranchRoot>,
        canonical_publication_route: crate::runtime::PreparedCanonicalPublicationRoute,
    ) -> Self {
        next_cell.install_root(Arc::clone(&root));
        Self {
            record_allocations,
            next_cell,
            root,
            canonical_publication_route,
        }
    }

    fn linearize(
        self,
        expected: RelationalBranchBasisDescriptor,
        expected_root: Arc<RelationalBranchRoot>,
        publication_cell: RelationalBranchPublicationCell,
        completion: crate::authority::commit::pipeline::PreparedCommitPublicationCompletion,
    ) -> RelationalPublicationOutcome {
        match self.preflight(expected, expected_root, publication_cell, completion) {
            Ok(preflight) => preflight.perform_cutover(),
            Err(outcome) => outcome,
        }
    }

    fn preflight(
        self,
        expected: RelationalBranchBasisDescriptor,
        expected_root: Arc<RelationalBranchRoot>,
        publication_cell: RelationalBranchPublicationCell,
        completion: crate::authority::commit::pipeline::PreparedCommitPublicationCompletion,
    ) -> Result<PreparedBranchPublicationPreflight, RelationalPublicationOutcome> {
        let next_state = self.next_cell.state_snapshot();
        if publication_cell.runtime_instance_id() != expected.runtime_instance_id()
            || publication_cell.branch_id() != expected.branch_id()
        {
            return Err(worth_proof::TransitionOutcome::denied(
                RelationalPublicationDenial::OwnerMismatch,
            ));
        }
        if self.next_cell.identity().runtime_instance_id() != publication_cell.runtime_instance_id()
            || self.next_cell.identity().branch_id() != publication_cell.branch_id()
            || self.next_cell.root().as_ref().map(Arc::as_ptr) != Some(Arc::as_ptr(&self.root))
        {
            return Err(worth_proof::TransitionOutcome::failed(
                RelationalPublicationFailure::new(
                    RelationalPublicationFailureKind::PreparedRootMismatch,
                    "prepared next root does not match its branch publication cell",
                ),
            ));
        }
        let next_descriptor = match crate::branch::descriptor_for_cell(&self.next_cell, &self.root)
        {
            Ok(descriptor) => descriptor,
            Err(denial) => {
                return Err(worth_proof::TransitionOutcome::failed(
                    RelationalPublicationFailure::new(
                        RelationalPublicationFailureKind::PreparedBasisDescriptor(denial.clone()),
                        format!("prepared next basis failed before movement: {denial:?}"),
                    ),
                ));
            }
        };
        let next_basis = crate::branch::issue_admitted_relational_branch_basis(
            next_descriptor,
            self.next_cell.identity().clone(),
            Arc::clone(&self.root),
        );
        let next_basis = match publication_cell.register_basis(next_basis) {
            Ok(basis) => basis,
            Err(denial) => {
                return Err(worth_proof::TransitionOutcome::failed(
                    RelationalPublicationFailure::new(
                        RelationalPublicationFailureKind::NextBasisAdmission(denial.clone()),
                        format!("next basis admission failed before movement: {denial:?}"),
                    ),
                ));
            }
        };
        Ok(PreparedBranchPublicationPreflight {
            movement: self,
            expected,
            expected_root,
            publication_cell,
            next_state,
            next_basis,
            completion,
        })
    }
}

impl PreparedBranchPublicationPreflight {
    fn perform_cutover(self) -> RelationalPublicationOutcome {
        let _publication_lifecycle = self
            .movement
            .canonical_publication_route
            .enter_publication();
        let _critical_section = self.publication_cell.coordination().enter();
        let mut publication_state = self.publication_cell.enter_state();
        let observed_cell = publication_state.snapshot_cell();
        let observed_root = observed_cell.root().unwrap_or(self.expected_root);
        let observed = match crate::branch::descriptor_for_cell(&observed_cell, &observed_root) {
            Ok(descriptor) => descriptor,
            Err(denial) => {
                return worth_proof::TransitionOutcome::failed(RelationalPublicationFailure::new(
                    RelationalPublicationFailureKind::BranchObservation(denial.clone()),
                    format!("branch observation failed before movement: {denial:?}"),
                ));
            }
        };
        if observed != self.expected {
            return worth_proof::TransitionOutcome::stale(StaleRelationalBranchObservation::new(
                self.expected,
                observed,
            ));
        }

        let positioned_commit = match self
            .movement
            .canonical_publication_route
            .record_performed_with_cutover(self.publication_cell.clone(), || {
                publication_state.replace_with(self.next_state);
                drop(publication_state);
            }) {
            Ok(positioned) => positioned,
            Err(crate::runtime::CanonicalPublicationRecordError::ReservationContended) => {
                return worth_proof::TransitionOutcome::deferred(
                    RelationalPublicationDeferred::PatchPositionReservationContended,
                );
            }
            Err(crate::runtime::CanonicalPublicationRecordError::PositionCapacityExhausted) => {
                return worth_proof::TransitionOutcome::failed(RelationalPublicationFailure::new(
                    RelationalPublicationFailureKind::PatchPositionCapacityExhausted,
                    "performed publication stream position capacity exhausted",
                ));
            }
        };
        drop(_critical_section);
        drop(_publication_lifecycle);
        self.movement.record_allocations.commit();
        worth_proof::TransitionOutcome::success(PerformedRelationalCommit::record(
            positioned_commit,
            self.next_basis,
            self.completion,
        ))
    }
}

impl crate::runtime::RelationalRuntime {
    pub fn publication_port(&self) -> RelationalPublicationPort {
        RelationalPublicationPort::new(self.runtime_instance_id())
    }

    pub fn patch_position_reservation_counters(
        &self,
    ) -> crate::runtime::RelationalPatchPositionReservationCounters {
        self.history.canonical_publication_reservation_counters()
    }
}
