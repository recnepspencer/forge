use worth_ui::facade::{WorthUiValidationReloadEvidence, WorthUiValidationReloadStatus};

fn main() {
    let _evidence = WorthUiValidationReloadEvidence {
        runtime_instance_witness: 0,
        status: WorthUiValidationReloadStatus::ReadyForFrameBoundary,
        denial_detail: None,
        active_artifact_digest_before: 1,
        active_artifact_digest_after: 2,
        active_plan_digest_before: 3,
        active_plan_digest_after: 4,
        source_revision_digest: Some(5),
        ordering_receipt_digest: Some(6),
        candidate_artifact_digest: Some(7),
        candidate_plan_digest: Some(8),
        raw_events_observed: 1,
        events_coalesced: 1,
        provider_reads: 1,
        source_revisions_emitted: 1,
        candidate_submissions_emitted: 1,
        frame_path_work: 0,
        active_runtime_mutations_before_activation: 0,
        query_bindings_compared: 0,
        query_rebind_entries: 0,
        durable_state_reconciliation_receipts: 0,
        query_binding_planning_ran: true,
        durable_state_planning_ran: true,
    };
}
