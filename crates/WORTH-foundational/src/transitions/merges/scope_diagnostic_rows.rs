use crate::diagnostics::{
    foundational_diagnostic_code, foundational_diagnostic_locator_transition,
    foundational_diagnostic_merge_verdict_subject, foundational_diagnostic_scope,
    FoundationalDiagnosticCounterSnapshot, FoundationalDiagnosticDecisionRow,
    FoundationalDiagnosticDenialClass, FoundationalDiagnosticEvidencePosture,
    FoundationalDiagnosticLocalityClaim, FoundationalDiagnosticOutcomeKind,
    FoundationalDiagnosticProvenanceReadyRow, FoundationalDiagnosticRow,
    FoundationalDiagnosticScopeId, FoundationalDiagnosticSemanticLabelSet,
    FoundationalDiagnosticSeverity, FoundationalDiagnosticSubject,
    FoundationalDiagnosticSupportEvidencePosture, FoundationalDiagnosticSupportRow,
    FoundationalDiagnosticWidenedFalloutPosture,
};
use crate::locators::{
    FoundationalMergeScopeLocator, FoundationalSelectedAspectScopeLocator,
    FoundationalSelectedNodeScopeLocator, FoundationalTransitionLocator,
};
use crate::transitions::{
    FoundationalBranchId, FoundationalMergeScopeFamily, FoundationalScopeAdmissionBasis,
    FoundationalScopedMergeDenialKind, FoundationalScopedMergeUnavailableOutcomeCategory,
    FoundationalScopedMergeUnavailableReason, FoundationalSelectedScopeNoOpCause,
};

use super::{
    FoundationalDeniedScopeLocus, FoundationalScopedMergeDenialEvidence,
    FoundationalSelectedAspectRequestEntry, FoundationalSelectedNodeLocus,
    FoundationalSelectedScopeLocus,
};

pub(crate) fn decision_row(
    code: &'static str,
    outcome_kind: FoundationalDiagnosticOutcomeKind,
    severity: FoundationalDiagnosticSeverity,
    subject: FoundationalDiagnosticSubject,
    locator: crate::diagnostics::FoundationalDiagnosticLocator,
    labels: FoundationalDiagnosticSemanticLabelSet,
    denial_class: Option<FoundationalDiagnosticDenialClass>,
    locality: FoundationalDiagnosticLocalityClaim,
) -> FoundationalDiagnosticRow {
    FoundationalDiagnosticRow::Decision(FoundationalDiagnosticDecisionRow::new(
        diagnostic_code(code),
        scope("transitions.merge-scope"),
        severity,
        subject,
        locator,
        outcome_kind,
        labels,
        denial_class,
        locality,
        FoundationalDiagnosticWidenedFalloutPosture::NotWidened,
    ))
}

pub(crate) fn support_row(
    code: &'static str,
    outcome_kind: FoundationalDiagnosticOutcomeKind,
    severity: FoundationalDiagnosticSeverity,
    subject: FoundationalDiagnosticSubject,
    locator: crate::diagnostics::FoundationalDiagnosticLocator,
    labels: FoundationalDiagnosticSemanticLabelSet,
    evidence_posture: FoundationalDiagnosticSupportEvidencePosture,
    locality: FoundationalDiagnosticLocalityClaim,
) -> FoundationalDiagnosticRow {
    FoundationalDiagnosticRow::Support(FoundationalDiagnosticSupportRow::new(
        diagnostic_code(code),
        scope("transitions.merge-scope.support"),
        severity,
        subject,
        locator,
        outcome_kind,
        labels,
        evidence_posture,
        locality,
        FoundationalDiagnosticWidenedFalloutPosture::NotWidened,
    ))
}

pub(crate) fn provenance_row(
    code: &'static str,
    outcome_kind: FoundationalDiagnosticOutcomeKind,
    severity: FoundationalDiagnosticSeverity,
    subject: FoundationalDiagnosticSubject,
    locator: crate::diagnostics::FoundationalDiagnosticLocator,
    origin: crate::diagnostics::FoundationalDiagnosticLocator,
    labels: FoundationalDiagnosticSemanticLabelSet,
) -> FoundationalDiagnosticRow {
    FoundationalDiagnosticRow::ProvenanceReady(FoundationalDiagnosticProvenanceReadyRow::new(
        diagnostic_code(code),
        scope("transitions.merge-scope.provenance"),
        severity,
        subject,
        locator,
        outcome_kind,
        labels,
        origin,
        FoundationalDiagnosticEvidencePosture::RetainedDirect,
    ))
}

