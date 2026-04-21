use crate::tiering::{
    CoalescedRecallReport, RecallCoalescingKey, RecallCompletionWitness,
    RecallExecutionDisposition, RetainedReadPlacementPath,
};

pub(crate) fn build_recall_report(
    coalescing_key: RecallCoalescingKey,
    disposition: RecallExecutionDisposition,
    artifact_key: &str,
    placement_path: RetainedReadPlacementPath,
    verification_label: &str,
    completion_witness: Option<RecallCompletionWitness>,
) -> CoalescedRecallReport {
    CoalescedRecallReport::new(
        coalescing_key,
        disposition,
        artifact_key,
        placement_path,
        verification_label,
        completion_witness,
    )
}
