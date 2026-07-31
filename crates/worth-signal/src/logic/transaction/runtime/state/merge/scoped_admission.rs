use worth_foundational::facade::{
    FoundationalBranchBasisDrift, FoundationalDeniedScopeLocus, FoundationalMergeAdmissionDeferred,
    FoundationalMergeAdmissionDenial, FoundationalMergeAdmissionFailure,
    FoundationalMergeAdmissionOutcome, FoundationalMergeAdmissionRebindRequired,
    FoundationalScopedMergeDenialEvidence, FoundationalScopedMergeDenialKind,
    FoundationalScopedMergeUnavailableOutcomeCategory, FoundationalScopedMergeUnavailablePosture,
    FoundationalScopedMergeUnavailableReason,
};
use worth_proof::TransitionOutcome;

use super::{
    foundational_branch_id, foundational_denied_aspect_locus, foundational_denied_node_locus,
};
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::transaction::runtime::state::LoweredScopedMergeCandidateSet;
use crate::logic::transaction::runtime::BranchMergeStrategy;
use crate::logic::transaction::{
    BranchMergeRequestScopeFamily, LoweredFoundationalMergeRequest,
    SignalSelectedAspectRequestEntry,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopedMergeAdmissionReady {
    request: LoweredFoundationalMergeRequest,
    scoped_candidates: LoweredScopedMergeCandidateSet,
}

impl ScopedMergeAdmissionReady {
    pub fn scoped_candidates(&self) -> &LoweredScopedMergeCandidateSet {
        &self.scoped_candidates
    }
}

pub type ScopedMergeAdmissionOutcome = FoundationalMergeAdmissionOutcome<ScopedMergeAdmissionReady>;

pub(crate) fn classify_initial_scoped_merge_admission(
    request: &LoweredFoundationalMergeRequest,
    scoped_candidates: &LoweredScopedMergeCandidateSet,
    source_graph: &SignalGraph,
    strategy_hint: Option<BranchMergeStrategy>,
) -> ScopedMergeAdmissionOutcome {
    if let Some(outcome) = classify_scoped_strategy_unavailable(request, strategy_hint) {
        return outcome;
    }
    if let Some(outcome) = classify_skipped_scope(request, scoped_candidates, source_graph) {
        return outcome;
    }
    TransitionOutcome::success(ScopedMergeAdmissionReady {
        request: request.clone(),
        scoped_candidates: scoped_candidates.clone(),
    })
}

pub(crate) fn deny_selected_target_correspondence_ambiguous(
    request: &LoweredFoundationalMergeRequest,
    source_node: NodeId,
) -> ScopedMergeAdmissionOutcome {
    deny_node(
        request,
        source_node,
        FoundationalScopedMergeDenialKind::SelectedTargetCorrespondenceAmbiguous,
    )
}

pub(crate) fn deny_selected_target_rejected_by_declaration(
    request: &LoweredFoundationalMergeRequest,
    source_node: NodeId,
) -> ScopedMergeAdmissionOutcome {
    deny_node(
        request,
        source_node,
        FoundationalScopedMergeDenialKind::SelectedTargetCorrespondenceRejectedByDeclaration,
    )
}

pub(crate) fn deny_selected_node_non_adoptable(
    request: &LoweredFoundationalMergeRequest,
    source_node: NodeId,
) -> ScopedMergeAdmissionOutcome {
    deny_node(
        request,
        source_node,
        FoundationalScopedMergeDenialKind::SelectedNodeNonAdoptable,
    )
}

fn classify_scoped_strategy_unavailable(
    request: &LoweredFoundationalMergeRequest,
    strategy_hint: Option<BranchMergeStrategy>,
) -> Option<ScopedMergeAdmissionOutcome> {
    let strategy_hint = strategy_hint?;
    if matches!(
        request.normalized_request().normalized_scope().family(),
        BranchMergeRequestScopeFamily::FullBranch
    ) || matches!(strategy_hint, BranchMergeStrategy::AdoptSourceSubset)
    {
        return None;
    }
    let reason = match request.normalized_request().normalized_scope().family() {
        BranchMergeRequestScopeFamily::SelectedNodes => {
            FoundationalScopedMergeUnavailableReason::RuntimeDoesNotSupportSelectedNodes
        }
        BranchMergeRequestScopeFamily::SelectedAspects => {
            FoundationalScopedMergeUnavailableReason::RuntimeDoesNotSupportSelectedAspects
        }
        BranchMergeRequestScopeFamily::FullBranch => return None,
    };
    Some(scope_unavailable(request, reason))
}

fn classify_skipped_scope(
    request: &LoweredFoundationalMergeRequest,
    scoped_candidates: &LoweredScopedMergeCandidateSet,
    source_graph: &SignalGraph,
) -> Option<ScopedMergeAdmissionOutcome> {
    if let Some(node) = scoped_candidates.skipped_nodes().first().copied() {
        let kind = if source_graph.is_alive(node) {
            FoundationalScopedMergeDenialKind::SelectedNodeMissingFromSourceScope
        } else {
            FoundationalScopedMergeDenialKind::UnknownSelectedNode
        };
        return Some(deny_node(request, node, kind));
    }
    scoped_candidates
        .skipped_aspects()
        .first()
        .cloned()
        .map(|entry| {
            deny_aspect(
                request,
                entry,
                FoundationalScopedMergeDenialKind::UnknownSelectedAspect,
            )
        })
}

fn deny_node(
    request: &LoweredFoundationalMergeRequest,
    source_node: NodeId,
    denial_kind: FoundationalScopedMergeDenialKind,
) -> ScopedMergeAdmissionOutcome {
    let evidence = FoundationalScopedMergeDenialEvidence::new(
        foundational_branch_id(&request.normalized_request().request().source_branch),
        foundational_branch_id(&request.normalized_request().request().target_branch),
        request.foundational_scope().clone(),
        denial_kind,
        FoundationalDeniedScopeLocus::Node(foundational_denied_node_locus(source_node)),
    )
    .expect("phase-6 scoped denial node evidence must stay inside the requested scope");
    TransitionOutcome::denied(FoundationalMergeAdmissionDenial::ScopedSelectionDenied(
        evidence,
    ))
}

fn deny_aspect(
    request: &LoweredFoundationalMergeRequest,
    entry: SignalSelectedAspectRequestEntry,
    denial_kind: FoundationalScopedMergeDenialKind,
) -> ScopedMergeAdmissionOutcome {
    let evidence = FoundationalScopedMergeDenialEvidence::new(
        foundational_branch_id(&request.normalized_request().request().source_branch),
        foundational_branch_id(&request.normalized_request().request().target_branch),
        request.foundational_scope().clone(),
        denial_kind,
        FoundationalDeniedScopeLocus::Aspect(foundational_denied_aspect_locus(&entry)),
    )
    .expect("phase-6 scoped denial aspect evidence must stay inside the requested scope");
    TransitionOutcome::denied(FoundationalMergeAdmissionDenial::ScopedSelectionDenied(
        evidence,
    ))
}

fn scope_unavailable(
    request: &LoweredFoundationalMergeRequest,
    reason: FoundationalScopedMergeUnavailableReason,
) -> ScopedMergeAdmissionOutcome {
    let posture = FoundationalScopedMergeUnavailablePosture::new(
        foundational_branch_id(&request.normalized_request().request().source_branch),
        foundational_branch_id(&request.normalized_request().request().target_branch),
        request.foundational_scope().clone(),
        reason,
    )
    .expect("phase-6 scoped unavailable posture must match the requested scope family");
    match posture.outcome_category() {
        FoundationalScopedMergeUnavailableOutcomeCategory::Deferred => TransitionOutcome::deferred(
            FoundationalMergeAdmissionDeferred::scope_unavailable(posture),
        ),
        FoundationalScopedMergeUnavailableOutcomeCategory::Stale => {
            TransitionOutcome::stale(FoundationalBranchBasisDrift::scope_unavailable(posture))
        }
        FoundationalScopedMergeUnavailableOutcomeCategory::RebindRequired => {
            TransitionOutcome::rebind_required(
                FoundationalMergeAdmissionRebindRequired::scope_unavailable(posture),
            )
        }
        FoundationalScopedMergeUnavailableOutcomeCategory::Failed => TransitionOutcome::failed(
            FoundationalMergeAdmissionFailure::scope_unavailable(posture),
        ),
    }
}
