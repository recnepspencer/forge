use crate::identities::EquivalenceBasisId;
use crate::transitions::FoundationalMergeVerdictKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalAuthorityTransitionOutcomeKind {
    NoOp,
    Committed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalAuthorityTransitionClass {
    NoOp,
    Commit,
    MetadataOnlyCommit,
    PromotionCommit,
    ReplayRevalidatedCommit,
}

impl FoundationalAuthorityTransitionClass {
    pub const fn outcome_kind(&self) -> FoundationalAuthorityTransitionOutcomeKind {
        match self {
            Self::NoOp => FoundationalAuthorityTransitionOutcomeKind::NoOp,
            Self::Commit
            | Self::MetadataOnlyCommit
            | Self::PromotionCommit
            | Self::ReplayRevalidatedCommit => {
                FoundationalAuthorityTransitionOutcomeKind::Committed
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalNoOpCause {
    AlreadyConverged,
    BasisEquivalent,
    StrategySuppressed,
    ChangeDenied,
    ReplayEquivalent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalCommitParentBasis(EquivalenceBasisId);

impl FoundationalCommitParentBasis {
    pub const fn new(basis_id: EquivalenceBasisId) -> Self {
        Self(basis_id)
    }

    pub const fn basis_id(&self) -> EquivalenceBasisId {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalCommitParentage {
    ordered_parents: Vec<FoundationalCommitParentBasis>,
}

impl FoundationalCommitParentage {
    pub fn new(
        parents: impl IntoIterator<Item = FoundationalCommitParentBasis>,
    ) -> Result<Self, FoundationalCommittedAuthorityConstructionDenial> {
        let mut ordered_parents: Vec<_> = parents.into_iter().collect();
        if ordered_parents.is_empty() {
            return Err(FoundationalCommittedAuthorityConstructionDenial::EmptyParentage);
        }

        ordered_parents.sort_unstable();

        if ordered_parents.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(FoundationalCommittedAuthorityConstructionDenial::DuplicateParentBasis);
        }

        Ok(Self { ordered_parents })
    }

    pub fn parents(&self) -> &[FoundationalCommitParentBasis] {
        &self.ordered_parents
    }

    pub fn contains(&self, basis: FoundationalCommitParentBasis) -> bool {
        self.ordered_parents.binary_search(&basis).is_ok()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalMergeAncestryBasis(EquivalenceBasisId);

impl FoundationalMergeAncestryBasis {
    pub const fn new(basis_id: EquivalenceBasisId) -> Self {
        Self(basis_id)
    }

    pub const fn basis_id(&self) -> EquivalenceBasisId {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalCommittedDeltaLocus {
    category: String,
    detail: String,
}

impl FoundationalCommittedDeltaLocus {
    pub fn new(category: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            detail: detail.into(),
        }
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalCommitDeltaSummary {
    loci: Vec<FoundationalCommittedDeltaLocus>,
}

impl FoundationalCommitDeltaSummary {
    pub fn new(loci: Vec<FoundationalCommittedDeltaLocus>) -> Self {
        Self { loci }
    }

    pub fn loci(&self) -> &[FoundationalCommittedDeltaLocus] {
        &self.loci
    }

    pub fn delta_count(&self) -> u64 {
        self.loci.len() as u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalAuthorityTransitionDenial {
    BranchCandidateNotAdmitted,
    MergeVerdictNotCommitEligible {
        verdict_kind: FoundationalMergeVerdictKind,
    },
    ReceiptRequiresCommittedAuthority,
    ParentBasisMissing,
    CurrentBasisReadmissionRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalCommittedAuthorityConstructionDenial {
    EmptyParentage,
    DuplicateParentBasis,
    PrimaryParentBasisNotInParentage,
    NoOpTransitionRequiresCause,
    CommittedTransitionMustNotCarryNoOpCause,
    NoOpTransitionMustNotCarryCommittedDeltas,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalCommittedAuthorityInput {
    transition_class: FoundationalAuthorityTransitionClass,
    no_op_cause: Option<FoundationalNoOpCause>,
    parent_basis: FoundationalCommitParentBasis,
    parentage: FoundationalCommitParentage,
    merge_ancestry_basis: Option<FoundationalMergeAncestryBasis>,
    committed_delta_summary: FoundationalCommitDeltaSummary,
}

impl FoundationalCommittedAuthorityInput {
    pub fn new(
        transition_class: FoundationalAuthorityTransitionClass,
        no_op_cause: Option<FoundationalNoOpCause>,
        parent_basis: FoundationalCommitParentBasis,
        parentage: FoundationalCommitParentage,
        merge_ancestry_basis: Option<FoundationalMergeAncestryBasis>,
        committed_delta_summary: FoundationalCommitDeltaSummary,
    ) -> Result<Self, FoundationalCommittedAuthorityConstructionDenial> {
        if !parentage.contains(parent_basis) {
            return Err(
                FoundationalCommittedAuthorityConstructionDenial::PrimaryParentBasisNotInParentage,
            );
        }

        match transition_class.outcome_kind() {
            FoundationalAuthorityTransitionOutcomeKind::NoOp => {
                if no_op_cause.is_none() {
                    return Err(
                        FoundationalCommittedAuthorityConstructionDenial::NoOpTransitionRequiresCause,
                    );
                }
                if committed_delta_summary.delta_count() != 0 {
                    return Err(
                        FoundationalCommittedAuthorityConstructionDenial::NoOpTransitionMustNotCarryCommittedDeltas,
                    );
                }
            }
            FoundationalAuthorityTransitionOutcomeKind::Committed => {
                if no_op_cause.is_some() {
                    return Err(
                        FoundationalCommittedAuthorityConstructionDenial::CommittedTransitionMustNotCarryNoOpCause,
                    );
                }
            }
        }

        Ok(Self {
            transition_class,
            no_op_cause,
            parent_basis,
            parentage,
            merge_ancestry_basis,
            committed_delta_summary,
        })
    }

    pub const fn transition_class(&self) -> FoundationalAuthorityTransitionClass {
        self.transition_class
    }

    pub const fn no_op_cause(&self) -> Option<FoundationalNoOpCause> {
        self.no_op_cause
    }

    pub const fn parent_basis(&self) -> FoundationalCommitParentBasis {
        self.parent_basis
    }

    pub fn parentage(&self) -> &FoundationalCommitParentage {
        &self.parentage
    }

    pub const fn merge_ancestry_basis(&self) -> Option<FoundationalMergeAncestryBasis> {
        self.merge_ancestry_basis
    }

    pub fn committed_delta_summary(&self) -> &FoundationalCommitDeltaSummary {
        &self.committed_delta_summary
    }
}
