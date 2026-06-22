use crate::{
    ForgeServerBinaryCertificationBundle, ForgeServerCompatibilityFileEnvelope,
    ForgeServerCompatibilityRead, ForgeServerFileTransferDisposition,
    ForgeServerOperationAdmissionPosture, ForgeServerReadValidator,
};

use super::super::project_binary_egress_envelope;

use super::{
    ForgeServerBinaryDownloadRequest, ForgeServerBinaryEgressPerformanceReceipt,
    ForgeServerBinaryIntegrityDigest, ForgeServerBinaryRetryPosture,
    ForgeServerConditionalRangeRequest, ForgeServerRangeRequest,
};

#[derive(Debug)]
pub struct ForgeServerBinaryEgressSession {
    operation_admission: ForgeServerOperationAdmissionPosture,
    read: ForgeServerCompatibilityRead,
    download_request: ForgeServerBinaryDownloadRequest,
    range_request: ForgeServerRangeRequest,
    conditional_range_request: ForgeServerConditionalRangeRequest,
    selected_start: usize,
    selected_end_exclusive: usize,
    range_honored: bool,
    head_only: bool,
    retry_posture: ForgeServerBinaryRetryPosture,
    canonical_digest: String,
}

impl ForgeServerBinaryEgressSession {
    pub(crate) fn new(
        operation_admission: ForgeServerOperationAdmissionPosture,
        read: ForgeServerCompatibilityRead,
        download_request: ForgeServerBinaryDownloadRequest,
        range_request: ForgeServerRangeRequest,
        conditional_range_request: ForgeServerConditionalRangeRequest,
        selected_start: usize,
        selected_end_exclusive: usize,
        range_honored: bool,
        head_only: bool,
        retry_posture: ForgeServerBinaryRetryPosture,
    ) -> Self {
        let canonical_digest = format!(
            "compat-http-binary-egress-session-v2|authority={}|read={}|download={}|range={}|conditional={}|selected={}-{}|range_honored={}|head_only={}|retry={}",
            operation_admission.canonical_digest(),
            read.canonical_digest(),
            download_request.canonical_digest(),
            range_request.canonical_digest(),
            conditional_range_request.canonical_digest(),
            selected_start,
            selected_end_exclusive,
            range_honored,
            head_only,
            retry_posture.canonical_digest(),
        );
        Self {
            operation_admission,
            read,
            download_request,
            range_request,
            conditional_range_request,
            selected_start,
            selected_end_exclusive,
            range_honored,
            head_only,
            retry_posture,
            canonical_digest,
        }
    }

    pub fn read(&self) -> &ForgeServerCompatibilityRead {
        &self.read
    }

    pub fn operation_admission(&self) -> &ForgeServerOperationAdmissionPosture {
        &self.operation_admission
    }

    pub fn download_request(&self) -> &ForgeServerBinaryDownloadRequest {
        &self.download_request
    }

    pub fn range_request(&self) -> &ForgeServerRangeRequest {
        &self.range_request
    }

    pub fn conditional_range_request(&self) -> &ForgeServerConditionalRangeRequest {
        &self.conditional_range_request
    }

    pub fn validator(&self) -> &ForgeServerReadValidator {
        self.read.validator()
    }

    pub fn selected_start(&self) -> usize {
        self.selected_start
    }

    pub fn selected_end_exclusive(&self) -> usize {
        self.selected_end_exclusive
    }

    pub fn selected_len(&self) -> usize {
        self.selected_end_exclusive - self.selected_start
    }

    pub fn range_honored(&self) -> bool {
        self.range_honored
    }

    pub fn head_only(&self) -> bool {
        self.head_only
    }

    pub fn retry_posture(&self) -> &ForgeServerBinaryRetryPosture {
        &self.retry_posture
    }

    pub fn content_range(&self) -> Option<String> {
        if !self.range_honored {
            return None;
        }
        Some(format!(
            "bytes {}-{}/{}",
            self.selected_start,
            self.selected_end_exclusive.saturating_sub(1),
            self.download_request.body_bytes().len(),
        ))
    }

    pub fn selected_body_bytes(&self) -> &[u8] {
        &self.download_request.body_bytes()[self.selected_start..self.selected_end_exclusive]
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Debug)]
pub struct ForgeServerBinaryDownload {
    session: ForgeServerBinaryEgressSession,
    payload_bytes: Vec<u8>,
    integrity_digest: ForgeServerBinaryIntegrityDigest,
    performance_receipt: ForgeServerBinaryEgressPerformanceReceipt,
    file_envelope: ForgeServerCompatibilityFileEnvelope,
    certification_bundle: ForgeServerBinaryCertificationBundle,
    canonical_digest: String,
}

impl ForgeServerBinaryDownload {
    pub(crate) fn new(
        session: ForgeServerBinaryEgressSession,
        payload_bytes: Vec<u8>,
        integrity_digest: ForgeServerBinaryIntegrityDigest,
        performance_receipt: ForgeServerBinaryEgressPerformanceReceipt,
        certification_bundle: ForgeServerBinaryCertificationBundle,
    ) -> Self {
        let file_envelope = project_binary_egress_envelope(
            session.read(),
            Some(session.download_request().content_type().to_string()),
            payload_bytes.len() as u64,
            session.range_honored(),
            if session.head_only() {
                ForgeServerFileTransferDisposition::HeadOnlyEgress
            } else {
                ForgeServerFileTransferDisposition::SelectedEgress
            },
        );
        let canonical_digest = format!(
            "compat-http-binary-download-v3|session={}|payload_bytes={}|integrity={}|performance={}|file_envelope={}|certification={}",
            session.canonical_digest(),
            payload_bytes.len(),
            integrity_digest.canonical_digest(),
            performance_digest(&performance_receipt),
            file_envelope.canonical_digest(),
            certification_bundle.canonical_digest(),
        );
        Self {
            session,
            payload_bytes,
            integrity_digest,
            performance_receipt,
            file_envelope,
            certification_bundle,
            canonical_digest,
        }
    }

    pub fn session(&self) -> &ForgeServerBinaryEgressSession {
        &self.session
    }

    pub fn read(&self) -> &ForgeServerCompatibilityRead {
        self.session.read()
    }

    pub fn payload_bytes(&self) -> &[u8] {
        &self.payload_bytes
    }

    pub fn integrity_digest(&self) -> &ForgeServerBinaryIntegrityDigest {
        &self.integrity_digest
    }

    pub fn performance_receipt(&self) -> &ForgeServerBinaryEgressPerformanceReceipt {
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

fn performance_digest(receipt: &ForgeServerBinaryEgressPerformanceReceipt) -> String {
    receipt
        .receipt()
        .counter_rows()
        .iter()
        .map(|row| format!("{}={}", row.name().as_str(), row.observed_count()))
        .collect::<Vec<_>>()
        .join("|")
}
