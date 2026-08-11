use crate::diagnostics::summary::EvaluationPlanSummary;

use super::super::model::{compare_value, push_mismatch, DiagnosticMismatchCategory, PlanDiff};

pub fn compare_plans(left: &EvaluationPlanSummary, right: &EvaluationPlanSummary) -> PlanDiff {
    let mut diff = PlanDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::PlanShape,
        "stage_count",
        left.stage_count,
        right.stage_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::PlanShape,
        "task_count",
        left.task_count,
        right.task_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::PlanShape,
        "max_stage_width",
        left.max_stage_width,
        right.max_stage_width,
    );
    if left.stage_widths != right.stage_widths {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::PlanShape,
            "stage_widths",
            format!("{:?}", left.stage_widths),
            format!("{:?}", right.stage_widths),
        );
    }
    if left.task_reason_counts != right.task_reason_counts {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::PlanShape,
            "task_reason_counts",
            format!("{:?}", left.task_reason_counts),
            format!("{:?}", right.task_reason_counts),
        );
    }
    diff
}
