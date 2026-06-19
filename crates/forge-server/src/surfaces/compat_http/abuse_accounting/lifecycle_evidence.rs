use forge_foundational::facade::{
    attachment, BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator, BoundaryHandle,
    EquivalenceBasisId, FoundationalBoundaryEvidenceAttachmentBundle,
    FoundationalBoundaryEvidenceLineageSubject, FoundationalBoundaryEvidenceReceiptBoundary,
    FoundationalBoundaryEvidenceReceiptFrontDoor,
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportFrontDoor, FoundationalCommitId,
    FoundationalCommitParentBasis, FoundationalCommitParentageLocator,
    FoundationalTransitionLocator,
};

use crate::{
    ForgeServerBinaryCounterSet, ForgeServerBinaryDownload, ForgeServerCompatHttpRouteFamily,
    ForgeServerExternalCounterSet, ForgeServerStreamCancellationKind,
    ForgeServerStreamCancellationReceipt, ForgeServerUploadCleanupReason,
    ForgeServerUploadCleanupReceipt,
};

use super::{
    budget_receipt::ForgeServerTransferByteClass,
    counters::{
        binary_counter_set, external_counter_set, BACKPRESSURE_ABORTS, CALLER_CANCELLATIONS,
        CLEANUP_OPERATIONS, CLEANUP_STAGED_BYTES, DISCONNECT_EVENTS, EXPIRY_EVENTS, RETRY_EVENTS,
        SEMANTIC_TRUTH_DRIFT, SLOWLORIS_CUTOFFS, STAGED_CLEANUP_EVENTS,
    },
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerTransferCleanupReason {
    ClientDisconnect,
    DownstreamBackpressure,
    CallerCancelled,
    DownloadRetryAdmitted,
    UploadInterrupted,
    UploadExpired,
    UploadOwnershipMismatch,
    UploadAbandoned,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerTransferCleanupEvidence {
    route_family: ForgeServerCompatHttpRouteFamily,
    byte_class: ForgeServerTransferByteClass,
    reason: ForgeServerTransferCleanupReason,
    tenant_id: String,
    workspace_digest: String,
    branch_digest: String,
    detail: String,
    attachment_bundle: FoundationalBoundaryEvidenceAttachmentBundle,
    external_counters: ForgeServerExternalCounterSet,
    binary_counters: ForgeServerBinaryCounterSet,
    canonical_digest: String,
}

impl ForgeServerTransferCleanupEvidence {
    pub fn route_family(&self) -> ForgeServerCompatHttpRouteFamily {
        self.route_family
    }

    pub fn byte_class(&self) -> ForgeServerTransferByteClass {
        self.byte_class
    }

    pub fn reason(&self) -> ForgeServerTransferCleanupReason {
        self.reason
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

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn attachment_bundle(&self) -> &FoundationalBoundaryEvidenceAttachmentBundle {
        &self.attachment_bundle
    }

    pub fn external_counters(&self) -> &ForgeServerExternalCounterSet {
        &self.external_counters
    }

    pub fn binary_counters(&self) -> &ForgeServerBinaryCounterSet {
        &self.binary_counters
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

impl ForgeServerStreamCancellationReceipt {
    pub fn transfer_cleanup_evidence(&self) -> ForgeServerTransferCleanupEvidence {
        let reason = match self.kind() {
            ForgeServerStreamCancellationKind::ClientDisconnect => {
                ForgeServerTransferCleanupReason::ClientDisconnect
            }
            ForgeServerStreamCancellationKind::DownstreamBackpressure => {
                ForgeServerTransferCleanupReason::DownstreamBackpressure
            }
            ForgeServerStreamCancellationKind::CallerCancelled => {
                ForgeServerTransferCleanupReason::CallerCancelled
            }
        };
        let external_rows = [
            (SLOWLORIS_CUTOFFS, 0),
            (
                DISCONNECT_EVENTS,
                u64::from(matches!(
                    reason,
                    ForgeServerTransferCleanupReason::ClientDisconnect
                )),
            ),
            (
                BACKPRESSURE_ABORTS,
                u64::from(matches!(
                    self.kind(),
                    ForgeServerStreamCancellationKind::DownstreamBackpressure
                )),
            ),
            (
                CALLER_CANCELLATIONS,
                u64::from(matches!(
                    reason,
                    ForgeServerTransferCleanupReason::CallerCancelled
                )),
            ),
            (RETRY_EVENTS, 0),
            (EXPIRY_EVENTS, 0),
            (STAGED_CLEANUP_EVENTS, 0),
            (SEMANTIC_TRUTH_DRIFT, 0),
        ];
        let binary_rows = [
            (CLEANUP_OPERATIONS, 1),
            (CLEANUP_STAGED_BYTES, 0),
            (SEMANTIC_TRUTH_DRIFT, 0),
        ];
        let provenance = self.transfer_provenance().provenance();
        ForgeServerTransferCleanupEvidence {
            route_family: ForgeServerCompatHttpRouteFamily::Streaming,
            byte_class: ForgeServerTransferByteClass::StructuredPayload,
            reason,
            tenant_id: self.tenant_id().to_string(),
            workspace_digest: self.workspace_digest().to_string(),
            branch_digest: self.branch_digest().to_string(),
            detail: self.detail().to_string(),
            attachment_bundle: attachment_bundle(
                "stream-cancellation",
                provenance,
                self.canonical_digest(),
            ),
            external_counters: external_counter_set(
                "compat_http.transfer.lifecycle.external",
                &external_rows,
            ),
            binary_counters: binary_counter_set(
                "compat_http.transfer.lifecycle.binary",
                &binary_rows,
            ),
            canonical_digest: format!(
                "forge-server-transfer-cleanup-evidence-v1|route=streaming|reason={}|tenant={}|workspace={}|branch={}|detail={}",
                reason.as_str(),
                self.tenant_id(),
                self.workspace_digest(),
                self.branch_digest(),
                self.detail(),
            ),
        }
    }
}

impl ForgeServerBinaryDownload {
    pub fn transfer_cleanup_evidence(&self) -> Option<ForgeServerTransferCleanupEvidence> {
        if !self.session().retry_posture().is_resume() {
            return None;
        }
        let provenance = self.file_envelope().transfer_provenance();
        let external_rows = [
            (SLOWLORIS_CUTOFFS, 0),
            (DISCONNECT_EVENTS, 0),
            (BACKPRESSURE_ABORTS, 0),
            (CALLER_CANCELLATIONS, 0),
            (RETRY_EVENTS, 1),
            (EXPIRY_EVENTS, 0),
            (STAGED_CLEANUP_EVENTS, 0),
            (SEMANTIC_TRUTH_DRIFT, 0),
        ];
        let binary_rows = [
            (CLEANUP_OPERATIONS, 0),
            (CLEANUP_STAGED_BYTES, 0),
            (SEMANTIC_TRUTH_DRIFT, 0),
        ];
        Some(ForgeServerTransferCleanupEvidence {
            route_family: ForgeServerCompatHttpRouteFamily::Download,
            byte_class: if self.session().head_only() {
                ForgeServerTransferByteClass::MetadataOnly
            } else {
                ForgeServerTransferByteClass::BinaryWire
            },
            reason: ForgeServerTransferCleanupReason::DownloadRetryAdmitted,
            tenant_id: provenance.tenant_id().to_string(),
            workspace_digest: provenance.workspace_digest().to_string(),
            branch_digest: provenance.branch_digest().to_string(),
            detail: self
                .session()
                .retry_posture()
                .canonical_digest()
                .to_string(),
            attachment_bundle: attachment_bundle(
                "download-retry",
                provenance.provenance(),
                self.canonical_digest(),
            ),
            external_counters: external_counter_set(
                "compat_http.transfer.lifecycle.external",
                &external_rows,
            ),
            binary_counters: binary_counter_set(
                "compat_http.transfer.lifecycle.binary",
                &binary_rows,
            ),
            canonical_digest: format!(
                "forge-server-transfer-cleanup-evidence-v1|route=download|reason=retry_admitted|tenant={}|workspace={}|branch={}|retry={}",
                provenance.tenant_id(),
                provenance.workspace_digest(),
                provenance.branch_digest(),
                self.session().retry_posture().canonical_digest(),
            ),
        })
    }
}

impl ForgeServerUploadCleanupReceipt {
    pub fn transfer_cleanup_evidence(&self) -> ForgeServerTransferCleanupEvidence {
        let reason = match self.reason() {
            ForgeServerUploadCleanupReason::Interrupted => {
                ForgeServerTransferCleanupReason::UploadInterrupted
            }
            ForgeServerUploadCleanupReason::Expired => {
                ForgeServerTransferCleanupReason::UploadExpired
            }
            ForgeServerUploadCleanupReason::OwnershipMismatch => {
                ForgeServerTransferCleanupReason::UploadOwnershipMismatch
            }
            ForgeServerUploadCleanupReason::Abandoned => {
                ForgeServerTransferCleanupReason::UploadAbandoned
            }
        };
        let cleanup_bytes = self
            .performance_receipt()
            .counter("compat_http.upload.cleanup_staged_bytes")
            .unwrap_or(0);
        let external_rows = [
            (SLOWLORIS_CUTOFFS, 0),
            (DISCONNECT_EVENTS, 0),
            (BACKPRESSURE_ABORTS, 0),
            (CALLER_CANCELLATIONS, 0),
            (RETRY_EVENTS, 0),
            (
                EXPIRY_EVENTS,
                u64::from(matches!(
                    reason,
                    ForgeServerTransferCleanupReason::UploadExpired
                )),
            ),
            (STAGED_CLEANUP_EVENTS, 1),
            (SEMANTIC_TRUTH_DRIFT, 0),
        ];
        let binary_rows = [
            (CLEANUP_OPERATIONS, 1),
            (CLEANUP_STAGED_BYTES, cleanup_bytes),
            (SEMANTIC_TRUTH_DRIFT, 0),
        ];
        let provenance = self.performance_provenance();
        ForgeServerTransferCleanupEvidence {
            route_family: ForgeServerCompatHttpRouteFamily::Upload,
            byte_class: ForgeServerTransferByteClass::BinaryAuthoritative,
            reason,
            tenant_id: self.tenant_id().to_string(),
            workspace_digest: self.workspace_digest().to_string(),
            branch_digest: self.branch_digest().to_string(),
            detail: self.canonical_digest().to_string(),
            attachment_bundle: attachment_bundle(
                "upload-cleanup",
                &provenance,
                self.canonical_digest(),
            ),
            external_counters: external_counter_set(
                "compat_http.transfer.lifecycle.external",
                &external_rows,
            ),
            binary_counters: binary_counter_set(
                "compat_http.transfer.lifecycle.binary",
                &binary_rows,
            ),
            canonical_digest: format!(
                "forge-server-transfer-cleanup-evidence-v1|route=upload|reason={}|tenant={}|workspace={}|branch={}|cleanup_bytes={cleanup_bytes}",
                reason.as_str(),
                self.tenant_id(),
                self.workspace_digest(),
                self.branch_digest(),
            ),
        }
    }
}

impl ForgeServerTransferCleanupReason {
    fn as_str(self) -> &'static str {
        match self {
            Self::ClientDisconnect => "client_disconnect",
            Self::DownstreamBackpressure => "downstream_backpressure",
            Self::CallerCancelled => "caller_cancelled",
            Self::DownloadRetryAdmitted => "download_retry_admitted",
            Self::UploadInterrupted => "upload_interrupted",
            Self::UploadExpired => "upload_expired",
            Self::UploadOwnershipMismatch => "upload_ownership_mismatch",
            Self::UploadAbandoned => "upload_abandoned",
        }
    }
}

fn attachment_bundle(
    kind: &str,
    provenance: &forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
    canonical_digest: &str,
) -> FoundationalBoundaryEvidenceAttachmentBundle {
    let locator = BoundaryArtifactLocator::new(
        BoundaryArtifactId::new(boundary_artifact_id(&[
            "forge-server.transfer-lifecycle".to_string(),
            kind.to_string(),
            canonical_digest.to_string(),
        ])),
        BoundaryArtifactField::Basis,
    );
    let boundary = FoundationalBoundaryEvidenceReceiptBoundary::transition(
        FoundationalTransitionLocator::CommitParentage(FoundationalCommitParentageLocator::new(
            FoundationalCommitId::new(BoundaryHandle::new(boundary_artifact_id(&[
                "forge-server.transfer-lifecycle.commit".to_string(),
                kind.to_string(),
                canonical_digest.to_string(),
            ]))),
            FoundationalCommitParentBasis::new(EquivalenceBasisId::new(boundary_artifact_id(&[
                "forge-server.transfer-lifecycle.parent".to_string(),
                kind.to_string(),
                canonical_digest.to_string(),
            ]))),
        )),
    );
    let executed_receipt = FoundationalBoundaryEvidenceReceiptFrontDoor
        .execution(boundary)
        .with_provenance(provenance.clone());
    let support = FoundationalBoundaryEvidenceSupportFrontDoor
        .transient_lifecycle(FoundationalBoundaryEvidenceLineageSubject::new(
            BoundaryHandle::new(boundary_artifact_id(&[
                "forge-server.transfer-lifecycle.subject".to_string(),
                kind.to_string(),
                canonical_digest.to_string(),
            ])),
        ))
        .with_basis_disclosure(FoundationalBoundaryEvidenceSupportBasisDisclosure::CompleteBasis)
        .opened_and_closed_within(executed_receipt.clone());
    attachment()
        .for_boundary_artifact(locator)
        .with_provenance_attachment(provenance.clone())
        .with_receipt_attachment(executed_receipt.completed_receipt().clone())
        .with_transient_lifecycle_support(support)
}

fn boundary_artifact_id(parts: &[String]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0x1f;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
