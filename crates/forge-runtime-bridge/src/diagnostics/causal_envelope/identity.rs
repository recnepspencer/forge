use std::sync::Arc;

use super::{causal_envelope_digest, digest_basis::BridgeCausalEnvelopeDigestArtifact};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeCausalEnvelopeIdentity {
    request_digest: Arc<str>,
    causal_observation_anchor_digest: Arc<str>,
    evidence_binding_digest: Arc<str>,
    counter_digest: Arc<str>,
    identity_digest: Arc<str>,
}

impl BridgeCausalEnvelopeIdentity {
    pub(super) fn new(
        request_digest: impl Into<Arc<str>>,
        causal_observation_anchor_digest: impl Into<Arc<str>>,
        evidence_binding_digest: impl Into<Arc<str>>,
        counter_digest: impl Into<Arc<str>>,
    ) -> Self {
        let request_digest = request_digest.into();
        let causal_observation_anchor_digest = causal_observation_anchor_digest.into();
        let evidence_binding_digest = evidence_binding_digest.into();
        let counter_digest = counter_digest.into();
        let identity_digest = causal_envelope_digest(
            BridgeCausalEnvelopeDigestArtifact::Identity,
            &[
                request_digest.as_ref(),
                causal_observation_anchor_digest.as_ref(),
                evidence_binding_digest.as_ref(),
                counter_digest.as_ref(),
            ],
        );
        Self {
            request_digest,
            causal_observation_anchor_digest,
            evidence_binding_digest,
            counter_digest,
            identity_digest: Arc::from(identity_digest),
        }
    }

    pub fn request_digest(&self) -> &str {
        self.request_digest.as_ref()
    }

    pub fn causal_observation_anchor_digest(&self) -> &str {
        self.causal_observation_anchor_digest.as_ref()
    }

    pub fn evidence_binding_digest(&self) -> &str {
        self.evidence_binding_digest.as_ref()
    }

    pub fn counter_digest(&self) -> &str {
        self.counter_digest.as_ref()
    }

    pub fn identity_digest(&self) -> &str {
        self.identity_digest.as_ref()
    }
}
