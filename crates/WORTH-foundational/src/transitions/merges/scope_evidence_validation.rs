use std::collections::BTreeSet;

use super::scope_evidence::{
    FoundationalSelectedScopeLocus, FoundationalSelectedScopeNoOpEvidence,
    FoundationalSkippedOutOfScopeEvidence,
};
use super::scoped::{
    FoundationalMergeScope, FoundationalMergeScopeFamily, FoundationalSelectedAspectRequestEntry,
    FoundationalSelectedNodeLocus,
};
use super::vocabulary::FoundationalMergeConstructionDenial;

pub(super) fn validate_scope_evidence_loci(
    requested_scope: &FoundationalMergeScope,
    admitted_nodes: &[FoundationalSelectedNodeLocus],
    admitted_aspects: &[FoundationalSelectedAspectRequestEntry],
) -> Result<(), FoundationalMergeConstructionDenial> {
    match requested_scope.family() {
        FoundationalMergeScopeFamily::FullBranch => {
            if !admitted_nodes.is_empty() || !admitted_aspects.is_empty() {
                return Err(
                    FoundationalMergeConstructionDenial::ScopedEvidenceOutsideRequestedScope,
                );
            }
        }
        FoundationalMergeScopeFamily::SelectedNodes => {
            if !admitted_aspects.is_empty()
                || admitted_nodes
                    .iter()
                    .any(|node| !requested_scope.selected_nodes_loci().contains(node))
            {
                return Err(
                    FoundationalMergeConstructionDenial::ScopedEvidenceOutsideRequestedScope,
                );
            }
        }
        FoundationalMergeScopeFamily::SelectedAspects => {
            if !admitted_nodes.is_empty()
                || admitted_aspects
                    .iter()
                    .any(|aspect| !requested_scope.selected_aspect_loci().contains(aspect))
            {
                return Err(
                    FoundationalMergeConstructionDenial::ScopedEvidenceOutsideRequestedScope,
                );
            }
        }
    }
    Ok(())
}

pub(super) fn validate_no_op_loci(
    requested_scope: &FoundationalMergeScope,
    selected_no_ops: &[FoundationalSelectedScopeNoOpEvidence],
) -> Result<(), FoundationalMergeConstructionDenial> {
    for selected_no_op in selected_no_ops {
        match (requested_scope.family(), selected_no_op.locus()) {
            (
                FoundationalMergeScopeFamily::SelectedNodes,
                FoundationalSelectedScopeLocus::Node(node),
            ) if requested_scope.selected_nodes_loci().contains(node) => {}
            (
                FoundationalMergeScopeFamily::SelectedAspects,
                FoundationalSelectedScopeLocus::Aspect(aspect),
            ) if requested_scope.selected_aspect_loci().contains(aspect) => {}
            _ => {
                return Err(
                    FoundationalMergeConstructionDenial::ScopedEvidenceOutsideRequestedScope,
                )
            }
        }
    }
    Ok(())
}

pub(super) fn validate_selected_loci_have_one_outcome(
    admitted_nodes: &[FoundationalSelectedNodeLocus],
    admitted_aspects: &[FoundationalSelectedAspectRequestEntry],
    selected_no_ops: &[FoundationalSelectedScopeNoOpEvidence],
) -> Result<(), FoundationalMergeConstructionDenial> {
    let mut admitted_loci = BTreeSet::new();
    admitted_loci.extend(
        admitted_nodes
            .iter()
            .cloned()
            .map(FoundationalSelectedScopeLocus::Node),
    );
    admitted_loci.extend(
        admitted_aspects
            .iter()
            .cloned()
            .map(FoundationalSelectedScopeLocus::Aspect),
    );
    if selected_no_ops
        .iter()
        .any(|no_op| admitted_loci.contains(no_op.locus()))
    {
        return Err(FoundationalMergeConstructionDenial::ScopedEvidenceLocusHasMultipleOutcomes);
    }
    Ok(())
}

