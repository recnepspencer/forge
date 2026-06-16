use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection, ForgeQueryWriteReceipt,
    ForgeQueryWriteReceiptInspection,
};

use super::{
    envelope::ForgeServerCompatibilityMutationEnvelope,
    precondition::ForgeServerMutationPrecondition,
    request::ForgeServerCompatibilityMutationRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCompatibilityMutation {
    mutation_request: ForgeServerCompatibilityMutationRequest,
    precondition: ForgeServerMutationPrecondition,
    mutation_result: ForgeServerCompatibilityMutationResult,
    envelope: ForgeServerCompatibilityMutationEnvelope,
    canonical_digest: String,
}

impl ForgeServerCompatibilityMutation {
    pub(crate) fn new(
        mutation_request: ForgeServerCompatibilityMutationRequest,
        precondition: ForgeServerMutationPrecondition,
        mutation_result: ForgeServerCompatibilityMutationResult,
        envelope: ForgeServerCompatibilityMutationEnvelope,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-compat-mutation-v1|request:{}|precondition:{}|result:{}|inspection:{}|envelope:{}",
            mutation_request.canonical_digest(),
            precondition.canonical_digest(),
            mutation_result.result_digest(),
            mutation_result.inspection_digest(),
            envelope.canonical_digest(),
        );
        Self {
            mutation_request,
            precondition,
            mutation_result,
            envelope,
            canonical_digest,
        }
    }

    pub(crate) fn to_replayed(
        &self,
        replay_receipt: super::idempotency::ForgeServerIdempotentReplayReceipt,
    ) -> Self {
        let envelope = ForgeServerCompatibilityMutationEnvelope::new(
            self.envelope.support_posture().clone(),
            self.envelope.workspace_name().to_string(),
            self.envelope.handoff_digest().to_string(),
            self.envelope.direct_context().clone(),
            self.envelope.response_envelope().clone(),
            replay_receipt,
        );
        Self::new(
            self.mutation_request.clone(),
            self.precondition.clone(),
            self.mutation_result.clone(),
            envelope,
        )
    }

    pub fn mutation_request(&self) -> &ForgeServerCompatibilityMutationRequest {
        &self.mutation_request
    }

    pub fn precondition(&self) -> &ForgeServerMutationPrecondition {
        &self.precondition
    }

    pub fn mutation_result(&self) -> &ForgeServerCompatibilityMutationResult {
        &self.mutation_result
    }

    pub fn envelope(&self) -> &ForgeServerCompatibilityMutationEnvelope {
        &self.envelope
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerCompatibilityMutationResult {
    Single {
        receipt: ForgeQueryWriteReceipt,
        inspection: ForgeQueryWriteReceiptInspection,
    },
    Batch {
        receipt: ForgeQueryBatchWriteReceipt,
        inspection: ForgeQueryBatchWriteReceiptInspection,
    },
}

impl ForgeServerCompatibilityMutationResult {
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
