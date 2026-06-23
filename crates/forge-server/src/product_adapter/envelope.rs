use forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact;

use crate::ForgeServerResponseReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerProductOperationEnvelopeKind {
    Success,
    Denial,
    Failure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductOperationEnvelope {
    kind: ForgeServerProductOperationEnvelopeKind,
    operation_name: String,
    canonical_digest: String,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    receipt: ForgeServerResponseReceipt,
}

impl ForgeServerProductOperationEnvelope {
    pub(crate) fn new(
        kind: ForgeServerProductOperationEnvelopeKind,
        operation_name: impl Into<String>,
        canonical_digest: impl Into<String>,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
        receipt: ForgeServerResponseReceipt,
    ) -> Self {
        Self {
            kind,
            operation_name: operation_name.into(),
            canonical_digest: canonical_digest.into(),
            provenance,
            receipt,
        }
    }

    pub fn kind(&self) -> ForgeServerProductOperationEnvelopeKind {
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

    pub fn receipt(&self) -> &ForgeServerResponseReceipt {
        &self.receipt
    }
}
