#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowDeclarationFamily {
    ConflictInspectionNarrow,
    PostMergeInspectionNarrow,
    MutationLoweringNarrow,
    MergeLoweringNarrow,
    WritebackLoweringNarrow,
}

impl WorkflowDeclarationFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConflictInspectionNarrow => "conflict_inspection_narrow",
            Self::PostMergeInspectionNarrow => "post_merge_inspection_narrow",
            Self::MutationLoweringNarrow => "mutation_lowering_narrow",
            Self::MergeLoweringNarrow => "merge_lowering_narrow",
            Self::WritebackLoweringNarrow => "writeback_lowering_narrow",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowAuthorityTargetFamily {
    QueryInspection,
    RelationalMutation,
    RelationalMerge,
    BridgeWriteback,
}

impl WorkflowAuthorityTargetFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::QueryInspection => "query_inspection",
            Self::RelationalMutation => "relational_mutation",
            Self::RelationalMerge => "relational_merge",
            Self::BridgeWriteback => "bridge_writeback",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowCostClass {
    InspectionNarrow,
    MutationLoweringNarrow,
    MergeLoweringNarrow,
    WritebackLoweringNarrow,
}

impl WorkflowCostClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InspectionNarrow => "inspection_narrow",
            Self::MutationLoweringNarrow => "mutation_lowering_narrow",
            Self::MergeLoweringNarrow => "merge_lowering_narrow",
            Self::WritebackLoweringNarrow => "writeback_lowering_narrow",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowBudgetClass {
    InspectionBounded,
    AuthorityTargetBounded,
    CrossBoundaryExpansion,
}

impl WorkflowBudgetClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InspectionBounded => "inspection_bounded",
            Self::AuthorityTargetBounded => "authority_target_bounded",
            Self::CrossBoundaryExpansion => "cross_boundary_expansion",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowFreshnessPolicy {
    ExactBasis,
    AllowExplicitRebind,
}

impl WorkflowFreshnessPolicy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExactBasis => "exact_basis",
            Self::AllowExplicitRebind => "allow_explicit_rebind",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowDeclarationRequest {
    declaration_family: WorkflowDeclarationFamily,
    authority_target_family: WorkflowAuthorityTargetFamily,
    cost_class: WorkflowCostClass,
    budget_class: WorkflowBudgetClass,
    freshness_policy: WorkflowFreshnessPolicy,
}

impl WorkflowDeclarationRequest {
    pub fn new(
        declaration_family: WorkflowDeclarationFamily,
        authority_target_family: WorkflowAuthorityTargetFamily,
        cost_class: WorkflowCostClass,
        budget_class: WorkflowBudgetClass,
        freshness_policy: WorkflowFreshnessPolicy,
    ) -> Self {
        Self {
            declaration_family,
            authority_target_family,
            cost_class,
            budget_class,
            freshness_policy,
        }
    }

    pub fn declaration_family(&self) -> &WorkflowDeclarationFamily {
        &self.declaration_family
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }

    pub fn cost_class(&self) -> &WorkflowCostClass {
        &self.cost_class
    }

    pub fn budget_class(&self) -> &WorkflowBudgetClass {
        &self.budget_class
    }

    pub fn freshness_policy(&self) -> &WorkflowFreshnessPolicy {
        &self.freshness_policy
    }
}
