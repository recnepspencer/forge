use serde_json::json;

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::lineage::data::{
    CorrespondencePromotionExecutionFailureClass, CorrespondenceResolution, LineageDecisionKind,
    LineageFinalizationArtifact, LineageEventKind,
};
use crate::lineage::logic::authority::phase_types::{
    ExecutionAuthorizedPromotionPlan, LoweredPromotionPlan,
};
use crate::lineage::logic::authority::LineageAuthority;

impl<'runtime> LineageAuthority<'runtime> {
    pub(super) fn execute_promotion_plan(
        &mut self,
        plan: LoweredPromotionPlan,
    ) -> CorrespondenceResolution {
        let candidate_id = plan.candidate_id();
        let plan = match self.authorize_promotion_execution(plan) {
            Ok(plan) => plan,
            Err(failure_class) => {
                self.record_execution_failure_diagnostic(
                    None,
                    0,
                    failure_class,
                );
                return CorrespondenceResolution::execution_failed(
                    candidate_id,
                    0,
                    failure_class,
                );
            }
        };
        let event = self.prepare_authoritative_lineage_event(
            plan.commit(),
            LineageEventKind::Correspond,
            plan.sources().iter().map(|entry| entry.lineage_id()).collect(),
            plan.targets().iter().map(|entry| entry.lineage_id()).collect(),
        );
        let event_id = event.event_id;
        let decision = self.accepted_decision_record(
            LineageDecisionKind::CorrespondencePromotionAccepted,
            &event,
            Some(plan.candidate_id()),
        );
        let artifact =
            LineageFinalizationArtifact::single_event(plan.branch_id().clone(), event, decision);
        self.runtime.performance_access().count_lineage_finalization(
            artifact.event_batch().events().len(),
            artifact.decision_log().decisions().len(),
        );
        let promotion_commit = match self.publish_promotion_commit(&plan, &artifact) {
            Ok(commit) => commit,
            Err(failure_class) => {
                self.record_execution_failure_diagnostic(
                    Some(&plan),
                    event_id,
                    failure_class,
                );
                return CorrespondenceResolution::execution_failed(
                    plan.candidate_id(),
                    event_id,
                    failure_class,
                );
            }
        };
        self.runtime
            .performance_access()
            .count_lineage_promotion_accepted();
        let resolution = CorrespondenceResolution::promoted(
            plan.candidate_id(),
            event_id,
            promotion_commit.commit_id,
        );
        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
                DiagnosticsScope::Lineage,
                DiagnosticsArtifactKind::MinimalSummary,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::LineagePromotionPublished,
                    message: "correspondence promoted into lineage".to_string(),
                    fields: json!({
                        "candidate_id": resolution.candidate_id(),
                        "event_id": event_id,
                        "commit_id": promotion_commit.commit_id.0,
                        "anchor_commit_id": plan.commit().commit_id.0,
                        "branch_id": plan.branch_id().0,
                    }),
                }],
            );
        resolution
    }

    fn record_execution_failure_diagnostic(
        &mut self,
        plan: Option<&ExecutionAuthorizedPromotionPlan>,
        event_id: u64,
        failure_class: CorrespondencePromotionExecutionFailureClass,
    ) {
        let detail = match failure_class {
            CorrespondencePromotionExecutionFailureClass::AnchorDriftedFromBranchHead => {
                "correspondence promotion execution observed branch-head drift after planning"
            }
            CorrespondencePromotionExecutionFailureClass::AuthorityPublicationFailed => {
                "correspondence promotion execution failed while publishing the finalized lineage artifact"
            }
        };
        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
                DiagnosticsScope::Lineage,
                DiagnosticsArtifactKind::MinimalSummary,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::LineagePromotionExecutionFailed,
                    message: detail.to_string(),
                    fields: json!({
                        "candidate_id": plan.map(|plan| plan.candidate_id()),
                        "event_id": event_id,
                        "anchor_commit_id": plan.map(|plan| plan.commit().commit_id.0),
                        "branch_id": plan.map(|plan| plan.branch_id().0.clone()),
                        "execution_failure_class": format!("{failure_class:?}"),
                    }),
                }],
            );
    }
}
