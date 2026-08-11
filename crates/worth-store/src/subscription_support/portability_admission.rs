use super::{
    SubscriptionSupportOperationalBasis, SubscriptionSupportPortabilityDecision, SupportActionId,
    SupportPortabilityManifestBudget, SupportProgramPathPolicy,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionSupportPortabilityBatchRequest {
    pub action_id: SupportActionId,
    pub affected_bases: Vec<SubscriptionSupportOperationalBasis>,
    pub included_support_count: u64,
    pub omitted_support_count: u64,
    pub manifest_budget: SupportPortabilityManifestBudget,
    pub decision: SubscriptionSupportPortabilityDecision,
    pub path: SupportProgramPathPolicy,
}
