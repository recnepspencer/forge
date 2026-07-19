use worth_relational::facade::history::BranchId;
use worth_relational::facade::identity::EntityId;
use worth_relational::facade::transactions::AspectFieldPatch;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MutationIntentFamily {
    IntentReconciliation,
}

impl MutationIntentFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IntentReconciliation => "intent_reconciliation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RelationalStrategyTarget {
    IntentReconciliation,
}

impl RelationalStrategyTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IntentReconciliation => "intent_reconciliation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MutationLoweringInput {
    IntentReconciliation {
        entity_id: EntityId,
        desired_aspect_fields: AspectFieldPatch,
    },
}

impl MutationLoweringInput {
    pub fn family(&self) -> MutationIntentFamily {
        match self {
            Self::IntentReconciliation { .. } => MutationIntentFamily::IntentReconciliation,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MergeWorkflowIntent {
    ReconcileIntoTarget,
}

impl MergeWorkflowIntent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReconcileIntoTarget => "reconcile_into_target",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MergeAuthorityTarget {
    PairwiseExecution,
}

impl MergeAuthorityTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PairwiseExecution => "pairwise_execution",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MergeLoweringInput {
    intent: MergeWorkflowIntent,
    target_branch: BranchId,
    source_branch: BranchId,
}

impl MergeLoweringInput {
    pub fn reconcile_into_target(target_branch: BranchId, source_branch: BranchId) -> Self {
        Self {
            intent: MergeWorkflowIntent::ReconcileIntoTarget,
            target_branch,
            source_branch,
        }
    }

    pub fn intent(&self) -> &MergeWorkflowIntent {
        &self.intent
    }

    pub fn target_branch(&self) -> &BranchId {
        &self.target_branch
    }

    pub fn source_branch(&self) -> &BranchId {
        &self.source_branch
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WritebackDeclarationFamily {
    ProjectedStateDiff,
}

impl WritebackDeclarationFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProjectedStateDiff => "projected_state_diff",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WritebackLoweringInput {
    family: WritebackDeclarationFamily,
}

impl WritebackLoweringInput {
    pub fn projected_state_diff() -> Self {
        Self {
            family: WritebackDeclarationFamily::ProjectedStateDiff,
        }
    }

    pub fn family(&self) -> &WritebackDeclarationFamily {
        &self.family
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowFreshnessBinding {
    RuntimeBasisExact,
    PreviewSessionBound,
    PreviewPromotionBound,
    BridgeAuthorityRebindRequired,
}

impl WorkflowFreshnessBinding {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimeBasisExact => "runtime_basis_exact",
            Self::PreviewSessionBound => "preview_session_bound",
            Self::PreviewPromotionBound => "preview_promotion_bound",
            Self::BridgeAuthorityRebindRequired => "bridge_authority_rebind_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowStalenessClass {
    ExactBasisPreserved,
    AuthorityValidationRequired,
    StaleDenied,
    ExplicitRebindRequired,
}

impl WorkflowStalenessClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExactBasisPreserved => "exact_basis_preserved",
            Self::AuthorityValidationRequired => "authority_validation_required",
            Self::StaleDenied => "stale_denied",
            Self::ExplicitRebindRequired => "explicit_rebind_required",
        }
    }
}
