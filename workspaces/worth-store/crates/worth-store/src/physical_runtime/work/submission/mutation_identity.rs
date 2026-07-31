use super::{
    request::allocate_operation_identity, PhysicalMutationSubmission,
    PhysicalWorkSubmissionFailure, PhysicalWorkSubmissionReceipt, PhysicalWorkSubmissionStale,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum PhysicalMutationIdentityReservationError {
    Stale(PhysicalWorkSubmissionStale),
    Failed(PhysicalWorkSubmissionFailure),
}

impl PhysicalMutationSubmission {
    pub(in crate::physical_runtime) fn reserve_mutation_identity(
        &self,
    ) -> Result<PhysicalWorkSubmissionReceipt, PhysicalMutationIdentityReservationError> {
        let shared =
            self.shared
                .upgrade()
                .ok_or(PhysicalMutationIdentityReservationError::Stale(
                    PhysicalWorkSubmissionStale::OwnerReleased,
                ))?;
        let _activity = shared
            .enter(self.generation)
            .map_err(PhysicalMutationIdentityReservationError::Stale)?;
        let identity = allocate_operation_identity(&shared)
            .map_err(PhysicalMutationIdentityReservationError::Failed)?;
        Ok(PhysicalWorkSubmissionReceipt {
            identity,
            signal_profile: shared.signal_profile,
        })
    }
}
