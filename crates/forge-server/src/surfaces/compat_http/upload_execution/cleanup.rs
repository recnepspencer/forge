use forge_foundational::facade::{
    BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceProvenanceFrontDoor, FoundationalBoundaryEvidenceSourceBasis,
};
use forge_proof::TransitionOutcome;

use crate::ForgeServerCompatibilityPreparedRequest;

use super::{
    lifecycle::ForgeServerStoredBinaryIngress,
    performance::{
        ForgeServerIngressMetricSnapshot, ForgeServerIngressPerformanceReceipt, CLEANUP_OPERATIONS,
        CLEANUP_STAGED_BYTES,
    },
    session::ForgeServerBinaryIngressSession,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerUploadCleanupReason {
    Interrupted,
    Expired,
    OwnershipMismatch,
    Abandoned,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerUploadCleanupReceipt {
    reason: ForgeServerUploadCleanupReason,
    session_digest: String,
    tenant_id: String,
    workspace_digest: String,
    branch_digest: String,
    truth_drift_free: bool,
    performance_provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    performance_receipt: ForgeServerIngressPerformanceReceipt,
    canonical_digest: String,
}

impl ForgeServerUploadCleanupReceipt {
    pub(crate) fn new(
        stored_ingress: &ForgeServerStoredBinaryIngress,
        reason: ForgeServerUploadCleanupReason,
    ) -> Self {
        let staged_bytes = stored_ingress.staged_authoritative_bytes();
        let performance_receipt = ForgeServerIngressPerformanceReceipt::build(
            ForgeServerIngressMetricSnapshot {
                cleanup_operations: 1,
                cleanup_staged_bytes: staged_bytes,
                ..ForgeServerIngressMetricSnapshot::default()
            },
            "compat_http.upload.cleanup",
        )
        .expect("static upload cleanup counters should be valid");
        let performance_provenance = build_cleanup_provenance(stored_ingress);
        let canonical_digest = format!(
            "forge-server-upload-cleanup-receipt-v1|reason={reason:?}|session={}|tenant={}|workspace={}|branch={}|truth_drift_free=true|cleanup_ops={}|cleanup_bytes={}",
            stored_ingress.session_digest(),
            stored_ingress.tenant_id(),
            stored_ingress.workspace_digest(),
            stored_ingress.branch_digest(),
            performance_receipt.counter(CLEANUP_OPERATIONS).unwrap_or(0),
            performance_receipt.counter(CLEANUP_STAGED_BYTES).unwrap_or(0),
        );
        Self {
            reason,
            session_digest: stored_ingress.session_digest().to_string(),
            tenant_id: stored_ingress.tenant_id().to_string(),
            workspace_digest: stored_ingress.workspace_digest().to_string(),
            branch_digest: stored_ingress.branch_digest().to_string(),
            truth_drift_free: true,
            performance_provenance,
            performance_receipt,
            canonical_digest,
        }
    }

    pub fn reason(&self) -> ForgeServerUploadCleanupReason {
        self.reason
    }

    pub fn session_digest(&self) -> &str {
        &self.session_digest
    }

    pub fn workspace_digest(&self) -> &str {
        &self.workspace_digest
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn branch_digest(&self) -> &str {
        &self.branch_digest
    }

    pub fn truth_drift_free(&self) -> bool {
        self.truth_drift_free
    }

    pub fn performance_receipt(&self) -> &ForgeServerIngressPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn performance_provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.performance_provenance
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

fn build_cleanup_provenance(
    stored_ingress: &ForgeServerStoredBinaryIngress,
) -> FoundationalBoundaryEvidenceProvenanceArtifact {
    let source_basis =
        FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(BoundaryArtifactLocator::new(
            BoundaryArtifactId::new(boundary_artifact_id(&[
                "forge-server.upload-cleanup".to_string(),
                stored_ingress.session_digest().to_string(),
                stored_ingress.workspace_digest().to_string(),
                stored_ingress.branch_digest().to_string(),
            ])),
            BoundaryArtifactField::Basis,
        ));
    match FoundationalBoundaryEvidenceProvenanceFrontDoor
        .branch_local(source_basis)
        .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained)
    {
        TransitionOutcome::Success(provenance) => provenance,
        outcome => panic!("upload cleanup provenance should stay admitted: {outcome:?}"),
    }
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

pub(crate) fn ownership_matches(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    session: &ForgeServerBinaryIngressSession,
) -> bool {
    let request_context = prepared_request.admission().request_context();
    request_context.workspace_target().tenant_id() == session.tenant_id()
        && request_context.workspace_target().workspace_digest() == session.workspace_digest()
        && request_context.branch_target().branch_digest() == session.branch_digest()
}
