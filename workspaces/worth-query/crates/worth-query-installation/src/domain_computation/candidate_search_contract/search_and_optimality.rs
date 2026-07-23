#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryCandidateSearchPosture {
    NotApplicable,
    Exhaustive,
    ProvenTopK { count: usize },
    Bounded { bound_identity: String },
    Sampled { sample_identity: String },
    Heuristic,
    Incomplete,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryCandidateOptimalityPosture {
    NotApplicable,
    ProvenOptimal,
    ProvenTopK { count: usize },
    BoundedGap { bound_identity: String },
    BestInDeclaredSample { sample_identity: String },
    ParetoForDeclaredSet { set_identity: String },
    FeasibleOnly,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCandidateSearchEvidenceFamilies {
    universe: String,
    termination: String,
    feasibility: String,
    comparison: String,
}

impl WorthQueryCandidateSearchEvidenceFamilies {
    pub fn new(
        universe: impl Into<String>,
        termination: impl Into<String>,
        feasibility: impl Into<String>,
        comparison: impl Into<String>,
    ) -> Self {
        Self {
            universe: universe.into(),
            termination: termination.into(),
            feasibility: feasibility.into(),
            comparison: comparison.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCandidateSearchContract {
    evidence: Option<WorthQueryCandidateSearchEvidenceFamilies>,
    search: WorthQueryCandidateSearchPosture,
    optimality: WorthQueryCandidateOptimalityPosture,
}

impl WorthQueryCandidateSearchContract {
    pub fn not_applicable() -> Self {
        Self {
            evidence: None,
            search: WorthQueryCandidateSearchPosture::NotApplicable,
            optimality: WorthQueryCandidateOptimalityPosture::NotApplicable,
        }
    }

    pub fn declared(
        evidence: WorthQueryCandidateSearchEvidenceFamilies,
        search: WorthQueryCandidateSearchPosture,
        optimality: WorthQueryCandidateOptimalityPosture,
    ) -> Self {
        Self {
            evidence: Some(evidence),
            search,
            optimality,
        }
    }

    pub fn search_posture(&self) -> &WorthQueryCandidateSearchPosture {
        &self.search
    }

    pub fn optimality_posture(&self) -> &WorthQueryCandidateOptimalityPosture {
        &self.optimality
    }

    pub fn universe_family(&self) -> Option<&str> {
        self.evidence
            .as_ref()
            .map(|evidence| evidence.universe.as_str())
    }

    pub fn termination_family(&self) -> Option<&str> {
        self.evidence
            .as_ref()
            .map(|evidence| evidence.termination.as_str())
    }

    pub fn feasibility_family(&self) -> Option<&str> {
        self.evidence
            .as_ref()
            .map(|evidence| evidence.feasibility.as_str())
    }

    pub fn comparison_family(&self) -> Option<&str> {
        self.evidence
            .as_ref()
            .map(|evidence| evidence.comparison.as_str())
    }

    pub(crate) fn postures_are_coherent(&self) -> bool {
        use WorthQueryCandidateOptimalityPosture as Optimality;
        use WorthQueryCandidateSearchPosture as Search;
        match (&self.search, &self.optimality) {
            (Search::NotApplicable, Optimality::NotApplicable) => true,
            (Search::Exhaustive, Optimality::ProvenOptimal | Optimality::FeasibleOnly) => true,
            (Search::Exhaustive, Optimality::ParetoForDeclaredSet { set_identity }) => {
                portable_identity(set_identity)
            }
            (Search::ProvenTopK { count: searched }, Optimality::ProvenTopK { count: claimed }) => {
                searched > &0 && searched == claimed
            }
            (
                Search::Bounded {
                    bound_identity: searched,
                },
                Optimality::BoundedGap {
                    bound_identity: claimed,
                },
            ) => portable_identity(searched) && searched == claimed,
            (
                Search::Sampled {
                    sample_identity: searched,
                },
                Optimality::BestInDeclaredSample {
                    sample_identity: claimed,
                },
            ) => portable_identity(searched) && searched == claimed,
            (
                Search::Bounded { .. }
                | Search::Sampled { .. }
                | Search::Heuristic
                | Search::Incomplete,
                Optimality::FeasibleOnly | Optimality::Unknown,
            ) => true,
            _ => false,
        }
    }
}

fn portable_identity(value: &str) -> bool {
    !value.trim().is_empty() && value.trim() == value && !value.chars().any(char::is_whitespace)
}
