use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationExecutionCostClass {
    SelectionOnly,
    SparseTopology,
    DenseTopology,
    PolicyBasis,
    ConstructionContext,
}

impl ForgeQueryGraphObligationExecutionCostClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SelectionOnly => "selection-only",
            Self::SparseTopology => "sparse-topology",
            Self::DenseTopology => "dense-topology",
            Self::PolicyBasis => "policy-basis",
            Self::ConstructionContext => "construction-context",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationExecutionScope {
    SelectionOnly,
    TouchedRelationKind,
    TouchedCollection,
    TouchedAspect,
    CandidateTopologyComponent,
    ConstructionFamily,
    PolicyScope,
}

impl ForgeQueryGraphObligationExecutionScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SelectionOnly => "selection-only",
            Self::TouchedRelationKind => "touched-relation-kind",
            Self::TouchedCollection => "touched-collection",
            Self::TouchedAspect => "touched-aspect",
            Self::CandidateTopologyComponent => "candidate-topology-component",
            Self::ConstructionFamily => "construction-family",
            Self::PolicyScope => "policy-scope",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationBudgetExceededPolicy {
    FailClosed,
    Advisory,
    DiagnosticOnly,
    DeferredToBackstop,
}

impl ForgeQueryGraphObligationBudgetExceededPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail-closed",
            Self::Advisory => "advisory",
            Self::DiagnosticOnly => "diagnostic-only",
            Self::DeferredToBackstop => "deferred-to-backstop",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationExecutionBudget {
    cost_class: ForgeQueryGraphObligationExecutionCostClass,
    execution_scope: ForgeQueryGraphObligationExecutionScope,
    max_state_scope: Option<usize>,
    budget_exceeded_policy: ForgeQueryGraphObligationBudgetExceededPolicy,
    budget_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationExecutionBudget {
    pub fn selection_only_deferred_execution() -> Self {
        Self::new(
            ForgeQueryGraphObligationExecutionCostClass::SelectionOnly,
            ForgeQueryGraphObligationExecutionScope::SelectionOnly,
            None,
            ForgeQueryGraphObligationBudgetExceededPolicy::DeferredToBackstop,
        )
    }

    pub fn declared(
        cost_class: ForgeQueryGraphObligationExecutionCostClass,
        execution_scope: ForgeQueryGraphObligationExecutionScope,
        budget_exceeded_policy: ForgeQueryGraphObligationBudgetExceededPolicy,
    ) -> Self {
        Self::new(cost_class, execution_scope, None, budget_exceeded_policy)
    }

    pub fn bounded_sparse(
        execution_scope: ForgeQueryGraphObligationExecutionScope,
        budget_exceeded_policy: ForgeQueryGraphObligationBudgetExceededPolicy,
    ) -> Self {
        Self::new(
            ForgeQueryGraphObligationExecutionCostClass::SparseTopology,
            execution_scope,
            None,
            budget_exceeded_policy,
        )
    }

    pub fn with_max_state_scope(mut self, max_state_scope: usize) -> Self {
        self.max_state_scope = Some(max_state_scope);
        self.budget_digest = self.build_digest();
        self
    }

    pub fn cost_class(&self) -> ForgeQueryGraphObligationExecutionCostClass {
        self.cost_class
    }

    pub fn execution_scope(&self) -> ForgeQueryGraphObligationExecutionScope {
        self.execution_scope
    }

    pub fn max_state_scope(&self) -> Option<usize> {
        self.max_state_scope
    }

    pub fn budget_exceeded_policy(&self) -> ForgeQueryGraphObligationBudgetExceededPolicy {
        self.budget_exceeded_policy
    }

    pub fn budget_digest(&self) -> &str {
        self.budget_digest.as_str()
    }

    pub(crate) fn budget_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.budget_digest
    }

    fn new(
        cost_class: ForgeQueryGraphObligationExecutionCostClass,
        execution_scope: ForgeQueryGraphObligationExecutionScope,
        max_state_scope: Option<usize>,
        budget_exceeded_policy: ForgeQueryGraphObligationBudgetExceededPolicy,
    ) -> Self {
        let mut budget = Self {
            cost_class,
            execution_scope,
            max_state_scope,
            budget_exceeded_policy,
            budget_digest: forge_query_evidence_identity(
                ForgeQueryEvidenceScope::GraphObligationExecutionBudget,
            )
            .seal(),
        };
        budget.budget_digest = budget.build_digest();
        budget
    }

    fn build_digest(&self) -> ForgeQueryEvidenceIdentity {
        let mut builder =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationExecutionBudget)
                .field_shape(
                    ForgeQueryEvidenceTag::new("cost_class"),
                    self.cost_class.as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("execution_scope"),
                    self.execution_scope.as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("budget_exceeded_policy"),
                    self.budget_exceeded_policy.as_str(),
                );
        if let Some(max_state_scope) = self.max_state_scope {
            builder = builder.field_usize(
                ForgeQueryEvidenceTag::new("max_state_scope"),
                max_state_scope,
            );
        }
        builder.seal()
    }
}
