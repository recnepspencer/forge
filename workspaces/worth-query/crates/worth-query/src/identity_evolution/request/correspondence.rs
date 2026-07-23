#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityEvolutionComparisonBasisFamily {
    InstalledOperation,
    BranchToBranch,
    CurrentToHistorical,
    HistoricalToHistorical,
    PreviewToAuthoritative,
}

impl IdentityEvolutionComparisonBasisFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InstalledOperation => "installed_operation",
            Self::BranchToBranch => "branch_to_branch",
            Self::CurrentToHistorical => "current_to_historical",
            Self::HistoricalToHistorical => "historical_to_historical",
            Self::PreviewToAuthoritative => "preview_to_authoritative",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityComparisonIntent {
    AdvisoryCandidateSet,
    AmbiguousCandidateSet,
    ExplicitContinuityBreak,
    AuthoritativeContinuityRequired,
}

impl IdentityComparisonIntent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AdvisoryCandidateSet => "advisory_candidate_set",
            Self::AmbiguousCandidateSet => "ambiguous_candidate_set",
            Self::ExplicitContinuityBreak => "explicit_continuity_break",
            Self::AuthoritativeContinuityRequired => "authoritative_continuity_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceIdentityComparison {
    left_identity: String,
    right_identity: String,
    intent: IdentityComparisonIntent,
}

impl CorrespondenceIdentityComparison {
    pub fn advisory_between(
        left_identity: impl Into<String>,
        right_identity: impl Into<String>,
    ) -> Self {
        Self::new(
            left_identity,
            right_identity,
            IdentityComparisonIntent::AdvisoryCandidateSet,
        )
    }

    pub fn authoritative_between(
        left_identity: impl Into<String>,
        right_identity: impl Into<String>,
    ) -> Self {
        Self::new(
            left_identity,
            right_identity,
            IdentityComparisonIntent::AuthoritativeContinuityRequired,
        )
    }

    pub(crate) fn ambiguous_between(
        left_identity: impl Into<String>,
        right_identity: impl Into<String>,
    ) -> Self {
        Self::new(
            left_identity,
            right_identity,
            IdentityComparisonIntent::AmbiguousCandidateSet,
        )
    }

    pub(crate) fn explicit_break(
        left_identity: impl Into<String>,
        right_identity: impl Into<String>,
    ) -> Self {
        Self::new(
            left_identity,
            right_identity,
            IdentityComparisonIntent::ExplicitContinuityBreak,
        )
    }

    pub fn left_identity(&self) -> &str {
        &self.left_identity
    }

    pub fn right_identity(&self) -> &str {
        &self.right_identity
    }

    pub fn intent(&self) -> IdentityComparisonIntent {
        self.intent
    }

    fn new(
        left_identity: impl Into<String>,
        right_identity: impl Into<String>,
        intent: IdentityComparisonIntent,
    ) -> Self {
        Self {
            left_identity: left_identity.into(),
            right_identity: right_identity.into(),
            intent,
        }
    }
}
