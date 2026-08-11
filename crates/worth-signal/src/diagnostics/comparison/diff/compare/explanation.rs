use crate::diagnostics::summary::ExplanationSummary;

use super::super::model::{
    compare_value, push_mismatch, DiagnosticMismatchCategory, ExplanationDiff,
};

pub fn compare_explanations(
    left: &ExplanationSummary,
    right: &ExplanationSummary,
) -> ExplanationDiff {
    let mut diff = ExplanationDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "materialization_mode",
        left.materialization_mode,
        right.materialization_mode,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "state",
        format!("{:?}", left.state),
        format!("{:?}", right.state),
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "changed_upstream_count",
        left.changed_upstream_count,
        right.changed_upstream_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "skipped_upstream_count",
        left.skipped_upstream_count,
        right.skipped_upstream_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "condition_deferred_count",
        left.condition_deferred_count,
        right.condition_deferred_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "conservative_cause_count",
        left.conservative_cause_count,
        right.conservative_cause_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "direct_scope_count",
        left.direct_scope_count,
        right.direct_scope_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "translated_scope_count",
        left.translated_scope_count,
        right.translated_scope_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "discarded_scope_count",
        left.discarded_scope_count,
        right.discarded_scope_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "insufficient_scope_count",
        left.insufficient_scope_count,
        right.insufficient_scope_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "rewired_dependency_count",
        left.rewired_dependency_count,
        right.rewired_dependency_count,
    );
    if left.direct_cause_kinds != right.direct_cause_kinds {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "direct_cause_kinds",
            format!("{:?}", left.direct_cause_kinds),
            format!("{:?}", right.direct_cause_kinds),
        );
    }
    if left.scope_provenance_kinds != right.scope_provenance_kinds {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "scope_provenance_kinds",
            format!("{:?}", left.scope_provenance_kinds),
            format!("{:?}", right.scope_provenance_kinds),
        );
    }
    if left.cause_note_samples != right.cause_note_samples {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "cause_note_samples",
            format!("{:?}", left.cause_note_samples),
            format!("{:?}", right.cause_note_samples),
        );
    }
    if left.triage_classes != right.triage_classes {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "triage_classes",
            format!("{:?}", left.triage_classes),
            format!("{:?}", right.triage_classes),
        );
    }
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "propagation_suppressed",
        left.propagation_suppressed,
        right.propagation_suppressed,
    );
    if left.output_change != right.output_change {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "output_change",
            format!("{:?}", left.output_change),
            format!("{:?}", right.output_change),
        );
    }
    if left.memoized_origin != right.memoized_origin {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "memoized_origin",
            format!("{:?}", left.memoized_origin),
            format!("{:?}", right.memoized_origin),
        );
    }
    if left.reuse_basis != right.reuse_basis {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "reuse_basis",
            format!("{:?}", left.reuse_basis),
            format!("{:?}", right.reuse_basis),
        );
    }
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "reuse_certification_proof_count",
        left.reuse_certification_proof_count,
        right.reuse_certification_proof_count,
    );
    diff
}
