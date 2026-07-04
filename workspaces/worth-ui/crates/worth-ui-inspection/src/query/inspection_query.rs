use crate::{
    UiEvidenceBudget, UiEvidenceRichness, UiInspectionRelevance, UiInspectionRelevanceAdmission,
    UiInspectionScope, UiInspectionTarget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionQuery {
    target: UiInspectionTarget,
    scope: UiInspectionScope,
    richness: UiEvidenceRichness,
    budget: UiEvidenceBudget,
    relevance: UiInspectionRelevance,
}

impl UiInspectionQuery {
    pub fn new(target: UiInspectionTarget, scope: UiInspectionScope) -> Self {
        Self {
            target,
            scope,
            richness: UiEvidenceRichness::summary(),
            budget: UiEvidenceBudget::default(),
            relevance: UiInspectionRelevance::default(),
        }
    }

    pub fn with_richness(mut self, richness: UiEvidenceRichness) -> Self {
        self.richness = richness;
        self
    }

    pub fn with_budget(mut self, budget: UiEvidenceBudget) -> Self {
        self.budget = budget;
        self
    }

    pub fn with_relevance(mut self, relevance: UiInspectionRelevance) -> Self {
        self.relevance = relevance;
        self
    }

    pub fn target(&self) -> &UiInspectionTarget {
        &self.target
    }

    pub fn scope(&self) -> UiInspectionScope {
        self.scope
    }

    pub fn richness(&self) -> UiEvidenceRichness {
        self.richness
    }

    pub fn budget(&self) -> UiEvidenceBudget {
        self.budget
    }

    pub fn relevance(&self) -> UiInspectionRelevance {
        self.relevance
    }

    pub fn admit_relevance(&self) -> UiInspectionRelevanceAdmission {
        UiInspectionRelevanceAdmission::from_query(self)
    }
}
