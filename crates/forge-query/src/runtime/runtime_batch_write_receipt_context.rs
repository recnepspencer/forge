use super::*;
use crate::runtime::runtime_writes::ForgeQueryWriteAdmissionExecutionRecord;

pub(super) fn batch_execution_provenance(
    shared_admission: Option<&ForgeQueryWriteAdmissionExecutionRecord>,
    combined_receipt: &ForgeQueryMutationReceipt,
) -> Option<ForgeQueryIntentExecutionProvenance> {
    shared_admission.map(|record| {
        let commit_label = combined_receipt
            .commit_identity
            .evidence_identity()
            .reporting_projection()
            .to_string();
        let snapshot_evidence_identity = combined_receipt.snapshot_identity.evidence_identity();
        ForgeQueryIntentExecutionProvenance::for_shared_execution_typed_parts(
            record.family,
            record.entrypoint,
            record.execution_seam,
            &record.decision_digest,
            &record.handoff_digest,
            &record.binding_digest,
            &commit_label,
            &snapshot_evidence_identity,
        )
    })
}

pub(super) fn batch_obligation_dispatch(
    shared_admission: Option<&ForgeQueryWriteAdmissionExecutionRecord>,
) -> Option<ForgeQueryAuthoritativeMutationObligationDispatch> {
    shared_admission.and_then(|record| record.obligation_dispatch.clone())
}

pub(super) fn batch_decision_trace_envelope(
    shared_admission: Option<&ForgeQueryWriteAdmissionExecutionRecord>,
    combined_receipt: &ForgeQueryMutationReceipt,
    batch_request_detail: &str,
) -> Option<ForgeQueryIntentDecisionTraceEnvelope> {
    shared_admission.map(|record| {
        let commit_label = combined_receipt
            .commit_identity
            .evidence_identity()
            .reporting_projection()
            .to_string();
        let obligation_dispatch_envelope_digest = record
            .obligation_dispatch
            .as_ref()
            .and_then(ForgeQueryAuthoritativeMutationObligationDispatch::envelope_digest);
        ForgeQueryIntentDecisionTraceEnvelope::for_admitted_execution_parts_with_obligation_dispatch(
            record.family,
            record.entrypoint,
            &record.request_detail,
            &record.request_digest,
            record.eligibility_trace.clone(),
            &record.decision_digest,
            &record.handoff_digest,
            record.execution_seam,
            obligation_dispatch_envelope_digest,
            batch_request_detail,
            &commit_label,
            "mutation-batch-write",
        )
    })
}
