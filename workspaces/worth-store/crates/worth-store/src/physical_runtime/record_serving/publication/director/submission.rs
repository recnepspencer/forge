use std::sync::Weak;

use worth_proof::TransitionOutcome;

use super::RecordPublicationDirector;
use crate::physical_runtime::{
    record_serving::{
        publication::{
            append::ManifestCapacityTransition, PhysicalMutationPreparationOutcome,
            PhysicalMutationPreparationStale,
        },
        AdmittedRecordPlacementPolicy, PublishedRecordBatch, RecordAppendBatch, RecordAppendDenial,
        RecordAppendError,
    },
    PhysicalMutationIdempotencyIssuanceDenial, PhysicalMutationIdempotencyKey,
    PhysicalMutationIdempotencyMaterial, PhysicalMutationRequest,
};

#[derive(Clone)]
pub struct PhysicalRecordSubmission {
    director: Weak<RecordPublicationDirector>,
}

pub struct PreparedRecordAppend {
    director: Weak<RecordPublicationDirector>,
    batch: RecordAppendBatch,
    placement: AdmittedRecordPlacementPolicy,
    capacity_transition: ManifestCapacityTransition,
}

impl PhysicalRecordSubmission {
    pub(super) const fn new(director: Weak<RecordPublicationDirector>) -> Self {
        Self { director }
    }

    pub fn issue_idempotency_key(
        &self,
        material: PhysicalMutationIdempotencyMaterial,
    ) -> Result<PhysicalMutationIdempotencyKey, PhysicalMutationIdempotencyIssuanceDenial> {
        let director = self
            .director
            .upgrade()
            .ok_or(PhysicalMutationIdempotencyIssuanceDenial::DurabilityAuthorityReleased)?;
        director.idempotency.issue_key(material)
    }

    pub fn prepare_append(
        &self,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
    ) -> Result<PreparedRecordAppend, RecordAppendError> {
        self.prepare_with_capacity_transition(
            batch,
            placement,
            ManifestCapacityTransition::PreserveCurrent,
        )
    }

    pub fn prepare_durable_append(
        &self,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
        request: PhysicalMutationRequest,
    ) -> PhysicalMutationPreparationOutcome {
        let director = match self.director.upgrade() {
            Some(director) => director,
            None => {
                return TransitionOutcome::stale(
                    PhysicalMutationPreparationStale::PublicationAuthorityReleased,
                )
                .into()
            }
        };
        director.prepare_durable_append(batch, placement, request)
    }

    pub fn append_prepared_wal(
        &self,
        prepared: crate::physical_runtime::PreparedPhysicalMutation,
    ) -> crate::physical_runtime::PhysicalWalAppendOutcome {
        let Some(director) = self.director.upgrade() else {
            return crate::physical_runtime::PhysicalWalAppendOutcome::ReservationDenied {
                prepared,
                cause:
                    crate::physical_runtime::PhysicalWalReservationDenial::PublicationAuthorityReleased,
            };
        };
        let prepared = match director.plan_prepared_data_for_wal(prepared) {
            Ok(prepared) => prepared,
            Err((prepared, denial)) => {
                return crate::physical_runtime::PhysicalWalAppendOutcome::ReservationDenied {
                    prepared,
                    cause: crate::physical_runtime::PhysicalWalReservationDenial::DataPlanning(
                        denial,
                    ),
                }
            }
        };
        director.wal.append_prepared(prepared)
    }

    pub fn wal_observation(&self) -> Option<crate::physical_runtime::PhysicalWalObservation> {
        self.director
            .upgrade()
            .map(|director| director.wal.observation())
    }

    pub fn synchronize_appended_wal(
        &self,
        appended: crate::physical_runtime::WalAppendedPhysicalMutation,
    ) -> crate::physical_runtime::PhysicalWalBarrierOutcome {
        let Some(director) = self.director.upgrade() else {
            return crate::physical_runtime::PhysicalWalBarrierOutcome::BarrierNotStarted {
                appended,
                cause: crate::physical_runtime::PhysicalWalBarrierFailureCause::RuntimeReleased,
            };
        };
        director.wal_barrier.synchronize_appended(appended)
    }

    pub fn dispatch_wal_durable_data(
        &self,
        durable: crate::physical_runtime::WalDurablePhysicalMutation,
    ) -> crate::physical_runtime::PhysicalDataDispatchOutcome {
        let Some(director) = self.director.upgrade() else {
            return crate::physical_runtime::PhysicalDataDispatchOutcome::NotStarted {
                durable,
                cause:
                    crate::physical_runtime::PhysicalDataDispatchFailureCause::PublicationAuthorityReleased,
            };
        };
        director.dispatch_wal_durable_data(durable)
    }

    pub fn append_batch(
        &self,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
    ) -> Result<PublishedRecordBatch, RecordAppendError> {
        self.prepare_append(batch, placement)?.publish()
    }

    pub fn append_batch_reconstructing_manifest_capacity(
        &self,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
    ) -> Result<PublishedRecordBatch, RecordAppendError> {
        self.prepare_with_capacity_transition(
            batch,
            placement,
            ManifestCapacityTransition::ReconstructToRequested,
        )?
        .publish()
    }

    fn prepare_with_capacity_transition(
        &self,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
        capacity_transition: ManifestCapacityTransition,
    ) -> Result<PreparedRecordAppend, RecordAppendError> {
        let director = self.director.upgrade().ok_or(RecordAppendError::Denied(
            RecordAppendDenial::PublicationAuthorityReleased,
        ))?;
        director
            .preflight(&batch, placement, capacity_transition)
            .map_err(|error| director.project_pressure(error))?;
        Ok(PreparedRecordAppend {
            director: self.director.clone(),
            batch,
            placement,
            capacity_transition,
        })
    }
}

impl PreparedRecordAppend {
    pub fn publish(self) -> Result<PublishedRecordBatch, RecordAppendError> {
        let director = self.director.upgrade().ok_or(RecordAppendError::Denied(
            RecordAppendDenial::PublicationAuthorityReleased,
        ))?;
        director
            .publish(self.batch, self.placement, self.capacity_transition)
            .map_err(|error| director.project_pressure(error))
    }
}
