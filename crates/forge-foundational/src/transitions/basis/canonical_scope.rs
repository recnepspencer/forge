use crate::canonicalization::CanonicalBasisEntry;
use crate::transitions::{
    FoundationalAdmittedMergeScopeEvidence, FoundationalDeniedScopeLocus, FoundationalMergeScope,
    FoundationalMergeScopeFamily, FoundationalScopeAdmissionBasis,
    FoundationalScopedMergeDenialEvidence, FoundationalScopedMergeDenialKind,
    FoundationalScopedMergeUnavailableOutcomeCategory, FoundationalScopedMergeUnavailablePosture,
    FoundationalScopedMergeUnavailableReason, FoundationalSelectedAspectRequestEntry,
    FoundationalSelectedScopeLocus, FoundationalSelectedScopeNoOpCause,
    FoundationalSelectedScopeNoOpEvidence,
};

use super::canonical_shared::{bool_entry, digest_entry, text_entry, u64_entry};

pub(super) fn merge_scope_entries(
    prefix: &str,
    scope: &FoundationalMergeScope,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        text_entry(
            &format!("{prefix}.family"),
            scope_family_token(scope.family()),
        ),
        u64_entry(
            &format!("{prefix}.requested_locus_count"),
            scope.requested_locus_count(),
        ),
    ];
    append_selected_node_entries(prefix, scope.selected_nodes_loci(), &mut entries);
    append_selected_aspect_entries(prefix, scope.selected_aspect_loci(), &mut entries);
    entries
}

pub(super) fn admitted_scope_entries(
    prefix: &str,
    evidence: &FoundationalAdmittedMergeScopeEvidence,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        text_entry(
            &format!("{prefix}.source_branch"),
            evidence.source_branch().as_str(),
        ),
        text_entry(
            &format!("{prefix}.target_branch"),
            evidence.target_branch().as_str(),
        ),
        text_entry(
            &format!("{prefix}.admission_basis"),
            admission_basis_token(evidence.admission_basis()),
        ),
        u64_entry(
            &format!("{prefix}.breadth.requested_locus_count"),
            evidence.breadth().requested_locus_count(),
        ),
        u64_entry(
            &format!("{prefix}.breadth.admitted_locus_count"),
            evidence.breadth().admitted_locus_count(),
        ),
        u64_entry(
            &format!("{prefix}.breadth.no_op_locus_count"),
            evidence.breadth().no_op_locus_count(),
        ),
        u64_entry(
            &format!("{prefix}.breadth.skipped_candidate_count"),
            evidence.breadth().skipped_candidate_count(),
        ),
        u64_entry(
            &format!("{prefix}.breadth.conflict_check_width"),
            evidence.breadth().conflict_check_width(),
        ),
        bool_entry(
            &format!("{prefix}.skipped.has_digest"),
            evidence.skipped().skipped_digest().is_some(),
        ),
    ];
    entries.extend(merge_scope_entries(
        &format!("{prefix}.requested_scope"),
        evidence.requested_scope(),
    ));
    append_selected_node_entries(prefix, evidence.admitted_nodes(), &mut entries);
    append_selected_aspect_entries(prefix, evidence.admitted_aspects(), &mut entries);
    append_selected_no_op_entries(prefix, evidence.selected_no_ops(), &mut entries);
    if let Some(digest) = evidence.skipped().skipped_digest() {
        entries.push(digest_entry(&format!("{prefix}.skipped.digest"), digest));
    }
    entries
}

pub(super) fn scoped_denial_entries(
    prefix: &str,
    evidence: &FoundationalScopedMergeDenialEvidence,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        text_entry(
            &format!("{prefix}.source_branch"),
            evidence.source_branch().as_str(),
        ),
        text_entry(
            &format!("{prefix}.target_branch"),
            evidence.target_branch().as_str(),
        ),
        text_entry(
            &format!("{prefix}.denial_kind"),
            denial_kind_token(evidence.denial_kind()),
        ),
    ];
    entries.extend(merge_scope_entries(
        &format!("{prefix}.requested_scope"),
        evidence.requested_scope(),
    ));
    append_denied_locus_entry(prefix, evidence.denied_locus(), &mut entries);
    entries
}

pub(super) fn scoped_unavailable_entries(
    prefix: &str,
    posture: &FoundationalScopedMergeUnavailablePosture,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        text_entry(
            &format!("{prefix}.source_branch"),
            posture.source_branch().as_str(),
        ),
        text_entry(
            &format!("{prefix}.target_branch"),
            posture.target_branch().as_str(),
        ),
        text_entry(
            &format!("{prefix}.reason"),
            unavailable_reason_token(posture.reason()),
        ),
        text_entry(
            &format!("{prefix}.outcome_category"),
            unavailable_category_token(posture.outcome_category()),
        ),
    ];
    entries.extend(merge_scope_entries(
        &format!("{prefix}.requested_scope"),
        posture.requested_scope(),
    ));
    entries
}

