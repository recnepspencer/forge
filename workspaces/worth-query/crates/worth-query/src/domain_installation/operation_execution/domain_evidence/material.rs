use super::{
    WorthQueryCandidateSearchSummary, WorthQueryDecisionSummary, WorthQueryDomainEvidenceSidecar,
    WorthQueryStructuralCounterObservation, WorthQueryTransformationSummary,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryDomainEvidenceMaterial {
    counters: Vec<WorthQueryStructuralCounterObservation>,
    decisions: Vec<WorthQueryDecisionSummary>,
    candidate_search: Option<WorthQueryCandidateSearchSummary>,
    transformation: Option<WorthQueryTransformationSummary>,
    sidecar: Option<WorthQueryDomainEvidenceSidecar>,
}

impl WorthQueryDomainEvidenceMaterial {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn counter(mut self, observation: WorthQueryStructuralCounterObservation) -> Self {
        self.counters.push(observation);
        self
    }

    pub fn decision(mut self, summary: WorthQueryDecisionSummary) -> Self {
        self.decisions.push(summary);
        self
    }

    pub fn candidate_search(mut self, summary: WorthQueryCandidateSearchSummary) -> Self {
        self.candidate_search = Some(summary);
        self
    }

    pub fn transformation(mut self, summary: WorthQueryTransformationSummary) -> Self {
        self.transformation = Some(summary);
        self
    }

    pub fn with_sidecar(mut self, sidecar: WorthQueryDomainEvidenceSidecar) -> Self {
        self.sidecar = Some(sidecar);
        self
    }

    pub(crate) fn into_parts(self) -> WorthQueryDomainEvidenceMaterialParts {
        WorthQueryDomainEvidenceMaterialParts {
            counters: self.counters,
            decisions: self.decisions,
            candidate_search: self.candidate_search,
            transformation: self.transformation,
            sidecar: self.sidecar.unwrap_or_default(),
        }
    }
}

pub(crate) struct WorthQueryDomainEvidenceMaterialParts {
    pub(crate) counters: Vec<WorthQueryStructuralCounterObservation>,
    pub(crate) decisions: Vec<WorthQueryDecisionSummary>,
    pub(crate) candidate_search: Option<WorthQueryCandidateSearchSummary>,
    pub(crate) transformation: Option<WorthQueryTransformationSummary>,
    pub(crate) sidecar: WorthQueryDomainEvidenceSidecar,
}
