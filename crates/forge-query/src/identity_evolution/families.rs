use crate::identity::LineageDigest;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityEvolutionQueryFamily {
    LineageTraversal,
    CorrespondenceIdentityComparison,
}

impl IdentityEvolutionQueryFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LineageTraversal => "lineage_traversal",
            Self::CorrespondenceIdentityComparison => "correspondence_identity_comparison",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum LineageTraversalFamily {
    DirectPredecessor,
    DirectSuccessor,
    DirectReplacement,
    DirectSplitSuccessors,
    DirectMergeSuccessor,
    BranchLocalDirectEvolution,
}

impl LineageTraversalFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectPredecessor => "direct_predecessor",
            Self::DirectSuccessor => "direct_successor",
            Self::DirectReplacement => "direct_replacement",
            Self::DirectSplitSuccessors => "direct_split_successors",
            Self::DirectMergeSuccessor => "direct_merge_successor",
            Self::BranchLocalDirectEvolution => "branch_local_direct_evolution",
        }
    }

    #[allow(dead_code)]
    pub(crate) fn digest(&self) -> LineageDigest {
        LineageDigest::from_parts(&[format!("identity-evolution-traversal:{}", self.as_str())])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityEvolutionOutcomeFamily {
    SingularIdentityContinuity,
    PluralIdentitySuccessorSet,
    AdvisoryIdentityCandidateSet,
    Ambiguity,
    Denied,
}

impl IdentityEvolutionOutcomeFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SingularIdentityContinuity => "singular_identity_continuity",
            Self::PluralIdentitySuccessorSet => "plural_identity_successor_set",
            Self::AdvisoryIdentityCandidateSet => "advisory_identity_candidate_set",
            Self::Ambiguity => "ambiguity",
            Self::Denied => "denied",
        }
    }
}
