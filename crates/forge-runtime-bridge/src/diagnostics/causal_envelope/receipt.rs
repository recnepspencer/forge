use std::sync::Arc;

use super::counters::BridgeCausalEnvelopeCounters;
use super::identity::BridgeCausalEnvelopeIdentity;
use super::{causal_envelope_digest, digest_basis::BridgeCausalEnvelopeDigestArtifact};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeCausalEnvelopeReceipt {
    envelope_identity_digest: Arc<str>,
    envelope_digest: Arc<str>,
    counter_digest: Arc<str>,
    receipt_digest: Arc<str>,
}

impl BridgeCausalEnvelopeReceipt {
    pub(super) fn new(
        identity: &BridgeCausalEnvelopeIdentity,
        envelope_digest: &str,
        counters: &BridgeCausalEnvelopeCounters,
    ) -> Self {
        let receipt_digest = causal_envelope_digest(
            BridgeCausalEnvelopeDigestArtifact::Receipt,
            &[
                identity.identity_digest(),
                envelope_digest,
                counters.counter_digest(),
            ],
        );
        Self {
            envelope_identity_digest: Arc::from(identity.identity_digest()),
            envelope_digest: Arc::from(envelope_digest),
            counter_digest: Arc::from(counters.counter_digest()),
            receipt_digest: Arc::from(receipt_digest),
        }
    }

    pub fn envelope_identity_digest(&self) -> &str {
        self.envelope_identity_digest.as_ref()
    }

    pub fn envelope_digest(&self) -> &str {
        self.envelope_digest.as_ref()
    }

    pub fn counter_digest(&self) -> &str {
        self.counter_digest.as_ref()
    }

    pub fn receipt_digest(&self) -> &str {
        self.receipt_digest.as_ref()
    }
}
