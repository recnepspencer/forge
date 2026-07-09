use crate::transitions::FoundationalBranchId;

use super::scoped::{
    FoundationalMergeScope, FoundationalMergeScopeFamily, FoundationalSelectedAspectRequestEntry,
    FoundationalSelectedNodeLocus,
};
use super::vocabulary::FoundationalMergeConstructionDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalScopedMergeDenialKind {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDeniedScopeLocus {
    Node(FoundationalSelectedNodeLocus),
    Aspect(FoundationalSelectedAspectRequestEntry),
    ScopeFamily(FoundationalMergeScopeFamily),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalScopedMergeDenialEvidence {
    source_branch: FoundationalBranchId,
    target_branch: FoundationalBranchId,
    requested_scope: FoundationalMergeScope,
    denial_kind: FoundationalScopedMergeDenialKind,
    denied_locus: FoundationalDeniedScopeLocus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalScopedMergeUnavailableOutcomeCategory {
    Deferred,
    Stale,
    RebindRequired,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalScopedMergeUnavailableReason {
    RuntimeDoesNotSupportSelectedNodes,
    RuntimeDoesNotSupportSelectedAspects,
    MaterializerUnavailable,
    IdentityCorrespondenceUnavailable,
    RetainedProofUnavailable,
}

impl FoundationalScopedMergeUnavailableReason {
    pub const fn outcome_category(self) -> FoundationalScopedMergeUnavailableOutcomeCategory {
        match self {
            Self::RuntimeDoesNotSupportSelectedNodes
            | Self::RuntimeDoesNotSupportSelectedAspects => {
                FoundationalScopedMergeUnavailableOutcomeCategory::Deferred
            }
            Self::MaterializerUnavailable => {
                FoundationalScopedMergeUnavailableOutcomeCategory::Failed
            }
            Self::IdentityCorrespondenceUnavailable => {
                FoundationalScopedMergeUnavailableOutcomeCategory::RebindRequired
            }
            Self::RetainedProofUnavailable => {
                FoundationalScopedMergeUnavailableOutcomeCategory::Stale
            }
        }
    }

    pub const fn reason(self) -> &'static str {
        match self {
            Self::RuntimeDoesNotSupportSelectedNodes => {
                "runtime does not support selected-node merge scope"
            }
            Self::RuntimeDoesNotSupportSelectedAspects => {
                "runtime does not support selected-aspect merge scope"
            }
            Self::MaterializerUnavailable => "selected-scope materializer unavailable",
            Self::IdentityCorrespondenceUnavailable => {
                "selected-scope identity correspondence unavailable"
            }
            Self::RetainedProofUnavailable => "selected-scope retained proof unavailable",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalScopedMergeUnavailablePosture {
    source_branch: FoundationalBranchId,
    target_branch: FoundationalBranchId,
    requested_scope: FoundationalMergeScope,
    reason: FoundationalScopedMergeUnavailableReason,
    outcome_category: FoundationalScopedMergeUnavailableOutcomeCategory,
}

impl FoundationalScopedMergeUnavailablePosture {
    pub fn new(
        source_branch: FoundationalBranchId,
        target_branch: FoundationalBranchId,
        requested_scope: FoundationalMergeScope,
        reason: FoundationalScopedMergeUnavailableReason,
    ) -> Result<Self, FoundationalMergeConstructionDenial> {
        validate_unavailable_reason_scope(reason, &requested_scope)?;
        Ok(Self {
            source_branch,
            target_branch,
            requested_scope,
            reason,
            outcome_category: reason.outcome_category(),
        })
    }

    pub fn source_branch(&self) -> &FoundationalBranchId {
        &self.source_branch
    }

    pub fn target_branch(&self) -> &FoundationalBranchId {
        &self.target_branch
    }

    pub fn requested_scope(&self) -> &FoundationalMergeScope {
        &self.requested_scope
    }

    pub const fn reason(&self) -> FoundationalScopedMergeUnavailableReason {
        self.reason
    }

    pub const fn outcome_category(&self) -> FoundationalScopedMergeUnavailableOutcomeCategory {
        self.outcome_category
    }
}

impl FoundationalScopedMergeDenialEvidence {
    pub fn new(
        source_branch: FoundationalBranchId,
        target_branch: FoundationalBranchId,
        requested_scope: FoundationalMergeScope,
        denial_kind: FoundationalScopedMergeDenialKind,
        denied_locus: FoundationalDeniedScopeLocus,
    ) -> Result<Self, FoundationalMergeConstructionDenial> {
        validate_denial_kind_locus(denial_kind, &denied_locus)?;
        validate_denied_locus_in_requested_scope(&requested_scope, &denied_locus)?;
        Ok(Self {
            source_branch,
            target_branch,
            requested_scope,
            denial_kind,
            denied_locus,
        })
    }

    pub fn source_branch(&self) -> &FoundationalBranchId {
        &self.source_branch
    }

    pub fn target_branch(&self) -> &FoundationalBranchId {
        &self.target_branch
    }

    pub fn requested_scope(&self) -> &FoundationalMergeScope {
        &self.requested_scope
    }

    pub const fn denial_kind(&self) -> FoundationalScopedMergeDenialKind {
        self.denial_kind
    }

    pub fn denied_locus(&self) -> &FoundationalDeniedScopeLocus {
        &self.denied_locus
    }
}

fn validate_denial_kind_locus(
    denial_kind: FoundationalScopedMergeDenialKind,
    denied_locus: &FoundationalDeniedScopeLocus,
) -> Result<(), FoundationalMergeConstructionDenial> {
    let valid = matches!(
        (denial_kind, denied_locus),
        (
            FoundationalScopedMergeDenialKind::UnknownSelectedNode
                | FoundationalScopedMergeDenialKind::SelectedNodeMissingFromSourceScope
                | FoundationalScopedMergeDenialKind::SelectedNodeDeletedBeforeAdmission
                | FoundationalScopedMergeDenialKind::SelectedTargetCorrespondenceAmbiguous
                | FoundationalScopedMergeDenialKind::SelectedTargetCorrespondenceRejectedByDeclaration
                | FoundationalScopedMergeDenialKind::SelectedNodeNonAdoptable,
            FoundationalDeniedScopeLocus::Node(_)
        ) | (
            FoundationalScopedMergeDenialKind::UnknownSelectedAspect
                | FoundationalScopedMergeDenialKind::SelectedAspectUnsupportedByNodeOrStrategy,
            FoundationalDeniedScopeLocus::Aspect(_)
        ) | (
            FoundationalScopedMergeDenialKind::ScopeFamilyRejectedByDeclaration,
            FoundationalDeniedScopeLocus::ScopeFamily(_)
        )
    );
    if valid {
        Ok(())
    } else {
        Err(FoundationalMergeConstructionDenial::ScopedDenialLocusMismatch)
    }
}

fn validate_denied_locus_in_requested_scope(
    requested_scope: &FoundationalMergeScope,
    denied_locus: &FoundationalDeniedScopeLocus,
) -> Result<(), FoundationalMergeConstructionDenial> {
    let in_scope = match denied_locus {
        FoundationalDeniedScopeLocus::Node(node) => {
            requested_scope.selected_nodes_loci().contains(node)
                || requested_scope
                    .selected_aspect_loci()
                    .iter()
                    .any(|entry| entry.node() == node)
        }
        FoundationalDeniedScopeLocus::Aspect(aspect) => {
            requested_scope.selected_aspect_loci().contains(aspect)
        }
        FoundationalDeniedScopeLocus::ScopeFamily(family) => requested_scope.family() == *family,
    };
    if in_scope {
        Ok(())
    } else {
        Err(FoundationalMergeConstructionDenial::ScopedEvidenceOutsideRequestedScope)
    }
}

fn validate_unavailable_reason_scope(
    reason: FoundationalScopedMergeUnavailableReason,
    requested_scope: &FoundationalMergeScope,
) -> Result<(), FoundationalMergeConstructionDenial> {
    let valid = match reason {
        FoundationalScopedMergeUnavailableReason::RuntimeDoesNotSupportSelectedNodes => {
            requested_scope.family() == FoundationalMergeScopeFamily::SelectedNodes
        }
        FoundationalScopedMergeUnavailableReason::RuntimeDoesNotSupportSelectedAspects => {
            requested_scope.family() == FoundationalMergeScopeFamily::SelectedAspects
        }
        FoundationalScopedMergeUnavailableReason::IdentityCorrespondenceUnavailable => {
            requested_scope.family() == FoundationalMergeScopeFamily::SelectedAspects
        }
        FoundationalScopedMergeUnavailableReason::MaterializerUnavailable
        | FoundationalScopedMergeUnavailableReason::RetainedProofUnavailable => true,
    };
    if valid {
        Ok(())
    } else {
        Err(FoundationalMergeConstructionDenial::ScopedUnavailableReasonScopeMismatch)
    }
}
