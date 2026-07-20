use super::{
    denial::WorthServerDenialEnvelope, success::WorthServerSuccessEnvelope,
    WorthServerResponseTransform,
};
use worth_foundational::facade::DiagnosticRichnessProfile;

#[derive(Clone, Debug, PartialEq, Eq)]
enum WorthServerResponseEnvelopeKind {
    Success(Box<WorthServerSuccessEnvelope>),
    Denial(Box<WorthServerDenialEnvelope>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthServerResponseEnvelope {
    inner: WorthServerResponseEnvelopeKind,
}

impl WorthServerResponseEnvelope {
    pub(crate) fn from_success(envelope: WorthServerSuccessEnvelope) -> Self {
        Self {
            inner: WorthServerResponseEnvelopeKind::Success(Box::new(envelope)),
        }
    }

    pub(crate) fn from_denial(envelope: WorthServerDenialEnvelope) -> Self {
        Self {
            inner: WorthServerResponseEnvelopeKind::Denial(Box::new(envelope)),
        }
    }

    pub fn success(&self) -> Option<&WorthServerSuccessEnvelope> {
        match &self.inner {
            WorthServerResponseEnvelopeKind::Success(envelope) => Some(envelope),
            WorthServerResponseEnvelopeKind::Denial(_) => None,
        }
    }

    pub fn denial(&self) -> Option<&WorthServerDenialEnvelope> {
        match &self.inner {
            WorthServerResponseEnvelopeKind::Success(_) => None,
            WorthServerResponseEnvelopeKind::Denial(envelope) => Some(envelope),
        }
    }

    pub fn canonical_digest(&self) -> &str {
        match &self.inner {
            WorthServerResponseEnvelopeKind::Success(envelope) => envelope.canonical_digest(),
            WorthServerResponseEnvelopeKind::Denial(envelope) => envelope.canonical_digest(),
        }
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        match &self.inner {
            WorthServerResponseEnvelopeKind::Success(envelope) => envelope.diagnostics_profile(),
            WorthServerResponseEnvelopeKind::Denial(envelope) => envelope.diagnostics_profile(),
        }
    }

    pub fn transform(&self) -> WorthServerResponseTransform {
        match &self.inner {
            WorthServerResponseEnvelopeKind::Success(envelope) => envelope.transform(),
            WorthServerResponseEnvelopeKind::Denial(envelope) => envelope.transform(),
        }
    }

    pub fn provenance(
        &self,
    ) -> &worth_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact {
        match &self.inner {
            WorthServerResponseEnvelopeKind::Success(envelope) => envelope.provenance(),
            WorthServerResponseEnvelopeKind::Denial(envelope) => envelope.provenance(),
        }
    }
}
