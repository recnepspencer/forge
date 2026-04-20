use crate::{
    failure::StoreError,
    live_query::{
        acknowledgment::admit_continuation_advance as admit_live_query_continuation_advance,
        basis::{validate_stable_basis_handle, validate_stable_basis_request},
        compatibility::plan_cursor_continuation as plan_live_query_cursor_continuation,
        LiveQueryBasisEvidence, LiveQueryContinuationSessionEvidence, Milestone8TruthSurface,
    },
};

use super::ForgeStore;

impl ForgeStore {
    pub fn plan_stable_basis_read(
        &self,
        request: crate::StableBasisReadRequest,
    ) -> Result<crate::StableBasisReadPlan, StoreError> {
        validate_stable_basis_request(&request)?;
        Ok(crate::StableBasisReadPlan::new(
            request.clone(),
            crate::StableBasisId::from_request(&request),
        ))
    }

    pub fn read_stable_basis(
        &mut self,
        request: crate::StableBasisReadRequest,
    ) -> Result<crate::StableBasisHandle, StoreError> {
        self.backend.publish_stable_basis(request)
    }

    pub fn fetch_stable_basis(
        &self,
        stable_basis_id: &crate::StableBasisId,
    ) -> Result<crate::StableBasisHandle, StoreError> {
        self.backend.fetch_stable_basis(stable_basis_id.as_str())
    }

    pub fn plan_cursor_continuation(
        &self,
        request: crate::CursorContinuationRequest,
    ) -> Result<crate::CursorContinuationPlan, StoreError> {
        validate_stable_basis_handle(request.stable_basis())?;
        let basis_survival = crate::live_query::restart::StableBasisSurvival::from_handle(
            request.stable_basis(),
        );
        self.backend.record_stable_basis_lookup();
        self.backend.record_stable_basis_read(
            1,
            1,
            !matches!(basis_survival, crate::live_query::restart::StableBasisSurvival::Retained),
        );
        self.backend.record_continuation_identity_lookup();
        self.backend.record_continuation_checkpoint_lookup();
        let resume_plan = self
            .backend
            .plan_cursor_resume(request.resume_request().clone())?;
        match plan_live_query_cursor_continuation(request, resume_plan) {
            Ok(planned) => {
                let (plan, effects) = planned.into_parts();
                self.backend.verify_cursor_continuation_budget(&plan)?;
                self.record_continuation_planning_effects(effects);
                Ok(plan)
            }
            Err(failure) => {
                let (error, effects) = failure.into_parts();
                self.record_continuation_planning_effects(effects);
                Err(error)
            }
        }
    }

    pub fn execute_cursor_continuation(
        &self,
        plan: crate::CursorContinuationPlan,
    ) -> Result<crate::ContinuationBatchResult, StoreError> {
        self.backend.execute_cursor_continuation(plan)
    }

    pub fn verify_cursor_continuation_budget(
        &self,
        plan: &crate::CursorContinuationPlan,
    ) -> Result<(), StoreError> {
        self.backend.verify_cursor_continuation_budget(plan)
    }

    pub fn admit_continuation_advance(
        &self,
        receipt: crate::ContinuationAdvanceReceipt,
    ) -> Result<crate::AcknowledgedContinuationAdvance, StoreError> {
        match admit_live_query_continuation_advance(receipt) {
            Ok(advance) => {
                self.record_continuation_ack_effects(vec![
                    crate::live_query::acknowledgment::ContinuationAcknowledgmentEffect::Parity,
                ]);
                Ok(advance)
            }
            Err(failure) => {
                let (error, effects) = failure.into_parts();
                self.record_continuation_ack_effects(effects);
                Err(error)
            }
        }
    }

    pub fn acknowledge_cursor_continuation(
        &mut self,
        receipt: crate::ContinuationAdvanceReceipt,
    ) -> Result<crate::AcknowledgedContinuationAdvance, StoreError> {
        self.backend.acknowledge_cursor_continuation(receipt)
    }

    pub fn milestone_8_certification_bundle(
        &self,
        request: crate::Milestone8CertificationRequest<'_>,
    ) -> Result<crate::Milestone8CertificationBundle, StoreError> {
        let primary_export = self.export_authoritative_records();
        let restored_export =
            Self::restore_from_authoritative_export(primary_export.clone().admit_restore())?
                .export_authoritative_records();
        let truth_surface =
            Milestone8TruthSurface::from_basis_and_frontier(request.basis(), request.final_frontier_commit_id());
        let basis = LiveQueryBasisEvidence::from_handle(request.basis());
        let continuation = LiveQueryContinuationSessionEvidence::from_batch_results(
            request.continuation_strategy(),
            request.basis().read_scope(),
            request.final_frontier_commit_id(),
            request.continuation_results(),
        )?;
        let control_continuation = LiveQueryContinuationSessionEvidence::from_batch_results(
            request.control_strategy(),
            request.basis().read_scope(),
            request.control_final_frontier_commit_id(),
            request.control_continuation_results(),
        )?;
        crate::Milestone8CertificationBundle::new(
            &primary_export,
            request.control_export(),
            &restored_export,
            truth_surface,
            basis,
            continuation,
            control_continuation,
            self.counters(),
            request.failure_markers(),
        )

    }
}
