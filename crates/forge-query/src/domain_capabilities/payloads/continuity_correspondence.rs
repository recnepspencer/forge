use crate::correspondence::{
    CorrespondenceEvaluationRequest, StructuralCandidateBudget, StructuralCandidateDiscoveryPlan,
    StructuralCandidateOrderingContract,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryContinuityCorrespondenceSemantics {
    LineageOnly {
        canonical_subject: String,
        authoritative_counterpart: String,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: StructuralCandidateBudget,
    },
    StructuralOnly {
        candidates: Vec<String>,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: StructuralCandidateBudget,
        ordering_contract: StructuralCandidateOrderingContract,
    },
    Mixed {
        canonical_subject: String,
        authoritative_counterpart: String,
        candidates: Vec<String>,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: StructuralCandidateBudget,
        ordering_contract: StructuralCandidateOrderingContract,
    },
}

impl ForgeQueryContinuityCorrespondenceSemantics {
    pub fn lineage_only(
        canonical_subject: impl Into<String>,
        authoritative_counterpart: impl Into<String>,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: usize,
    ) -> Self {
        Self::LineageOnly {
            canonical_subject: canonical_subject.into(),
            authoritative_counterpart: authoritative_counterpart.into(),
            discovery_plan,
            budget: StructuralCandidateBudget::bounded(budget),
        }
    }

    pub fn structural_only(
        candidates: impl IntoIterator<Item = impl Into<String>>,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: usize,
        ordering_contract: StructuralCandidateOrderingContract,
    ) -> Self {
        Self::StructuralOnly {
            candidates: candidates.into_iter().map(Into::into).collect(),
            discovery_plan,
            budget: StructuralCandidateBudget::bounded(budget),
            ordering_contract,
        }
    }

    pub fn mixed(
        canonical_subject: impl Into<String>,
        authoritative_counterpart: impl Into<String>,
        candidates: impl IntoIterator<Item = impl Into<String>>,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: usize,
        ordering_contract: StructuralCandidateOrderingContract,
    ) -> Self {
        Self::Mixed {
            canonical_subject: canonical_subject.into(),
            authoritative_counterpart: authoritative_counterpart.into(),
            candidates: candidates.into_iter().map(Into::into).collect(),
            discovery_plan,
            budget: StructuralCandidateBudget::bounded(budget),
            ordering_contract,
        }
    }

    pub fn to_request(&self) -> CorrespondenceEvaluationRequest {
        match self {
            Self::LineageOnly {
                canonical_subject,
                authoritative_counterpart,
                discovery_plan,
                budget,
            } => CorrespondenceEvaluationRequest::lineage_only(
                canonical_subject.clone(),
                authoritative_counterpart.clone(),
                discovery_plan.clone(),
                budget.max_candidates(),
            ),
            Self::StructuralOnly {
                candidates,
                discovery_plan,
                budget,
                ordering_contract,
            } => CorrespondenceEvaluationRequest::structural_only(
                candidates.clone(),
                discovery_plan.clone(),
                budget.max_candidates(),
                ordering_contract.clone(),
            ),
            Self::Mixed {
                canonical_subject,
                authoritative_counterpart,
                candidates,
                discovery_plan,
                budget,
                ordering_contract,
            } => CorrespondenceEvaluationRequest::mixed(
                canonical_subject.clone(),
                authoritative_counterpart.clone(),
                candidates.clone(),
                discovery_plan.clone(),
                budget.max_candidates(),
                ordering_contract.clone(),
            ),
        }
    }

    pub fn digest_fragment(&self) -> String {
        match self {
            Self::LineageOnly {
                canonical_subject,
                authoritative_counterpart,
                discovery_plan,
                budget,
            } => format!(
                "lineage:{}:{}:{}:{}",
                canonical_subject,
                authoritative_counterpart,
                discovery_plan.as_str(),
                budget.max_candidates()
            ),
            Self::StructuralOnly {
                candidates,
                discovery_plan,
                budget,
                ordering_contract,
            } => format!(
                "structural:{}:{}:{}:{}",
                candidates.join("|"),
                discovery_plan.as_str(),
                budget.max_candidates(),
                ordering_contract.as_str()
            ),
            Self::Mixed {
                canonical_subject,
                authoritative_counterpart,
                candidates,
                discovery_plan,
                budget,
                ordering_contract,
            } => format!(
                "mixed:{}:{}:{}:{}:{}:{}",
                canonical_subject,
                authoritative_counterpart,
                candidates.join("|"),
                discovery_plan.as_str(),
                budget.max_candidates(),
                ordering_contract.as_str()
            ),
        }
    }
}
