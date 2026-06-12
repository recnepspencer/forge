use super::*;
use crate::runtime::runtime_writes::ForgeQueryWriteAdmissionExecutionRecord;

pub(super) fn batch_execution_provenance(
    shared_admission: Option<&ForgeQueryWriteAdmissionExecutionRecord>,
    combined_receipt: &ForgeQueryMutationReceipt,
) -> Option<ForgeQueryIntentExecutionProvenance> {
    shared_admission.map(|record| {
        let commit_evidence_identity = combined_receipt.commit_identity.evidence_identity();
        let snapshot_evidence_identity = combined_receipt.snapshot_identity.evidence_identity();
        ForgeQueryIntentExecutionProvenance::for_shared_execution_typed_parts(
            record.family,
            record.entrypoint,
            record.execution_seam,
            &record.decision_digest,
            &record.handoff_digest,
            &record.binding_digest,
            commit_evidence_identity.as_str(),
            &snapshot_evidence_identity,
        )
    })
}

pub(super) fn batch_decision_trace_envelope(
    shared_admission: Option<&ForgeQueryWriteAdmissionExecutionRecord>,
    combined_receipt: &ForgeQueryMutationReceipt,
    batch_request_detail: &str,
) -> Option<ForgeQueryIntentDecisionTraceEnvelope> {
    shared_admission.map(|record| {
        let commit_evidence_identity = combined_receipt.commit_identity.evidence_identity();
        ForgeQueryIntentDecisionTraceEnvelope::for_admitted_execution_parts(
            record.family,
            record.entrypoint,
            &record.request_detail,
            &record.request_digest,
            record.eligibility_trace.clone(),
            &record.decision_digest,
            &record.handoff_digest,
            record.execution_seam,
            batch_request_detail,
            commit_evidence_identity.as_str(),
            "mutation-batch-write",
        )
    })
}
