use worth_proof::TransitionOutcome;

use super::RecordPublicationDirector;
use crate::physical_runtime::{
    durability::{
        AdmittedPhysicalMutation, PhysicalMutationFingerprintInput,
        PhysicalMutationIdempotencyRegistryAdmission,
        PhysicalMutationIdempotencyRegistryAdmissionError,
        PhysicalMutationIdempotencyRegistryDenial, PhysicalMutationOperationFamily,
        PhysicalMutationPayloadDigest, PhysicalMutationRequestScope, PhysicalMutationSecurityBasis,
    },
    record_serving::{
        planning::batch_placement::preflight_placement,
        publication::{
            prepare_canonical_payload, record_append_scope_identity,
            CanonicalPayloadPreparationError, CanonicalRecordAppendPayload,
            PhysicalMutationPreparationDeferred, PhysicalMutationPreparationDenial,
            PhysicalMutationPreparationFailure, PhysicalMutationPreparationOutcome,
            PhysicalMutationPreparationRebindRequired, PhysicalMutationPreparationStale,
            PhysicalMutationResourceShape, PreparedPhysicalMutation,
        },
        AdmittedRecordPlacementPolicy, RecordAppendBatch, RecordAppendDenial, RecordAppendError,
    },
    work::PhysicalMutationIdentityReservationError,
    PhysicalMutationDeadline, PhysicalMutationIdempotencyKey, PhysicalMutationRequest,
    PhysicalMutationRequestFingerprint, PhysicalWorkSubmissionFailure, PhysicalWorkSubmissionStale,
};

struct AdmittedMutationPreparation {
    admission: AdmittedPhysicalMutation,
    deadline: PhysicalMutationDeadline,
}

impl RecordPublicationDirector {
    pub(super) fn prepare_durable_append(
        &self,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
        request: PhysicalMutationRequest,
    ) -> PhysicalMutationPreparationOutcome {
        if let Err(outcome) = self.preflight_durable_append(&batch, placement) {
            return outcome;
        }
        let payload = match canonical_payload(batch) {
            Ok(payload) => payload,
            Err(outcome) => return outcome,
        };
        let admitted = match self.admit_mutation_preparation(placement, payload.digest, request) {
            Ok(admitted) => admitted,
            Err(outcome) => return outcome,
        };
        let resources =
            PhysicalMutationResourceShape::prepared(payload.record_count, payload.payload_bytes);
        TransitionOutcome::success(PreparedPhysicalMutation::new(
            admitted.admission,
            payload.batch,
            placement,
            admitted.deadline,
            self.signal_profile,
            self.durability_policy_basis.clone(),
            resources,
        ))
        .into()
    }

    fn preflight_durable_append(
        &self,
        batch: &RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
    ) -> Result<(), PhysicalMutationPreparationOutcome> {
        if !placement.admits(self.format) {
            return Err(TransitionOutcome::denied(
                PhysicalMutationPreparationDenial::RecordAppend(
                    RecordAppendDenial::PlacementFormatMismatch,
                ),
            )
            .into());
        }
        batch.preflight(self.access).map_err(map_record_denial)?;
        preflight_placement(self.format, placement, batch).map_err(map_record_preflight)
    }

    fn admit_mutation_preparation(
        &self,
        placement: AdmittedRecordPlacementPolicy,
        payload_digest: [u8; 32],
        request: PhysicalMutationRequest,
    ) -> Result<AdmittedMutationPreparation, PhysicalMutationPreparationOutcome> {
        let (key, deadline, durability_request) = request.into_parts();
        let lease = key.lease();
        let fingerprint = self
            .derive_record_append_fingerprint(placement, payload_digest, durability_request)
            .map_err(|()| canonical_request_failure())?;
        let admission = self
            .admit_idempotency_binding(key, fingerprint)
            .map_err(map_idempotency_admission)?;
        let admission = match admission {
            PhysicalMutationIdempotencyRegistryAdmission::Fresh(binding) => {
                AdmittedPhysicalMutation::Fresh(binding)
            }
            PhysicalMutationIdempotencyRegistryAdmission::DuplicateUnresolved(existing) => {
                AdmittedPhysicalMutation::DuplicateUnresolved { existing, lease }
            }
        };
        Ok(AdmittedMutationPreparation {
            admission,
            deadline,
        })
    }

    fn derive_record_append_fingerprint(
        &self,
        placement: AdmittedRecordPlacementPolicy,
        payload_digest: [u8; 32],
        durability_request: crate::physical_runtime::durability::PhysicalMutationDurabilityRequest,
    ) -> Result<PhysicalMutationRequestFingerprint, ()> {
        let scope = record_append_scope_identity(self.format, placement);
        let security = [PhysicalMutationSecurityBasis::from_admitted_security(
            self.security_basis,
        )];
        PhysicalMutationRequestFingerprint::derive(PhysicalMutationFingerprintInput {
            store: self.durability.store_identity(),
            durability_policy: self.durability.policy_identity(),
            scope: PhysicalMutationRequestScope::record_append(scope),
            payload: PhysicalMutationPayloadDigest::from_validated_payload(payload_digest),
            durability_request,
            operation_family: PhysicalMutationOperationFamily::RecordAppend,
            security_bases: &security,
        })
        .map_err(|_| ())
    }

