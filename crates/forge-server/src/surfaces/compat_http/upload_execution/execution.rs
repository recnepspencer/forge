use forge_proof::TransitionOutcome;

use crate::{
    ForgeServerBinaryCertificationBundle, ForgeServerCompatibilityFacade,
    ForgeServerCompatibilityFileEnvelope, ForgeServerCompatibilityPreparedRequest,
    ForgeServerQueryHandoffDeferred, ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffFailure,
    ForgeServerQueryHandoffRebindRequired, ForgeServerQueryHandoffStale,
};

use super::super::{
    abuse_accounting::{
        denied_budget_receipt_for_prepared_request, ForgeServerAbuseBudgetDenialClass,
        ForgeServerTransferByteClass,
    },
    build_upload_certification_bundle, project_upload_envelope,
};

use super::{
    admission::validate_upload_admission,
    cleanup::{ownership_matches, ForgeServerUploadCleanupReason, ForgeServerUploadCleanupReceipt},
    ingress_policy::admit_binary_ingress,
    integrity::ForgeServerIngressIntegrityDigest,
    lifecycle::{
        cleanup_active_session, remove_active_session, require_active_session,
        stage_binary_ingress, upload_request_invalid,
    },
    performance::ForgeServerIngressPerformanceReceipt,
    request::ForgeServerMultipartUpload,
    session::{ForgeServerBinaryIngressSession, ForgeServerVerifiedBinaryIngress},
};

pub type ForgeServerCompatibilityUploadOutcome<T> = TransitionOutcome<
    T,
    ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDeferred,
    ForgeServerQueryHandoffStale,
    ForgeServerQueryHandoffRebindRequired,
    ForgeServerQueryHandoffFailure,
>;

#[derive(Clone, Debug)]
pub struct ForgeServerCompatibilityUploadExecutionInput {
    prepared_request: ForgeServerCompatibilityPreparedRequest,
    operation_name: String,
    upload: ForgeServerMultipartUpload,
}

