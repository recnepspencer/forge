use super::contracts::StructuralCandidateBudget;
use super::cost::{StructuralCandidateDiscoveryPlan, StructuralCandidateOrderingContract};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceCandidateSet {
    candidates: Vec<String>,
    discovery_plan: StructuralCandidateDiscoveryPlan,
    budget: StructuralCandidateBudget,
    ordering_contract: StructuralCandidateOrderingContract,
}

impl CorrespondenceCandidateSet {
    pub fn candidates(&self) -> &[String] {
        &self.candidates
    }

    pub fn len(&self) -> usize {
        self.candidates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }

    pub fn discovery_plan(&self) -> &StructuralCandidateDiscoveryPlan {
        &self.discovery_plan
    }

    pub fn budget(&self) -> &StructuralCandidateBudget {
        &self.budget
    }

    pub fn ordering_contract(&self) -> &StructuralCandidateOrderingContract {
        &self.ordering_contract
    }

    pub(crate) fn new(
        candidates: Vec<String>,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: StructuralCandidateBudget,
        ordering_contract: StructuralCandidateOrderingContract,
    ) -> Self {
        Self {
            candidates,
            discovery_plan,
            budget,
            ordering_contract,
        }
    }
}
