mod authority_continuity_fields;
mod authority_continuity_mismatch_fields;
mod durable_identity_fields;

use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::durability::authority::diagnostics::recovery::authority_continuity_fields::{
    recovery_authority_continuity_evaluated_fields, recovery_checkpoint_selected_fields,
    recovery_range_replayed_fields,
};
use crate::durability::data::{DurableCheckpointId, DurableSegmentId, RecoveryPlan};

pub(in crate::durability::authority) fn recovery_authority_continuity_evaluated(
    plan: &RecoveryPlan,
) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry::new(
        DiagnosticCode::DurableRecoveryAuthorityContinuityEvaluated,
        "durable recovery authority continuity evaluated before recovery execution",
        recovery_authority_continuity_evaluated_fields(plan),
    )
}

pub(in crate::durability::authority) fn recovery_checkpoint_selected(
    checkpoint_id: Option<DurableCheckpointId>,
    skipped_corrupt_checkpoints: &[DurableCheckpointId],
) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry::new(
        DiagnosticCode::RecoveryCheckpointSelected,
        "recovery checkpoint selected",
        recovery_checkpoint_selected_fields(checkpoint_id, skipped_corrupt_checkpoints),
    )
}

pub(in crate::durability::authority) fn recovery_range_replayed(
    segment_ids: &[DurableSegmentId],
    tail_commits: usize,
) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry::new(
        DiagnosticCode::RecoveryRangeReplayed,
        "durable tail replayed",
        recovery_range_replayed_fields(segment_ids, tail_commits),
    )
}
