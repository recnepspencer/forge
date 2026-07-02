use crate::{
    UiEvidenceBudget, UiInspectionObligationEvidenceQuery, UiInspectionRelevance,
    UiInspectionScope, UiInspectionTarget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionQuery {
    target: UiInspectionTarget,
    scope: UiInspectionScope,
    budget: UiEvidenceBudget,
    relevance: UiInspectionRelevance,
    obligation_evidence: Option<UiInspectionObligationEvidenceQuery>,
}

impl UiInspectionQuery {
    pub fn new(target: UiInspectionTarget, scope: UiInspectionScope) -> Self {
        Self {
            target,
            scope,
            budget: UiEvidenceBudget::default(),
            relevance: UiInspectionRelevance::default(),
            obligation_evidence: None,
        }
    }

    pub fn with_budget(mut self, budget: UiEvidenceBudget) -> Self {
        self.budget = budget;
        self
    }

    pub fn with_relevance(mut self, relevance: UiInspectionRelevance) -> Self {
        self.relevance = relevance;
        self
    }

    pub fn with_obligation_evidence(
        mut self,
        obligation_evidence: UiInspectionObligationEvidenceQuery,
    ) -> Self {
        self.obligation_evidence = Some(obligation_evidence);
        self
    }

    pub fn target(&self) -> &UiInspectionTarget {
        &self.target
    }

    pub fn scope(&self) -> UiInspectionScope {
        self.scope
    }

    pub fn budget(&self) -> UiEvidenceBudget {
        self.budget
    }

    pub fn relevance(&self) -> UiInspectionRelevance {
        self.relevance
    }

    pub fn obligation_evidence(&self) -> Option<UiInspectionObligationEvidenceQuery> {
        self.obligation_evidence
    }
}
