use std::sync::Arc;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncCertificationCounters {
    request_identity_count: usize,
    completion_receipt_count: usize,
    denied_completion_receipt_count: usize,
    supersession_receipt_count: usize,
    forward_causality_receipt_count: usize,
    writeback_receipt_count: usize,
    localized_failure_count: usize,
    consumer_projection_count: usize,
    digest: Arc<str>,
}

impl BridgeTemporalAsyncCertificationCounters {
    pub(crate) fn new(
        request_identity_count: usize,
        completion_receipt_count: usize,
        denied_completion_receipt_count: usize,
        supersession_receipt_count: usize,
        forward_causality_receipt_count: usize,
        writeback_receipt_count: usize,
        localized_failure_count: usize,
        consumer_projection_count: usize,
    ) -> Self {
        let canonical_basis = format!(
            "bridge-temporal-async-certification-counters|requests={request_identity_count}|completions={completion_receipt_count}|denied={denied_completion_receipt_count}|supersession={supersession_receipt_count}|forward={forward_causality_receipt_count}|writeback={writeback_receipt_count}|failures={localized_failure_count}|consumers={consumer_projection_count}"
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            request_identity_count,
            completion_receipt_count,
            denied_completion_receipt_count,
            supersession_receipt_count,
            forward_causality_receipt_count,
            writeback_receipt_count,
            localized_failure_count,
            consumer_projection_count,
            digest: Arc::from(format!(
                "bridge-temporal-async-certification-counters:sha256:{digest:x}"
            )),
        }
    }

    pub fn request_identity_count(&self) -> usize {
        self.request_identity_count
    }

    pub fn localized_failure_count(&self) -> usize {
        self.localized_failure_count
    }

    pub fn consumer_projection_count(&self) -> usize {
        self.consumer_projection_count
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