pub(crate) fn subject(
    source_branch: &FoundationalBranchId,
    target_branch: &FoundationalBranchId,
) -> FoundationalDiagnosticSubject {
    foundational_diagnostic_merge_verdict_subject(source_branch.clone(), target_branch.clone())
}

pub(crate) fn scope_locator(
    source_branch: &FoundationalBranchId,
    target_branch: &FoundationalBranchId,
    family: FoundationalMergeScopeFamily,
) -> crate::diagnostics::FoundationalDiagnosticLocator {
    foundational_diagnostic_locator_transition(FoundationalTransitionLocator::MergeScope(
        FoundationalMergeScopeLocator::new(source_branch.clone(), target_branch.clone(), family),
    ))
}

pub(crate) fn node_locator(
    source_branch: &FoundationalBranchId,
    target_branch: &FoundationalBranchId,
    node: &FoundationalSelectedNodeLocus,
) -> crate::diagnostics::FoundationalDiagnosticLocator {
    foundational_diagnostic_locator_transition(FoundationalTransitionLocator::SelectedNodeScope(
        FoundationalSelectedNodeScopeLocator::new(
            source_branch.clone(),
            target_branch.clone(),
            node.clone(),
        ),
    ))
}

pub(crate) fn aspect_locator(
    source_branch: &FoundationalBranchId,
    target_branch: &FoundationalBranchId,
    aspect: &FoundationalSelectedAspectRequestEntry,
) -> crate::diagnostics::FoundationalDiagnosticLocator {
    foundational_diagnostic_locator_transition(FoundationalTransitionLocator::SelectedAspectScope(
        FoundationalSelectedAspectScopeLocator::new(
            source_branch.clone(),
            target_branch.clone(),
            aspect.clone(),
        ),
    ))
}

pub(crate) fn locus_locator(
    source_branch: &FoundationalBranchId,
    target_branch: &FoundationalBranchId,
    locus: &FoundationalSelectedScopeLocus,
) -> crate::diagnostics::FoundationalDiagnosticLocator {
    match locus {
        FoundationalSelectedScopeLocus::Node(node) => {
            node_locator(source_branch, target_branch, node)
        }
        FoundationalSelectedScopeLocus::Aspect(aspect) => {
            aspect_locator(source_branch, target_branch, aspect)
        }
    }
}

pub(crate) fn denied_locus_locator(
    evidence: &FoundationalScopedMergeDenialEvidence,
) -> crate::diagnostics::FoundationalDiagnosticLocator {
    match evidence.denied_locus() {
        FoundationalDeniedScopeLocus::Node(node) => {
            node_locator(evidence.source_branch(), evidence.target_branch(), node)
        }
        FoundationalDeniedScopeLocus::Aspect(aspect) => {
            aspect_locator(evidence.source_branch(), evidence.target_branch(), aspect)
        }
        FoundationalDeniedScopeLocus::ScopeFamily(family) => {
            scope_locator(evidence.source_branch(), evidence.target_branch(), *family)
        }
    }
}

pub(crate) fn outcome_for_unavailable(
    category: FoundationalScopedMergeUnavailableOutcomeCategory,
) -> FoundationalDiagnosticOutcomeKind {
    match category {
        FoundationalScopedMergeUnavailableOutcomeCategory::Deferred => {
            FoundationalDiagnosticOutcomeKind::Deferred
        }
        FoundationalScopedMergeUnavailableOutcomeCategory::Stale
        | FoundationalScopedMergeUnavailableOutcomeCategory::RebindRequired => {
            FoundationalDiagnosticOutcomeKind::Partial
        }
        FoundationalScopedMergeUnavailableOutcomeCategory::Failed => {
            FoundationalDiagnosticOutcomeKind::Unsupported
        }
    }
}

pub(crate) fn counter_snapshot(
    requested_loci: u64,
    admitted_loci: u64,
    summarized_loci: u64,
) -> FoundationalDiagnosticCounterSnapshot {
    FoundationalDiagnosticCounterSnapshot::new(
        saturating_u32(requested_loci + admitted_loci),
        saturating_u32(summarized_loci),
        0,
        0,
        0,
        0,
    )
}

