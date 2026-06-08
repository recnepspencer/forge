use forge_foundational::facade::{
    prepare_merge_scope_for_canonical_basis, prepare_scoped_merge_denial_for_canonical_basis,
    prepare_scoped_merge_unavailable_for_canonical_basis, CanonicalBasisConstructionDenial,
    CanonicalBasisReadyArtifact, CanonicalizationRuleVersion, FoundationalDeniedScopeLocus,
    FoundationalMergeScope, FoundationalScopedMergeDenialEvidence,
    FoundationalScopedMergeDenialKind, FoundationalScopedMergeUnavailablePosture,
    FoundationalScopedMergeUnavailableReason,
};
use forge_proof::TransitionOutcome;

use crate::data::handle::NodeId;
use crate::logic::transaction::runtime::{
    BranchMergeExecutionSummary, BranchMergeRequestScopeFamily, BranchMergeResult,
    BranchMergeScopedDenialFailureEvidence, BranchMergeScopedDenialKind,
    BranchMergeScopedDeniedLocus, BranchMergeScopedUnavailableFailureEvidence,
    BranchMergeScopedUnavailableReason, LoweredMergePlan, ScopedMergeProofPacket,
    SignalSelectedAspectRequestEntry,
};
use crate::state::SignalBranchId;

use super::foundational_scope::{foundational_denied_aspect_locus, foundational_denied_node_locus};
use super::locator::foundational_branch_id_from_runtime_id;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalScopedMergeCanonicalBasisBundle {
    declaration: CanonicalBasisReadyArtifact,
    admitted: CanonicalBasisReadyArtifact,
    skipped: Option<CanonicalBasisReadyArtifact>,
    no_op: Option<CanonicalBasisReadyArtifact>,
}

impl SignalScopedMergeCanonicalBasisBundle {
    pub fn declaration(&self) -> &CanonicalBasisReadyArtifact {
        &self.declaration
    }

    pub fn admitted(&self) -> &CanonicalBasisReadyArtifact {
        &self.admitted
    }

    pub fn skipped(&self) -> Option<&CanonicalBasisReadyArtifact> {
        self.skipped.as_ref()
    }

    pub fn no_op(&self) -> Option<&CanonicalBasisReadyArtifact> {
        self.no_op.as_ref()
    }
}

impl LoweredMergePlan {
    pub fn prepare_scoped_merge_canonical_basis_bundle(
        &self,
        version: CanonicalizationRuleVersion,
    ) -> TransitionOutcome<SignalScopedMergeCanonicalBasisBundle, CanonicalBasisConstructionDenial>
    {
        prepare_scoped_merge_canonical_basis_bundle(
            version,
            self.source_branch_id(),
            self.target_branch_id(),
            self.scoped_merge_proof(),
        )
    }
}

impl BranchMergeExecutionSummary {
    pub fn prepare_scoped_merge_canonical_basis_bundle(
        &self,
        version: CanonicalizationRuleVersion,
    ) -> TransitionOutcome<SignalScopedMergeCanonicalBasisBundle, CanonicalBasisConstructionDenial>
    {
        prepare_scoped_merge_canonical_basis_bundle(
            version,
            self.source_branch_id,
            self.target_branch_id,
            &self.scoped_merge_proof,
        )
    }
}

impl BranchMergeResult {
    pub fn prepare_scoped_merge_canonical_basis_bundle(
        &self,
        version: CanonicalizationRuleVersion,
    ) -> TransitionOutcome<SignalScopedMergeCanonicalBasisBundle, CanonicalBasisConstructionDenial>
    {
        prepare_scoped_merge_canonical_basis_bundle(
            version,
            self.source_branch,
            self.target_branch,
            &self.scoped_merge_proof,
        )
    }
}