fn append_selected_node_entries(
    prefix: &str,
    nodes: &[crate::transitions::FoundationalSelectedNodeLocus],
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    for (index, node) in nodes.iter().enumerate() {
        entries.push(text_entry(
            &format!("{prefix}.selected_node.{index}.locus"),
            node.as_str(),
        ));
    }
}

fn append_selected_aspect_entries(
    prefix: &str,
    aspects: &[FoundationalSelectedAspectRequestEntry],
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    for (index, aspect) in aspects.iter().enumerate() {
        entries.push(text_entry(
            &format!("{prefix}.selected_aspect.{index}.node"),
            aspect.node().as_str(),
        ));
        entries.push(text_entry(
            &format!("{prefix}.selected_aspect.{index}.aspect"),
            aspect.aspect().as_str(),
        ));
    }
}

fn append_selected_no_op_entries(
    prefix: &str,
    no_ops: &[FoundationalSelectedScopeNoOpEvidence],
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    for (index, no_op) in no_ops.iter().enumerate() {
        append_scope_locus_entries(
            &format!("{prefix}.selected_no_op.{index}.locus"),
            no_op.locus(),
            entries,
        );
        entries.push(text_entry(
            &format!("{prefix}.selected_no_op.{index}.cause"),
            no_op_cause_token(no_op.cause()),
        ));
    }
}

fn append_denied_locus_entry(
    prefix: &str,
    locus: &FoundationalDeniedScopeLocus,
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    match locus {
        FoundationalDeniedScopeLocus::Node(node) => {
            entries.push(text_entry(&format!("{prefix}.denied_locus.kind"), "node"));
            entries.push(text_entry(
                &format!("{prefix}.denied_locus.node"),
                node.as_str(),
            ));
        }
        FoundationalDeniedScopeLocus::Aspect(aspect) => {
            entries.push(text_entry(&format!("{prefix}.denied_locus.kind"), "aspect"));
            entries.push(text_entry(
                &format!("{prefix}.denied_locus.node"),
                aspect.node().as_str(),
            ));
            entries.push(text_entry(
                &format!("{prefix}.denied_locus.aspect"),
                aspect.aspect().as_str(),
            ));
        }
        FoundationalDeniedScopeLocus::ScopeFamily(family) => {
            entries.push(text_entry(
                &format!("{prefix}.denied_locus.kind"),
                "scope-family",
            ));
            entries.push(text_entry(
                &format!("{prefix}.denied_locus.scope_family"),
                scope_family_token(*family),
            ));
        }
    }
}

fn append_scope_locus_entries(
    prefix: &str,
    locus: &FoundationalSelectedScopeLocus,
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    match locus {
        FoundationalSelectedScopeLocus::Node(node) => {
            entries.push(text_entry(&format!("{prefix}.kind"), "node"));
            entries.push(text_entry(&format!("{prefix}.node"), node.as_str()));
        }
        FoundationalSelectedScopeLocus::Aspect(aspect) => {
            entries.push(text_entry(&format!("{prefix}.kind"), "aspect"));
            entries.push(text_entry(
                &format!("{prefix}.node"),
                aspect.node().as_str(),
            ));
            entries.push(text_entry(
                &format!("{prefix}.aspect"),
                aspect.aspect().as_str(),
            ));
        }
    }
}

fn scope_family_token(family: FoundationalMergeScopeFamily) -> &'static str {
    match family {
        FoundationalMergeScopeFamily::FullBranch => "full-branch",
        FoundationalMergeScopeFamily::SelectedNodes => "selected-nodes",
        FoundationalMergeScopeFamily::SelectedAspects => "selected-aspects",
    }
}

fn admission_basis_token(basis: FoundationalScopeAdmissionBasis) -> &'static str {
    match basis {
        FoundationalScopeAdmissionBasis::DirectSourceIdentity => "direct-source-identity",
        FoundationalScopeAdmissionBasis::IdentityCorresponded => "identity-corresponded",
    }
}

fn no_op_cause_token(cause: FoundationalSelectedScopeNoOpCause) -> &'static str {
    match cause {
        FoundationalSelectedScopeNoOpCause::UnchangedSourceTruth => "unchanged-source-truth",
        FoundationalSelectedScopeNoOpCause::EquivalentTargetTruth => "equivalent-target-truth",
    }
}

fn denial_kind_token(kind: FoundationalScopedMergeDenialKind) -> &'static str {
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

fn unavailable_reason_token(reason: FoundationalScopedMergeUnavailableReason) -> &'static str {
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

fn unavailable_category_token(
    category: FoundationalScopedMergeUnavailableOutcomeCategory,
) -> &'static str {
    match category {
        FoundationalScopedMergeUnavailableOutcomeCategory::Deferred => "deferred",
        FoundationalScopedMergeUnavailableOutcomeCategory::Stale => "stale",
        FoundationalScopedMergeUnavailableOutcomeCategory::RebindRequired => "rebind-required",
        FoundationalScopedMergeUnavailableOutcomeCategory::Failed => "failed",
    }
}
