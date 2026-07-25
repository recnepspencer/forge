use std::sync::Arc;

use worth_store_physical_format::RecordArtifactFile;

use super::super::{
    append::{next_nonzero_random, ManifestCapacityTransition},
    payload_progression,
};
use super::RecordPublicationDirector;
use crate::physical_runtime::record_serving::{
    planning::{
        batch_placement::{append_operation_allocation_bytes, classify_batch, preflight_placement},
        placement_context::PlacementPlanningContext,
        prepared_payload::prepare_payload_plan,
        rebased_root::{RebasableRecordPublicationPlan, RootRebaseContext},
    },
    residency::frame_ports::StoreCandidateFramePublicationSession,
    AdmittedRecordPlacementPolicy, PublishedRecordBatch, RecordAppendBatch, RecordAppendDenial,
    RecordAppendError,
};

struct PublishedTailReservation {
    director: Arc<RecordPublicationDirector>,
    active: bool,
}

impl RecordPublicationDirector {
    pub(super) fn preflight(
        &self,
        batch: &RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
        capacity_transition: ManifestCapacityTransition,
    ) -> Result<(), RecordAppendError> {
        let runtime = self.runtime.upgrade().ok_or(RecordAppendError::Denied(
            RecordAppendDenial::PublicationAuthorityReleased,
        ))?;
        if runtime.health.requires_inspection() {
            return Err(RecordAppendError::Denied(
                RecordAppendDenial::ServingRequiresInspection,
            ));
        }
        if !placement.admits(self.format) {
            return Err(RecordAppendError::Denied(
                RecordAppendDenial::PlacementFormatMismatch,
            ));
        }
        batch
            .preflight(self.access)
            .map_err(RecordAppendError::Denied)?;
        preflight_placement(self.format, placement, batch)?;
        if capacity_transition == ManifestCapacityTransition::PreserveCurrent
            && placement.manifest_capacity().get()
                != self
                    .state
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .current_root
                    .node_capacity()
        {
            return Err(RecordAppendError::Denied(
                RecordAppendDenial::ManifestCapacityMigrationRequired,
            ));
        }
        Ok(())
    }

    pub(super) fn publish(
        self: &Arc<Self>,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
        capacity_transition: ManifestCapacityTransition,
    ) -> Result<PublishedRecordBatch, RecordAppendError> {
        let _call = Self::begin(self)?;
        self.preflight(&batch, placement, capacity_transition)?;
        let runtime = self.runtime.upgrade().ok_or(RecordAppendError::Denied(
            RecordAppendDenial::PublicationAuthorityReleased,
        ))?;
        let _allocation = self.begin_append_allocation(&batch, placement)?;
        let counters_before = runtime.executor.record_serving_media().counters();
        let result = self
            .prepare_rebasable(&runtime, batch, placement)
            .and_then(|(plan, reservation)| {
                let replacement = self
                    .mutation
                    .prepare_catalog_replacement_dependency(plan.publication.candidate)
                    .map_err(|failure| {
                        super::super::unpublished_prepared_physical_work(
                            runtime.executor.record_serving_media(),
                            &plan.publication,
                            counters_before,
                            super::super::RecordPublicationStage::CatalogReplacement,
                            &failure,
                        )
                    })?;
                self.materialize_payload(&runtime, plan)
                    .and_then(|plan| {
                        self.publish_rebased_root(
                            &runtime,
                            plan,
                            replacement,
                            placement,
                            capacity_transition,
                            counters_before,
                        )
                    })
                    .map(|published| (published, reservation))
            })
            .map(|(published, _reservation)| published);
        self.observe_result(&runtime, result)
    }

    fn begin_append_allocation(
        &self,
        batch: &RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
    ) -> Result<worth_store_buffer_pool::OperationAllocationGrant, RecordAppendError> {
        self.frame_ports
            .begin_operation(
                worth_store_buffer_pool::OperationAllocationScope::ForegroundWrite,
                append_operation_allocation_bytes(self.format, placement, batch),
            )
            .map_err(|reason| {
                RecordAppendError::Denied(RecordAppendDenial::ResidencyUnavailable(reason))
            })
    }

