use super::{
    SubscriptionSupportOperationalBasis, SubscriptionSupportRetentionDecision, SupportActionId,
    SupportProgramPathPolicy,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionSupportRetentionBatchRequest {
    pub action_id: SupportActionId,
    pub affected_bases: Vec<SubscriptionSupportOperationalBasis>,
    pub decision: SubscriptionSupportRetentionDecision,
    pub path: SupportProgramPathPolicy,
}