impl ForgeServerCompatibilityUploadExecutionInput {
    pub fn new(
        prepared_request: ForgeServerCompatibilityPreparedRequest,
        operation_name: impl Into<String>,
        upload: ForgeServerMultipartUpload,
    ) -> Self {
        Self {
            prepared_request,
            operation_name: operation_name.into().trim().to_string(),
            upload,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ForgeServerPreparedMultipartUpload {
    prepared_request: ForgeServerCompatibilityPreparedRequest,
    operation_name: String,
    upload: ForgeServerMultipartUpload,
    mutation_request: crate::ForgeServerCompatibilityMutationRequest,
}

impl ForgeServerPreparedMultipartUpload {
    pub fn prepared_request(&self) -> &ForgeServerCompatibilityPreparedRequest {
        &self.prepared_request
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn upload(&self) -> &ForgeServerMultipartUpload {
        &self.upload
    }

    pub fn mutation_request(&self) -> &crate::ForgeServerCompatibilityMutationRequest {
        &self.mutation_request
    }

    pub fn requires_early_admission(&self) -> bool {
        self.upload.expectation().requires_early_admission()
    }

    fn into_parts(
        self,
    ) -> (
        ForgeServerCompatibilityPreparedRequest,
        String,
        ForgeServerMultipartUpload,
        crate::ForgeServerCompatibilityMutationRequest,
    ) {
        (
            self.prepared_request,
            self.operation_name,
            self.upload,
            self.mutation_request,
        )
    }
}

impl ForgeServerCompatibilityFacade {
    pub fn prepare_upload(
        &self,
        input: ForgeServerCompatibilityUploadExecutionInput,
    ) -> ForgeServerCompatibilityUploadOutcome<ForgeServerPreparedMultipartUpload> {
        if let Err(denial) = validate_upload_admission(
            &input.prepared_request,
            &input.upload,
            &input.operation_name,
        ) {
            return TransitionOutcome::Denied(denial);
        }
        let diagnostics_profile = input
            .prepared_request
            .admission()
            .request_context()
            .diagnostics_profile();
        let mutation_request =
            match parse_upload_mutation_request(&input.upload, diagnostics_profile) {
                Ok(value) => value,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        TransitionOutcome::Success(ForgeServerPreparedMultipartUpload {
            prepared_request: input.prepared_request,
            operation_name: input.operation_name,
            upload: input.upload,
            mutation_request,
        })
    }

    pub fn begin_binary_ingress(
        &self,
        prepared: ForgeServerPreparedMultipartUpload,
    ) -> ForgeServerCompatibilityUploadOutcome<ForgeServerBinaryIngressSession> {
        let (prepared_request, operation_name, upload, mutation_request) = prepared.into_parts();
        let diagnostics_profile = prepared_request
            .admission()
            .request_context()
            .diagnostics_profile();
        let metrics = match admit_binary_ingress(&upload, diagnostics_profile) {
            Ok(value) => value,
            Err(denial) => {
                let detail = denial.detail().to_string();
                return TransitionOutcome::Denied(denial.with_abuse_budget_receipt(
                    denied_budget_receipt_for_prepared_request(
                        &prepared_request,
                        ForgeServerTransferByteClass::BinaryWire,
                        detail,
                        ForgeServerAbuseBudgetDenialClass::SlowlorisCutoff,
                    ),
                ));
            }
        };
        let performance_receipt = match ForgeServerIngressPerformanceReceipt::build(
            metrics,
            "compat_http.upload.ingress",
        ) {
            Ok(value) => value,
            Err(_) => {
                return TransitionOutcome::Failed(ForgeServerQueryHandoffFailure::new(
                    "compatibility_upload_ingress_performance_receipt_failed",
                ))
            }
        };
        let session = ForgeServerBinaryIngressSession::new(
            prepared_request,
            operation_name,
            upload,
            mutation_request,
            performance_receipt,
        );
        if let Err(denial) = stage_binary_ingress(self, &session) {
            return TransitionOutcome::Denied(denial);
        }
        TransitionOutcome::Success(session)
    }

    pub fn verify_binary_ingress(
        &self,
        session: ForgeServerBinaryIngressSession,
    ) -> ForgeServerCompatibilityUploadOutcome<ForgeServerVerifiedBinaryIngress> {
        if let Err(denial) = require_active_session(self, &session) {
            return TransitionOutcome::Denied(denial);
        }
        let diagnostics_profile = session
            .prepared_request()
            .admission()
            .request_context()
            .diagnostics_profile();
        let integrity_digest = match ForgeServerIngressIntegrityDigest::verify(
            session.upload(),
            diagnostics_profile,
        ) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        TransitionOutcome::Success(ForgeServerVerifiedBinaryIngress::new(
            session,
            integrity_digest,
        ))
    }

    pub fn finalize_upload(
        &self,
        verified: ForgeServerVerifiedBinaryIngress,
    ) -> ForgeServerCompatibilityUploadOutcome<ForgeServerCompatibilityUpload> {
        let (session, integrity_digest) = verified.into_parts();
        if let Err(denial) = require_active_session(self, &session) {
            return TransitionOutcome::Denied(denial);
        }
        let (
            prepared_request,
            operation_name,
            upload,
            mutation_request,
            _tenant_id,
            _workspace_digest,
            _branch_digest,
            _session_digest,
            performance_receipt,
        ) = session.clone().into_parts();
        let outcome =
            crate::surfaces::compat_http::mutation_execution::execute_compatibility_mutation_request(
                self,
                prepared_request,
                operation_name,
                mutation_request,
            )
            .map_success(|mutation| {
                let file_envelope = project_upload_envelope(&session, &mutation);
                let certification_bundle = build_upload_certification_bundle(
                    &self.operator_evidence,
                    mutation.envelope().support_posture(),
                    &file_envelope,
                    mutation.envelope().response_envelope(),
                    &performance_receipt,
                );
                ForgeServerCompatibilityUpload::new(
                    upload,
                    integrity_digest,
                    performance_receipt,
                    mutation,
                    file_envelope,
                    certification_bundle,
                )
            });
        if matches!(outcome, TransitionOutcome::Success(_)) {
            remove_active_session(self, &session);
        }
        outcome
    }

    pub fn execute_upload(
        &self,
        prepared: ForgeServerPreparedMultipartUpload,
    ) -> ForgeServerCompatibilityUploadOutcome<ForgeServerCompatibilityUpload> {
        match self.begin_binary_ingress(prepared) {
            TransitionOutcome::Success(session) => match self.verify_binary_ingress(session) {
                TransitionOutcome::Success(verified) => self.finalize_upload(verified),
                TransitionOutcome::Denied(value) => TransitionOutcome::Denied(value),
                TransitionOutcome::Deferred(value) => TransitionOutcome::Deferred(value),
                TransitionOutcome::Stale(value) => TransitionOutcome::Stale(value),
                TransitionOutcome::RebindRequired(value) => {
                    TransitionOutcome::RebindRequired(value)
                }
                TransitionOutcome::Failed(value) => TransitionOutcome::Failed(value),
            },
            TransitionOutcome::Denied(value) => TransitionOutcome::Denied(value),
            TransitionOutcome::Deferred(value) => TransitionOutcome::Deferred(value),
            TransitionOutcome::Stale(value) => TransitionOutcome::Stale(value),
            TransitionOutcome::RebindRequired(value) => TransitionOutcome::RebindRequired(value),
            TransitionOutcome::Failed(value) => TransitionOutcome::Failed(value),
        }
    }

    pub fn upload(
        &self,
        input: ForgeServerCompatibilityUploadExecutionInput,
    ) -> ForgeServerCompatibilityUploadOutcome<ForgeServerCompatibilityUpload> {
        match self.prepare_upload(input) {
            TransitionOutcome::Success(value) => self.execute_upload(value),
            TransitionOutcome::Denied(value) => TransitionOutcome::Denied(value),
            TransitionOutcome::Deferred(value) => TransitionOutcome::Deferred(value),
            TransitionOutcome::Stale(value) => TransitionOutcome::Stale(value),
            TransitionOutcome::RebindRequired(value) => TransitionOutcome::RebindRequired(value),
            TransitionOutcome::Failed(value) => TransitionOutcome::Failed(value),
        }
    }

    pub fn interrupt_binary_ingress(
        &self,
        session: &ForgeServerBinaryIngressSession,
    ) -> Result<ForgeServerUploadCleanupReceipt, ForgeServerQueryHandoffDenial> {
        cleanup_active_session(self, session, ForgeServerUploadCleanupReason::Interrupted)
    }

    pub fn expire_binary_ingress(
        &self,
        session: &ForgeServerBinaryIngressSession,
    ) -> Result<ForgeServerUploadCleanupReceipt, ForgeServerQueryHandoffDenial> {
        cleanup_active_session(self, session, ForgeServerUploadCleanupReason::Expired)
    }

    pub fn abandon_binary_ingress(
        &self,
        session: &ForgeServerBinaryIngressSession,
    ) -> Result<ForgeServerUploadCleanupReceipt, ForgeServerQueryHandoffDenial> {
        cleanup_active_session(self, session, ForgeServerUploadCleanupReason::Abandoned)
    }

    pub fn cleanup_mismatched_binary_ingress(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        session: &ForgeServerBinaryIngressSession,
    ) -> Result<ForgeServerUploadCleanupReceipt, ForgeServerQueryHandoffDenial> {
        if ownership_matches(prepared_request, session) {
            return Err(upload_request_invalid(
                prepared_request
                    .admission()
                    .request_context()
                    .diagnostics_profile(),
                "compatibility upload mismatch cleanup requires a tenant or branch mismatch",
            ));
        }
        cleanup_active_session(
            self,
            session,
            ForgeServerUploadCleanupReason::OwnershipMismatch,
        )
    }
}

fn parse_upload_mutation_request(
    upload: &ForgeServerMultipartUpload,
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
) -> Result<crate::ForgeServerCompatibilityMutationRequest, ForgeServerQueryHandoffDenial> {
    crate::surfaces::compat_http::mutation_execution::ForgeServerCompatibilityMutationRequest::parse(
        upload.manifest().metadata_body().clone(),
        diagnostics_profile,
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCompatibilityUpload {
    upload: ForgeServerMultipartUpload,
    ingress_integrity: ForgeServerIngressIntegrityDigest,
    ingress_performance: ForgeServerIngressPerformanceReceipt,
    mutation: crate::ForgeServerCompatibilityMutation,
    file_envelope: ForgeServerCompatibilityFileEnvelope,
    certification_bundle: ForgeServerBinaryCertificationBundle,
    canonical_digest: String,
}

impl ForgeServerCompatibilityUpload {
    pub(crate) fn new(
        upload: ForgeServerMultipartUpload,
        ingress_integrity: ForgeServerIngressIntegrityDigest,
        ingress_performance: ForgeServerIngressPerformanceReceipt,
        mutation: crate::ForgeServerCompatibilityMutation,
        file_envelope: ForgeServerCompatibilityFileEnvelope,
        certification_bundle: ForgeServerBinaryCertificationBundle,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-compat-upload-v4|upload={}|integrity={}|ingress_performance={}|mutation={}|file_envelope={}|certification={}",
            upload.canonical_digest(),
            ingress_integrity.canonical_digest(),
            ingress_performance_digest(&ingress_performance),
            mutation.canonical_digest(),
            file_envelope.canonical_digest(),
            certification_bundle.canonical_digest(),
        );
        Self {
            upload,
            ingress_integrity,
            ingress_performance,
            mutation,
            file_envelope,
            certification_bundle,
            canonical_digest,
        }
    }

    pub fn upload(&self) -> &ForgeServerMultipartUpload {
        &self.upload
    }

    pub fn ingress_integrity(&self) -> &ForgeServerIngressIntegrityDigest {
        &self.ingress_integrity
    }

    pub fn ingress_performance(&self) -> &ForgeServerIngressPerformanceReceipt {
        &self.ingress_performance
    }

    pub fn mutation(&self) -> &crate::ForgeServerCompatibilityMutation {
        &self.mutation
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

fn ingress_performance_digest(receipt: &ForgeServerIngressPerformanceReceipt) -> String {
    receipt
        .receipt()
        .counter_rows()
        .iter()
        .map(|row| format!("{}={}", row.name().as_str(), row.observed_count()))
        .collect::<Vec<_>>()
        .join("|")
}
