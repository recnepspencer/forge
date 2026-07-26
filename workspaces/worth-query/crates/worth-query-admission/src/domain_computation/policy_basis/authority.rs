use crate::admission_digest::hash_parts;

use super::PolicyAspectMask;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyEpoch {
    Synthetic(u64),
}

impl PolicyEpoch {
    pub fn as_u64(&self) -> u64 {
        match self {
            Self::Synthetic(epoch) => *epoch,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BranchAccessGrantClass {
    Granted,
    Denied,
}

impl BranchAccessGrantClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Granted => "granted",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyCostPosture {
    ConstantProof,
    BoundedRelationshipProof,
    NonDisclosingFieldUse,
    UnknownCost,
    CrossTenantFanout,
}

impl PolicyCostPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConstantProof => "constant_proof",
            Self::BoundedRelationshipProof => "bounded_relationship_proof",
            Self::NonDisclosingFieldUse => "non_disclosing_field_use",
            Self::UnknownCost => "unknown_cost",
            Self::CrossTenantFanout => "cross_tenant_fanout",
        }
    }

    pub(crate) fn phase_one_admitted(&self) -> bool {
        matches!(
            self,
            Self::ConstantProof | Self::BoundedRelationshipProof | Self::NonDisclosingFieldUse
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PolicyWorkBudget {
    max_relationship_checks: u32,
    max_policy_predicates: u32,
    max_schema_variants: u32,
}

impl PolicyWorkBudget {
    pub fn bounded(
        max_relationship_checks: u32,
        max_policy_predicates: u32,
        max_schema_variants: u32,
    ) -> Self {
        Self {
            max_relationship_checks,
            max_policy_predicates,
            max_schema_variants,
        }
    }

    pub fn max_relationship_checks(&self) -> u32 {
        self.max_relationship_checks
    }

    pub fn max_policy_predicates(&self) -> u32 {
        self.max_policy_predicates
    }

    pub fn max_schema_variants(&self) -> u32 {
        self.max_schema_variants
    }

    pub fn digest_part(&self) -> String {
        format!(
            "budget:{}:{}:{}",
            self.max_relationship_checks, self.max_policy_predicates, self.max_schema_variants
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyRuleSnapshot {
    policy_basis_label: String,
    rule_set_digest: String,
    policy_epoch: PolicyEpoch,
    admits_query_family: bool,
    narrows_projection: bool,
    admits_non_disclosing_use: bool,
    cost_posture: PolicyCostPosture,
    work_budget: Option<PolicyWorkBudget>,
    projection_mask: Option<PolicyAspectMask>,
    digest: String,
}

impl PolicyRuleSnapshot {
    pub fn synthetic_authority(
        policy_basis_label: impl Into<String>,
        rule_set_label: impl Into<String>,
        policy_epoch: PolicyEpoch,
    ) -> Self {
        Self::synthetic_authority_with_budget(
            policy_basis_label,
            rule_set_label,
            policy_epoch,
            true,
            PolicyCostPosture::ConstantProof,
            Some(PolicyWorkBudget::bounded(1, 1, 1)),
        )
    }

    pub fn synthetic_authority_with_query_admission(
        policy_basis_label: impl Into<String>,
        rule_set_label: impl Into<String>,
        policy_epoch: PolicyEpoch,
        admits_query_family: bool,
    ) -> Self {
        Self::synthetic_authority_with_budget(
            policy_basis_label,
            rule_set_label,
            policy_epoch,
            admits_query_family,
            PolicyCostPosture::ConstantProof,
            Some(PolicyWorkBudget::bounded(1, 1, 1)),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn synthetic_authority_with_budget(
        policy_basis_label: impl Into<String>,
        rule_set_label: impl Into<String>,
        policy_epoch: PolicyEpoch,
        admits_query_family: bool,
        cost_posture: PolicyCostPosture,
        work_budget: Option<PolicyWorkBudget>,
    ) -> Self {
        Self::synthetic_authority_with_budget_and_projection(
            policy_basis_label,
            rule_set_label,
            policy_epoch,
            admits_query_family,
            false,
            false,
            cost_posture,
            work_budget,
            None,
        )
    }

    pub fn synthetic_authority_with_projection(
        policy_basis_label: impl Into<String>,
        rule_set_label: impl Into<String>,
        policy_epoch: PolicyEpoch,
        projection_mask: PolicyAspectMask,
    ) -> Self {
        let narrows_projection = projection_mask.has_restricted_fields();
        let admits_non_disclosing_use = projection_mask.has_non_disclosing_fields();
        let cost_posture = if admits_non_disclosing_use {
            PolicyCostPosture::NonDisclosingFieldUse
        } else if narrows_projection {
            PolicyCostPosture::BoundedRelationshipProof
        } else {
            PolicyCostPosture::ConstantProof
        };
        Self::synthetic_authority_with_budget_and_projection(
            policy_basis_label,
            rule_set_label,
            policy_epoch,
            true,
            narrows_projection,
            admits_non_disclosing_use,
            cost_posture,
            Some(PolicyWorkBudget::bounded(1, 1, 1)),
            Some(projection_mask),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn synthetic_authority_with_projection_budget(
        policy_basis_label: impl Into<String>,
        rule_set_label: impl Into<String>,
        policy_epoch: PolicyEpoch,
        admits_query_family: bool,
        cost_posture: PolicyCostPosture,
        work_budget: Option<PolicyWorkBudget>,
        projection_mask: PolicyAspectMask,
    ) -> Self {
        let narrows_projection = projection_mask.has_restricted_fields();
        let admits_non_disclosing_use = projection_mask.has_non_disclosing_fields();
        Self::synthetic_authority_with_budget_and_projection(
            policy_basis_label,
            rule_set_label,
            policy_epoch,
            admits_query_family,
            narrows_projection,
            admits_non_disclosing_use,
            cost_posture,
            work_budget,
            Some(projection_mask),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn synthetic_authority_with_budget_and_projection(
        policy_basis_label: impl Into<String>,
        rule_set_label: impl Into<String>,
        policy_epoch: PolicyEpoch,
        admits_query_family: bool,
        narrows_projection: bool,
        admits_non_disclosing_use: bool,
        cost_posture: PolicyCostPosture,
        work_budget: Option<PolicyWorkBudget>,
        projection_mask: Option<PolicyAspectMask>,
    ) -> Self {
        let policy_basis_label = policy_basis_label.into();
        let rule_set_label = rule_set_label.into();
        let rule_set_digest = hash_parts(&["policy_rule_set".to_string(), rule_set_label]);
        let mut digest_parts = vec![
            format!("policy_basis:{policy_basis_label}"),
            format!("rule_set:{rule_set_digest}"),
            format!("epoch:{}", policy_epoch.as_u64()),
            format!("admits_query_family:{admits_query_family}"),
            format!("narrows_projection:{narrows_projection}"),
            format!("admits_non_disclosing_use:{admits_non_disclosing_use}"),
            format!("cost_posture:{}", cost_posture.as_str()),
            format!(
                "work_budget:{}",
                work_budget
                    .as_ref()
                    .map(PolicyWorkBudget::digest_part)
                    .unwrap_or_else(|| "missing".to_string())
            ),
        ];
        if let Some(mask) = projection_mask.as_ref() {
            digest_parts.extend(mask.digest_parts());
        } else {
            digest_parts.push("policy_aspect_mask:unspecified".to_string());
        }
        let digest = hash_parts(&digest_parts);
        Self {
            policy_basis_label,
            rule_set_digest,
            policy_epoch,
            admits_query_family,
            narrows_projection,
            admits_non_disclosing_use,
            cost_posture,
            work_budget,
            projection_mask,
            digest,
        }
    }

    pub fn policy_basis_label(&self) -> &str {
        &self.policy_basis_label
    }

    pub fn rule_set_digest(&self) -> &str {
        &self.rule_set_digest
    }

    pub fn policy_epoch(&self) -> PolicyEpoch {
        self.policy_epoch
    }

    pub fn admits_query_family(&self) -> bool {
        self.admits_query_family
    }

    pub fn narrows_projection(&self) -> bool {
        self.narrows_projection
    }

    pub fn admits_non_disclosing_use(&self) -> bool {
        self.admits_non_disclosing_use
    }

    pub fn cost_posture(&self) -> PolicyCostPosture {
        self.cost_posture
    }

    pub fn work_budget(&self) -> Option<PolicyWorkBudget> {
        self.work_budget
    }

    pub(crate) fn projection_mask(&self) -> Option<&PolicyAspectMask> {
        self.projection_mask.as_ref()
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
