use crate::{
    ForgeServerCompatibilityMutationRequest, ForgeServerCompatibilityPreparedRequest,
    ForgeServerMultipartUpload, ForgeServerOperationAdmissionPosture,
};

use super::{
    integrity::ForgeServerIngressIntegrityDigest, performance::ForgeServerIngressPerformanceReceipt,
};

#[derive(Clone, Debug)]
pub struct ForgeServerBinaryIngressSession {
    prepared_request: ForgeServerCompatibilityPreparedRequest,
    operation_name: String,
    upload: ForgeServerMultipartUpload,
    mutation_request: ForgeServerCompatibilityMutationRequest,
    operation_admission: ForgeServerOperationAdmissionPosture,
    tenant_id: String,
    workspace_digest: String,
    branch_digest: String,
    session_digest: String,
    performance_receipt: ForgeServerIngressPerformanceReceipt,
}

impl ForgeServerBinaryIngressSession {
    pub(crate) fn new(
        prepared_request: ForgeServerCompatibilityPreparedRequest,
        operation_name: String,
        upload: ForgeServerMultipartUpload,
        mutation_request: ForgeServerCompatibilityMutationRequest,
        operation_admission: ForgeServerOperationAdmissionPosture,
        performance_receipt: ForgeServerIngressPerformanceReceipt,
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
            "forge-server-binary-ingress-session-v2|tenant={}|workspace={}|branch={}|operation={}|upload={}|authority={}",
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

    pub fn prepared_request(&self) -> &ForgeServerCompatibilityPreparedRequest {
        &self.prepared_request
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn upload(&self) -> &ForgeServerMultipartUpload {
        &self.upload
    }

    pub fn mutation_request(&self) -> &ForgeServerCompatibilityMutationRequest {
        &self.mutation_request
    }

    pub fn operation_admission(&self) -> &ForgeServerOperationAdmissionPosture {
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

    pub fn performance_receipt(&self) -> &ForgeServerIngressPerformanceReceipt {
        &self.performance_receipt
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeServerCompatibilityPreparedRequest,
        String,
        ForgeServerMultipartUpload,
        ForgeServerCompatibilityMutationRequest,
        ForgeServerOperationAdmissionPosture,
        String,
        String,
        String,
        String,
        ForgeServerIngressPerformanceReceipt,
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
pub struct ForgeServerVerifiedBinaryIngress {
    session: ForgeServerBinaryIngressSession,
    integrity_digest: ForgeServerIngressIntegrityDigest,
}

impl ForgeServerVerifiedBinaryIngress {
    pub(crate) fn new(
        session: ForgeServerBinaryIngressSession,
        integrity_digest: ForgeServerIngressIntegrityDigest,
    ) -> Self {
        Self {
            session,
            integrity_digest,
        }
    }

    pub fn session(&self) -> &ForgeServerBinaryIngressSession {
        &self.session
    }

    pub fn integrity_digest(&self) -> &ForgeServerIngressIntegrityDigest {
        &self.integrity_digest
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeServerBinaryIngressSession,
        ForgeServerIngressIntegrityDigest,
    ) {
        (self.session, self.integrity_digest)
    }
}
