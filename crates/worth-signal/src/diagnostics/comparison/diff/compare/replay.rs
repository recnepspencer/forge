use crate::diagnostics::replay::ReplaySlice;

use super::super::model::{compare_value, push_mismatch, DiagnosticMismatchCategory, ReplayDiff};

pub fn compare_replay_slices(left: &ReplaySlice, right: &ReplaySlice) -> ReplayDiff {
    let mut diff = ReplayDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::ExecutionRecord,
        "start",
        format!("{:?}", left.start),
        format!("{:?}", right.start),
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::ExecutionRecord,
        "end",
        format!("{:?}", left.end),
        format!("{:?}", right.end),
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::ExecutionRecord,
        "frame_count",
        left.frames.len(),
        right.frames.len(),
    );
    if left.frames != right.frames {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::ExecutionRecord,
            "frames",
            format!("{:?}", left.frames),
            format!("{:?}", right.frames),
        );
    }
    diff
}
