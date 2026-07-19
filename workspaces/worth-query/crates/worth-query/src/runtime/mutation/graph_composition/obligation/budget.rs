use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGraphObligationExecutionCostClass {
    SelectionOnly,
    SparseTopology,
    DenseTopology,
    PolicyBasis,
    ConstructionContext,
}

impl WorthQueryGraphObligationExecutionCostClass {
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
pub enum WorthQueryGraphObligationExecutionScope {
    SelectionOnly,
    TouchedRelationKind,
    TouchedCollection,
    TouchedAspect,
    CandidateTopologyComponent,
    ConstructionFamily,
    PolicyScope,
}

impl WorthQueryGraphObligationExecutionScope {
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
pub enum WorthQueryGraphObligationBudgetExceededPolicy {
    FailClosed,
    Advisory,
    DiagnosticOnly,
    DeferredToBackstop,
}

impl WorthQueryGraphObligationBudgetExceededPolicy {
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
pub struct WorthQueryGraphObligationExecutionBudget {
    cost_class: WorthQueryGraphObligationExecutionCostClass,
    execution_scope: WorthQueryGraphObligationExecutionScope,
    max_state_scope: Option<usize>,
    budget_exceeded_policy: WorthQueryGraphObligationBudgetExceededPolicy,
    budget_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationExecutionBudget {
    pub fn selection_only_deferred_execution() -> Self {
        Self::new(
            WorthQueryGraphObligationExecutionCostClass::SelectionOnly,
            WorthQueryGraphObligationExecutionScope::SelectionOnly,
            None,
            WorthQueryGraphObligationBudgetExceededPolicy::DeferredToBackstop,
        )
    }

    pub fn declared(
        cost_class: WorthQueryGraphObligationExecutionCostClass,
        execution_scope: WorthQueryGraphObligationExecutionScope,
        budget_exceeded_policy: WorthQueryGraphObligationBudgetExceededPolicy,
    ) -> Self {
        Self::new(cost_class, execution_scope, None, budget_exceeded_policy)
    }

    pub fn bounded_sparse(
        execution_scope: WorthQueryGraphObligationExecutionScope,
        budget_exceeded_policy: WorthQueryGraphObligationBudgetExceededPolicy,
    ) -> Self {
        Self::new(
            WorthQueryGraphObligationExecutionCostClass::SparseTopology,
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

    pub fn cost_class(&self) -> WorthQueryGraphObligationExecutionCostClass {
        self.cost_class
    }

    pub fn execution_scope(&self) -> WorthQueryGraphObligationExecutionScope {
        self.execution_scope
    }

    pub fn max_state_scope(&self) -> Option<usize> {
        self.max_state_scope
    }

    pub fn budget_exceeded_policy(&self) -> WorthQueryGraphObligationBudgetExceededPolicy {
        self.budget_exceeded_policy
    }

    pub fn budget_digest(&self) -> &str {
        self.budget_digest.as_str()
    }

    pub(crate) fn budget_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.budget_digest
    }

    fn new(
        cost_class: WorthQueryGraphObligationExecutionCostClass,
        execution_scope: WorthQueryGraphObligationExecutionScope,
        max_state_scope: Option<usize>,
        budget_exceeded_policy: WorthQueryGraphObligationBudgetExceededPolicy,
    ) -> Self {
        let mut budget = Self {
            cost_class,
            execution_scope,
            max_state_scope,
            budget_exceeded_policy,
            budget_digest: worth_query_evidence_identity(
                WorthQueryEvidenceScope::GraphObligationExecutionBudget,
            )
            .seal(),
        };
        budget.budget_digest = budget.build_digest();
        budget
    }

    fn build_digest(&self) -> WorthQueryEvidenceIdentity {
        let mut builder =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationExecutionBudget)
                .field_shape(
                    WorthQueryEvidenceTag::new("cost_class"),
                    self.cost_class.as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("execution_scope"),
                    self.execution_scope.as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("budget_exceeded_policy"),
                    self.budget_exceeded_policy.as_str(),
                );
        if let Some(max_state_scope) = self.max_state_scope {
            builder = builder.field_usize(
                WorthQueryEvidenceTag::new("max_state_scope"),
                max_state_scope,
            );
        }
        builder.seal()
    }
}
