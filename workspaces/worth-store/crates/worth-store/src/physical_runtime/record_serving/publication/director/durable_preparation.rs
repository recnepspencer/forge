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
        planning::{
            batch_placement::{
                append_operation_allocation_bytes, classify_batch, preflight_placement,
            },
            placement_context::PlacementPlanningContext,
            prepared_payload::prepare_payload_plan,
        },
        publication::{
            materialize_durable_data, prepare_canonical_payload, record_append_scope_identity,
            CanonicalPayloadPreparationError, CanonicalRecordAppendPayload,
            PhysicalMutationAdmissionDisposition, PhysicalMutationPreparationDeferred,
            PhysicalMutationPreparationDenial, PhysicalMutationPreparationFailure,
            PhysicalMutationPreparationOutcome, PhysicalMutationPreparationRebindRequired,
            PhysicalMutationPreparationStale, PhysicalMutationResourceShape,
            PreparedPhysicalMutation,
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

    pub(super) fn plan_prepared_data_for_wal(
        &self,
        prepared: PreparedPhysicalMutation,
    ) -> Result<PreparedPhysicalMutation, (PreparedPhysicalMutation, RecordAppendDenial)> {
        let identity = prepared.mutation_identity();
        if identity.store_identity() != self.durability.store_identity()
            || identity.runtime_identity() != self.durability.runtime_identity()
            || prepared.signal_profile() != self.signal_profile
        {
            return Ok(prepared);
        }
        if prepared.data_is_planned()
            || prepared.disposition() == PhysicalMutationAdmissionDisposition::DuplicateUnresolved
        {
            return Ok(prepared);
        }
        let planned = self.build_durable_data_plan(&prepared);
        match planned {
            Ok((data, continuation)) => Ok(prepared.attach_data_plan(data, continuation)),
            Err(error) => Err((prepared, data_planning_denial(error))),
        }
    }

    fn build_durable_data_plan(
        &self,
        prepared: &PreparedPhysicalMutation,
    ) -> Result<
        (
            crate::physical_runtime::durability::PreparedPhysicalDataPlan,
            crate::physical_runtime::record_serving::planning::prepared_payload::
                PreparedRecordPayloadPlan,
        ),
        RecordAppendError,
    >{
        let runtime = self.runtime.upgrade().ok_or(RecordAppendError::Denied(
            RecordAppendDenial::PublicationAuthorityReleased,
        ))?;
        let batch = prepared.duplicate_prepared_batch();
        let bytes = append_operation_allocation_bytes(self.format, prepared.placement(), &batch);
        let allocation = self
            .residency
            .begin_foreground_write_operation(
                std::num::NonZeroU64::new(bytes)
                    .expect("an admitted nonempty append has nonzero planning bytes"),
            )
            .map_err(|denial| {
                RecordAppendError::Denied(RecordAppendDenial::from_residency(denial))
            })?;
        let (current_root, current_free_space) = {
            let state = self
                .state
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            (state.current_root.clone(), state.free_space.clone())
        };
        let admitted = batch
            .admit(self.access)
            .map_err(RecordAppendError::Denied)?;
        let reader =
            crate::physical_runtime::record_serving::access::manifest_routing::ManifestReader::
                serving(
                    self.residency.clone(),
                    self.format,
                    self.access,
                    current_root.clone(),
                );
        let classified = classify_batch(&reader, &allocation, prepared.placement(), admitted)?;
        let shape = classified.identity_reservation_shape()?;
        let mut preparation = self
            .preparation
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut candidate_frontier = preparation
            .allocation_frontier
            .reserve(shape.segments(), shape.pages(), shape.extents())
            .ok_or(RecordAppendError::Denied(
                RecordAppendDenial::PhysicalIdentityExhausted,
            ))?;
        drop(preparation);
        let payload = prepare_payload_plan(
            PlacementPlanningContext {
                allocation: &allocation,
                media: runtime.executor.record_serving_media(),
                format: self.format,
                access: self.access,
                current_root: &current_root,
                current_free_space: &current_free_space,
                frontier: &mut candidate_frontier,
                placement: prepared.placement(),
                residency: self.residency.clone(),
            },
            classified,
            false,
        )?;
        materialize_durable_data(payload, self.format)
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

fn data_planning_denial(error: RecordAppendError) -> RecordAppendDenial {
    match error {
        RecordAppendError::Denied(denial) => denial,
        RecordAppendError::PhysicalPressure { .. } => RecordAppendDenial::PhysicalPressure,
        RecordAppendError::StreamFailed(_)
        | RecordAppendError::Unpublished(_)
        | RecordAppendError::Indeterminate(_) => RecordAppendDenial::PublishedLayoutDamaged,
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
