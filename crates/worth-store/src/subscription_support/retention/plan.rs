use super::super::{
    classification_error, SubscriptionSupportOperationalVerdict, SupportActionId,
    SupportProgramDensityClass, SupportProgramPathPlan,
};
use super::affected_set::SupportAffectedSet;
use super::decision::SubscriptionSupportRetentionDecision;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportRetentionBatchPlan {
    action_id: SupportActionId,
    affected_set: SupportAffectedSet,
    path_plan: SupportProgramPathPlan,
    decision: SubscriptionSupportRetentionDecision,
}

impl SupportRetentionBatchPlan {
    pub(crate) fn new(
        action_id: SupportActionId,
        affected_set: SupportAffectedSet,
        path_plan: SupportProgramPathPlan,
        decision: SubscriptionSupportRetentionDecision,
    ) -> Result<Self, StoreError> {
        if path_plan.density_class() == SupportProgramDensityClass::StoreGlobalDebt {
            return Err(classification_error(
                "subscription-support retention cannot admit store-global density",
            ));
        }
        if path_plan.batch_width() != affected_set.affected_count() {
            return Err(classification_error(
                "subscription-support retention plan width must match affected-set breadth",
            ));
        }
        Ok(Self {
            action_id,
            affected_set,
            path_plan,
            decision,
        })
    }

    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub fn affected_set(&self) -> &SupportAffectedSet {
        &self.affected_set
    }

    pub fn path_plan(&self) -> &SupportProgramPathPlan {
        &self.path_plan
    }

    pub fn decision(&self) -> &SubscriptionSupportRetentionDecision {
        &self.decision
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.decision.verdict()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        SupportActionId,
        SupportAffectedSet,
        SupportProgramPathPlan,
        SubscriptionSupportRetentionDecision,
    ) {
        (
            self.action_id,
            self.affected_set,
            self.path_plan,
            self.decision,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRetentionPlan {
    batch_plan: SupportRetentionBatchPlan,
}

impl SubscriptionSupportRetentionPlan {
    #[allow(dead_code)]
    pub(crate) fn new(batch_plan: SupportRetentionBatchPlan) -> Self {
        Self { batch_plan }
    }

    pub fn batch_plan(&self) -> &SupportRetentionBatchPlan {
        &self.batch_plan
    }
}
