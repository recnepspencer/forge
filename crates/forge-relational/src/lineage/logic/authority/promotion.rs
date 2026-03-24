use crate::history::data::CommitReference;
use crate::lineage::data::{
    CorrespondenceCandidateId, CorrespondencePromotionOutcome,
    CorrespondencePromotionRejectionClass,
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
                self.runtime
                    .performance_access()
                    .count_lineage_candidate_validation(0, 0);
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
        Ok(self.execute_promotion_plan(plan).into())
    }

    pub fn try_promote_correspondence(
        &mut self,
        candidate_id: CorrespondenceCandidateId,
        commit: CommitReference,
    ) -> CorrespondencePromotionOutcome {
        match self.promote_correspondence(candidate_id, commit) {
            Ok(resolution) => resolution,
            Err(rejection_class) => {
                CorrespondencePromotionOutcome::rejected(candidate_id, rejection_class)
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn promote_correspondence_with_post_plan_hook_for_test(
        &mut self,
        candidate_id: CorrespondenceCandidateId,
        commit: CommitReference,
        after_plan_lowered: impl FnOnce(&mut crate::logic::runtime::RelationalRuntime),
    ) -> Result<CorrespondencePromotionOutcome, CorrespondencePromotionRejectionClass> {
        let recorded = self.recorded_candidate(candidate_id)?;
        let validated = self.validate_candidate(recorded)?;
        let eligible = self.promotion_eligible_candidate(validated, &commit)?;
        let plan = self.lower_promotion_plan(eligible, commit)?;
        after_plan_lowered(self.runtime);
        Ok(self.execute_promotion_plan(plan).into())
    }
}
