use crate::WorthServerFileTransferProvenance;

use super::performance::WorthServerStreamingPerformanceReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerStreamCancellationKind {
    ClientDisconnect,
    DownstreamBackpressure,
    CallerCancelled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerStreamCancellationReceipt {
    kind: WorthServerStreamCancellationKind,
    chunks_emitted: usize,
    bytes_emitted: usize,
    canonical_result_completed: bool,
    transport_completed: bool,
    detail: String,
    tenant_id: String,
    workspace_digest: String,
    branch_digest: String,
    transfer_provenance: WorthServerFileTransferProvenance,
    performance_receipt: WorthServerStreamingPerformanceReceipt,
    canonical_digest: String,
}

pub(crate) struct WorthServerStreamCancellationReceiptParts {
    pub(crate) kind: WorthServerStreamCancellationKind,
    pub(crate) chunks_emitted: usize,
    pub(crate) bytes_emitted: usize,
    pub(crate) canonical_result_completed: bool,
    pub(crate) detail: String,
    pub(crate) tenant_id: String,
    pub(crate) workspace_digest: String,
    pub(crate) branch_digest: String,
    pub(crate) transfer_provenance: WorthServerFileTransferProvenance,
    pub(crate) performance_receipt: WorthServerStreamingPerformanceReceipt,
}

impl WorthServerStreamCancellationReceipt {
    pub(crate) fn new(parts: WorthServerStreamCancellationReceiptParts) -> Self {
        let WorthServerStreamCancellationReceiptParts {
            kind,
            chunks_emitted,
            bytes_emitted,
            canonical_result_completed,
            detail,
            tenant_id,
            workspace_digest,
            branch_digest,
            transfer_provenance,
            performance_receipt,
        } = parts;
        let canonical_digest = format!(
            "compat-http-stream-cancellation-v2|kind:{kind:?}|tenant:{tenant_id}|workspace:{workspace_digest}|branch:{branch_digest}|chunks:{chunks_emitted}|bytes:{bytes_emitted}|semantic_complete:{canonical_result_completed}|detail:{detail}"
        );
        Self {
            kind,
            chunks_emitted,
            bytes_emitted,
            canonical_result_completed,
            transport_completed: false,
            detail,
            tenant_id,
            workspace_digest,
            branch_digest,
            transfer_provenance,
            performance_receipt,
            canonical_digest,
        }
    }

    pub fn kind(&self) -> WorthServerStreamCancellationKind {
        self.kind
    }

    pub fn chunks_emitted(&self) -> usize {
        self.chunks_emitted
    }

    pub fn bytes_emitted(&self) -> usize {
        self.bytes_emitted
    }

    pub fn canonical_result_completed(&self) -> bool {
        self.canonical_result_completed
    }

    pub fn transport_completed(&self) -> bool {
        self.transport_completed
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn workspace_digest(&self) -> &str {
        &self.workspace_digest
    }

    pub fn branch_digest(&self) -> &str {
        &self.branch_digest
    }

    pub fn transfer_provenance(&self) -> &WorthServerFileTransferProvenance {
        &self.transfer_provenance
    }

    pub fn performance_receipt(&self) -> &WorthServerStreamingPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
