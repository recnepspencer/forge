use worth_proof::TransitionOutcome;

use crate::{
    WorthServerCompatibilityFacade, WorthServerCompatibilityPreparedRequest,
    WorthServerOperationAdmissionPosture, WorthServerOperationFamily,
    WorthServerOperationInputEnvelope, WorthServerOperationRequestFacade,
    WorthServerQueryHandoffDeferred, WorthServerQueryHandoffDenial, WorthServerQueryHandoffFailure,
    WorthServerQueryHandoffRebindRequired, WorthServerQueryHandoffStale,
};

use super::super::{
    abuse_accounting::{
        denied_budget_receipt_for_prepared_request, WorthServerAbuseBudgetDenialClass,
        WorthServerTransferByteClass,
    },
    build_upload_certification_bundle, project_upload_envelope,
};

use super::{
    admission::validate_upload_admission,
    cleanup::{ownership_matches, WorthServerUploadCleanupReason, WorthServerUploadCleanupReceipt},
    ingress_policy::admit_binary_ingress,
    integrity::WorthServerIngressIntegrityDigest,
    lifecycle::{
        cleanup_active_session, remove_active_session, require_active_session,
        stage_binary_ingress, upload_request_invalid,
    },
    performance::WorthServerIngressPerformanceReceipt,
    request::WorthServerMultipartUpload,
    response::WorthServerCompatibilityUpload,
    session::{WorthServerBinaryIngressSession, WorthServerVerifiedBinaryIngress},
};

pub type WorthServerCompatibilityUploadOutcome<T> = TransitionOutcome<
    T,
    WorthServerQueryHandoffDenial,
    WorthServerQueryHandoffDeferred,
    WorthServerQueryHandoffStale,
    WorthServerQueryHandoffRebindRequired,
    WorthServerQueryHandoffFailure,
>;

#[derive(Clone, Debug)]
pub struct WorthServerCompatibilityUploadExecutionInput {
    prepared_request: WorthServerCompatibilityPreparedRequest,
    operation_name: String,
    upload: WorthServerMultipartUpload,
}

