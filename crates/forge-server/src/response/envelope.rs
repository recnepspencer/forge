use super::{
    denial::ForgeServerDenialEnvelope, success::ForgeServerSuccessEnvelope,
    ForgeServerResponseTransform,
};
use forge_foundational::facade::DiagnosticRichnessProfile;

#[derive(Clone, Debug, PartialEq, Eq)]
enum ForgeServerResponseEnvelopeKind {
    Success(ForgeServerSuccessEnvelope),
    Denial(ForgeServerDenialEnvelope),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForgeServerResponseEnvelope {
    inner: ForgeServerResponseEnvelopeKind,
}

impl ForgeServerResponseEnvelope {
    pub(crate) fn from_success(envelope: ForgeServerSuccessEnvelope) -> Self {
        Self {
            inner: ForgeServerResponseEnvelopeKind::Success(envelope),
        }
    }

    pub(crate) fn from_denial(envelope: ForgeServerDenialEnvelope) -> Self {
        Self {
            inner: ForgeServerResponseEnvelopeKind::Denial(envelope),
        }
    }

    pub fn success(&self) -> Option<&ForgeServerSuccessEnvelope> {
        match &self.inner {
            ForgeServerResponseEnvelopeKind::Success(envelope) => Some(envelope),
            ForgeServerResponseEnvelopeKind::Denial(_) => None,
        }
    }

    pub fn denial(&self) -> Option<&ForgeServerDenialEnvelope> {
        match &self.inner {
            ForgeServerResponseEnvelopeKind::Success(_) => None,
            ForgeServerResponseEnvelopeKind::Denial(envelope) => Some(envelope),
        }
    }

    pub fn canonical_digest(&self) -> &str {
        match &self.inner {
            ForgeServerResponseEnvelopeKind::Success(envelope) => envelope.canonical_digest(),
            ForgeServerResponseEnvelopeKind::Denial(envelope) => envelope.canonical_digest(),
        }
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        match &self.inner {
            ForgeServerResponseEnvelopeKind::Success(envelope) => envelope.diagnostics_profile(),
            ForgeServerResponseEnvelopeKind::Denial(envelope) => envelope.diagnostics_profile(),
        }
    }

    pub fn transform(&self) -> ForgeServerResponseTransform {
        match &self.inner {
            ForgeServerResponseEnvelopeKind::Success(envelope) => envelope.transform(),
            ForgeServerResponseEnvelopeKind::Denial(envelope) => envelope.transform(),
        }
    }

    pub fn provenance(
        &self,
    ) -> &forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact {
        match &self.inner {
            ForgeServerResponseEnvelopeKind::Success(envelope) => envelope.provenance(),
            ForgeServerResponseEnvelopeKind::Denial(envelope) => envelope.provenance(),
        }
    }
}
