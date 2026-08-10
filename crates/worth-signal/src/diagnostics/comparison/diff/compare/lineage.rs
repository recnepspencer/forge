use crate::diagnostics::lineage::LineageRecord;

use super::super::model::{compare_value, push_mismatch, DiagnosticMismatchCategory, LineageDiff};

pub fn compare_lineage_records(left: &[LineageRecord], right: &[LineageRecord]) -> LineageDiff {
    let mut diff = LineageDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "record_count",
        left.len(),
        right.len(),
    );
    if left != right {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "records",
            format!("{:?}", left),
            format!("{:?}", right),
        );
    }
    diff
}
