use worth_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact;

use crate::WorthServerResponseReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerProductOperationEnvelopeKind {
    Success,
    Denial,
    Failure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductOperationEnvelope {
    kind: WorthServerProductOperationEnvelopeKind,
    operation_name: String,
    canonical_digest: String,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    receipt: WorthServerResponseReceipt,
}

impl WorthServerProductOperationEnvelope {
    pub(crate) fn new(
        kind: WorthServerProductOperationEnvelopeKind,
        operation_name: impl Into<String>,
        canonical_digest: impl Into<String>,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
        receipt: WorthServerResponseReceipt,
    ) -> Self {
        Self {
            kind,
            operation_name: operation_name.into(),
            canonical_digest: canonical_digest.into(),
            provenance,
            receipt,
        }
    }

    pub fn kind(&self) -> WorthServerProductOperationEnvelopeKind {
        self.kind
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn receipt(&self) -> &WorthServerResponseReceipt {
        &self.receipt
    }
}
