use super::performance::ForgeServerStreamingPerformanceReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerStreamCancellationKind {
    ClientDisconnect,
    DownstreamBackpressure,
    CallerCancelled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerStreamCancellationReceipt {
    kind: ForgeServerStreamCancellationKind,
    chunks_emitted: usize,
    bytes_emitted: usize,
    canonical_result_completed: bool,
    transport_completed: bool,
    detail: String,
    performance_receipt: ForgeServerStreamingPerformanceReceipt,
    canonical_digest: String,
}

impl ForgeServerStreamCancellationReceipt {
    pub(crate) fn new(
        kind: ForgeServerStreamCancellationKind,
        chunks_emitted: usize,
        bytes_emitted: usize,
        canonical_result_completed: bool,
        detail: impl Into<String>,
        performance_receipt: ForgeServerStreamingPerformanceReceipt,
    ) -> Self {
        let detail = detail.into();
        let canonical_digest = format!(
            "compat-http-stream-cancellation-v1|kind:{kind:?}|chunks:{chunks_emitted}|bytes:{bytes_emitted}|semantic_complete:{canonical_result_completed}|detail:{detail}"
        );
        Self {
            kind,
            chunks_emitted,
            bytes_emitted,
            canonical_result_completed,
            transport_completed: false,
            detail,
            performance_receipt,
            canonical_digest,
        }
    }

    pub fn kind(&self) -> ForgeServerStreamCancellationKind {
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

    pub fn performance_receipt(&self) -> &ForgeServerStreamingPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
