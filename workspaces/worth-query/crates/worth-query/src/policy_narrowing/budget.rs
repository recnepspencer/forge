use crate::policy_basis::{PolicyCostPosture, PolicyWorkBudget};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyNarrowingCostPosture {
    ConstantProof,
    BoundedRelationshipProof,
    NonDisclosingFieldUse,
}

impl PolicyNarrowingCostPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConstantProof => "constant_proof",
            Self::BoundedRelationshipProof => "bounded_relationship_proof",
            Self::NonDisclosingFieldUse => "non_disclosing_field_use",
        }
    }

    pub fn from_policy(value: PolicyCostPosture) -> Option<Self> {
        match value {
            PolicyCostPosture::ConstantProof => Some(Self::ConstantProof),
            PolicyCostPosture::BoundedRelationshipProof => Some(Self::BoundedRelationshipProof),
            PolicyCostPosture::NonDisclosingFieldUse => Some(Self::NonDisclosingFieldUse),
            PolicyCostPosture::UnknownCost | PolicyCostPosture::CrossTenantFanout => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PolicyNarrowingWorkBudget {
    max_field_references: usize,
    max_projected_fields: usize,
    max_masked_fields: usize,
    max_relationship_descriptors: usize,
    max_relationship_topology_width: usize,
    max_validation_denials_retained: usize,
    max_digest_part_count: usize,
}

impl PolicyNarrowingWorkBudget {
    pub fn bounded(
        max_field_references: usize,
        max_projected_fields: usize,
        max_masked_fields: usize,
        max_relationship_descriptors: usize,
        max_relationship_topology_width: usize,
        max_validation_denials_retained: usize,
        max_digest_part_count: usize,
    ) -> Self {
        Self {
            max_field_references,
            max_projected_fields,
            max_masked_fields,
            max_relationship_descriptors,
            max_relationship_topology_width,
            max_validation_denials_retained,
            max_digest_part_count,
        }
    }

    pub fn from_policy_budget(policy_budget: PolicyWorkBudget) -> Self {
        Self::bounded(
            usize::try_from(policy_budget.max_policy_predicates()).unwrap_or(usize::MAX) + 16,
            usize::try_from(policy_budget.max_policy_predicates()).unwrap_or(usize::MAX) + 16,
            usize::try_from(policy_budget.max_policy_predicates()).unwrap_or(usize::MAX) + 16,
            usize::try_from(policy_budget.max_relationship_checks()).unwrap_or(usize::MAX),
            usize::try_from(policy_budget.max_relationship_checks()).unwrap_or(usize::MAX),
            8,
            64,
        )
    }

    pub fn max_field_references(&self) -> usize {
        self.max_field_references
    }

    pub fn max_projected_fields(&self) -> usize {
        self.max_projected_fields
    }

    pub fn max_masked_fields(&self) -> usize {
        self.max_masked_fields
    }

    pub fn max_relationship_descriptors(&self) -> usize {
        self.max_relationship_descriptors
    }

    pub fn max_relationship_topology_width(&self) -> usize {
        self.max_relationship_topology_width
    }

    pub fn max_validation_denials_retained(&self) -> usize {
        self.max_validation_denials_retained
    }

    pub fn max_digest_part_count(&self) -> usize {
        self.max_digest_part_count
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "narrowing_budget:{}:{}:{}:{}:{}:{}:{}",
            self.max_field_references,
            self.max_projected_fields,
            self.max_masked_fields,
            self.max_relationship_descriptors,
            self.max_relationship_topology_width,
            self.max_validation_denials_retained,
            self.max_digest_part_count
        )
    }
}
