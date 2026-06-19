use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection, ForgeQueryWriteReceipt,
    ForgeQueryWriteReceiptInspection,
};

use crate::{
    ForgeServerDirectContextArtifact, ForgeServerQuerySupportPosture, ForgeServerResponseEnvelope,
};

#[derive(Debug)]
pub struct ForgeServerDirectMutation {
    operation_request: crate::ForgeServerOperationRequest,
    plan_proof: crate::ForgeServerOperationPlanProof,
    support_posture: ForgeServerQuerySupportPosture,
    workspace_name: String,
    handoff_digest: String,
    direct_context: ForgeServerDirectContextArtifact,
    mutation_result: ForgeServerDirectMutationResult,
    response_envelope: ForgeServerResponseEnvelope,
    canonical_digest: String,
}

impl ForgeServerDirectMutation {
    pub(crate) fn new(
        operation_request: crate::ForgeServerOperationRequest,
        plan_proof: crate::ForgeServerOperationPlanProof,
        support_posture: ForgeServerQuerySupportPosture,
        workspace_name: String,
        handoff_digest: String,
        direct_context: ForgeServerDirectContextArtifact,
        mutation_result: ForgeServerDirectMutationResult,
        response_envelope: ForgeServerResponseEnvelope,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-direct-mutation-v1:{}:{}:{}:{}",
            operation_request.canonical_digest(),
            handoff_digest,
            mutation_result.result_digest(),
            mutation_result.inspection_digest()
        );
        Self {
            operation_request,
            plan_proof,
            support_posture,
            workspace_name,
            handoff_digest,
            direct_context,
            mutation_result,
            response_envelope,
            canonical_digest,
        }
    }

    pub fn operation_request(&self) -> &crate::ForgeServerOperationRequest {
        &self.operation_request
    }

    pub fn plan_proof(&self) -> &crate::ForgeServerOperationPlanProof {
        &self.plan_proof
    }

    pub fn support_posture(&self) -> &ForgeServerQuerySupportPosture {
        &self.support_posture
    }

    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }

    pub fn direct_context(&self) -> &ForgeServerDirectContextArtifact {
        &self.direct_context
    }

    pub fn mutation_result(&self) -> &ForgeServerDirectMutationResult {
        &self.mutation_result
    }

    pub fn response_envelope(&self) -> &ForgeServerResponseEnvelope {
        &self.response_envelope
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Debug)]
pub enum ForgeServerDirectMutationResult {
    Single {
        receipt: ForgeQueryWriteReceipt,
        inspection: ForgeQueryWriteReceiptInspection,
    },
    Batch {
        receipt: ForgeQueryBatchWriteReceipt,
        inspection: ForgeQueryBatchWriteReceiptInspection,
    },
}

impl ForgeServerDirectMutationResult {
    pub fn single_receipt(&self) -> Option<&ForgeQueryWriteReceipt> {
        match self {
            Self::Single { receipt, .. } => Some(receipt),
            Self::Batch { .. } => None,
        }
    }

    pub fn single_inspection(&self) -> Option<&ForgeQueryWriteReceiptInspection> {
        match self {
            Self::Single { inspection, .. } => Some(inspection),
            Self::Batch { .. } => None,
        }
    }

    pub fn batch_receipt(&self) -> Option<&ForgeQueryBatchWriteReceipt> {
        match self {
            Self::Single { .. } => None,
            Self::Batch { receipt, .. } => Some(receipt),
        }
    }

    pub fn batch_inspection(&self) -> Option<&ForgeQueryBatchWriteReceiptInspection> {
        match self {
            Self::Single { .. } => None,
            Self::Batch { inspection, .. } => Some(inspection),
        }
    }

    pub fn execution_provenance_digest(&self) -> &str {
        match self {
            Self::Single { receipt, .. } => receipt
                .execution_provenance_chain_digest()
                .expect("direct single mutation should preserve execution provenance"),
            Self::Batch { receipt, .. } => receipt
                .execution_provenance()
                .expect("direct batch mutation should preserve execution provenance")
                .execution_provenance_chain_digest(),
        }
    }

    pub fn result_digest(&self) -> &str {
        match self {
            Self::Single { receipt, .. } => receipt
                .commit_evidence_identity()
                .terminal_projection_for_reporting(),
            Self::Batch { receipt, .. } => receipt.batch_digest(),
        }
    }

    pub fn inspection_digest(&self) -> &str {
        match self {
            Self::Single { inspection, .. } => inspection.inspection_digest(),
            Self::Batch { inspection, .. } => inspection.inspection_digest(),
        }
    }
}
