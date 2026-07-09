use worth_foundational::facade::{
    FoundationalDeniedScopeLocus, FoundationalMergeAdmissionDenial,
    FoundationalMergeAdmissionOutcome, FoundationalMergeScope, FoundationalMergeScopeFamily,
    FoundationalScopedMergeDenialEvidence, FoundationalScopedMergeDenialKind,
    FoundationalScopedMergeUnavailablePosture, FoundationalScopedMergeUnavailableReason,
};
use worth_proof::TransitionOutcome;
use serde::{Deserialize, Serialize};

use crate::data::aspect::Aspect;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::logic::transaction::canonical_digest;
use crate::logic::transaction::runtime::IdentityCorrespondenceStatus;
use crate::logic::transaction::{
    BranchMergeFailureEvidence, BranchMergeFailureKind, BranchMergeRequestScopeFamily,
    LoweredFoundationalMergeRequest, SignalSelectedAspectRequestEntry,
};

use super::scoped_admission::{ScopedMergeAdmissionOutcome, ScopedMergeAdmissionReady};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchMergeScopedDenialKind {
    UnknownSelectedNode,
    UnknownSelectedAspect,
    SelectedNodeMissingFromSourceScope,
    SelectedNodeDeletedBeforeAdmission,
    SelectedTargetCorrespondenceAmbiguous,
    SelectedTargetCorrespondenceRejectedByDeclaration,
    SelectedNodeNonAdoptable,
    SelectedAspectUnsupportedByNodeOrStrategy,
    ScopeFamilyRejectedByDeclaration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchMergeScopedDeniedLocus {
    Node(NodeId),
    Aspect(SignalSelectedAspectRequestEntry),
    ScopeFamily(BranchMergeRequestScopeFamily),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchMergeScopedUnavailableReason {
    RuntimeDoesNotSupportSelectedNodes,
    RuntimeDoesNotSupportSelectedAspects,
    MaterializerUnavailable,
    IdentityCorrespondenceUnavailable,
    RetainedProofUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchMergeScopedUnavailableOutcomeKind {
    Deferred,
    Stale,
    RebindRequired,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeScopedDenialFailureEvidence {
    pub scope_family: BranchMergeRequestScopeFamily,
    pub scope_digest: String,
    pub requested_nodes: Vec<NodeId>,
    pub requested_aspects: Vec<SignalSelectedAspectRequestEntry>,
    pub denial_kind: BranchMergeScopedDenialKind,
    pub denied_locus: BranchMergeScopedDeniedLocus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeScopedUnavailableFailureEvidence {
    pub scope_family: BranchMergeRequestScopeFamily,
    pub scope_digest: String,
    pub requested_nodes: Vec<NodeId>,
    pub requested_aspects: Vec<SignalSelectedAspectRequestEntry>,
    pub reason: BranchMergeScopedUnavailableReason,
    pub outcome_kind: BranchMergeScopedUnavailableOutcomeKind,
}

pub(crate) fn rewrite_identity_scoped_admission_error(
    request: &LoweredFoundationalMergeRequest,
    error: SignalError,
) -> SignalError {
    if matches!(
        request.normalized_request().normalized_scope().family(),
        BranchMergeRequestScopeFamily::FullBranch
    ) {
        return error;
    }
    let scoped = match &error {
        SignalError::BranchMergeFailed {
            evidence: Some(BranchMergeFailureEvidence::Identity(identity)),
            ..
        } if identity.correspondence.records.iter().any(|record| {
            matches!(
                record.status,
                IdentityCorrespondenceStatus::AmbiguousCandidates
            )
        }) =>
        {
            Some(deny_selected_target_correspondence_ambiguous(
                request,
                identity.source_node,
            ))
        }
        _ => None,
    };
    match scoped {
        Some(outcome) => scoped_admission_outcome_to_signal_error(outcome),
        None => error,
    }
}

pub(crate) fn scoped_admission_outcome_to_signal_error(
    outcome: ScopedMergeAdmissionOutcome,
) -> SignalError {
    match outcome {
        TransitionOutcome::Success(ScopedMergeAdmissionReady { .. }) => SignalError::internal(
            "scoped merge admission success cannot be converted into a merge failure error",
        ),
        TransitionOutcome::Denied(FoundationalMergeAdmissionDenial::ScopedSelectionDenied(
            evidence,
        )) => {
            let projected = project_denial_evidence(&evidence);
            SignalError::branch_merge_failed_with_evidence(
                BranchMergeFailureKind::ScopedMergeDenied,
                scoped_denial_message(&projected),
                BranchMergeFailureEvidence::ScopedDenial(projected),
            )
        }
        TransitionOutcome::Denied(denial) => SignalError::branch_merge_failed(
            BranchMergeFailureKind::ScopedMergeDenied,
            format!("scoped merge admission denied: {denial:?}"),
        ),
        TransitionOutcome::Deferred(posture) => unavailable_to_signal_error(
            posture.scope_unavailable_posture(),
            posture.reason(),
            BranchMergeScopedUnavailableOutcomeKind::Deferred,
        ),
        TransitionOutcome::Stale(drift) => {
            if let Some(posture) = drift.scope_unavailable_posture() {
                unavailable_to_signal_error(
                    Some(posture),
                    drift.reason(),
                    BranchMergeScopedUnavailableOutcomeKind::Stale,
                )
            } else {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::ScopedMergeUnavailable,
                    "scoped merge admission became stale before planning could continue",
                )
            }
        }
        TransitionOutcome::RebindRequired(posture) => unavailable_to_signal_error(
            posture.scope_unavailable_posture(),
            posture.reason(),
            BranchMergeScopedUnavailableOutcomeKind::RebindRequired,
        ),
        TransitionOutcome::Failed(posture) => unavailable_to_signal_error(
            posture.scope_unavailable_posture(),
            posture.reason(),
            BranchMergeScopedUnavailableOutcomeKind::Failed,
        ),
    }
}

fn deny_selected_target_correspondence_ambiguous(
    request: &LoweredFoundationalMergeRequest,
    source_node: NodeId,
) -> FoundationalMergeAdmissionOutcome<ScopedMergeAdmissionReady> {
    super::scoped_admission::deny_selected_target_correspondence_ambiguous(request, source_node)
}

fn project_denial_evidence(
    evidence: &FoundationalScopedMergeDenialEvidence,
) -> BranchMergeScopedDenialFailureEvidence {
    BranchMergeScopedDenialFailureEvidence {
        scope_family: requested_scope_family(evidence.requested_scope()),
        scope_digest: foundational_scope_digest(evidence.requested_scope()),
        requested_nodes: evidence
            .requested_scope()
            .selected_nodes_loci()
            .iter()
            .map(|node| parse_signal_node_locus(node.as_str()))
            .collect(),
        requested_aspects: evidence
            .requested_scope()
            .selected_aspect_loci()
            .iter()
            .map(|entry| {
                SignalSelectedAspectRequestEntry::new(
                    parse_signal_node_locus(entry.node().as_str()),
                    parse_signal_aspect_locus(entry.aspect().as_str()),
                )
            })
            .collect(),
        denial_kind: project_denial_kind(evidence.denial_kind()),
        denied_locus: project_denied_locus(evidence.denied_locus()),
    }
}

fn project_denial_kind(kind: FoundationalScopedMergeDenialKind) -> BranchMergeScopedDenialKind {
    match kind {
        FoundationalScopedMergeDenialKind::UnknownSelectedNode => {
            BranchMergeScopedDenialKind::UnknownSelectedNode
        }
        FoundationalScopedMergeDenialKind::UnknownSelectedAspect => {
            BranchMergeScopedDenialKind::UnknownSelectedAspect
        }
        FoundationalScopedMergeDenialKind::SelectedNodeMissingFromSourceScope => {
            BranchMergeScopedDenialKind::SelectedNodeMissingFromSourceScope
        }
        FoundationalScopedMergeDenialKind::SelectedNodeDeletedBeforeAdmission => {
            BranchMergeScopedDenialKind::SelectedNodeDeletedBeforeAdmission
        }
        FoundationalScopedMergeDenialKind::SelectedTargetCorrespondenceAmbiguous => {
            BranchMergeScopedDenialKind::SelectedTargetCorrespondenceAmbiguous
        }
        FoundationalScopedMergeDenialKind::SelectedTargetCorrespondenceRejectedByDeclaration => {
            BranchMergeScopedDenialKind::SelectedTargetCorrespondenceRejectedByDeclaration
        }
        FoundationalScopedMergeDenialKind::SelectedNodeNonAdoptable => {
            BranchMergeScopedDenialKind::SelectedNodeNonAdoptable
        }
        FoundationalScopedMergeDenialKind::SelectedAspectUnsupportedByNodeOrStrategy => {
            BranchMergeScopedDenialKind::SelectedAspectUnsupportedByNodeOrStrategy
        }
        FoundationalScopedMergeDenialKind::ScopeFamilyRejectedByDeclaration => {
            BranchMergeScopedDenialKind::ScopeFamilyRejectedByDeclaration
        }
    }
}

fn project_denied_locus(locus: &FoundationalDeniedScopeLocus) -> BranchMergeScopedDeniedLocus {
    match locus {
        FoundationalDeniedScopeLocus::Node(node) => {
            BranchMergeScopedDeniedLocus::Node(parse_signal_node_locus(node.as_str()))
        }
        FoundationalDeniedScopeLocus::Aspect(entry) => {
            BranchMergeScopedDeniedLocus::Aspect(SignalSelectedAspectRequestEntry::new(
                parse_signal_node_locus(entry.node().as_str()),
                parse_signal_aspect_locus(entry.aspect().as_str()),
            ))
        }
        FoundationalDeniedScopeLocus::ScopeFamily(family) => {
            BranchMergeScopedDeniedLocus::ScopeFamily(requested_scope_family_from_foundational(
                *family,
            ))
        }
    }
}

fn requested_scope_family(scope: &FoundationalMergeScope) -> BranchMergeRequestScopeFamily {
    requested_scope_family_from_foundational(scope.family())
}

fn requested_scope_family_from_foundational(
    family: FoundationalMergeScopeFamily,
) -> BranchMergeRequestScopeFamily {
    match family {
        FoundationalMergeScopeFamily::FullBranch => BranchMergeRequestScopeFamily::FullBranch,
        FoundationalMergeScopeFamily::SelectedNodes => BranchMergeRequestScopeFamily::SelectedNodes,
        FoundationalMergeScopeFamily::SelectedAspects => {
            BranchMergeRequestScopeFamily::SelectedAspects
        }
    }
}

fn foundational_scope_digest(scope: &FoundationalMergeScope) -> String {
    match scope.family() {
        FoundationalMergeScopeFamily::FullBranch => {
            canonical_digest(&BranchMergeRequestScopeFamily::FullBranch)
        }
        FoundationalMergeScopeFamily::SelectedNodes => {
            let selected_nodes = scope
                .selected_nodes_loci()
                .iter()
                .map(|node| parse_signal_node_locus(node.as_str()))
                .collect::<Vec<_>>();
            canonical_digest(&(
                BranchMergeRequestScopeFamily::SelectedNodes,
                &selected_nodes,
            ))
        }
        FoundationalMergeScopeFamily::SelectedAspects => {
            let selected_aspects = scope
                .selected_aspect_loci()
                .iter()
                .map(|entry| {
                    SignalSelectedAspectRequestEntry::new(
                        parse_signal_node_locus(entry.node().as_str()),
                        parse_signal_aspect_locus(entry.aspect().as_str()),
                    )
                })
                .collect::<Vec<_>>();
            canonical_digest(&(
                BranchMergeRequestScopeFamily::SelectedAspects,
                &selected_aspects,
            ))
        }
    }
}

fn scoped_denial_message(evidence: &BranchMergeScopedDenialFailureEvidence) -> String {
    format!(
        "scoped merge denied at {:?} for {:?}",
        evidence.denied_locus, evidence.denial_kind
    )
}

fn unavailable_to_signal_error(
    posture: Option<&FoundationalScopedMergeUnavailablePosture>,
    fallback_reason: &'static str,
    outcome_kind: BranchMergeScopedUnavailableOutcomeKind,
) -> SignalError {
    let Some(posture) = posture else {
        return SignalError::branch_merge_failed(
            BranchMergeFailureKind::ScopedMergeUnavailable,
            fallback_reason,
        );
    };
    SignalError::branch_merge_failed_with_evidence(
        BranchMergeFailureKind::ScopedMergeUnavailable,
        posture.reason().reason(),
            BranchMergeFailureEvidence::ScopedUnavailable(
            BranchMergeScopedUnavailableFailureEvidence {
                scope_family: requested_scope_family(posture.requested_scope()),
                scope_digest: foundational_scope_digest(posture.requested_scope()),
                requested_nodes: posture
                    .requested_scope()
                    .selected_nodes_loci()
                    .iter()
                    .map(|node| parse_signal_node_locus(node.as_str()))
                    .collect(),
                requested_aspects: posture
                    .requested_scope()
                    .selected_aspect_loci()
                    .iter()
                    .map(|entry| {
                        SignalSelectedAspectRequestEntry::new(
                            parse_signal_node_locus(entry.node().as_str()),
                            parse_signal_aspect_locus(entry.aspect().as_str()),
                        )
                    })
                    .collect(),
                reason: match posture.reason() {
                    FoundationalScopedMergeUnavailableReason::RuntimeDoesNotSupportSelectedNodes => {
                        BranchMergeScopedUnavailableReason::RuntimeDoesNotSupportSelectedNodes
                    }
                    FoundationalScopedMergeUnavailableReason::RuntimeDoesNotSupportSelectedAspects => {
                        BranchMergeScopedUnavailableReason::RuntimeDoesNotSupportSelectedAspects
                    }
                    FoundationalScopedMergeUnavailableReason::MaterializerUnavailable => {
                        BranchMergeScopedUnavailableReason::MaterializerUnavailable
                    }
                    FoundationalScopedMergeUnavailableReason::IdentityCorrespondenceUnavailable => {
                        BranchMergeScopedUnavailableReason::IdentityCorrespondenceUnavailable
                    }
                    FoundationalScopedMergeUnavailableReason::RetainedProofUnavailable => {
                        BranchMergeScopedUnavailableReason::RetainedProofUnavailable
                    }
                },
                outcome_kind,
            },
        ),
    )
}

fn parse_signal_node_locus(raw: &str) -> NodeId {
    let mut parts = raw.split(':');
    let _ = parts.next();
    let index = parts
        .next()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or_default();
    let generation = parts
        .next()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or_default();
    NodeId::new(index, generation)
}

fn parse_signal_aspect_locus(raw: &str) -> Aspect {
    let aspect_id = raw
        .split(':')
        .next_back()
        .and_then(|value| value.parse::<u8>().ok())
        .unwrap_or_default();
    Aspect::new(aspect_id)
}
