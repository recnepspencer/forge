use crate::diagnostics::flow::FlowSummary;

use super::super::model::{push_mismatch, DiagnosticMismatchCategory, FlowDiff};
use super::{
    execution::compare_execution_reports, explanation::compare_explanations, plan::compare_plans,
};

pub fn compare_flows(left: &FlowSummary, right: &FlowSummary) -> FlowDiff {
    let mut diff = FlowDiff::default();
    if left.change != right.change {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "change",
            format!("{:?}", left.change),
            format!("{:?}", right.change),
        );
    }
    if left.invalidation != right.invalidation {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "invalidation",
            format!("{:?}", left.invalidation),
            format!("{:?}", right.invalidation),
        );
    }
    diff.mismatches
        .extend(compare_plans(&left.planning.plan, &right.planning.plan).mismatches);
    if left.precompute != right.precompute {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "precompute",
            format!("{:?}", left.precompute),
            format!("{:?}", right.precompute),
        );
    }
    diff.mismatches
        .extend(compare_execution_reports(&left.apply.report, &right.apply.report).mismatches);
    if left.apply != right.apply {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "apply",
            format!("{:?}", left.apply),
            format!("{:?}", right.apply),
        );
    }
    if left.rollback != right.rollback {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "rollback",
            format!("{:?}", left.rollback),
            format!("{:?}", right.rollback),
        );
    }
    match (&left.explanation, &right.explanation) {
        (Some(left), Some(right)) => diff
            .mismatches
            .extend(compare_explanations(left, right).mismatches),
        (None, Some(_)) | (Some(_), None) => push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "explanation_presence",
            left.explanation.is_some(),
            right.explanation.is_some(),
        ),
        (None, None) => {}
    }
    if left.cause_samples != right.cause_samples {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "cause_samples",
            format!("{:?}", left.cause_samples),
            format!("{:?}", right.cause_samples),
        );
    }
    if left.event_epochs != right.event_epochs {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "event_epochs",
            format!("{:?}", left.event_epochs),
            format!("{:?}", right.event_epochs),
        );
    }
    diff
}