impl BranchMergeScopedDenialFailureEvidence {
    pub fn prepare_canonical_basis(
        &self,
        version: CanonicalizationRuleVersion,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
        let evidence = match foundational_denial_evidence(self, source_branch_id, target_branch_id)
        {
            Ok(evidence) => evidence,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        prepare_scoped_merge_denial_for_canonical_basis(version, &evidence)
    }
}

impl BranchMergeScopedUnavailableFailureEvidence {
    pub fn prepare_canonical_basis(
        &self,
        version: CanonicalizationRuleVersion,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
        let posture =
            match foundational_unavailable_posture(self, source_branch_id, target_branch_id) {
                Ok(posture) => posture,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        prepare_scoped_merge_unavailable_for_canonical_basis(version, &posture)
    }
}

fn prepare_scoped_merge_canonical_basis_bundle(
    version: CanonicalizationRuleVersion,
    source_branch_id: SignalBranchId,
    target_branch_id: SignalBranchId,
    proof: &ScopedMergeProofPacket,
) -> TransitionOutcome<SignalScopedMergeCanonicalBasisBundle, CanonicalBasisConstructionDenial> {
    let requested_scope = match foundational_scope_from_signal(
        proof.scope_family(),
        proof.requested_nodes(),
        proof.requested_aspects(),
    ) {
        Ok(scope) => scope,
        Err(denial) => return TransitionOutcome::Denied(denial),
    };
    let admitted_scope = match foundational_scope_from_signal(
        proof.scope_family(),
        proof.admitted_nodes(),
        proof.admitted_aspects(),
    ) {
        Ok(scope) => scope,
        Err(denial) => return TransitionOutcome::Denied(denial),
    };
    let skipped_scope = match optional_scope(
        proof.scope_family(),
        proof.skipped_nodes(),
        proof.skipped_aspects(),
    ) {
        Ok(scope) => scope,
        Err(denial) => return TransitionOutcome::Denied(denial),
    };
    let no_op_scope = match optional_scope(
        proof.scope_family(),
        proof.no_op_nodes(),
        proof.no_op_aspects(),
    ) {
        Ok(scope) => scope,
        Err(denial) => return TransitionOutcome::Denied(denial),
    };

    let declaration =
        match prepare_merge_scope_for_canonical_basis(version.clone(), &requested_scope) {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(denial) => return TransitionOutcome::Denied(denial),
            other => unreachable!("canonical basis construction should not produce {other:?}"),
        };
    let admitted = match prepare_merge_scope_for_canonical_basis(version.clone(), &admitted_scope) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => return TransitionOutcome::Denied(denial),
        other => unreachable!("canonical basis construction should not produce {other:?}"),
    };
    let skipped = match optional_basis(version.clone(), skipped_scope.as_ref()) {
        Ok(skipped) => skipped,
        Err(denial) => return TransitionOutcome::Denied(denial),
    };
    let no_op = match optional_basis(version, no_op_scope.as_ref()) {
        Ok(no_op) => no_op,
        Err(denial) => return TransitionOutcome::Denied(denial),
    };
    let _ = (source_branch_id, target_branch_id);
    TransitionOutcome::success(SignalScopedMergeCanonicalBasisBundle {
        declaration,
        admitted,
        skipped,
        no_op,
    })
}

fn optional_basis(
    version: CanonicalizationRuleVersion,
    scope: Option<&FoundationalMergeScope>,
) -> Result<Option<CanonicalBasisReadyArtifact>, CanonicalBasisConstructionDenial> {
    let Some(scope) = scope else {
        return Ok(None);
    };
    match prepare_merge_scope_for_canonical_basis(version, scope) {
        TransitionOutcome::Success(ready) => Ok(Some(ready)),
        TransitionOutcome::Denied(denial) => Err(denial),
        other => unreachable!("canonical basis construction should not produce {other:?}"),
    }
}

fn optional_scope(
    scope_family: BranchMergeRequestScopeFamily,
    nodes: &[NodeId],
    aspects: &[SignalSelectedAspectRequestEntry],
) -> Result<Option<FoundationalMergeScope>, CanonicalBasisConstructionDenial> {
    if nodes.is_empty() && aspects.is_empty() {
        Ok(None)
    } else {
        foundational_scope_from_signal(scope_family, nodes, aspects).map(Some)
    }
}

fn foundational_scope_from_signal(
    scope_family: BranchMergeRequestScopeFamily,
    nodes: &[NodeId],
    aspects: &[SignalSelectedAspectRequestEntry],
) -> Result<FoundationalMergeScope, CanonicalBasisConstructionDenial> {
    match scope_family {
        BranchMergeRequestScopeFamily::FullBranch => Ok(FoundationalMergeScope::full_branch()),
        BranchMergeRequestScopeFamily::SelectedNodes => Ok(FoundationalMergeScope::selected_nodes(
            nodes
                .iter()
                .copied()
                .map(foundational_denied_node_locus)
                .collect::<Vec<_>>(),
        )
        .expect("retained selected-node scope should already be canonicalizable")),
        BranchMergeRequestScopeFamily::SelectedAspects => {
            Ok(FoundationalMergeScope::selected_aspects(
                aspects
                    .iter()
                    .map(foundational_denied_aspect_locus)
                    .collect::<Vec<_>>(),
            )
            .expect("retained selected-aspect scope should already be canonicalizable"))
        }
    }
}

pub(crate) fn foundational_denial_evidence(
    evidence: &BranchMergeScopedDenialFailureEvidence,
    source_branch_id: SignalBranchId,
    target_branch_id: SignalBranchId,
) -> Result<FoundationalScopedMergeDenialEvidence, CanonicalBasisConstructionDenial> {
    let requested_scope = foundational_scope_from_signal(
        evidence.scope_family,
        &evidence.requested_nodes,
        &evidence.requested_aspects,
    )?;
    FoundationalScopedMergeDenialEvidence::new(
        foundational_branch_id_from_runtime_id(source_branch_id),
        foundational_branch_id_from_runtime_id(target_branch_id),
        requested_scope,
        foundational_denial_kind(evidence.denial_kind),
        match &evidence.denied_locus {
            BranchMergeScopedDeniedLocus::Node(node) => {
                FoundationalDeniedScopeLocus::Node(foundational_denied_node_locus(*node))
            }
            BranchMergeScopedDeniedLocus::Aspect(aspect) => {
                FoundationalDeniedScopeLocus::Aspect(foundational_denied_aspect_locus(aspect))
            }
            BranchMergeScopedDeniedLocus::ScopeFamily(family) => {
                FoundationalDeniedScopeLocus::ScopeFamily(match family {
                    BranchMergeRequestScopeFamily::FullBranch => {
                        forge_foundational::facade::FoundationalMergeScopeFamily::FullBranch
                    }
                    BranchMergeRequestScopeFamily::SelectedNodes => {
                        forge_foundational::facade::FoundationalMergeScopeFamily::SelectedNodes
                    }
                    BranchMergeRequestScopeFamily::SelectedAspects => {
                        forge_foundational::facade::FoundationalMergeScopeFamily::SelectedAspects
                    }
                })
            }
        },
    )
    .map_err(|_| {
        panic!("retained scoped denial evidence should already lower into foundational denial evidence")
    })
}

pub(crate) fn foundational_unavailable_posture(
    evidence: &BranchMergeScopedUnavailableFailureEvidence,
    source_branch_id: SignalBranchId,
    target_branch_id: SignalBranchId,
) -> Result<FoundationalScopedMergeUnavailablePosture, CanonicalBasisConstructionDenial> {
    FoundationalScopedMergeUnavailablePosture::new(
        foundational_branch_id_from_runtime_id(source_branch_id),
        foundational_branch_id_from_runtime_id(target_branch_id),
        foundational_scope_from_signal(
            evidence.scope_family,
            &evidence.requested_nodes,
            &evidence.requested_aspects,
        )?,
        match evidence.reason {
            BranchMergeScopedUnavailableReason::RuntimeDoesNotSupportSelectedNodes => {
                FoundationalScopedMergeUnavailableReason::RuntimeDoesNotSupportSelectedNodes
            }
            BranchMergeScopedUnavailableReason::RuntimeDoesNotSupportSelectedAspects => {
                FoundationalScopedMergeUnavailableReason::RuntimeDoesNotSupportSelectedAspects
            }
            BranchMergeScopedUnavailableReason::MaterializerUnavailable => {
                FoundationalScopedMergeUnavailableReason::MaterializerUnavailable
            }
            BranchMergeScopedUnavailableReason::IdentityCorrespondenceUnavailable => {
                FoundationalScopedMergeUnavailableReason::IdentityCorrespondenceUnavailable
            }
            BranchMergeScopedUnavailableReason::RetainedProofUnavailable => {
                FoundationalScopedMergeUnavailableReason::RetainedProofUnavailable
            }
        },
    )
    .map_err(|_| {
        panic!(
            "retained scoped unavailable evidence should already lower into foundational unavailable posture"
        )
    })
}

fn foundational_denial_kind(
    kind: BranchMergeScopedDenialKind,
) -> FoundationalScopedMergeDenialKind {
    match kind {
        BranchMergeScopedDenialKind::UnknownSelectedNode => {
            FoundationalScopedMergeDenialKind::UnknownSelectedNode
        }
        BranchMergeScopedDenialKind::UnknownSelectedAspect => {
            FoundationalScopedMergeDenialKind::UnknownSelectedAspect
        }
        BranchMergeScopedDenialKind::SelectedNodeMissingFromSourceScope => {
            FoundationalScopedMergeDenialKind::SelectedNodeMissingFromSourceScope
        }
        BranchMergeScopedDenialKind::SelectedNodeDeletedBeforeAdmission => {
            FoundationalScopedMergeDenialKind::SelectedNodeDeletedBeforeAdmission
        }
        BranchMergeScopedDenialKind::SelectedTargetCorrespondenceAmbiguous => {
            FoundationalScopedMergeDenialKind::SelectedTargetCorrespondenceAmbiguous
        }
        BranchMergeScopedDenialKind::SelectedTargetCorrespondenceRejectedByDeclaration => {
            FoundationalScopedMergeDenialKind::SelectedTargetCorrespondenceRejectedByDeclaration
        }
        BranchMergeScopedDenialKind::SelectedNodeNonAdoptable => {
            FoundationalScopedMergeDenialKind::SelectedNodeNonAdoptable
        }
        BranchMergeScopedDenialKind::SelectedAspectUnsupportedByNodeOrStrategy => {
            FoundationalScopedMergeDenialKind::SelectedAspectUnsupportedByNodeOrStrategy
        }
        BranchMergeScopedDenialKind::ScopeFamilyRejectedByDeclaration => {
            FoundationalScopedMergeDenialKind::ScopeFamilyRejectedByDeclaration
        }
    }
}
