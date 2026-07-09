use crate::{
    WorthServerBinaryCertificationBundle, WorthServerCompatibilityFileEnvelope,
    WorthServerCompatibilityRead,
};

use super::super::project_binary_egress_envelope;
use super::{performance::WorthServerStreamingPerformanceReceipt, WorthServerStreamSelection};

#[derive(Debug)]
pub struct WorthServerCompatibilityExport {
    read: WorthServerCompatibilityRead,
    payload_bytes: Vec<u8>,
    estimated_payload_bytes: usize,
    selection: WorthServerStreamSelection,
    performance_receipt: WorthServerStreamingPerformanceReceipt,
    file_envelope: WorthServerCompatibilityFileEnvelope,
    certification_bundle: WorthServerBinaryCertificationBundle,
    canonical_digest: String,
}

impl WorthServerCompatibilityExport {
    pub(crate) fn new(
        read: WorthServerCompatibilityRead,
        payload_bytes: Vec<u8>,
        estimated_payload_bytes: usize,
        selection: WorthServerStreamSelection,
        performance_receipt: WorthServerStreamingPerformanceReceipt,
        certification_bundle: WorthServerBinaryCertificationBundle,
    ) -> Self {
        let file_envelope = project_binary_egress_envelope(
            &read,
            Some("application/json".to_string()),
            payload_bytes.len() as u64,
            false,
            crate::WorthServerFileTransferDisposition::SelectedEgress,
        );
        let canonical_digest = format!(
            "compat-http-export-v2|read:{}|selection:{}|estimated_bytes:{}|payload_bytes:{}|file_envelope:{}|certification:{}",
            read.canonical_digest(),
            selection.canonical_digest(),
            estimated_payload_bytes,
            payload_bytes.len(),
            file_envelope.canonical_digest(),
            certification_bundle.canonical_digest(),
        );
        Self {
            read,
            payload_bytes,
            estimated_payload_bytes,
            selection,
            performance_receipt,
            file_envelope,
            certification_bundle,
            canonical_digest,
        }
    }

    pub fn read(&self) -> &WorthServerCompatibilityRead {
        &self.read
    }

    pub fn payload_bytes(&self) -> &[u8] {
        &self.payload_bytes
    }

    pub fn estimated_payload_bytes(&self) -> usize {
        self.estimated_payload_bytes
    }

    pub fn selection(&self) -> &WorthServerStreamSelection {
        &self.selection
    }

    pub fn performance_receipt(&self) -> &WorthServerStreamingPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn file_envelope(&self) -> &WorthServerCompatibilityFileEnvelope {
        &self.file_envelope
    }

    pub fn certification_bundle(&self) -> &WorthServerBinaryCertificationBundle {
        &self.certification_bundle
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Debug)]
pub struct WorthServerBackgroundExportRequest {
    read: WorthServerCompatibilityRead,
    estimated_payload_bytes: usize,
    selection: WorthServerStreamSelection,
    detail: String,
    performance_receipt: WorthServerStreamingPerformanceReceipt,
    file_envelope: WorthServerCompatibilityFileEnvelope,
    certification_bundle: WorthServerBinaryCertificationBundle,
    canonical_digest: String,
}

impl WorthServerBackgroundExportRequest {
    pub(crate) fn new(
        read: WorthServerCompatibilityRead,
        estimated_payload_bytes: usize,
        selection: WorthServerStreamSelection,
        detail: impl Into<String>,
        performance_receipt: WorthServerStreamingPerformanceReceipt,
        certification_bundle: WorthServerBinaryCertificationBundle,
    ) -> Self {
        let detail = detail.into();
        let file_envelope = project_binary_egress_envelope(
            &read,
            Some("application/json".to_string()),
            0,
            false,
            crate::WorthServerFileTransferDisposition::MetadataOnlyObservation,
        );
        let canonical_digest = format!(
            "compat-http-background-export-v2|read:{}|selection:{}|estimated_bytes:{}|detail:{}|file_envelope:{}|certification:{}",
            read.canonical_digest(),
            selection.canonical_digest(),
            estimated_payload_bytes,
            detail,
            file_envelope.canonical_digest(),
            certification_bundle.canonical_digest(),
        );
        Self {
            read,
            estimated_payload_bytes,
            selection,
            detail,
            performance_receipt,
            file_envelope,
            certification_bundle,
            canonical_digest,
        }
    }

    pub fn read(&self) -> &WorthServerCompatibilityRead {
        &self.read
    }

    pub fn estimated_payload_bytes(&self) -> usize {
        self.estimated_payload_bytes
    }

    pub fn selection(&self) -> &WorthServerStreamSelection {
        &self.selection
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn performance_receipt(&self) -> &WorthServerStreamingPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn file_envelope(&self) -> &WorthServerCompatibilityFileEnvelope {
        &self.file_envelope
    }

    pub fn certification_bundle(&self) -> &WorthServerBinaryCertificationBundle {
        &self.certification_bundle
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
