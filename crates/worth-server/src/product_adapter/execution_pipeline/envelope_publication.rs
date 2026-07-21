use crate::{
    WorthServerProductOperationEnvelope, WorthServerProductOperationEnvelopeKind,
    WorthServerProductOperationOutcome, WorthServerResponseReceipt,
    WorthServerScheduledProductOperation,
};

pub(crate) fn build_envelope(
    scheduled: &WorthServerScheduledProductOperation,
    outcome: &WorthServerProductOperationOutcome,
) -> WorthServerProductOperationEnvelope {
    build_envelope_with_completion(scheduled, outcome, None)
}

pub(crate) fn build_durable_envelope(
    scheduled: &WorthServerScheduledProductOperation,
    outcome: &WorthServerProductOperationOutcome,
    completion_digest: &str,
) -> WorthServerProductOperationEnvelope {
    build_envelope_with_completion(scheduled, outcome, Some(completion_digest))
}

fn build_envelope_with_completion(
    scheduled: &WorthServerScheduledProductOperation,
    outcome: &WorthServerProductOperationOutcome,
    completion_digest: Option<&str>,
) -> WorthServerProductOperationEnvelope {
    let (kind, outcome_label, receipt): (
        WorthServerProductOperationEnvelopeKind,
        &str,
        WorthServerResponseReceipt,
    ) = match outcome {
        WorthServerProductOperationOutcome::Success(success) => (
            WorthServerProductOperationEnvelopeKind::Success,
            success.result_artifact().artifact_digest(),
            crate::response::build_success_receipt(
                &format!(
                    "product success {} {}",
                    scheduled.plan().declaration().operation_name(),
                    success.result_key()
                ),
                scheduled.canonical_digest(),
                crate::response::build_provenance("product-success", scheduled.canonical_digest()),
            ),
        ),
        WorthServerProductOperationOutcome::Denied(denial) => (
            WorthServerProductOperationEnvelopeKind::Denial,
            denial.reason_key(),
            crate::response::build_denial_receipt(
                &format!(
                    "product denial {} {}",
                    scheduled.plan().declaration().operation_name(),
                    denial.reason_key()
                ),
                scheduled.canonical_digest(),
                crate::response::build_provenance("product-denial", scheduled.canonical_digest()),
            ),
        ),
        WorthServerProductOperationOutcome::Failed(failure) => (
            WorthServerProductOperationEnvelopeKind::Failure,
            failure.reason_key(),
            crate::response::build_denial_receipt(
                &format!(
                    "product failure {} {}",
                    scheduled.plan().declaration().operation_name(),
                    failure.reason_key()
                ),
                scheduled.canonical_digest(),
                crate::response::build_provenance("product-failure", scheduled.canonical_digest()),
            ),
        ),
    };
    let mut canonical_digest = crate::canonical_digest::WorthServerCanonicalDigestBuilder::new(
        "worth-server-product-operation-envelope-v2",
    )
    .field("operation", scheduled.plan().declaration().operation_name())
    .field("scheduled", scheduled.canonical_digest())
    .field("outcome", outcome_label);
    if let Some(completion_digest) = completion_digest {
        canonical_digest = canonical_digest.field("durable_completion", completion_digest);
    }
    let canonical_digest = canonical_digest.finish();
    let provenance =
        crate::response::build_provenance("product-operation-envelope", &canonical_digest);
    WorthServerProductOperationEnvelope::new(
        kind,
        scheduled.plan().declaration().operation_name(),
        canonical_digest,
        provenance,
        receipt,
    )
}

pub(in crate::product_adapter) fn build_early_envelope(
    operation_name: &str,
    request: &crate::WorthServerOperationRequest,
    outcome: &WorthServerProductOperationOutcome,
) -> WorthServerProductOperationEnvelope {
    let canonical_digest = crate::canonical_digest::WorthServerCanonicalDigestBuilder::new(
        "worth-server-product-operation-envelope-v2",
    )
    .field("request", request.canonical_digest())
    .field("outcome", &format!("{outcome:?}"))
    .finish();
    let provenance =
        crate::response::build_provenance("product-operation-envelope", &canonical_digest);
    let receipt = match outcome {
        WorthServerProductOperationOutcome::Success(_) => crate::response::build_success_receipt(
            &format!("product success {operation_name}"),
            &canonical_digest,
            provenance.clone(),
        ),
        WorthServerProductOperationOutcome::Denied(_)
        | WorthServerProductOperationOutcome::Failed(_) => crate::response::build_denial_receipt(
            &format!("product denial {operation_name}"),
            &canonical_digest,
            provenance.clone(),
        ),
    };
    WorthServerProductOperationEnvelope::new(
        match outcome {
            WorthServerProductOperationOutcome::Success(_) => {
                WorthServerProductOperationEnvelopeKind::Success
            }
            WorthServerProductOperationOutcome::Denied(_) => {
                WorthServerProductOperationEnvelopeKind::Denial
            }
            WorthServerProductOperationOutcome::Failed(_) => {
                WorthServerProductOperationEnvelopeKind::Failure
            }
        },
        operation_name,
        canonical_digest,
        provenance,
        receipt,
    )
}
