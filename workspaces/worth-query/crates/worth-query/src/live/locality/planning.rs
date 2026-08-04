use super::super::promotion::LiveQueryPlan;
use super::admission::{
    LocalityAdmissionClass, LocalityBreadthBudget, LocalityCostPosture, LocalityPerformanceStatus,
    LocalityScopeAdmission, LocalitySemanticBasis, LocalityWideningBudget, LocalityWideningPolicy,
    StreamLoweringAdmissionClass, StreamLoweringCostPosture, StreamMemberWidthBudget,
    StreamWindowWidthBudget,
};
use super::matching::{LocalityAwareRelevanceContract, RegionScopedSubscriptionIdentity};
use super::scope_contract::LocalityPredicateContract;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionScopedPlanningReport {
    pub(in crate::live) query_digest: String,
    pub(in crate::live) locality_digest: String,
    pub(in crate::live) subscription_identity_digest: String,
    pub(in crate::live) relevance_contract_digest: String,
    pub(in crate::live) semantic_basis: LocalitySemanticBasis,
    pub(in crate::live) scope_admission: LocalityScopeAdmission,
    pub(in crate::live) stream_lowering_admission: StreamLoweringAdmissionClass,
    pub(in crate::live) widening_policy: LocalityWideningPolicy,
    pub(in crate::live) performance_status: LocalityPerformanceStatus,
}

impl RegionScopedPlanningReport {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn locality_digest(&self) -> &str {
        &self.locality_digest
    }

    pub fn subscription_identity_digest(&self) -> &str {
        &self.subscription_identity_digest
    }

    pub fn relevance_contract_digest(&self) -> &str {
        &self.relevance_contract_digest
    }

    pub fn semantic_basis(&self) -> &LocalitySemanticBasis {
        &self.semantic_basis
    }

    pub fn scope_admission(&self) -> &LocalityScopeAdmission {
        &self.scope_admission
    }

    pub fn stream_lowering_admission(&self) -> &StreamLoweringAdmissionClass {
        &self.stream_lowering_admission
    }

    pub fn widening_policy(&self) -> &LocalityWideningPolicy {
        &self.widening_policy
    }

    pub fn performance_status(&self) -> &LocalityPerformanceStatus {
        &self.performance_status
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionScopedLivePlan {
    pub(in crate::live) live: LiveQueryPlan,
    pub(in crate::live) locality: LocalityPredicateContract,
    pub(in crate::live) admission_class: LocalityAdmissionClass,
    pub(in crate::live) subscription_identity: RegionScopedSubscriptionIdentity,
    pub(in crate::live) relevance_contract: LocalityAwareRelevanceContract,
    pub(in crate::live) planning_report: RegionScopedPlanningReport,
    pub(in crate::live) locality_cost_posture: LocalityCostPosture,
    pub(in crate::live) locality_performance_status: LocalityPerformanceStatus,
    pub(in crate::live) locality_breadth_budget: LocalityBreadthBudget,
    pub(in crate::live) locality_widening_policy: LocalityWideningPolicy,
    pub(in crate::live) locality_widening_budget: LocalityWideningBudget,
    pub(in crate::live) stream_lowering_cost_posture: StreamLoweringCostPosture,
    pub(in crate::live) stream_member_width_budget: StreamMemberWidthBudget,
    pub(in crate::live) stream_window_width_budget: StreamWindowWidthBudget,
}

impl RegionScopedLivePlan {
    pub fn live(&self) -> &LiveQueryPlan {
        &self.live
    }

    pub fn locality(&self) -> &LocalityPredicateContract {
        &self.locality
    }

    pub fn admission_class(&self) -> &LocalityAdmissionClass {
        &self.admission_class
    }

    pub fn subscription_identity(&self) -> &RegionScopedSubscriptionIdentity {
        &self.subscription_identity
    }

    pub fn locality_subscription_digest(&self) -> &str {
        self.subscription_identity.digest()
    }

    pub fn relevance_contract(&self) -> &LocalityAwareRelevanceContract {
        &self.relevance_contract
    }

    pub fn planning_report(&self) -> &RegionScopedPlanningReport {
        &self.planning_report
    }

    pub fn locality_cost_posture(&self) -> &LocalityCostPosture {
        &self.locality_cost_posture
    }

    pub fn locality_performance_status(&self) -> &LocalityPerformanceStatus {
        &self.locality_performance_status
    }

    pub fn locality_breadth_budget(&self) -> &LocalityBreadthBudget {
        &self.locality_breadth_budget
    }

    pub fn locality_widening_policy(&self) -> &LocalityWideningPolicy {
        &self.locality_widening_policy
    }

    pub fn locality_widening_budget(&self) -> &LocalityWideningBudget {
        &self.locality_widening_budget
    }

    pub fn stream_lowering_cost_posture(&self) -> &StreamLoweringCostPosture {
        &self.stream_lowering_cost_posture
    }

    pub fn stream_member_width_budget(&self) -> &StreamMemberWidthBudget {
        &self.stream_member_width_budget
    }

    pub fn stream_window_width_budget(&self) -> &StreamWindowWidthBudget {
        &self.stream_window_width_budget
    }
}
