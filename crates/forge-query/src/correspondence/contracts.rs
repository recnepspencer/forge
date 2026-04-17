#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CorrespondencePerformanceStatusMarker {
    Verified,
    Debt,
}

impl CorrespondencePerformanceStatusMarker {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Debt => "debt",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuralCandidateBudget {
    max_candidates: usize,
}

impl StructuralCandidateBudget {
    pub fn bounded(max_candidates: usize) -> Self {
        Self {
            max_candidates: max_candidates.max(1),
        }
    }

    pub fn max_candidates(&self) -> usize {
        self.max_candidates
    }

    #[cfg(test)]
    pub(crate) fn new(max_candidates: usize) -> Self {
        Self::bounded(max_candidates)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceComplexityContract {
    contract_name: &'static str,
    status_marker: CorrespondencePerformanceStatusMarker,
}

impl CorrespondenceComplexityContract {
    pub fn contract_name(&self) -> &'static str {
        self.contract_name
    }

    pub fn status_marker(&self) -> &CorrespondencePerformanceStatusMarker {
        &self.status_marker
    }

    pub(crate) fn lineage_direct() -> Self {
        Self {
            contract_name: "correspondence_lineage_direct",
            status_marker: CorrespondencePerformanceStatusMarker::Verified,
        }
    }

    pub(crate) fn structural_candidate_bounded() -> Self {
        Self {
            contract_name: "correspondence_structural_candidate_bounded",
            status_marker: CorrespondencePerformanceStatusMarker::Debt,
        }
    }

    pub(crate) fn structural_ambiguity_bounded() -> Self {
        Self {
            contract_name: "correspondence_structural_ambiguity_bounded",
            status_marker: CorrespondencePerformanceStatusMarker::Debt,
        }
    }

    pub(crate) fn lineage_structural_disagreement() -> Self {
        Self {
            contract_name: "correspondence_lineage_structural_disagreement",
            status_marker: CorrespondencePerformanceStatusMarker::Verified,
        }
    }

    pub(crate) fn denied() -> Self {
        Self {
            contract_name: "correspondence_denied",
            status_marker: CorrespondencePerformanceStatusMarker::Verified,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UniqueStructuralCorrespondenceWitness {
    _private: (),
}

impl UniqueStructuralCorrespondenceWitness {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}
