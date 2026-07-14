use super::{
    WorthQueryPreviewJourneyOutcome, WorthQueryPromotionEligiblePreviewRequest,
    WorthQueryReadOnlyPreviewCompletion, WorthQueryReadOnlyPreviewRequest,
};
use crate::ordinary::workflow::{
    WorthQueryLoweredWorkflowPlan, WorthQueryWorkflowAftermath, WorthQueryWorkflowCounters,
    WorthQueryWorkflowOutcome, WorthQueryWorkflowRequest, WorthQueryWorkflowStop,
    WorthQueryWorkflowStopSource,
};
use crate::runtime::{WorthQueryOrdinaryAuthorityDrift, WorthQueryWorkspace};

impl WorthQueryReadOnlyPreviewRequest {
    pub fn open_and_close(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryPreviewJourneyOutcome {
        let counters = WorthQueryWorkflowCounters::context_checked();
        if self.context.authority.session_label() != Some(&self.declaration.label) {
            return WorthQueryPreviewJourneyOutcome::Stopped(WorthQueryWorkflowStop::denied(
                WorthQueryWorkflowStopSource::CrossSession,
                counters,
            ));
        }
        match workspace.ordinary_authority_drift(&self.context.authority) {
            WorthQueryOrdinaryAuthorityDrift::ForeignOwner => {
                return WorthQueryPreviewJourneyOutcome::Stopped(WorthQueryWorkflowStop::denied(
                    WorthQueryWorkflowStopSource::ForeignAuthority,
                    counters,
                ));
            }
            WorthQueryOrdinaryAuthorityDrift::StaleSnapshot => {
                return WorthQueryPreviewJourneyOutcome::Stopped(WorthQueryWorkflowStop::denied(
                    WorthQueryWorkflowStopSource::StalePreview,
                    counters,
                ));
            }
            WorthQueryOrdinaryAuthorityDrift::Current => {}
        }
        let counters = counters.execution_attempted();
        let execution = match workspace
            .execute_ordinary_read_only_preview(self.declaration.label, &self.declaration.identity)
        {
            Ok(execution) => execution,
            Err(error) => {
                return WorthQueryPreviewJourneyOutcome::Stopped(WorthQueryWorkflowStop::runtime(
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
        WorthQueryPreviewJourneyOutcome::ReadOnlyCompleted(
            WorthQueryReadOnlyPreviewCompletion::new(
                lowered_plan,
                aftermath,
                execution.into_outcome(),
                counters.execution_completed(),
            ),
        )
    }
}

impl WorthQueryPromotionEligiblePreviewRequest {
    pub fn open_and_close(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryPreviewJourneyOutcome {
        let request = WorthQueryWorkflowRequest {
            declaration: self.declaration.workflow,
            context: crate::ordinary::workflow::WorthQueryWorkflowContext {
                authority: self.context.authority,
            },
        };
        match request.run(workspace) {
            WorthQueryWorkflowOutcome::Completed(completion) => {
                WorthQueryPreviewJourneyOutcome::PromotionCompleted(completion)
            }
            WorthQueryWorkflowOutcome::Stopped(stop) => {
                WorthQueryPreviewJourneyOutcome::Stopped(stop)
            }
        }
    }
}
