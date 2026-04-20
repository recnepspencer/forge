use crate::{
    delta::{
        BranchDeltaReadRequest, BranchDeltaReadStrategy, BranchDeltaRewritePlan,
        BranchDeltaRewritePolicyDecision, BranchDeltaRewriteRecommendation,
        BranchDeltaRewriteRequest, BranchDeltaRewriteStrategy, RewriteEligibleDeltaSegment,
        MAX_REWRITE_LAYER_WIDTH, RECOMMENDED_REWRITE_LAYER_WIDTH,
    },
    failure::{StoreError, StoreErrorKind},
};

use crate::backend::records::StoreState;

impl StoreState {
    pub fn plan_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewritePlan, StoreError> {
        let read_plan = self.plan_branch_delta_read(BranchDeltaReadRequest::new(
            request.branch_id,
            request.target_commit_id,
        ))?;
        match read_plan.strategy {
            BranchDeltaReadStrategy::EmptyBranchReuse => Ok(BranchDeltaRewritePlan::new(
                BranchDeltaRewriteStrategy::NotNeeded,
                None,
                0,
            )),
            BranchDeltaReadStrategy::AuthorityReplayControl => Ok(BranchDeltaRewritePlan::new(
                BranchDeltaRewriteStrategy::RejectAsTooBroad,
                None,
                read_plan.performance.replay_commit_count,
            )),
            BranchDeltaReadStrategy::DirectLayerRead => {
                if read_plan.used_layer_ids.len() <= 1 {
                    return Ok(BranchDeltaRewritePlan::new(
                        BranchDeltaRewriteStrategy::NotNeeded,
                        None,
                        read_plan.used_layer_ids.len(),
                    ));
                }
                if read_plan.used_layer_ids.len() > MAX_REWRITE_LAYER_WIDTH {
                    return Err(StoreError::new(
                        StoreErrorKind::BranchDeltaRewriteBudgetExceeded,
                        format!(
                            "branch delta rewrite planning for branch `{}` target {} exceeded the admitted rewrite width budget (width={}, max_width={})",
                            read_plan.locality.branch_id.0,
                            read_plan.locality.target_commit_id.0,
                            read_plan.used_layer_ids.len(),
                            MAX_REWRITE_LAYER_WIDTH
                        ),
                    ));
                }
                Ok(BranchDeltaRewritePlan::new(
                    BranchDeltaRewriteStrategy::ReplaceContiguousSegment,
                    Some(RewriteEligibleDeltaSegment::new(
                        read_plan.locality.branch_id.clone(),
                        read_plan.locality.base_frontier_commit_id,
                        read_plan.locality.target_commit_id,
                        read_plan.used_layer_ids.clone(),
                        read_plan.commit_ids.clone(),
                    )),
                    read_plan.used_layer_ids.len(),
                ))
            }
        }
    }

    pub fn recommend_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewriteRecommendation, StoreError> {
        let plan = self.plan_delta_rewrite(request)?;
        let decision = match plan.strategy() {
            BranchDeltaRewriteStrategy::NotNeeded => BranchDeltaRewritePolicyDecision::NoAction,
            BranchDeltaRewriteStrategy::RejectAsTooBroad => {
                BranchDeltaRewritePolicyDecision::RejectAsTooBroad
            }
            BranchDeltaRewriteStrategy::ReplaceContiguousSegment => {
                if plan.rewrite_breadth() >= RECOMMENDED_REWRITE_LAYER_WIDTH {
                    BranchDeltaRewritePolicyDecision::CompactNow
                } else {
                    BranchDeltaRewritePolicyDecision::Defer
                }
            }
        };
        Ok(BranchDeltaRewriteRecommendation {
            decision,
            plan,
            recommended_layer_width: RECOMMENDED_REWRITE_LAYER_WIDTH,
        })
    }
}