    fn prepare_rebasable(
        self: &Arc<Self>,
        runtime: &crate::physical_runtime::instance::PhysicalStoreWorkRuntime,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
    ) -> Result<(RebasableRecordPublicationPlan, PublishedTailReservation), RecordAppendError> {
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
        let reader = crate::physical_runtime::record_serving::access::manifest_routing::ManifestReader::with_loader(
            runtime.executor.record_serving_media(),
            self.frame_ports.loader(),
            self.format,
            self.access,
            &current_root,
        );
        let classified = classify_batch(&reader, placement, admitted)?;
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
        let allow_published_reuse = !preparation.published_tail_reserved;
        if allow_published_reuse {
            preparation.published_tail_reserved = true;
        }
        drop(preparation);
        let reservation = PublishedTailReservation {
            director: Arc::clone(self),
            active: allow_published_reuse,
        };
        let prepared = prepare_payload_plan(
            PlacementPlanningContext {
                media: runtime.executor.record_serving_media(),
                format: self.format,
                access: self.access,
                current_root: &current_root,
                current_free_space: &current_free_space,
                frontier: &mut candidate_frontier,
                placement,
                frame_load: self.frame_ports.loader(),
            },
            classified,
            allow_published_reuse,
        )?;
        let candidate = RecordArtifactFile::CatalogCandidate {
            publication: next_nonzero_random()?,
        };
        let plan = RebasableRecordPublicationPlan::begin(prepared, &current_root, candidate)?
            .attach_payload();
        Ok((plan, reservation))
    }

    fn materialize_payload(
        &self,
        runtime: &crate::physical_runtime::instance::PhysicalStoreWorkRuntime,
        plan: RebasableRecordPublicationPlan,
    ) -> Result<RebasableRecordPublicationPlan, RecordAppendError> {
        let RebasableRecordPublicationPlan {
            publication,
            prepared,
        } = plan;
        let declaration = publication
            .payload_candidate_frame_set(self.format)
            .map_err(RecordAppendError::Denied)?;
        let mut residency =
            StoreCandidateFramePublicationSession::begin(self.frame_ports.publisher(), declaration)
                .map_err(RecordAppendError::Denied)?;
        let before = runtime.executor.record_serving_media().counters();
        let publication = payload_progression::execute(
            &self.mutation,
            runtime.executor.record_serving_media(),
            self.format,
            publication,
            &mut residency,
            before,
        )?;
        residency.require_complete().map_err(|violation| {
            super::super::unpublished_candidate_frame_contract(
                runtime.executor.record_serving_media(),
                &publication,
                before,
                super::super::RecordPublicationStage::PayloadManifestSynchronization,
                violation,
            )
        })?;
        Ok(RebasableRecordPublicationPlan::resume(
            publication,
            prepared,
        ))
    }

    fn publish_rebased_root(
        &self,
        runtime: &crate::physical_runtime::instance::PhysicalStoreWorkRuntime,
        plan: RebasableRecordPublicationPlan,
        replacement: super::super::super::PreparedCatalogReplacement,
        placement: AdmittedRecordPlacementPolicy,
        capacity_transition: ManifestCapacityTransition,
        counters_before: worth_store_physical_backend::MediaCounterSnapshot,
    ) -> Result<PublishedRecordBatch, RecordAppendError> {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let frontier = self
            .preparation
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .allocation_frontier
            .clone();
        let (plan, free_space) = plan.rebase(RootRebaseContext {
            media: runtime.executor.record_serving_media(),
            frame_load: self.frame_ports.loader(),
            format: self.format,
            access: self.access,
            current_root: &state.current_root,
            current_free_space: &state.free_space,
            frontier: &frontier,
            placement,
            capacity_transition,
        })?;
        let declaration = plan
            .root_candidate_frame_set()
            .map_err(RecordAppendError::Denied)?;
        let mut residency =
            StoreCandidateFramePublicationSession::begin(self.frame_ports.publisher(), declaration)
                .map_err(RecordAppendError::Denied)?;
        let (published, successor) = super::super::publication_progression::execute_prepared_root(
            &self.mutation,
            runtime.executor.record_serving_media(),
            plan,
            replacement,
            &mut residency,
            counters_before,
            #[cfg(feature = "certification-test-authority")]
            self.take_catalog_eligibility_join_rejection(),
        )?;
        state.current_root = successor;
        state.free_space = free_space;
        Ok(published)
    }

    fn observe_result(
        &self,
        runtime: &crate::physical_runtime::instance::PhysicalStoreWorkRuntime,
        result: Result<PublishedRecordBatch, RecordAppendError>,
    ) -> Result<PublishedRecordBatch, RecordAppendError> {
        let error = match result {
            Ok(published) => return Ok(published),
            Err(error) => error,
        };
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match &error {
            RecordAppendError::Unpublished(failure) => {
                state.residue = state.residue.merge(failure.residue());
                if failure.requires_inspection() {
                    runtime.health.revoke();
                }
            }
            RecordAppendError::Indeterminate(failure) => {
                state.residue = failure.residue();
                runtime.health.revoke();
            }
            RecordAppendError::StreamFailed(failure) => {
                if failure.requires_inspection() {
                    runtime.health.revoke();
                }
            }
            RecordAppendError::Denied(denial) => runtime.health.observe_append_denial(denial),
        }
        Err(error)
    }
}

impl Drop for PublishedTailReservation {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        self.director
            .preparation
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .published_tail_reserved = false;
    }
}
