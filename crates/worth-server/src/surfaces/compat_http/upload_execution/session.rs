use crate::{
    WorthServerCompatibilityMutationRequest, WorthServerCompatibilityPreparedRequest,
    WorthServerMultipartUpload, WorthServerOperationAdmissionPosture,
};

use super::{
    integrity::WorthServerIngressIntegrityDigest, performance::WorthServerIngressPerformanceReceipt,
};

#[derive(Clone, Debug)]
pub struct WorthServerBinaryIngressSession {
    prepared_request: WorthServerCompatibilityPreparedRequest,
    operation_name: String,
    upload: WorthServerMultipartUpload,
    mutation_request: WorthServerCompatibilityMutationRequest,
    operation_admission: WorthServerOperationAdmissionPosture,
    tenant_id: String,
    workspace_digest: String,
    branch_digest: String,
    session_digest: String,
    performance_receipt: WorthServerIngressPerformanceReceipt,
}

impl WorthServerBinaryIngressSession {
    pub(crate) fn new(
        prepared_request: WorthServerCompatibilityPreparedRequest,
        operation_name: String,
        upload: WorthServerMultipartUpload,
        mutation_request: WorthServerCompatibilityMutationRequest,
        operation_admission: WorthServerOperationAdmissionPosture,
        performance_receipt: WorthServerIngressPerformanceReceipt,
    ) -> Self {
        let tenant_id = prepared_request
            .admission()
            .request_context()
            .workspace_target()
            .tenant_id()
            .to_string();
        let workspace_digest = prepared_request
            .admission()
            .request_context()
            .workspace_target()
            .workspace_digest();
        let branch_digest = prepared_request
            .admission()
            .request_context()
            .branch_target()
            .branch_digest();
        let session_digest = format!(
            "worth-server-binary-ingress-session-v2|tenant={}|workspace={}|branch={}|operation={}|upload={}|authority={}",
            tenant_id,
            workspace_digest,
            branch_digest,
            operation_name.trim(),
            upload.canonical_digest(),
            operation_admission.canonical_digest(),
        );
        Self {
            prepared_request,
            operation_name,
            upload,
            mutation_request,
            operation_admission,
            tenant_id,
            workspace_digest,
            branch_digest,
            session_digest,
            performance_receipt,
        }
    }

    pub fn prepared_request(&self) -> &WorthServerCompatibilityPreparedRequest {
        &self.prepared_request
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn upload(&self) -> &WorthServerMultipartUpload {
        &self.upload
    }

    pub fn mutation_request(&self) -> &WorthServerCompatibilityMutationRequest {
        &self.mutation_request
    }

    pub fn operation_admission(&self) -> &WorthServerOperationAdmissionPosture {
        &self.operation_admission
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

    pub fn session_digest(&self) -> &str {
        &self.session_digest
    }

    pub fn performance_receipt(&self) -> &WorthServerIngressPerformanceReceipt {
        &self.performance_receipt
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthServerCompatibilityPreparedRequest,
        String,
        WorthServerMultipartUpload,
        WorthServerCompatibilityMutationRequest,
        WorthServerOperationAdmissionPosture,
        String,
        String,
        String,
        String,
        WorthServerIngressPerformanceReceipt,
    ) {
        (
            self.prepared_request,
            self.operation_name,
            self.upload,
            self.mutation_request,
            self.operation_admission,
            self.tenant_id,
            self.workspace_digest,
            self.branch_digest,
            self.session_digest,
            self.performance_receipt,
        )
    }
}

#[derive(Clone, Debug)]
pub struct WorthServerVerifiedBinaryIngress {
    session: WorthServerBinaryIngressSession,
    integrity_digest: WorthServerIngressIntegrityDigest,
}

impl WorthServerVerifiedBinaryIngress {
    pub(crate) fn new(
        session: WorthServerBinaryIngressSession,
        integrity_digest: WorthServerIngressIntegrityDigest,
    ) -> Self {
        Self {
            session,
            integrity_digest,
        }
    }

    pub fn session(&self) -> &WorthServerBinaryIngressSession {
        &self.session
    }

    pub fn integrity_digest(&self) -> &WorthServerIngressIntegrityDigest {
        &self.integrity_digest
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthServerBinaryIngressSession,
        WorthServerIngressIntegrityDigest,
    ) {
        (self.session, self.integrity_digest)
    }
}
