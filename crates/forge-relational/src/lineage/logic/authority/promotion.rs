use crate::history::data::CommitReference;
use crate::lineage::data::{
    CorrespondenceCandidateId, CorrespondencePromotionOutcome,
    CorrespondencePromotionRejectionClass, LineageResolutionStatus,
};
use crate::lineage::logic::authority::LineageAuthority;

impl<'runtime> LineageAuthority<'runtime> {
    pub fn promote_correspondence(
        &mut self,
        candidate_id: CorrespondenceCandidateId,
        commit: CommitReference,
    ) -> Result<CorrespondencePromotionOutcome, CorrespondencePromotionRejectionClass> {
        let recorded = match self.recorded_candidate(candidate_id) {
            Ok(recorded) => recorded,
            Err(rejection_class) => {
                self.emit_promotion_rejection(
                    &commit.branch_id,
                    candidate_id,
                    rejection_class,
                    "correspondence promotion candidate was missing",
                );
                return Err(rejection_class);
            }
        };
        let validated = self.validate_candidate(recorded)?;
        let eligible = self.promotion_eligible_candidate(validated, &commit)?;
        let plan = self.lower_promotion_plan(eligible, commit)?;
        self.execute_promotion_plan(plan)
    }

    pub fn try_promote_correspondence(
        &mut self,
        candidate_id: CorrespondenceCandidateId,
        commit: CommitReference,
    ) -> CorrespondencePromotionOutcome {
        match self.promote_correspondence(candidate_id, commit) {
            Ok(resolution) => resolution,
            Err(rejection_class) => CorrespondencePromotionOutcome {
                candidate_id,
                status: LineageResolutionStatus::Rejected,
                promoted_event_id: None,
                promoted_commit_id: None,
                rejection_class: Some(rejection_class),
            },
        }
    }
}