impl WorthServerCompatibilityUploadExecutionInput {
    pub fn new(
        prepared_request: WorthServerCompatibilityPreparedRequest,
        operation_name: impl Into<String>,
        upload: WorthServerMultipartUpload,
    ) -> Self {
        Self {
            prepared_request,
            operation_name: operation_name.into().trim().to_string(),
            upload,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WorthServerPreparedMultipartUpload {
    prepared_request: WorthServerCompatibilityPreparedRequest,
    operation_name: String,
    upload: WorthServerMultipartUpload,
    mutation_request: crate::WorthServerCompatibilityMutationRequest,
    operation_admission: WorthServerOperationAdmissionPosture,
}

impl WorthServerPreparedMultipartUpload {
    pub fn prepared_request(&self) -> &WorthServerCompatibilityPreparedRequest {
        &self.prepared_request
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn upload(&self) -> &WorthServerMultipartUpload {
        &self.upload
    }

    pub fn mutation_request(&self) -> &crate::WorthServerCompatibilityMutationRequest {
        &self.mutation_request
    }

    pub fn operation_admission(&self) -> &WorthServerOperationAdmissionPosture {
        &self.operation_admission
    }

    pub fn requires_early_admission(&self) -> bool {
        self.upload.expectation().requires_early_admission()
    }

    fn into_parts(
        self,
    ) -> (
        WorthServerCompatibilityPreparedRequest,
        String,
        WorthServerMultipartUpload,
        crate::WorthServerCompatibilityMutationRequest,
        WorthServerOperationAdmissionPosture,
    ) {
        (
            self.prepared_request,
            self.operation_name,
            self.upload,
            self.mutation_request,
            self.operation_admission,
        )
    }
}

impl WorthServerCompatibilityFacade {
    pub fn prepare_upload(
        &self,
        input: WorthServerCompatibilityUploadExecutionInput,
    ) -> WorthServerCompatibilityUploadOutcome<WorthServerPreparedMultipartUpload> {
        if let Err(denial) = self.admit_operation_family_for_query(
            input
                .prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            WorthServerOperationFamily::BinaryTransfer,
        ) {
            return TransitionOutcome::Denied(denial);
        }
        if let Err(denial) = self.admit_operation_family_for_query(
            input
                .prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            WorthServerOperationFamily::QueryDirectSubmission,
        ) {
            return TransitionOutcome::Denied(denial);
        }
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
        let operation_request =
            match WorthServerOperationRequestFacade::new(self.operation_registry.clone())
                .admit_from_compat_http(
                    &input.prepared_request,
                    WorthServerOperationFamily::BinaryTransfer,
                    &input.operation_name,
                    None,
                ) {
                Ok(value) => value,
                Err(denial) => {
                    return TransitionOutcome::Denied(WorthServerQueryHandoffDenial::new(
                        crate::WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
                        denial.diagnostics_profile(),
                        denial.detail(),
                    ));
                }
            };
        let operation_admission =
            match crate::WorthServerOperationAdmissionFacade::with_operation_registry(
                self.operation_registry.clone(),
            )
            .admit_declared(input.prepared_request.admission(), &operation_request)
            {
                Ok(value) => value,
                Err(denial) => {
                    return TransitionOutcome::Denied(
                        crate::surfaces::compat_http::map_operation_admission_denial(denial),
                    );
                }
            };
        TransitionOutcome::Success(WorthServerPreparedMultipartUpload {
            prepared_request: input.prepared_request,
            operation_name: input.operation_name,
            upload: input.upload,
            mutation_request,
            operation_admission,
        })
    }

    pub fn begin_binary_ingress(
        &self,
        prepared: WorthServerPreparedMultipartUpload,
    ) -> WorthServerCompatibilityUploadOutcome<WorthServerBinaryIngressSession> {
        let (prepared_request, operation_name, upload, mutation_request, operation_admission) =
            prepared.into_parts();
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
                        WorthServerTransferByteClass::BinaryWire,
                        detail,
                        WorthServerAbuseBudgetDenialClass::SlowlorisCutoff,
                    ),
                ));
            }
        };
        let performance_receipt = match WorthServerIngressPerformanceReceipt::build(
            metrics,
            "compat_http.upload.ingress",
        ) {
            Ok(value) => value,
            Err(_) => {
                return TransitionOutcome::Failed(WorthServerQueryHandoffFailure::new(
                    "compatibility_upload_ingress_performance_receipt_failed",
                ));
            }
        };
        let session = WorthServerBinaryIngressSession::new(
            prepared_request,
            operation_name,
            upload,
            mutation_request,
            operation_admission,
            performance_receipt,
        );
        if let Err(denial) = stage_binary_ingress(self, &session) {
            return TransitionOutcome::Denied(denial);
        }
        TransitionOutcome::Success(session)
    }

    pub fn verify_binary_ingress(
        &self,
        session: WorthServerBinaryIngressSession,
    ) -> WorthServerCompatibilityUploadOutcome<WorthServerVerifiedBinaryIngress> {
        if let Err(denial) = require_active_session(self, &session) {
            return TransitionOutcome::Denied(denial);
        }
        let diagnostics_profile = session
            .prepared_request()
            .admission()
            .request_context()
            .diagnostics_profile();
        let integrity_digest = match WorthServerIngressIntegrityDigest::verify(
            session.upload(),
            diagnostics_profile,
        ) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        TransitionOutcome::Success(WorthServerVerifiedBinaryIngress::new(
            session,
            integrity_digest,
        ))
    }

    pub fn finalize_upload(
        &self,
        verified: WorthServerVerifiedBinaryIngress,
    ) -> WorthServerCompatibilityUploadOutcome<WorthServerCompatibilityUpload> {
        let (session, integrity_digest) = verified.into_parts();
        if let Err(denial) = require_active_session(self, &session) {
            return TransitionOutcome::Denied(denial);
        }
        let (
            prepared_request,
            operation_name,
            upload,
            mutation_request,
            _binary_operation_admission,
            _tenant_id,
            _workspace_digest,
            _branch_digest,
            _session_digest,
            performance_receipt,
        ) = session.clone().into_parts();
        let operation_request =
            match WorthServerOperationRequestFacade::new(self.operation_registry.clone())
                .admit_from_compat_http(
                    &prepared_request,
                    WorthServerOperationFamily::QueryDirectSubmission,
                    &operation_name,
                    Some(WorthServerOperationInputEnvelope::json(
                        "compat-http.query-mutation.v1",
                        upload.manifest().metadata_body(),
                    )),
                ) {
                Ok(value) => value,
                Err(denial) => {
                    return TransitionOutcome::Denied(WorthServerQueryHandoffDenial::new(
                    crate::WorthServerQueryHandoffDenialCode::CompatibilityMutationRequestInvalid,
                    denial.diagnostics_profile(),
                    denial.detail(),
                ));
                }
            };
        let outcome =
            crate::surfaces::compat_http::mutation_execution::execute_compatibility_mutation_request(
                self,
                prepared_request,
                operation_request,
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
                WorthServerCompatibilityUpload::new(
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
        prepared: WorthServerPreparedMultipartUpload,
    ) -> WorthServerCompatibilityUploadOutcome<WorthServerCompatibilityUpload> {
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
        input: WorthServerCompatibilityUploadExecutionInput,
    ) -> WorthServerCompatibilityUploadOutcome<WorthServerCompatibilityUpload> {
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
        session: &WorthServerBinaryIngressSession,
    ) -> Result<WorthServerUploadCleanupReceipt, WorthServerQueryHandoffDenial> {
        cleanup_active_session(self, session, WorthServerUploadCleanupReason::Interrupted)
    }

    pub fn expire_binary_ingress(
        &self,
        session: &WorthServerBinaryIngressSession,
    ) -> Result<WorthServerUploadCleanupReceipt, WorthServerQueryHandoffDenial> {
        cleanup_active_session(self, session, WorthServerUploadCleanupReason::Expired)
    }

    pub fn abandon_binary_ingress(
        &self,
        session: &WorthServerBinaryIngressSession,
    ) -> Result<WorthServerUploadCleanupReceipt, WorthServerQueryHandoffDenial> {
        cleanup_active_session(self, session, WorthServerUploadCleanupReason::Abandoned)
    }

    pub fn cleanup_mismatched_binary_ingress(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        session: &WorthServerBinaryIngressSession,
    ) -> Result<WorthServerUploadCleanupReceipt, WorthServerQueryHandoffDenial> {
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
            WorthServerUploadCleanupReason::OwnershipMismatch,
        )
    }
}

fn parse_upload_mutation_request(
    upload: &WorthServerMultipartUpload,
    diagnostics_profile: worth_foundational::facade::DiagnosticRichnessProfile,
) -> Result<crate::WorthServerCompatibilityMutationRequest, WorthServerQueryHandoffDenial> {
    crate::surfaces::compat_http::mutation_execution::WorthServerCompatibilityMutationRequest::parse(
        upload.manifest().metadata_body().clone(),
        diagnostics_profile,
    )
}
