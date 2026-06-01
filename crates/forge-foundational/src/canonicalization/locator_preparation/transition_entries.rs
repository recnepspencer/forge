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
        FoundationalTransitionLocator::MergeScope(locator) => vec![
            transition_locator_text_entry("transition.merge_scope.kind", "merge-scope"),
            transition_locator_text_entry(
                "transition.merge_scope.source_branch",
                locator.source_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.merge_scope.target_branch",
                locator.target_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.merge_scope.family",
                merge_scope_family_name(locator.scope_family()),
            ),
        ],
        FoundationalTransitionLocator::SelectedNodeScope(locator) => vec![
            transition_locator_text_entry(
                "transition.selected_node_scope.kind",
                "selected-node-scope",
            ),
            transition_locator_text_entry(
                "transition.selected_node_scope.source_branch",
                locator.source_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.selected_node_scope.target_branch",
                locator.target_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.selected_node_scope.node",
                locator.selected_node().as_str(),
            ),
        ],
        FoundationalTransitionLocator::SelectedAspectScope(locator) => vec![
            transition_locator_text_entry(
                "transition.selected_aspect_scope.kind",
                "selected-aspect-scope",
            ),
            transition_locator_text_entry(
                "transition.selected_aspect_scope.source_branch",
                locator.source_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.selected_aspect_scope.target_branch",
                locator.target_branch().as_str(),
            ),
            transition_locator_text_entry(
                "transition.selected_aspect_scope.node",
                locator.selected_aspect().node().as_str(),
            ),
            transition_locator_text_entry(
                "transition.selected_aspect_scope.aspect",
                locator.selected_aspect().aspect().as_str(),
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

fn merge_scope_family_name(
    family: crate::transitions::FoundationalMergeScopeFamily,
) -> &'static str {
    match family {
        crate::transitions::FoundationalMergeScopeFamily::FullBranch => "full-branch",
        crate::transitions::FoundationalMergeScopeFamily::SelectedNodes => "selected-nodes",
        crate::transitions::FoundationalMergeScopeFamily::SelectedAspects => "selected-aspects",
    }
}
