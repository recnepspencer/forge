use crate::ForgeServerCompatibilityRead;

use super::{performance::ForgeServerStreamingPerformanceReceipt, ForgeServerStreamSelection};

#[derive(Debug)]
pub struct ForgeServerCompatibilityExport {
    read: ForgeServerCompatibilityRead,
    payload_bytes: Vec<u8>,
    estimated_payload_bytes: usize,
    selection: ForgeServerStreamSelection,
    performance_receipt: ForgeServerStreamingPerformanceReceipt,
    canonical_digest: String,
}

impl ForgeServerCompatibilityExport {
    pub(crate) fn new(
        read: ForgeServerCompatibilityRead,
        payload_bytes: Vec<u8>,
        estimated_payload_bytes: usize,
        selection: ForgeServerStreamSelection,
        performance_receipt: ForgeServerStreamingPerformanceReceipt,
    ) -> Self {
        let canonical_digest = format!(
            "compat-http-export-v1|read:{}|selection:{}|estimated_bytes:{}|payload_bytes:{}",
            read.canonical_digest(),
            selection.canonical_digest(),
            estimated_payload_bytes,
            payload_bytes.len(),
        );
        Self {
            read,
            payload_bytes,
            estimated_payload_bytes,
            selection,
            performance_receipt,
            canonical_digest,
        }
    }

    pub fn read(&self) -> &ForgeServerCompatibilityRead {
        &self.read
    }

    pub fn payload_bytes(&self) -> &[u8] {
        &self.payload_bytes
    }

    pub fn estimated_payload_bytes(&self) -> usize {
        self.estimated_payload_bytes
    }

    pub fn selection(&self) -> &ForgeServerStreamSelection {
        &self.selection
    }

    pub fn performance_receipt(&self) -> &ForgeServerStreamingPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Debug)]
pub struct ForgeServerBackgroundExportRequest {
    read: ForgeServerCompatibilityRead,
    estimated_payload_bytes: usize,
    selection: ForgeServerStreamSelection,
    detail: String,
    performance_receipt: ForgeServerStreamingPerformanceReceipt,
    canonical_digest: String,
}

impl ForgeServerBackgroundExportRequest {
    pub(crate) fn new(
        read: ForgeServerCompatibilityRead,
        estimated_payload_bytes: usize,
        selection: ForgeServerStreamSelection,
        detail: impl Into<String>,
        performance_receipt: ForgeServerStreamingPerformanceReceipt,
    ) -> Self {
        let detail = detail.into();
        let canonical_digest = format!(
            "compat-http-background-export-v1|read:{}|selection:{}|estimated_bytes:{}|detail:{}",
            read.canonical_digest(),
            selection.canonical_digest(),
            estimated_payload_bytes,
            detail,
        );
        Self {
            read,
            estimated_payload_bytes,
            selection,
            detail,
            performance_receipt,
            canonical_digest,
        }
    }

    pub fn read(&self) -> &ForgeServerCompatibilityRead {
        &self.read
    }

    pub fn estimated_payload_bytes(&self) -> usize {
        self.estimated_payload_bytes
    }

    pub fn selection(&self) -> &ForgeServerStreamSelection {
        &self.selection
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
