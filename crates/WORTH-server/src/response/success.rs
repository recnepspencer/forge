use worth_foundational::facade::DiagnosticRichnessProfile;

use crate::{WorthServerQueryHandoffOperation, WorthServerQuerySupportPosture};

use super::{receipt::WorthServerResponseReceipt, WorthServerResponseTransform};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerSuccessKind {
    QueryRead,
    DirectRead,
    DirectState,
    DirectInspection,
    DirectProjection,
    DirectMutation,
    QueryMutation,
    DownstreamDelivery,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerSuccessPayload {
    kind: WorthServerSuccessKind,
    operation: WorthServerQueryHandoffOperation,
    support_posture: WorthServerQuerySupportPosture,
    workspace_name: String,
}

impl WorthServerSuccessPayload {
    pub(crate) fn new(
        kind: WorthServerSuccessKind,
        operation: WorthServerQueryHandoffOperation,
        support_posture: WorthServerQuerySupportPosture,
        workspace_name: String,
    ) -> Self {
        Self {
            kind,
            operation,
            support_posture,
            workspace_name,
        }
    }

    pub fn kind(&self) -> WorthServerSuccessKind {
        self.kind
    }

    pub fn operation(&self) -> &WorthServerQueryHandoffOperation {
        &self.operation
    }

    pub fn support_posture(&self) -> &WorthServerQuerySupportPosture {
        &self.support_posture
    }

    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthServerSuccessEnvelope {
    transform: WorthServerResponseTransform,
    diagnostics_profile: DiagnosticRichnessProfile,
    payload: WorthServerSuccessPayload,
    provenance: worth_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
    receipt: WorthServerResponseReceipt,
    canonical_digest: String,
}

impl WorthServerSuccessEnvelope {
    pub(crate) fn new(
        transform: WorthServerResponseTransform,
        diagnostics_profile: DiagnosticRichnessProfile,
        payload: WorthServerSuccessPayload,
        provenance: worth_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
        receipt: WorthServerResponseReceipt,
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

    pub fn transform(&self) -> WorthServerResponseTransform {
        self.transform
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn payload(&self) -> &WorthServerSuccessPayload {
        &self.payload
    }

    pub fn provenance(
        &self,
    ) -> &worth_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn receipt(&self) -> &WorthServerResponseReceipt {
        &self.receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
