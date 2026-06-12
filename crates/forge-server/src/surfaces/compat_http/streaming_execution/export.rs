use crate::{
    ForgeServerBinaryCertificationBundle, ForgeServerCompatibilityFileEnvelope,
    ForgeServerCompatibilityRead,
};

use super::super::project_binary_egress_envelope;
use super::{performance::ForgeServerStreamingPerformanceReceipt, ForgeServerStreamSelection};

#[derive(Debug)]
pub struct ForgeServerCompatibilityExport {
    read: ForgeServerCompatibilityRead,
    payload_bytes: Vec<u8>,
    estimated_payload_bytes: usize,
    selection: ForgeServerStreamSelection,
    performance_receipt: ForgeServerStreamingPerformanceReceipt,
    file_envelope: ForgeServerCompatibilityFileEnvelope,
    certification_bundle: ForgeServerBinaryCertificationBundle,
    canonical_digest: String,
}

impl ForgeServerCompatibilityExport {
    pub(crate) fn new(
        read: ForgeServerCompatibilityRead,
        payload_bytes: Vec<u8>,
        estimated_payload_bytes: usize,
        selection: ForgeServerStreamSelection,
        performance_receipt: ForgeServerStreamingPerformanceReceipt,
        certification_bundle: ForgeServerBinaryCertificationBundle,
    ) -> Self {
        let file_envelope = project_binary_egress_envelope(
            &read,
            Some("application/json".to_string()),
            payload_bytes.len() as u64,
            false,
            crate::ForgeServerFileTransferDisposition::SelectedEgress,
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

    pub fn file_envelope(&self) -> &ForgeServerCompatibilityFileEnvelope {
        &self.file_envelope
    }

    pub fn certification_bundle(&self) -> &ForgeServerBinaryCertificationBundle {
        &self.certification_bundle
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
    file_envelope: ForgeServerCompatibilityFileEnvelope,
    certification_bundle: ForgeServerBinaryCertificationBundle,
    canonical_digest: String,
}

impl ForgeServerBackgroundExportRequest {
    pub(crate) fn new(
        read: ForgeServerCompatibilityRead,
        estimated_payload_bytes: usize,
        selection: ForgeServerStreamSelection,
        detail: impl Into<String>,
        performance_receipt: ForgeServerStreamingPerformanceReceipt,
        certification_bundle: ForgeServerBinaryCertificationBundle,
    ) -> Self {
        let detail = detail.into();
        let file_envelope = project_binary_egress_envelope(
            &read,
            Some("application/json".to_string()),
            0,
            false,
            crate::ForgeServerFileTransferDisposition::MetadataOnlyObservation,
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

    pub fn file_envelope(&self) -> &ForgeServerCompatibilityFileEnvelope {
        &self.file_envelope
    }

    pub fn certification_bundle(&self) -> &ForgeServerBinaryCertificationBundle {
        &self.certification_bundle
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
