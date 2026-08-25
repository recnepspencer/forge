use crate::ordinary::workflow::{
    WorthQueryAdmittedWorkflowEffect, WorthQueryLoweredWorkflowPlan, WorthQueryWorkflowCounters,
};
use crate::runtime::{WorthQueryMergeAuthorityValidationError, WorthQueryWorkspace};

use super::{
    WorthQueryBranchMergeAftermath, WorthQueryBranchMergeCompletion, WorthQueryBranchMergeDeferred,
    WorthQueryBranchMergeOutcome, WorthQueryBranchMergeRequest,
    WorthQueryBranchMergeSettlementDeferred, WorthQueryBranchMergeStop,
    WorthQueryBranchMergeStopSource,
};

impl WorthQueryBranchMergeRequest {
    pub fn run(self, workspace: &mut WorthQueryWorkspace) -> WorthQueryBranchMergeOutcome {
        let counters = WorthQueryWorkflowCounters::context_checked();
        if self.declaration.identity() != &self.context.declaration_identity {
            return WorthQueryBranchMergeOutcome::Stopped(WorthQueryBranchMergeStop::denied(
                WorthQueryBranchMergeStopSource::MismatchedContext,
                "branch-merge context was captured for a different declaration",
                counters,
            ));
        }
        let authority = match workspace.validate_ordinary_merge_authority(self.context.authority) {
            Ok(authority) => authority,
            Err(WorthQueryMergeAuthorityValidationError::ForeignOwner) => {
                return WorthQueryBranchMergeOutcome::Stopped(WorthQueryBranchMergeStop::denied(
                    WorthQueryBranchMergeStopSource::ForeignAuthority,
                    "branch-merge context belongs to a different runtime authority",
                    counters,
                ));
            }
            Err(WorthQueryMergeAuthorityValidationError::StaleSnapshot) => {
                return WorthQueryBranchMergeOutcome::Stopped(WorthQueryBranchMergeStop::denied(
                    WorthQueryBranchMergeStopSource::StaleAuthority,
                    "branch-merge context is stale against the current branch basis",
                    counters,
                ));
            }
        };
        let materialize_inspection = self
            .declaration
            .inspection_policy()
            .materializes_rich_inspection();
        if materialize_inspection {
            if let Err(error) = workspace.admit_ordinary_rich_inspection() {
                return WorthQueryBranchMergeOutcome::Stopped(WorthQueryBranchMergeStop::denied(
                    WorthQueryBranchMergeStopSource::InspectionUnavailable,
                    error.to_string(),
                    counters,
                ));
            }
        }
        let counters = counters.lower_runtime_attempted();
        let execution = match workspace.execute_ordinary_merge(
            authority,
            self.declaration.identity().evidence_identity(),
            materialize_inspection,
        ) {
            Ok(execution) => execution,
            Err(crate::runtime::WorthQueryOrdinaryMergeExecutionError::Deferred { message }) => {
                return WorthQueryBranchMergeOutcome::Deferred(WorthQueryBranchMergeDeferred::new(
                    message, counters,
                ));
            }
            Err(crate::runtime::WorthQueryOrdinaryMergeExecutionError::Denied {
                stage,
                message,
            }) => {
                return WorthQueryBranchMergeOutcome::Stopped(
                    WorthQueryBranchMergeStop::from_execution_denial(stage, message, counters),
                );
            }
            Err(crate::runtime::WorthQueryOrdinaryMergeExecutionError::SettlementDeferred {
                message,
                settlement,
            }) => {
                return WorthQueryBranchMergeOutcome::SettlementDeferred(
                    WorthQueryBranchMergeSettlementDeferred::new(message, settlement, counters),
                );
            }
        };
        let aftermath = match WorthQueryBranchMergeAftermath::from_receipt(execution.receipt()) {
            Some(aftermath) => aftermath,
            None => {
                return WorthQueryBranchMergeOutcome::Stopped(WorthQueryBranchMergeStop::denied(
                    WorthQueryBranchMergeStopSource::RelationalExecution,
                    "branch-merge execution returned non-merge aftermath evidence",
                    counters,
                ));
            }
        };
        let admitted_effect = WorthQueryAdmittedWorkflowEffect::from_effect_lifecycle(
            execution.admitted_effect_identity().clone(),
        );
        let lowered_plan =
            WorthQueryLoweredWorkflowPlan::new(execution.lowered_plan_identity().clone());
        let (receipt, diagnostics) = execution.into_parts();
        WorthQueryBranchMergeOutcome::Completed(WorthQueryBranchMergeCompletion::new(
            admitted_effect,
            lowered_plan,
            receipt,
            aftermath,
            diagnostics,
            counters.execution_completed(materialize_inspection),
        ))
    }
}
