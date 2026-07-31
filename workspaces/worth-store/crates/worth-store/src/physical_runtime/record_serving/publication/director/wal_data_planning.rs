use super::RecordPublicationDirector;
use crate::physical_runtime::record_serving::{
    planning::{
        batch_placement::{append_operation_allocation_bytes, classify_batch},
        placement_context::PlacementPlanningContext,
        prepared_payload::{prepare_payload_plan, PreparedRecordPayloadPlan},
    },
    publication::{
        materialize_durable_data, PhysicalMutationAdmissionDisposition, PreparedPhysicalMutation,
    },
    RecordAppendDenial, RecordAppendError,
};

impl RecordPublicationDirector {
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
        match self.build_durable_data_plan(&prepared) {
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
            PreparedRecordPayloadPlan,
        ),
        RecordAppendError,
    > {
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
            true,
        )?;
        materialize_durable_data(payload, self.format)
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
