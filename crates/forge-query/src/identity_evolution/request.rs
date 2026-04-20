use crate::identity::{BasisDigest, CanonicalQueryDigest};

use super::families::{IdentityEvolutionQueryFamily, LineageTraversalFamily};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityEvolutionComparisonBasisFamily {
    BranchToBranch,
    CurrentToHistorical,
    HistoricalToHistorical,
    PreviewToAuthoritative,
}

impl IdentityEvolutionComparisonBasisFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
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
    AuthoritativeContinuityRequired,
}

impl IdentityComparisonIntent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AdvisoryCandidateSet => "advisory_candidate_set",
            Self::AuthoritativeContinuityRequired => "authoritative_continuity_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineageTraversalDescriptor {
    family: LineageTraversalFamily,
    anchor_identity: String,
}

impl LineageTraversalDescriptor {
    pub fn direct_predecessor(anchor_identity: impl Into<String>) -> Self {
        Self::from_family(LineageTraversalFamily::DirectPredecessor, anchor_identity)
    }

    pub fn direct_successor(anchor_identity: impl Into<String>) -> Self {
        Self::from_family(LineageTraversalFamily::DirectSuccessor, anchor_identity)
    }

    pub fn direct_replacement(anchor_identity: impl Into<String>) -> Self {
        Self::from_family(LineageTraversalFamily::DirectReplacement, anchor_identity)
    }

    pub fn direct_split_successors(anchor_identity: impl Into<String>) -> Self {
        Self::from_family(LineageTraversalFamily::DirectSplitSuccessors, anchor_identity)
    }

    pub fn direct_merge_successor(anchor_identity: impl Into<String>) -> Self {
        Self::from_family(LineageTraversalFamily::DirectMergeSuccessor, anchor_identity)
    }

    pub fn branch_local_direct_evolution(anchor_identity: impl Into<String>) -> Self {
        Self::from_family(LineageTraversalFamily::BranchLocalDirectEvolution, anchor_identity)
    }

    pub fn family(&self) -> LineageTraversalFamily {
        self.family
    }

    pub fn anchor_identity(&self) -> &str {
        &self.anchor_identity
    }

    fn from_family(family: LineageTraversalFamily, anchor_identity: impl Into<String>) -> Self {
        Self {
            family,
            anchor_identity: anchor_identity.into(),
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
        Self {
            left_identity: left_identity.into(),
            right_identity: right_identity.into(),
            intent: IdentityComparisonIntent::AdvisoryCandidateSet,
        }
    }

    pub fn authoritative_between(
        left_identity: impl Into<String>,
        right_identity: impl Into<String>,
    ) -> Self {
        Self {
            left_identity: left_identity.into(),
            right_identity: right_identity.into(),
            intent: IdentityComparisonIntent::AuthoritativeContinuityRequired,
        }
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum IdentityEvolutionQuerySubject {
    LineageTraversal(LineageTraversalDescriptor),
    CorrespondenceIdentityComparison {
        comparison_basis_family: IdentityEvolutionComparisonBasisFamily,
        left_basis_digest: BasisDigest,
        right_basis_digest: BasisDigest,
        comparison: CorrespondenceIdentityComparison,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionQueryContext {
    query_digest: CanonicalQueryDigest,
    basis_digest: BasisDigest,
    family: IdentityEvolutionQueryFamily,
    subject: IdentityEvolutionQuerySubject,
}

impl IdentityEvolutionQueryContext {
    pub fn lineage_traversal(
        query_digest: CanonicalQueryDigest,
        basis_digest: BasisDigest,
        descriptor: LineageTraversalDescriptor,
    ) -> Self {
        Self {
            query_digest,
            basis_digest,
            family: IdentityEvolutionQueryFamily::LineageTraversal,
            subject: IdentityEvolutionQuerySubject::LineageTraversal(descriptor),
        }
    }

    pub fn correspondence_identity_comparison(
        query_digest: CanonicalQueryDigest,
        comparison_basis_family: IdentityEvolutionComparisonBasisFamily,
        left_basis_digest: BasisDigest,
        right_basis_digest: BasisDigest,
        comparison: CorrespondenceIdentityComparison,
    ) -> Self {
        Self {
            query_digest,
            basis_digest: left_basis_digest.clone(),
            family: IdentityEvolutionQueryFamily::CorrespondenceIdentityComparison,
            subject: IdentityEvolutionQuerySubject::CorrespondenceIdentityComparison {
                comparison_basis_family,
                left_basis_digest,
                right_basis_digest,
                comparison,
            },
        }
    }

    pub fn query_digest(&self) -> &CanonicalQueryDigest {
        &self.query_digest
    }

    pub fn basis_digest(&self) -> &BasisDigest {
        &self.basis_digest
    }

    pub fn family(&self) -> IdentityEvolutionQueryFamily {
        self.family
    }

    pub fn lineage_traversal_descriptor(&self) -> Option<&LineageTraversalDescriptor> {
        match &self.subject {
            IdentityEvolutionQuerySubject::LineageTraversal(descriptor) => Some(descriptor),
            IdentityEvolutionQuerySubject::CorrespondenceIdentityComparison { .. } => None,
        }
    }

    pub fn correspondence_identity_comparison_descriptor(
        &self,
    ) -> Option<(
        IdentityEvolutionComparisonBasisFamily,
        &BasisDigest,
        &BasisDigest,
        &CorrespondenceIdentityComparison,
    )> {
        match &self.subject {
            IdentityEvolutionQuerySubject::LineageTraversal(_) => None,
            IdentityEvolutionQuerySubject::CorrespondenceIdentityComparison {
                comparison_basis_family,
                left_basis_digest,
                right_basis_digest,
                comparison,
            } => Some((
                *comparison_basis_family,
                left_basis_digest,
                right_basis_digest,
                comparison,
            )),
        }
    }
}
