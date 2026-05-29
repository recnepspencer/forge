use crate::canonicalization::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalIntegerWidth,
};
use crate::locators::FoundationalTransitionLocator;
use crate::transitions::FoundationalCommitParentBasis;

pub(super) fn transition_locator_entries(
    locator: FoundationalTransitionLocator,
) -> Vec<CanonicalBasisEntry> {
    match locator {
        FoundationalTransitionLocator::BranchCandidate(locator) => vec![
            transition_locator_text_entry("transition.branch_candidate.kind", "branch-candidate"),
            transition_locator_text_entry(
                "transition.branch_candidate.branch_id",
                locator.branch_id().as_str(),
            ),
            transition_locator_integer_entry(
                "transition.branch_candidate.candidate_id",
                u128::from(locator.candidate_id().handle().get()),
            ),
        ],
        FoundationalTransitionLocator::MergeConflict(locator) => vec![
            transition_locator_text_entry("transition.merge_conflict.kind", "merge-conflict"),
            transition_locator_text_entry(
                "transition.merge_conflict.source_branch",
                locator.source_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.merge_conflict.target_branch",
                locator.target_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.merge_conflict.category",
                locator.conflict_locus().category(),
            ),
            transition_locator_text_entry(
                "transition.merge_conflict.source_detail",
                locator.conflict_locus().source_detail(),
            ),
            transition_locator_text_entry(
                "transition.merge_conflict.target_detail",
                locator.conflict_locus().target_detail(),
            ),
        ],
        FoundationalTransitionLocator::CommitParentage(locator) => vec![
            transition_locator_text_entry("transition.parentage.kind", "commit-parentage"),
            transition_locator_integer_entry(
                "transition.parentage.commit_id",
                u128::from(locator.commit_id().handle().get()),
            ),
            parent_basis_locator_entry("transition.parentage.parent_basis", locator.parent_basis()),
        ],
        FoundationalTransitionLocator::CommittedDelta(locator) => vec![
            transition_locator_text_entry("transition.delta.kind", "committed-delta"),
            transition_locator_integer_entry(
                "transition.delta.commit_id",
                u128::from(locator.commit_id().handle().get()),
            ),
            transition_locator_text_entry(
                "transition.delta.category",
                locator.delta_locus().category(),
            ),
            transition_locator_text_entry(
                "transition.delta.detail",
                locator.delta_locus().detail(),
            ),
        ],
    }
}

fn transition_locator_text_entry(
    locus: impl Into<String>,
    value: impl Into<String>,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Locator,
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::TransitionLocator,
        CanonicalBasisValue::ExactText(value.into().into()),
    )
}

fn transition_locator_integer_entry(locus: impl Into<String>, value: u128) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Locator,
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::TransitionLocator,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value,
        },
    )
}

fn parent_basis_locator_entry(
    locus: impl Into<String>,
    basis: FoundationalCommitParentBasis,
) -> CanonicalBasisEntry {
    transition_locator_integer_entry(locus, u128::from(basis.basis_id().get()))
}
