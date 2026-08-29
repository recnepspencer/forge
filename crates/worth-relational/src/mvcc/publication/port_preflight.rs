use std::sync::Arc;

use super::super::candidate::PreparedRelationalPublicationParts;
use super::{
    PreparedBranchPublicationPreflight, PreparedCanonicalBranchMovement,
    RelationalPublicationDeferred, RelationalPublicationDenial, RelationalPublicationFailure,
    RelationalPublicationFailureKind, RelationalPublicationOutcome,
};

impl PreparedCanonicalBranchMovement {
    pub(super) fn preflight(
        parts: PreparedRelationalPublicationParts,
        retention_binding: crate::history::retention::RelationalBranchRetentionBinding,
        branch_head_versions: crate::runtime::BranchHeadVersionIndexAuthority,
    ) -> Result<PreparedBranchPublicationPreflight, RelationalPublicationOutcome> {
        let PreparedRelationalPublicationParts {
            runtime_instance_id,
            publication_binding,
            expected,
            expected_root,
            publication_cell,
            movement,
            completion,
            candidate_retention,
            control,
            expires_at,
            maximum_lifetime_millis,
            maximum_published_snapshot_handles,
        } = parts;
        match control.observe(crate::mvcc::RelationalInterruptionBoundary::PublicationPreflight) {
            Some(event)
                if event.interruption()
                    == crate::mvcc::RelationalOperationInterruption::Cancelled =>
            {
                retention_binding.record_interruption(event);
                return Err(RelationalPublicationOutcome::interrupted(event));
            }
            Some(event) => {
                retention_binding.record_interruption(event);
                return Err(RelationalPublicationOutcome::interrupted(event));
            }
            None => {}
        }
        let next_state = movement.next_cell.state_snapshot();
        if publication_cell.runtime_instance_id() != expected.runtime_instance_id()
            || publication_cell.branch_id() != expected.branch_id()
        {
            return Err(RelationalPublicationOutcome::denied(
                RelationalPublicationDenial::OwnerMismatch,
            ));
        }
        if movement.next_cell.identity().runtime_instance_id()
            != publication_cell.runtime_instance_id()
            || movement.next_cell.identity().branch_id() != publication_cell.branch_id()
            || movement.next_cell.root().as_ref().map(Arc::as_ptr)
                != Some(Arc::as_ptr(&movement.root))
        {
            return Err(RelationalPublicationOutcome::failed(
                RelationalPublicationFailure::new(
                    RelationalPublicationFailureKind::PreparedRootMismatch,
                    "prepared next root does not match its branch publication cell",
                ),
            ));
        }
        let next_descriptor =
            match crate::branch::descriptor_for_cell(&movement.next_cell, &movement.root) {
                Ok(descriptor) => descriptor,
                Err(denial) => {
                    return Err(RelationalPublicationOutcome::failed(
                        RelationalPublicationFailure::new(
                            RelationalPublicationFailureKind::PreparedBasisDescriptor(
                                denial.clone(),
                            ),
                            format!("prepared next basis failed before movement: {denial:?}"),
                        ),
                    ));
                }
            };
        let next_basis = crate::branch::issue_admitted_relational_branch_basis_with_retention(
            next_descriptor,
            movement.next_cell.identity().clone(),
            Arc::clone(&movement.root),
            publication_cell.clone(),
            &retention_binding,
        );
        let next_basis = match next_basis.and_then(|basis| publication_cell.register_basis(basis)) {
            Ok(basis) => basis,
            Err(crate::branch::RelationalBranchBasisDenial::RetentionCapacityExhausted) => {
                return Err(RelationalPublicationOutcome::deferred(
                    RelationalPublicationDeferred::RetentionBackpressure,
                ));
            }
            Err(crate::branch::RelationalBranchBasisDenial::UnavailableRetainedTarget) => {
                return Err(RelationalPublicationOutcome::denied(
                    RelationalPublicationDenial::OwnerUnavailable {
                        runtime_instance_id: expected.runtime_instance_id(),
                    },
                ));
            }
            Err(denial) => {
                return Err(RelationalPublicationOutcome::failed(
                    RelationalPublicationFailure::new(
                        RelationalPublicationFailureKind::NextBasisAdmission(denial.clone()),
                        format!("next basis admission failed before movement: {denial:?}"),
                    ),
                ));
            }
        };
        let head_retirement = match retention_binding.reserve_head_retirement(
            movement.next_cell.identity(),
            &expected_root,
            publication_cell.head_retention(),
        ) {
            Ok(reservation) => reservation,
            Err(
                crate::history::retention::RelationalRetentionAcquisitionDenial::CapacityExhausted,
            ) => {
                return Err(RelationalPublicationOutcome::deferred(
                    RelationalPublicationDeferred::RetentionBackpressure,
                ));
            }
            Err(
                crate::history::retention::RelationalRetentionAcquisitionDenial::OwnerUnavailable,
            ) => {
                return Err(RelationalPublicationOutcome::denied(
                    RelationalPublicationDenial::OwnerUnavailable {
                        runtime_instance_id: expected.runtime_instance_id(),
                    },
                ));
            }
            Err(
                crate::history::retention::RelationalRetentionAcquisitionDenial::IdentityExhausted,
            ) => {
                return Err(RelationalPublicationOutcome::failed(
                    RelationalPublicationFailure::new(
                        RelationalPublicationFailureKind::RetentionIdentityExhausted,
                        "head-retirement retention identity exhausted before movement",
                    ),
                ));
            }
            Err(denial) => {
                return Err(RelationalPublicationOutcome::failed(
                    RelationalPublicationFailure::new(
                        RelationalPublicationFailureKind::RetentionOwner,
                        format!("head-retirement reservation failed before movement: {denial:?}"),
                    ),
                ));
            }
        };
        // The pending settlement record is installed here, before the critical
        // section, so no observer can ever see a moved branch head whose owner
        // recovery lookup has no record. Every stop below this point releases
        // the reservation by dropping it.
        let published_snapshot_basis =
            crate::visibility::snapshot_states::VisibilitySnapshotBasis::from_observation(
                &next_basis.observation(),
            );
        let settlement_reservation = match publication_binding.reserve_pending_settlement(
            movement.canonical_publication_route.commit_id(),
            runtime_instance_id,
            maximum_published_snapshot_handles,
            crate::runtime::ReservedRelationalSettlement {
                completion,
                published_snapshot_basis,
                control: control.clone(),
            },
        ) {
            Ok(reservation) => reservation,
            Err(crate::runtime::RelationalSettlementReservationDenial::CapacityExhausted {
                maximum_handles,
            }) => {
                return Err(RelationalPublicationOutcome::deferred(
                    RelationalPublicationDeferred::PublishedSnapshotCapacityExhausted {
                        maximum_handles,
                    },
                ));
            }
            Err(crate::runtime::RelationalSettlementReservationDenial::OwnerUnavailable) => {
                return Err(RelationalPublicationOutcome::denied(
                    RelationalPublicationDenial::OwnerUnavailable {
                        runtime_instance_id: expected.runtime_instance_id(),
                    },
                ));
            }
            Err(crate::runtime::RelationalSettlementReservationDenial::DuplicateCommitIdentity) => {
                return Err(RelationalPublicationOutcome::failed(
                    RelationalPublicationFailure::new(
                        RelationalPublicationFailureKind::PendingSettlementIdentityConflict,
                        "another pending settlement already owns this commit identity",
                    ),
                ));
            }
        };
        Ok(PreparedBranchPublicationPreflight {
            movement,
            expected,
            publication_cell,
            next_state,
            next_basis,
            settlement_reservation,
            head_retirement,
            candidate_retention,
            control,
            expires_at,
            maximum_lifetime_millis,
            branch_head_versions,
        })
    }
}
