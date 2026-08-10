use crate::diagnostics::failure::FailureSummary;

use super::super::model::{compare_value, push_mismatch, DiagnosticMismatchCategory, FailureDiff};

pub fn compare_failures(left: &FailureSummary, right: &FailureSummary) -> FailureDiff {
    let mut diff = FailureDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::FailureState,
        "phase",
        format!("{:?}", left.phase),
        format!("{:?}", right.phase),
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::FailureState,
        "rolled_back",
        left.rolled_back,
        right.rolled_back,
    );
    if left.node != right.node {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::FailureState,
            "node",
            format!("{:?}", left.node),
            format!("{:?}", right.node),
        );
    }
    if left.message != right.message {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::FailureState,
            "message",
            &left.message,
            &right.message,
        );
    }
    diff
}
