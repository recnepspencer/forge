#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructuralFingerprintFamily {
    TopologyFingerprint,
    FacetShapeFingerprint,
    BranchComparisonFingerprint,
    RestoreCandidateFingerprint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructuralFingerprintNormalizationRule {
    SchemaDeclaredCanonicalForm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructuralFingerprintOrderingRule {
    SchemaDeclaredCanonicalOrder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructuralFingerprintOmissionPolicy {
    SchemaDeclaredOmissionPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructuralComparisonMode {
    AdvisoryRemap,
    BranchComparison,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructuralTruthViewBasisKind {
    ExplicitSnapshot,
    ExplicitHistoricalVersion,
    ExplicitBranchHead,
    ExplicitBranchPairComparison,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructuralCandidateSearchScope {
    DeclaredStructuralIndexCohort,
    LineageNeighborhoodCohort,
    BranchLocalCohort,
    ExplicitWidenedDebtScan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructuralMismatchClass {
    NoStructuralMatch,
    AmbiguousStructuralMatch,
    IdentityAuthorityConflict,
    LineageStructuralDivergence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructuralMatchOutcomeClass {
    ExactAdvisoryMatch,
    AdvisoryReuseCandidate,
    BranchComparisonArtifact,
    RejectedNoStructuralMatch,
    RejectedAmbiguousStructuralMatch,
    RejectedIdentityAuthorityConflict,
    RejectedLineageStructuralDivergence,
}

impl StructuralMatchOutcomeClass {
    pub fn mismatch_class(self) -> Option<StructuralMismatchClass> {
        match self {
            Self::ExactAdvisoryMatch
            | Self::AdvisoryReuseCandidate
            | Self::BranchComparisonArtifact => None,
            Self::RejectedNoStructuralMatch => Some(StructuralMismatchClass::NoStructuralMatch),
            Self::RejectedAmbiguousStructuralMatch => {
                Some(StructuralMismatchClass::AmbiguousStructuralMatch)
            }
            Self::RejectedIdentityAuthorityConflict => {
                Some(StructuralMismatchClass::IdentityAuthorityConflict)
            }
            Self::RejectedLineageStructuralDivergence => {
                Some(StructuralMismatchClass::LineageStructuralDivergence)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{StructuralMatchOutcomeClass, StructuralMismatchClass};

    #[test]
    fn structural_match_outcomes_remain_closed_world() {
        assert_eq!(
            StructuralMatchOutcomeClass::ExactAdvisoryMatch.mismatch_class(),
            None
        );
        assert_eq!(
            StructuralMatchOutcomeClass::RejectedAmbiguousStructuralMatch.mismatch_class(),
            Some(StructuralMismatchClass::AmbiguousStructuralMatch)
        );
        assert_eq!(
            StructuralMatchOutcomeClass::RejectedLineageStructuralDivergence.mismatch_class(),
            Some(StructuralMismatchClass::LineageStructuralDivergence)
        );
    }
}
