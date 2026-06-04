use crate::diagnostics::{
    FoundationalDiagnosticDenialClass, FoundationalDiagnosticEvidencePosture,
    FoundationalDiagnosticExplanationInput, FoundationalDiagnosticLocalityClaim,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticPartiality, FoundationalDiagnosticRow,
    FoundationalDiagnosticSeverity, FoundationalDiagnosticSubject,
    FoundationalDiagnosticSupportEvidencePosture, FoundationalDiagnosticSurfaceAvailability,
};
use crate::transitions::FoundationalBranchId;

use super::scope_diagnostic_rows::*;
use super::{
    FoundationalAdmittedMergeScopeEvidence, FoundationalMergeScope,
    FoundationalScopedMergeDenialEvidence, FoundationalScopedMergeUnavailablePosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalScopedMergeDiagnosticInput {
    ScopeRequest {
        source_branch: FoundationalBranchId,
        target_branch: FoundationalBranchId,
        requested_scope: FoundationalMergeScope,
    },
    AdmittedScope(FoundationalAdmittedMergeScopeEvidence),
    DeniedScope(FoundationalScopedMergeDenialEvidence),
    UnavailableScope(FoundationalScopedMergeUnavailablePosture),
}

pub fn prepare_scoped_merge_diagnostic_explanation(
    input: FoundationalScopedMergeDiagnosticInput,
) -> FoundationalDiagnosticExplanationInput {
    match input {
        FoundationalScopedMergeDiagnosticInput::ScopeRequest {
            source_branch,
            target_branch,
            requested_scope,
        } => scope_request_explanation(source_branch, target_branch, requested_scope),
        FoundationalScopedMergeDiagnosticInput::AdmittedScope(evidence) => {
            admitted_scope_explanation(evidence)
        }
        FoundationalScopedMergeDiagnosticInput::DeniedScope(evidence) => {
            denied_scope_explanation(evidence)
        }
        FoundationalScopedMergeDiagnosticInput::UnavailableScope(posture) => {
            unavailable_scope_explanation(posture)
        }
    }
}

fn scope_request_explanation(
    source_branch: FoundationalBranchId,
    target_branch: FoundationalBranchId,
    requested_scope: FoundationalMergeScope,
) -> FoundationalDiagnosticExplanationInput {
    let subject = subject(&source_branch, &target_branch);
    let locator = scope_locator(&source_branch, &target_branch, requested_scope.family());
    FoundationalDiagnosticExplanationInput::new(
        subject.clone(),
        FoundationalDiagnosticOutcomeKind::Advisory,
        vec![decision_row(
            "merge-scope.requested",
            FoundationalDiagnosticOutcomeKind::Advisory,
            FoundationalDiagnosticSeverity::Info,
            subject.clone(),
            locator.clone(),
            labels([
                "merge-scope",
                "requested",
                scope_family_label(requested_scope.family()),
            ]),
            None,
            FoundationalDiagnosticLocalityClaim::ExactSubject,
        )],
        requested_locus_support_rows(
            &source_branch,
            &target_branch,
            &subject,
            &requested_scope,
            "merge-scope.requested-locus",
        ),
        Vec::new(),
        FoundationalDiagnosticSurfaceAvailability::retained_hot(),
        FoundationalDiagnosticPartiality::Complete,
        counter_snapshot(requested_scope.requested_locus_count(), 0, 0),
        Vec::new(),
    )
}

fn admitted_scope_explanation(
    evidence: FoundationalAdmittedMergeScopeEvidence,
) -> FoundationalDiagnosticExplanationInput {
    let subject = subject(evidence.source_branch(), evidence.target_branch());
    let locator = scope_locator(
        evidence.source_branch(),
        evidence.target_branch(),
        evidence.requested_scope().family(),
    );
    let breadth = evidence.breadth();
    let mut standard_rows = vec![
        support_row(
            "merge-scope.skipped",
            FoundationalDiagnosticOutcomeKind::Advisory,
            FoundationalDiagnosticSeverity::Info,
            subject.clone(),
            locator.clone(),
            labels(["merge-scope", "skipped"]),
            FoundationalDiagnosticSupportEvidencePosture::Present(
                FoundationalDiagnosticEvidencePosture::Summarized,
            ),
            FoundationalDiagnosticLocalityClaim::ExactSubject,
        ),
        support_row(
            "merge-scope.no-op",
            FoundationalDiagnosticOutcomeKind::Advisory,
            FoundationalDiagnosticSeverity::Info,
            subject.clone(),
            locator.clone(),
            labels(["merge-scope", "no-op"]),
            FoundationalDiagnosticSupportEvidencePosture::Present(
                FoundationalDiagnosticEvidencePosture::Summarized,
            ),
            FoundationalDiagnosticLocalityClaim::ExactSubject,
        ),
    ];
    standard_rows.extend(admitted_locus_support_rows(&evidence, &subject));

    let forensic_rows = evidence
        .selected_no_ops()
        .iter()
        .map(|entry| {
            support_row(
                "merge-scope.no-op-locus",
                FoundationalDiagnosticOutcomeKind::Advisory,
                FoundationalDiagnosticSeverity::Info,
                subject.clone(),
                locus_locator(
                    evidence.source_branch(),
                    evidence.target_branch(),
                    entry.locus(),
                ),
                labels(["merge-scope", "no-op", no_op_cause_label(entry.cause())]),
                FoundationalDiagnosticSupportEvidencePosture::Present(
                    FoundationalDiagnosticEvidencePosture::RetainedDirect,
                ),
                FoundationalDiagnosticLocalityClaim::ExactSubject,
            )
        })
        .collect();

    FoundationalDiagnosticExplanationInput::new(
        subject.clone(),
        FoundationalDiagnosticOutcomeKind::Accepted,
        vec![decision_row(
            "merge-scope.admitted",
            FoundationalDiagnosticOutcomeKind::Accepted,
            FoundationalDiagnosticSeverity::Info,
            subject.clone(),
            locator,
            labels([
                "merge-scope",
                "admitted",
                admission_basis_label(evidence.admission_basis()),
            ]),
            None,
            FoundationalDiagnosticLocalityClaim::ExactSubject,
        )],
        standard_rows,
        forensic_rows,
        FoundationalDiagnosticSurfaceAvailability::retained_hot(),
        FoundationalDiagnosticPartiality::Complete,
        counter_snapshot(
            breadth.requested_locus_count(),
            breadth.admitted_locus_count(),
            breadth.skipped_candidate_count() + breadth.no_op_locus_count(),
        ),
        Vec::new(),
    )
}

fn denied_scope_explanation(
    evidence: FoundationalScopedMergeDenialEvidence,
) -> FoundationalDiagnosticExplanationInput {
    let subject = subject(evidence.source_branch(), evidence.target_branch());
    let locator = denied_locus_locator(&evidence);
    FoundationalDiagnosticExplanationInput::new(
        subject.clone(),
        FoundationalDiagnosticOutcomeKind::Denied,
        vec![decision_row(
            "merge-scope.denied",
            FoundationalDiagnosticOutcomeKind::Denied,
            FoundationalDiagnosticSeverity::Denial,
            subject.clone(),
            locator.clone(),
            labels([
                "merge-scope",
                "denied",
                denial_kind_label(evidence.denial_kind()),
            ]),
            Some(FoundationalDiagnosticDenialClass::DomainDenied),
            FoundationalDiagnosticLocalityClaim::ExactSubject,
        )],
        Vec::new(),
        vec![provenance_row(
            "merge-scope.denied-origin",
            FoundationalDiagnosticOutcomeKind::Denied,
            FoundationalDiagnosticSeverity::Info,
            subject,
            locator.clone(),
            locator,
            labels(["merge-scope", "denied", "origin"]),
        )],
        FoundationalDiagnosticSurfaceAvailability::retained_hot(),
        FoundationalDiagnosticPartiality::Complete,
        counter_snapshot(evidence.requested_scope().requested_locus_count(), 0, 0),
        Vec::new(),
    )
}

fn unavailable_scope_explanation(
    posture: FoundationalScopedMergeUnavailablePosture,
) -> FoundationalDiagnosticExplanationInput {
    let subject = subject(posture.source_branch(), posture.target_branch());
    let locator = scope_locator(
        posture.source_branch(),
        posture.target_branch(),
        posture.requested_scope().family(),
    );
    FoundationalDiagnosticExplanationInput::new(
        subject.clone(),
        outcome_for_unavailable(posture.outcome_category()),
        vec![decision_row(
            "merge-scope.unavailable",
            outcome_for_unavailable(posture.outcome_category()),
            FoundationalDiagnosticSeverity::Warning,
            subject.clone(),
            locator.clone(),
            labels([
                "merge-scope",
                "unavailable",
                unavailable_reason_label(posture.reason()),
            ]),
            Some(FoundationalDiagnosticDenialClass::EvidenceUnavailableDenied),
            FoundationalDiagnosticLocalityClaim::ExactSubject,
        )],
        Vec::new(),
        vec![provenance_row(
            "merge-scope.unavailable-origin",
            outcome_for_unavailable(posture.outcome_category()),
            FoundationalDiagnosticSeverity::Info,
            subject,
            locator.clone(),
            locator,
            labels(["merge-scope", "unavailable", "origin"]),
        )],
        FoundationalDiagnosticSurfaceAvailability::retained_hot(),
        FoundationalDiagnosticPartiality::Complete,
        counter_snapshot(posture.requested_scope().requested_locus_count(), 0, 0),
        Vec::new(),
    )
}

fn requested_locus_support_rows(
    source_branch: &FoundationalBranchId,
    target_branch: &FoundationalBranchId,
    subject: &FoundationalDiagnosticSubject,
    scope: &FoundationalMergeScope,
    code: &'static str,
) -> Vec<FoundationalDiagnosticRow> {
    let node_rows = scope.selected_nodes_loci().iter().map(|node| {
        support_row(
            code,
            FoundationalDiagnosticOutcomeKind::Advisory,
            FoundationalDiagnosticSeverity::Info,
            subject.clone(),
            node_locator(source_branch, target_branch, node),
            labels(["merge-scope", "requested", "selected-node"]),
            FoundationalDiagnosticSupportEvidencePosture::Present(
                FoundationalDiagnosticEvidencePosture::RetainedDirect,
            ),
            FoundationalDiagnosticLocalityClaim::ExactSubject,
        )
    });
    let aspect_rows = scope.selected_aspect_loci().iter().map(|aspect| {
        support_row(
            code,
            FoundationalDiagnosticOutcomeKind::Advisory,
            FoundationalDiagnosticSeverity::Info,
            subject.clone(),
            aspect_locator(source_branch, target_branch, aspect),
            labels(["merge-scope", "requested", "selected-aspect"]),
            FoundationalDiagnosticSupportEvidencePosture::Present(
                FoundationalDiagnosticEvidencePosture::RetainedDirect,
            ),
            FoundationalDiagnosticLocalityClaim::ExactSubject,
        )
    });
    node_rows.chain(aspect_rows).collect()
}

fn admitted_locus_support_rows(
    evidence: &FoundationalAdmittedMergeScopeEvidence,
    subject: &FoundationalDiagnosticSubject,
) -> Vec<FoundationalDiagnosticRow> {
    let node_rows = evidence.admitted_nodes().iter().map(|node| {
        support_row(
            "merge-scope.admitted-locus",
            FoundationalDiagnosticOutcomeKind::Accepted,
            FoundationalDiagnosticSeverity::Info,
            subject.clone(),
            node_locator(evidence.source_branch(), evidence.target_branch(), node),
            labels(["merge-scope", "admitted", "selected-node"]),
            FoundationalDiagnosticSupportEvidencePosture::Present(
                FoundationalDiagnosticEvidencePosture::RetainedDirect,
            ),
            FoundationalDiagnosticLocalityClaim::ExactSubject,
        )
    });
    let aspect_rows = evidence.admitted_aspects().iter().map(|aspect| {
        support_row(
            "merge-scope.admitted-locus",
            FoundationalDiagnosticOutcomeKind::Accepted,
            FoundationalDiagnosticSeverity::Info,
            subject.clone(),
            aspect_locator(evidence.source_branch(), evidence.target_branch(), aspect),
            labels(["merge-scope", "admitted", "selected-aspect"]),
            FoundationalDiagnosticSupportEvidencePosture::Present(
                FoundationalDiagnosticEvidencePosture::RetainedDirect,
            ),
            FoundationalDiagnosticLocalityClaim::ExactSubject,
        )
    });
    node_rows.chain(aspect_rows).collect()
}
