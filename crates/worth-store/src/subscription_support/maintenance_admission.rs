use super::{
    SubscriptionSupportMaintenanceDecision, SubscriptionSupportOperationalBasis, SupportActionId,
    SupportProgramPathPolicy,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionSupportMaintenanceBatchRequest {
    pub action_id: SupportActionId,
    pub affected_bases: Vec<SubscriptionSupportOperationalBasis>,
    pub decision: SubscriptionSupportMaintenanceDecision,
    pub path: SupportProgramPathPolicy,
}
