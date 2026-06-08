use forge_foundational::facade::DiagnosticRichnessProfile;

use crate::{ForgeServerQueryHandoffOperation, ForgeServerQuerySupportPosture};

use super::{receipt::ForgeServerResponseReceipt, ForgeServerResponseTransform};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerSuccessKind {
    QueryRead,
    QueryMutation,
    DownstreamDelivery,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerSuccessPayload {
    kind: ForgeServerSuccessKind,
    operation: ForgeServerQueryHandoffOperation,
    support_posture: ForgeServerQuerySupportPosture,
    workspace_name: String,
}

impl ForgeServerSuccessPayload {
    pub(crate) fn new(
        kind: ForgeServerSuccessKind,
        operation: ForgeServerQueryHandoffOperation,
        support_posture: ForgeServerQuerySupportPosture,
        workspace_name: String,
    ) -> Self {
        Self {
            kind,
            operation,
            support_posture,
            workspace_name,
        }
    }

    pub fn kind(&self) -> ForgeServerSuccessKind {
        self.kind
    }

    pub fn operation(&self) -> &ForgeServerQueryHandoffOperation {
        &self.operation
    }

    pub fn support_posture(&self) -> &ForgeServerQuerySupportPosture {
        &self.support_posture
    }

    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForgeServerSuccessEnvelope {
    transform: ForgeServerResponseTransform,
    diagnostics_profile: DiagnosticRichnessProfile,
    payload: ForgeServerSuccessPayload,
    provenance: forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
    receipt: ForgeServerResponseReceipt,
    canonical_digest: String,
}

impl ForgeServerSuccessEnvelope {
    pub(crate) fn new(
        transform: ForgeServerResponseTransform,
        diagnostics_profile: DiagnosticRichnessProfile,
        payload: ForgeServerSuccessPayload,
        provenance: forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
        receipt: ForgeServerResponseReceipt,
        canonical_digest: String,
    ) -> Self {
        Self {
            transform,
            diagnostics_profile,
            payload,
            provenance,
            receipt,
            canonical_digest,
        }
    }

    pub fn transform(&self) -> ForgeServerResponseTransform {
        self.transform
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn payload(&self) -> &ForgeServerSuccessPayload {
        &self.payload
    }

    pub fn provenance(
        &self,
    ) -> &forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn receipt(&self) -> &ForgeServerResponseReceipt {
        &self.receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