    fn admit_idempotency_binding(
        &self,
        key: PhysicalMutationIdempotencyKey,
        fingerprint: PhysicalMutationRequestFingerprint,
    ) -> Result<
        PhysicalMutationIdempotencyRegistryAdmission,
        PhysicalMutationIdempotencyRegistryAdmissionError<PhysicalMutationIdentityReservationError>,
    > {
        self.idempotency
            .admit_unallocated_with(key, fingerprint, || {
                self.mutation_identity
                    .reserve_mutation_identity()
                    .map(|receipt| {
                        crate::physical_runtime::PhysicalMutationIdentity::from_reserved_operation(
                            receipt.identity(),
                        )
                    })
            })
    }
}

fn canonical_payload(
    batch: RecordAppendBatch,
) -> Result<CanonicalRecordAppendPayload, PhysicalMutationPreparationOutcome> {
    prepare_canonical_payload(batch).map_err(|error| match error {
        CanonicalPayloadPreparationError::RecordSlots { required_records } => {
            TransitionOutcome::deferred(PhysicalMutationPreparationDeferred::PreparedRecordSlots {
                required_records,
            })
            .into()
        }
        CanonicalPayloadPreparationError::PayloadBytes { required_bytes } => {
            TransitionOutcome::deferred(PhysicalMutationPreparationDeferred::PreparedPayloadBytes {
                required_bytes,
            })
            .into()
        }
        CanonicalPayloadPreparationError::Failed(failure) => {
            TransitionOutcome::failed(PhysicalMutationPreparationFailure::Stream(failure)).into()
        }
    })
}

fn map_record_preflight(error: RecordAppendError) -> PhysicalMutationPreparationOutcome {
    match error {
        RecordAppendError::Denied(denial) => map_record_denial(denial),
        _ => canonical_request_failure(),
    }
}

fn map_record_denial(denial: RecordAppendDenial) -> PhysicalMutationPreparationOutcome {
    TransitionOutcome::denied(PhysicalMutationPreparationDenial::RecordAppend(denial)).into()
}

fn canonical_request_failure() -> PhysicalMutationPreparationOutcome {
    TransitionOutcome::failed(PhysicalMutationPreparationFailure::CanonicalRequestRejected).into()
}

fn map_idempotency_admission(
    error: PhysicalMutationIdempotencyRegistryAdmissionError<
        PhysicalMutationIdentityReservationError,
    >,
) -> PhysicalMutationPreparationOutcome {
    match error {
        PhysicalMutationIdempotencyRegistryAdmissionError::Reservation(error) => {
            map_identity_reservation(error)
        }
        PhysicalMutationIdempotencyRegistryAdmissionError::Denied(denial) => {
            map_idempotency_denial(denial)
        }
    }
}

fn map_idempotency_denial(
    denial: PhysicalMutationIdempotencyRegistryDenial,
) -> PhysicalMutationPreparationOutcome {
    match denial {
        PhysicalMutationIdempotencyRegistryDenial::AuthorityReleased => {
            TransitionOutcome::stale(PhysicalMutationPreparationStale::DurabilityAuthorityReleased)
                .into()
        }
        PhysicalMutationIdempotencyRegistryDenial::ForeignStore
        | PhysicalMutationIdempotencyRegistryDenial::ForeignMutationStore => {
            TransitionOutcome::rebind_required(
                PhysicalMutationPreparationRebindRequired::ForeignStore,
            )
            .into()
        }
        PhysicalMutationIdempotencyRegistryDenial::ForeignPolicy => {
            TransitionOutcome::rebind_required(
                PhysicalMutationPreparationRebindRequired::ForeignDurabilityPolicy,
            )
            .into()
        }
        PhysicalMutationIdempotencyRegistryDenial::ForeignMutationRuntime => {
            TransitionOutcome::rebind_required(
                PhysicalMutationPreparationRebindRequired::ForeignRuntime,
            )
            .into()
        }
        PhysicalMutationIdempotencyRegistryDenial::Expired => {
            TransitionOutcome::denied(PhysicalMutationPreparationDenial::IdempotencyExpired).into()
        }
        PhysicalMutationIdempotencyRegistryDenial::Conflict => {
            TransitionOutcome::denied(PhysicalMutationPreparationDenial::IdempotencyConflict).into()
        }
        PhysicalMutationIdempotencyRegistryDenial::PendingUnresolvedLimitReached => {
            TransitionOutcome::deferred(
                PhysicalMutationPreparationDeferred::PendingUnresolvedLimitReached,
            )
            .into()
        }
    }
}

fn map_identity_reservation(
    error: PhysicalMutationIdentityReservationError,
) -> PhysicalMutationPreparationOutcome {
    match error {
        PhysicalMutationIdentityReservationError::Stale(stale) => {
            let stale = match stale {
                PhysicalWorkSubmissionStale::OwnerReleased => {
                    PhysicalMutationPreparationStale::WorkOwnerReleased
                }
                PhysicalWorkSubmissionStale::LifecycleGenerationAdvanced => {
                    PhysicalMutationPreparationStale::LifecycleGenerationAdvanced
                }
                PhysicalWorkSubmissionStale::AdmissionStopped => {
                    PhysicalMutationPreparationStale::AdmissionStopped
                }
                PhysicalWorkSubmissionStale::SignalOwnerUnavailable => {
                    PhysicalMutationPreparationStale::SignalOwnerUnavailable
                }
            };
            TransitionOutcome::stale(stale).into()
        }
        PhysicalMutationIdentityReservationError::Failed(
            PhysicalWorkSubmissionFailure::OperationIdentityExhausted,
        ) => TransitionOutcome::failed(
            PhysicalMutationPreparationFailure::OperationIdentityExhausted,
        )
        .into(),
    }
}
