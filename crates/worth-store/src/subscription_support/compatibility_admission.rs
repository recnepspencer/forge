use super::{
    SubscriptionSupportCompatibilityDecision, SubscriptionSupportOperationalBasis, SupportActionId,
    SupportCompatibilityReceiptWitness, SupportProgramPathPolicy,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionSupportCompatibilityBatchRequest {
    pub action_id: SupportActionId,
    pub affected_bases: Vec<SubscriptionSupportOperationalBasis>,
    pub compatibility_receipt: SupportCompatibilityReceiptWitness,
    pub semantic_digest: String,
    pub decision: SubscriptionSupportCompatibilityDecision,
    pub path: SupportProgramPathPolicy,
}
