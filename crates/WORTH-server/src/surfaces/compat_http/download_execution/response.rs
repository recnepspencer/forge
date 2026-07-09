use crate::{
    WorthServerBinaryCertificationBundle, WorthServerCompatibilityFileEnvelope,
    WorthServerCompatibilityRead, WorthServerFileTransferDisposition,
    WorthServerOperationAdmissionPosture, WorthServerReadValidator,
};

use super::super::project_binary_egress_envelope;

use super::{
    WorthServerBinaryDownloadRequest, WorthServerBinaryEgressPerformanceReceipt,
    WorthServerBinaryIntegrityDigest, WorthServerBinaryRetryPosture,
    WorthServerConditionalRangeRequest, WorthServerRangeRequest,
};

#[derive(Debug)]
pub struct WorthServerBinaryEgressSession {
    operation_admission: WorthServerOperationAdmissionPosture,
    read: WorthServerCompatibilityRead,
    download_request: WorthServerBinaryDownloadRequest,
    range_request: WorthServerRangeRequest,
    conditional_range_request: WorthServerConditionalRangeRequest,
    selected_start: usize,
    selected_end_exclusive: usize,
    range_honored: bool,
    head_only: bool,
    retry_posture: WorthServerBinaryRetryPosture,
    canonical_digest: String,
}

impl WorthServerBinaryEgressSession {
    pub(crate) fn new(
        operation_admission: WorthServerOperationAdmissionPosture,
        read: WorthServerCompatibilityRead,
        download_request: WorthServerBinaryDownloadRequest,
        range_request: WorthServerRangeRequest,
        conditional_range_request: WorthServerConditionalRangeRequest,
        selected_start: usize,
        selected_end_exclusive: usize,
        range_honored: bool,
        head_only: bool,
        retry_posture: WorthServerBinaryRetryPosture,
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

    pub fn read(&self) -> &WorthServerCompatibilityRead {
        &self.read
    }

    pub fn operation_admission(&self) -> &WorthServerOperationAdmissionPosture {
        &self.operation_admission
    }

    pub fn download_request(&self) -> &WorthServerBinaryDownloadRequest {
        &self.download_request
    }

    pub fn range_request(&self) -> &WorthServerRangeRequest {
        &self.range_request
    }

    pub fn conditional_range_request(&self) -> &WorthServerConditionalRangeRequest {
        &self.conditional_range_request
    }

    pub fn validator(&self) -> &WorthServerReadValidator {
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

    pub fn retry_posture(&self) -> &WorthServerBinaryRetryPosture {
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
pub struct WorthServerBinaryDownload {
    session: WorthServerBinaryEgressSession,
    payload_bytes: Vec<u8>,
    integrity_digest: WorthServerBinaryIntegrityDigest,
    performance_receipt: WorthServerBinaryEgressPerformanceReceipt,
    file_envelope: WorthServerCompatibilityFileEnvelope,
    certification_bundle: WorthServerBinaryCertificationBundle,
    canonical_digest: String,
}

impl WorthServerBinaryDownload {
    pub(crate) fn new(
        session: WorthServerBinaryEgressSession,
        payload_bytes: Vec<u8>,
        integrity_digest: WorthServerBinaryIntegrityDigest,
        performance_receipt: WorthServerBinaryEgressPerformanceReceipt,
        certification_bundle: WorthServerBinaryCertificationBundle,
    ) -> Self {
        let file_envelope = project_binary_egress_envelope(
            session.read(),
            Some(session.download_request().content_type().to_string()),
            payload_bytes.len() as u64,
            session.range_honored(),
            if session.head_only() {
                WorthServerFileTransferDisposition::HeadOnlyEgress
            } else {
                WorthServerFileTransferDisposition::SelectedEgress
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

    pub fn session(&self) -> &WorthServerBinaryEgressSession {
        &self.session
    }

    pub fn read(&self) -> &WorthServerCompatibilityRead {
        self.session.read()
    }

    pub fn payload_bytes(&self) -> &[u8] {
        &self.payload_bytes
    }

    pub fn integrity_digest(&self) -> &WorthServerBinaryIntegrityDigest {
        &self.integrity_digest
    }

    pub fn performance_receipt(&self) -> &WorthServerBinaryEgressPerformanceReceipt {
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

fn performance_digest(receipt: &WorthServerBinaryEgressPerformanceReceipt) -> String {
    receipt
        .receipt()
        .counter_rows()
        .iter()
        .map(|row| format!("{}={}", row.name().as_str(), row.observed_count()))
        .collect::<Vec<_>>()
        .join("|")
}
