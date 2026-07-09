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
    IdentityBreak,
    Denied,
}

impl IdentityEvolutionOutcomeFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SingularIdentityContinuity => "singular_identity_continuity",
            Self::PluralIdentitySuccessorSet => "plural_identity_successor_set",
            Self::AdvisoryIdentityCandidateSet => "advisory_identity_candidate_set",
            Self::Ambiguity => "ambiguity",
            Self::IdentityBreak => "identity_break",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityEvolutionAmbiguityReason {
    MultipleAuthoritativeContinuities,
    AmbiguousCorrespondenceCandidates,
}

impl IdentityEvolutionAmbiguityReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MultipleAuthoritativeContinuities => "multiple_authoritative_continuities",
            Self::AmbiguousCorrespondenceCandidates => "ambiguous_correspondence_candidates",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityEvolutionIdentityBreakReason {
    ExplicitIdentityBreak,
}

impl IdentityEvolutionIdentityBreakReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExplicitIdentityBreak => "explicit_identity_break",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityEvolutionDenialReason {
    RecursiveTraversalDeferred,
    BroadLineageScanRequired,
    ComplexityContractViolationDenied,
    LineageToCorrespondenceFallbackForbidden,
    BranchCrossingLineageWithoutAdmittedBasisPairing,
    AuthoritativeContinuityRequiresAuthorityEvidence,
}

impl IdentityEvolutionDenialReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RecursiveTraversalDeferred => "recursive_traversal_deferred",
            Self::BroadLineageScanRequired => "broad_lineage_scan_required",
            Self::ComplexityContractViolationDenied => "complexity_contract_violation_denied",
            Self::LineageToCorrespondenceFallbackForbidden => {
                "lineage_to_correspondence_fallback_forbidden"
            }
            Self::BranchCrossingLineageWithoutAdmittedBasisPairing => {
                "branch_crossing_lineage_without_admitted_basis_pairing"
            }
            Self::AuthoritativeContinuityRequiresAuthorityEvidence => {
                "authoritative_continuity_requires_authority_evidence"
            }
        }
    }
}
