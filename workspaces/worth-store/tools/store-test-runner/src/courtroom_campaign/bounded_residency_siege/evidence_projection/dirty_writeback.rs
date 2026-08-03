use serde_json::{json, Value};

use super::super::protocol::BoundedResidencyDirtyObservation;

pub(super) fn value(dirty: BoundedResidencyDirtyObservation) -> Value {
    json!({
        "primary_publication": dirty.primary_publication,
        "retry_publication": dirty.retry_publication,
        "primary_candidate_writebacks": dirty.primary_candidate_writebacks,
        "retry_candidate_writebacks": dirty.retry_candidate_writebacks,
        "primary_candidate_publications": dirty.primary_candidate_publications,
        "retry_candidate_publications": dirty.retry_candidate_publications,
        "denied_candidate_publications": dirty.denied_candidate_publications,
        "primary_last_candidate_operation": dirty.primary_last_candidate_operation,
        "retry_last_candidate_operation": dirty.retry_last_candidate_operation,
        "primary_records": dirty.primary_records,
        "retry_records": dirty.retry_records,
        "dirty_at_dispatch": dirty.dirty_at_dispatch,
        "dirty_peak": dirty.dirty_peak,
        "dirty_after_denial": dirty.dirty_after_denial,
        "dirty_after_primary": dirty.dirty_after_primary,
        "dirty_terminal": dirty.dirty_terminal,
        "active_claims_at_dispatch": dirty.active_claims_at_dispatch,
        "active_writebehind_at_dispatch": dirty.active_writebehind_at_dispatch,
        "peak_writebehind": dirty.peak_writebehind,
        "terminal_writebehind": dirty.terminal_writebehind,
        "pressure_requested": dirty.pressure_requested,
        "pressure_admitted": dirty.pressure_admitted,
        "pressure_limit": dirty.pressure_limit,
        "pressure_basis_exact": dirty.pressure_basis_exact,
        "pressure_retry_after_settlement": dirty.pressure_retry_after_settlement,
        "pressure_effect_free": dirty.pressure_effect_free,
        "cleanup_deletions": dirty.cleanup_deletions,
        "cleanup_complete": dirty.cleanup_complete,
        "writebehind_attempts": dirty.writebehind_attempts,
        "writebehind_admissions": dirty.writebehind_admissions,
        "writebehind_denials": dirty.writebehind_denials,
        "writebehind_completions": dirty.writebehind_completions,
        "writeback_attempts": dirty.writeback_attempts,
        "exact_receipts": dirty.exact_receipts,
        "retryable_writebacks": dirty.retryable_writebacks,
        "indeterminate_writebacks": dirty.indeterminate_writebacks,
        "inspection_required_writebacks": dirty.inspection_required_writebacks,
        "candidate_publications": dirty.candidate_publications,
        "writebacks": dirty.writebacks,
        "positioned_writes": dirty.positioned_writes,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::value;
    use crate::courtroom_campaign::bounded_residency_siege::protocol::parse_dirty;

    #[test]
    fn projection_preserves_every_dirty_and_writebehind_field() {
        let marker = "\
BOUNDED_RESIDENCY_DIRTY 501 502 1 1 3 3 1 601 602 1 1 1 2 1 0 0 1 1 1 0 1 1 1 true true true \
1 true 3 2 1 2 3 2 0 0 0 7 2 2";
        let dirty = parse_dirty(&[marker.to_owned()]).unwrap();

        assert_eq!(
            value(dirty),
            json!({
                "primary_publication": 501,
                "retry_publication": 502,
                "primary_candidate_writebacks": 1,
                "retry_candidate_writebacks": 1,
                "primary_candidate_publications": 3,
                "retry_candidate_publications": 3,
                "denied_candidate_publications": 1,
                "primary_last_candidate_operation": 601,
                "retry_last_candidate_operation": 602,
                "primary_records": 1,
                "retry_records": 1,
                "dirty_at_dispatch": 1,
                "dirty_peak": 2,
                "dirty_after_denial": 1,
                "dirty_after_primary": 0,
                "dirty_terminal": 0,
                "active_claims_at_dispatch": 1,
                "active_writebehind_at_dispatch": 1,
                "peak_writebehind": 1,
                "terminal_writebehind": 0,
                "pressure_requested": 1,
                "pressure_admitted": 1,
                "pressure_limit": 1,
                "pressure_basis_exact": true,
                "pressure_retry_after_settlement": true,
                "pressure_effect_free": true,
                "cleanup_deletions": 1,
                "cleanup_complete": true,
                "writebehind_attempts": 3,
                "writebehind_admissions": 2,
                "writebehind_denials": 1,
                "writebehind_completions": 2,
                "writeback_attempts": 3,
                "exact_receipts": 2,
                "retryable_writebacks": 0,
                "indeterminate_writebacks": 0,
                "inspection_required_writebacks": 0,
                "candidate_publications": 7,
                "writebacks": 2,
                "positioned_writes": 2,
            })
        );
    }
}
