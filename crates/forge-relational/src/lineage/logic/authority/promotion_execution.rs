use serde_json::json;

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::lineage::data::{
    CorrespondenceResolution, LineageDecisionKind, LineageFinalizationArtifact,
    LineageEventKind, LineageResolutionStatus,
};
use crate::lineage::logic::authority::phase_types::LoweredPromotionPlan;
use crate::lineage::logic::authority::LineageAuthority;

impl<'runtime> LineageAuthority<'runtime> {
    pub(super) fn execute_promotion_plan(
        &mut self,
        plan: LoweredPromotionPlan,
    ) -> Result<CorrespondenceResolution, crate::lineage::data::CorrespondencePromotionRejectionClass>
    {
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
        let promotion_commit =
            self.publish_promotion_commit(plan.commit(), plan.candidate_id(), &artifact)?;
        let resolution = CorrespondenceResolution {
            candidate_id: plan.candidate_id(),
            status: LineageResolutionStatus::Promoted,
            promoted_event_id: Some(event_id),
            promoted_commit_id: Some(promotion_commit.commit_id),
            rejection_class: None,
        };
        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
                DiagnosticsScope::Lineage,
                DiagnosticsArtifactKind::MinimalSummary,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::CommitPublished,
                    message: "correspondence promoted into lineage".to_string(),
                    fields: json!({
                        "candidate_id": resolution.candidate_id,
                        "event_id": event_id,
                        "commit_id": promotion_commit.commit_id.0,
                        "anchor_commit_id": plan.commit().commit_id.0,
                        "branch_id": plan.branch_id().0,
                    }),
                }],
            );
        Ok(resolution)
    }
}