pub(super) fn validate_selected_scope_has_complete_outcomes(
    requested_scope: &FoundationalMergeScope,
    admitted_nodes: &[FoundationalSelectedNodeLocus],
    admitted_aspects: &[FoundationalSelectedAspectRequestEntry],
    selected_no_ops: &[FoundationalSelectedScopeNoOpEvidence],
) -> Result<(), FoundationalMergeConstructionDenial> {
    match requested_scope.family() {
        FoundationalMergeScopeFamily::FullBranch => Ok(()),
        FoundationalMergeScopeFamily::SelectedNodes => {
            let outcome_loci = selected_node_outcome_loci(admitted_nodes, selected_no_ops);
            if requested_scope
                .selected_nodes_loci()
                .iter()
                .any(|node| !outcome_loci.contains(node))
            {
                return Err(
                    FoundationalMergeConstructionDenial::ScopedEvidenceMissingSelectedOutcome,
                );
            }
            Ok(())
        }
        FoundationalMergeScopeFamily::SelectedAspects => {
            let outcome_loci = selected_aspect_outcome_loci(admitted_aspects, selected_no_ops);
            if requested_scope
                .selected_aspect_loci()
                .iter()
                .any(|aspect| !outcome_loci.contains(aspect))
            {
                return Err(
                    FoundationalMergeConstructionDenial::ScopedEvidenceMissingSelectedOutcome,
                );
            }
            Ok(())
        }
    }
}

pub(super) fn validate_skipped_evidence(
    requested_scope: &FoundationalMergeScope,
    skipped: FoundationalSkippedOutOfScopeEvidence,
) -> Result<(), FoundationalMergeConstructionDenial> {
    if requested_scope.family() == FoundationalMergeScopeFamily::FullBranch
        && skipped.skipped_candidate_count() > 0
    {
        return Err(FoundationalMergeConstructionDenial::FullBranchScopeCannotSkipOutOfScope);
    }
    Ok(())
}

pub(super) fn sorted_unique_no_ops(
    selected_no_ops: impl IntoIterator<Item = FoundationalSelectedScopeNoOpEvidence>,
) -> Result<Vec<FoundationalSelectedScopeNoOpEvidence>, FoundationalMergeConstructionDenial> {
    let mut seen = BTreeSet::new();
    let mut no_ops = Vec::new();
    for selected_no_op in selected_no_ops {
        if !seen.insert(selected_no_op.locus().clone()) {
            return Err(FoundationalMergeConstructionDenial::DuplicateSelectedNoOpLocus);
        }
        no_ops.push(selected_no_op);
    }
    no_ops.sort();
    Ok(no_ops)
}

fn selected_node_outcome_loci(
    admitted_nodes: &[FoundationalSelectedNodeLocus],
    selected_no_ops: &[FoundationalSelectedScopeNoOpEvidence],
) -> BTreeSet<FoundationalSelectedNodeLocus> {
    let mut outcome_loci: BTreeSet<_> = admitted_nodes.iter().cloned().collect();
    outcome_loci.extend(
        selected_no_ops
            .iter()
            .filter_map(|no_op| match no_op.locus() {
                FoundationalSelectedScopeLocus::Node(node) => Some(node.clone()),
                FoundationalSelectedScopeLocus::Aspect(_) => None,
            }),
    );
    outcome_loci
}

fn selected_aspect_outcome_loci(
    admitted_aspects: &[FoundationalSelectedAspectRequestEntry],
    selected_no_ops: &[FoundationalSelectedScopeNoOpEvidence],
) -> BTreeSet<FoundationalSelectedAspectRequestEntry> {
    let mut outcome_loci: BTreeSet<_> = admitted_aspects.iter().cloned().collect();
    outcome_loci.extend(
        selected_no_ops
            .iter()
            .filter_map(|no_op| match no_op.locus() {
                FoundationalSelectedScopeLocus::Aspect(aspect) => Some(aspect.clone()),
                FoundationalSelectedScopeLocus::Node(_) => None,
            }),
    );
    outcome_loci
}
