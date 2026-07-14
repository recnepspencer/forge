use super::{
    WorthQueryAdmittedWorkflowEffect, WorthQueryLoweredWorkflowPlan,
    WorthQueryPromotionEligibility, WorthQueryWorkflowAftermath, WorthQueryWorkflowCompletion,
    WorthQueryWorkflowCounters, WorthQueryWorkflowFamily, WorthQueryWorkflowOutcome,
    WorthQueryWorkflowRequest, WorthQueryWorkflowStop, WorthQueryWorkflowStopSource,
};
use crate::runtime::{WorthQueryOrdinaryAuthorityDrift, WorthQueryWorkspace};

impl WorthQueryWorkflowRequest {
    pub fn run(self, workspace: &mut WorthQueryWorkspace) -> WorthQueryWorkflowOutcome {
        let counters = WorthQueryWorkflowCounters::context_checked();
        let (identity, family, label, mutation) = self.declaration.into_parts();
        if self.context.authority.session_label() != Some(&label) {
            return WorthQueryWorkflowOutcome::Stopped(WorthQueryWorkflowStop::denied(
                WorthQueryWorkflowStopSource::CrossSession,
                counters,
            ));
        }
        if family == WorthQueryWorkflowFamily::DeferredWriteback {
            return WorthQueryWorkflowOutcome::Stopped(WorthQueryWorkflowStop::denied(
                WorthQueryWorkflowStopSource::UnsupportedWriteback,
                counters,
            ));
        }
        match workspace.ordinary_authority_drift(&self.context.authority) {
            WorthQueryOrdinaryAuthorityDrift::ForeignOwner => {
                return WorthQueryWorkflowOutcome::Stopped(WorthQueryWorkflowStop::denied(
                    WorthQueryWorkflowStopSource::ForeignAuthority,
                    counters,
                ));
            }
            WorthQueryOrdinaryAuthorityDrift::StaleSnapshot => {
                return WorthQueryWorkflowOutcome::Stopped(WorthQueryWorkflowStop::denied(
                    WorthQueryWorkflowStopSource::StalePreview,
                    counters,
                ));
            }
            WorthQueryOrdinaryAuthorityDrift::Current => {}
        }
        debug_assert!(self.context.authority.preview_basis().is_some());
        let eligibility = WorthQueryPromotionEligibility::from_authority(&self.context.authority);
        let admitted_effect = WorthQueryAdmittedWorkflowEffect::new(
            identity.evidence_identity(),
            &self.context.authority,
        );
        let counters = counters.execution_attempted();
        let execution = match workspace.execute_ordinary_preview_promotion(
            label,
            identity.evidence_identity(),
            mutation.into_command(),
        ) {
            Ok(execution) => execution,
            Err(error) => {
                return WorthQueryWorkflowOutcome::Stopped(WorthQueryWorkflowStop::runtime(
                    error, counters,
                ));
            }
        };
        let lowered_plan = WorthQueryLoweredWorkflowPlan::new(execution.request_identity().clone());
        let aftermath = WorthQueryWorkflowAftermath::new(
            execution.outcome(),
            execution.receipt_identity().clone(),
            execution.aftermath_identity().clone(),
            execution.inspection_identity().clone(),
        );
        WorthQueryWorkflowOutcome::Completed(WorthQueryWorkflowCompletion::new(
            eligibility,
            admitted_effect,
            lowered_plan,
            aftermath,
            execution.into_outcome(),
            counters.execution_completed(),
        ))
    }
}