pub(crate) fn labels<const N: usize>(
    values: [&'static str; N],
) -> FoundationalDiagnosticSemanticLabelSet {
    FoundationalDiagnosticSemanticLabelSet::new(values.into_iter().map(diagnostic_code))
}

pub(crate) fn scope_family_label(family: FoundationalMergeScopeFamily) -> &'static str {
    match family {
        FoundationalMergeScopeFamily::FullBranch => "full-branch",
        FoundationalMergeScopeFamily::SelectedNodes => "selected-nodes",
        FoundationalMergeScopeFamily::SelectedAspects => "selected-aspects",
    }
}

pub(crate) fn admission_basis_label(basis: FoundationalScopeAdmissionBasis) -> &'static str {
    match basis {
        FoundationalScopeAdmissionBasis::DirectSourceIdentity => "direct-source-identity",
        FoundationalScopeAdmissionBasis::IdentityCorresponded => "identity-corresponded",
    }
}

pub(crate) fn no_op_cause_label(cause: FoundationalSelectedScopeNoOpCause) -> &'static str {
    match cause {
        FoundationalSelectedScopeNoOpCause::UnchangedSourceTruth => "unchanged-source-truth",
        FoundationalSelectedScopeNoOpCause::EquivalentTargetTruth => "equivalent-target-truth",
    }
}

pub(crate) fn denial_kind_label(kind: FoundationalScopedMergeDenialKind) -> &'static str {
    match kind {
        FoundationalScopedMergeDenialKind::UnknownSelectedNode => "unknown-selected-node",
        FoundationalScopedMergeDenialKind::UnknownSelectedAspect => "unknown-selected-aspect",
        FoundationalScopedMergeDenialKind::SelectedNodeMissingFromSourceScope => {
            "selected-node-missing-from-source-scope"
        }
        FoundationalScopedMergeDenialKind::SelectedNodeDeletedBeforeAdmission => {
            "selected-node-deleted-before-admission"
        }
        FoundationalScopedMergeDenialKind::SelectedTargetCorrespondenceAmbiguous => {
            "selected-target-correspondence-ambiguous"
        }
        FoundationalScopedMergeDenialKind::SelectedTargetCorrespondenceRejectedByDeclaration => {
            "selected-target-correspondence-rejected-by-declaration"
        }
        FoundationalScopedMergeDenialKind::SelectedNodeNonAdoptable => {
            "selected-node-non-adoptable"
        }
        FoundationalScopedMergeDenialKind::SelectedAspectUnsupportedByNodeOrStrategy => {
            "selected-aspect-unsupported-by-node-or-strategy"
        }
        FoundationalScopedMergeDenialKind::ScopeFamilyRejectedByDeclaration => {
            "scope-family-rejected-by-declaration"
        }
    }
}

pub(crate) fn unavailable_reason_label(
    reason: FoundationalScopedMergeUnavailableReason,
) -> &'static str {
    match reason {
        FoundationalScopedMergeUnavailableReason::RuntimeDoesNotSupportSelectedNodes => {
            "runtime-does-not-support-selected-nodes"
        }
        FoundationalScopedMergeUnavailableReason::RuntimeDoesNotSupportSelectedAspects => {
            "runtime-does-not-support-selected-aspects"
        }
        FoundationalScopedMergeUnavailableReason::MaterializerUnavailable => {
            "materializer-unavailable"
        }
        FoundationalScopedMergeUnavailableReason::IdentityCorrespondenceUnavailable => {
            "identity-correspondence-unavailable"
        }
        FoundationalScopedMergeUnavailableReason::RetainedProofUnavailable => {
            "retained-proof-unavailable"
        }
    }
}

fn diagnostic_code(value: &'static str) -> crate::diagnostics::FoundationalDiagnosticCodeId {
    foundational_diagnostic_code(value).expect("static scoped merge diagnostic code")
}

fn scope(value: &'static str) -> FoundationalDiagnosticScopeId {
    foundational_diagnostic_scope(value).expect("static scoped merge diagnostic scope")
}

fn saturating_u32(value: u64) -> u32 {
    value.try_into().unwrap_or(u32::MAX)
}
