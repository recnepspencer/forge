use crate::runtime::{
    WorthUiFileRustReplacementParityCounters, WorthUiFileRustReplacementPipelineReport,
    WorthUiFileRustReplacementSemanticReceipt,
};

#[derive(Debug, PartialEq)]
pub struct WorthUiFileRustReplacementParityReceipt {
    file_report: WorthUiFileRustReplacementPipelineReport,
    rust_report: WorthUiFileRustReplacementPipelineReport,
    semantic_receipt: WorthUiFileRustReplacementSemanticReceipt,
    counters: WorthUiFileRustReplacementParityCounters,
}

impl WorthUiFileRustReplacementParityReceipt {
    pub(crate) fn new(
        file_report: WorthUiFileRustReplacementPipelineReport,
        rust_report: WorthUiFileRustReplacementPipelineReport,
        semantic_receipt: WorthUiFileRustReplacementSemanticReceipt,
        counters: WorthUiFileRustReplacementParityCounters,
    ) -> Self {
        Self {
            file_report,
            rust_report,
            semantic_receipt,
            counters,
        }
    }

    pub fn file_report(&self) -> &WorthUiFileRustReplacementPipelineReport {
        &self.file_report
    }

    pub fn rust_report(&self) -> &WorthUiFileRustReplacementPipelineReport {
        &self.rust_report
    }

    pub fn semantic_receipt(&self) -> WorthUiFileRustReplacementSemanticReceipt {
        self.semantic_receipt
    }

    pub fn counters(&self) -> WorthUiFileRustReplacementParityCounters {
        self.counters
    }
}
