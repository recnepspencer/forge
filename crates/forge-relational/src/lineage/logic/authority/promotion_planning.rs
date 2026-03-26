use crate::history::data::CommitReference;
use crate::lineage::data::CorrespondencePromotionRejectionClass;
use crate::lineage::logic::authority::phase_types::{
    BranchScopedCommitReference, LoweredPromotionPlan, PromotionAuthority,
    PromotionEligibleCorrespondenceCandidate, ValidatedCorrespondenceCandidate,
};
use crate::lineage::logic::authority::LineageAuthority;

impl PromotionAuthority {
    fn new(branch_id: crate::history::data::BranchId) -> Self {
        Self { branch_id }
    }
}

impl PromotionEligibleCorrespondenceCandidate {
    fn new(
        candidate: crate::lineage::data::CorrespondenceCandidate,
        authority: PromotionAuthority,
        branch_scoped_sources: Vec<
            crate::lineage::logic::authority::phase_types::BranchScopedLineageRef,
        >,
        branch_scoped_targets: Vec<
            crate::lineage::logic::authority::phase_types::BranchScopedLineageRef,
        >,
    ) -> Self {
        Self {
            candidate,
            authority,
            branch_scoped_sources,
            branch_scoped_targets,
        }
    }
}

impl BranchScopedCommitReference {
    fn new(
        authority: &PromotionAuthority,
        commit: CommitReference,
    ) -> Result<Self, CorrespondencePromotionRejectionClass> {
        if commit.branch_id != *authority.branch_id() {
            return Err(CorrespondencePromotionRejectionClass::BranchScopeMismatch);
        }
        Ok(Self { commit })
    }
}

impl LoweredPromotionPlan {
    fn new(
        candidate_id: crate::lineage::data::CorrespondenceCandidateId,
        authority: PromotionAuthority,
        commit: BranchScopedCommitReference,
        sources: Vec<crate::lineage::logic::authority::phase_types::BranchScopedLineageRef>,
        targets: Vec<crate::lineage::logic::authority::phase_types::BranchScopedLineageRef>,
    ) -> Result<Self, CorrespondencePromotionRejectionClass> {
        if !sources
            .iter()
            .all(|entry| entry.branch_id() == authority.branch_id())
            || !targets
                .iter()
                .all(|entry| entry.branch_id() == authority.branch_id())
        {
            return Err(CorrespondencePromotionRejectionClass::BranchScopeMismatch);
        }
        Ok(Self {
            candidate_id,
            authority,
            commit,
            sources,
            targets,
        })
    }
}

impl<'runtime> LineageAuthority<'runtime> {
    pub(super) fn promotion_eligible_candidate(
        &mut self,
        validated: ValidatedCorrespondenceCandidate,
        commit: &CommitReference,
    ) -> Result<PromotionEligibleCorrespondenceCandidate, CorrespondencePromotionRejectionClass>
    {
        let candidate = validated.candidate();
        let validated_width =
            validated.branch_scoped_sources().len() + validated.branch_scoped_targets().len();
        if candidate.branch_id != commit.branch_id {
            self.runtime
                .performance_access()
                .count_lineage_promotion_plan_lowering(0);
            self.record_rejected_promotion_for_candidate(
                Some(candidate),
                &commit.branch_id,
                candidate.candidate_id,
                CorrespondencePromotionRejectionClass::CommitBranchMismatch,
                "correspondence promotion commit branch must match candidate branch scope",
            );
            return Err(CorrespondencePromotionRejectionClass::CommitBranchMismatch);
        }
        if self
            .runtime
            .history_access()
            .branch_head(&commit.branch_id)
            .map(|head| head.commit_id)
            != Some(commit.commit_id)
        {
            self.runtime
                .performance_access()
                .count_lineage_promotion_plan_lowering(0);
            self.record_rejected_promotion_for_candidate(
                Some(candidate),
                &commit.branch_id,
                candidate.candidate_id,
                CorrespondencePromotionRejectionClass::CommitNotBranchHead,
                "correspondence promotion must target the current branch head commit",
            );
            return Err(CorrespondencePromotionRejectionClass::CommitNotBranchHead);
        }
        self.runtime
            .performance_access()
            .count_lineage_promotion_plan_lowering(validated_width);
        Ok(PromotionEligibleCorrespondenceCandidate::new(
            candidate.clone(),
            PromotionAuthority::new(candidate.branch_id.clone()),
            validated.branch_scoped_sources().to_vec(),
            validated.branch_scoped_targets().to_vec(),
        ))
    }

    pub(super) fn lower_promotion_plan(
        &self,
        eligible: PromotionEligibleCorrespondenceCandidate,
        commit: CommitReference,
    ) -> Result<LoweredPromotionPlan, CorrespondencePromotionRejectionClass> {
        LoweredPromotionPlan::new(
            eligible.candidate().candidate_id,
            eligible.authority().clone(),
            BranchScopedCommitReference::new(eligible.authority(), commit)?,
            eligible.branch_scoped_sources().to_vec(),
            eligible.branch_scoped_targets().to_vec(),
        )
    }
}
