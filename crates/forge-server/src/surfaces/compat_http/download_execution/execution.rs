use crate::{
    ForgeServerCanonicalHeaderSet, ForgeServerCompatibilityExecutionInput,
    ForgeServerCompatibilityExecutionOutcome, ForgeServerCompatibilityFacade,
    ForgeServerCompatibilityPreparedRequest, ForgeServerExternalRequestContract,
    ForgeServerOperationFamily, ForgeServerOperationRequestFacade, ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDenialCode, ForgeServerQueryHandoffFailure,
};
use forge_proof::TransitionOutcome;
use std::collections::BTreeMap;

use super::{
    performance::{
        ForgeServerBinaryEgressMetricSnapshot, ForgeServerBinaryEgressPerformanceReceipt,
    },
    retry_posture::derive_retry_posture,
    ForgeServerBinaryDownload, ForgeServerBinaryDownloadRequest, ForgeServerBinaryEgressSession,
    ForgeServerBinaryIntegrityDigest, ForgeServerBinarySessionResume,
    ForgeServerConditionalRangeRequest, ForgeServerRangeRequest,
};

pub type ForgeServerBinaryDownloadOutcome<T> = ForgeServerCompatibilityExecutionOutcome<T>;

#[derive(Clone, Debug)]
pub struct ForgeServerBinaryDownloadExecutionInput {
    prepared_request: ForgeServerCompatibilityPreparedRequest,
    operation_name: String,
    download: ForgeServerBinaryDownloadRequest,
}

impl ForgeServerBinaryDownloadExecutionInput {
    pub fn new(
        prepared_request: ForgeServerCompatibilityPreparedRequest,
        operation_name: impl Into<String>,
        download: ForgeServerBinaryDownloadRequest,
    ) -> Self {
        Self {
            prepared_request,
            operation_name: operation_name.into().trim().to_string(),
            download,
        }
    }
}

impl ForgeServerCompatibilityFacade {
    pub fn prepare_binary_egress(
        &self,
        input: ForgeServerBinaryDownloadExecutionInput,
    ) -> ForgeServerBinaryDownloadOutcome<ForgeServerBinaryEgressSession> {
        if let Err(denial) = self.admit_operation_family_for_query(
            input
                .prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            ForgeServerOperationFamily::BinaryTransfer,
        ) {
            return TransitionOutcome::Denied(denial);
        }
        if let Err(denial) = validate_download_request(&input.prepared_request) {
            return TransitionOutcome::Denied(denial);
        }
        if let Err(denial) = crate::surfaces::compat_http::validate_canonical_filename(
            &input.operation_name,
            input
                .prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            crate::ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
        ) {
            return TransitionOutcome::Denied(denial);
        }
        if let Err(denial) = crate::surfaces::compat_http::validate_operation_name_binding(
            input.prepared_request.request_contract(),
            &input.operation_name,
            crate::ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            input
                .prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
        ) {
            return TransitionOutcome::Denied(denial);
        }
        let range_request =
            match ForgeServerRangeRequest::from_prepared_request(&input.prepared_request) {
                Ok(value) => value,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        let conditional_range_request =
            match ForgeServerConditionalRangeRequest::from_prepared_request(&input.prepared_request)
            {
                Ok(value) => value,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        let operation_request =
            match ForgeServerOperationRequestFacade::new(self.operation_registry.clone())
                .admit_from_compat_http(
                    &input.prepared_request,
                    ForgeServerOperationFamily::BinaryTransfer,
                    &input.operation_name,
                    None,
                ) {
                Ok(value) => value,
                Err(denial) => {
                    return TransitionOutcome::Denied(ForgeServerQueryHandoffDenial::new(
                        ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
                        denial.diagnostics_profile(),
                        denial.detail(),
                    ));
                }
            };
        let operation_admission =
            match crate::ForgeServerOperationAdmissionFacade::with_operation_registry(
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
        let head_only = input.prepared_request.request_contract().method() == "HEAD";
        let metadata_read_request = metadata_read_prepared_request(&input.prepared_request);
        let read = match self.read(ForgeServerCompatibilityExecutionInput::new(
            metadata_read_request,
            input.operation_name,
        )) {
            TransitionOutcome::Success(value) => value,
            TransitionOutcome::Denied(value) => return TransitionOutcome::Denied(value),
            TransitionOutcome::Deferred(value) => return TransitionOutcome::Deferred(value),
            TransitionOutcome::Stale(value) => return TransitionOutcome::Stale(value),
            TransitionOutcome::RebindRequired(value) => {
                return TransitionOutcome::RebindRequired(value);
            }
            TransitionOutcome::Failed(value) => return TransitionOutcome::Failed(value),
        };
        let range_admitted = conditional_range_request.admits_range(read.validator());
        let (selected_start, selected_end_exclusive, partial_requested) = match range_request
            .resolve(
                input.download.body_bytes().len(),
                range_admitted,
                &input.prepared_request,
            ) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        if let Err(denial) = input.download.authorization().admit_selected_span(
            input
                .prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            selected_start,
            selected_end_exclusive,
        ) {
            return TransitionOutcome::Denied(denial);
        }
        let retry_posture = match derive_retry_posture(
            &read,
            &input.download,
            selected_start,
            selected_end_exclusive,
            head_only,
            partial_requested,
            input
                .prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
        ) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        TransitionOutcome::Success(ForgeServerBinaryEgressSession::new(
            operation_admission,
            read,
            input.download,
            range_request,
            conditional_range_request,
            selected_start,
            selected_end_exclusive,
            partial_requested,
            head_only,
            retry_posture,
        ))
    }

    pub fn plan_binary_resume(
        &self,
        session: &ForgeServerBinaryEgressSession,
    ) -> ForgeServerBinaryDownloadOutcome<ForgeServerBinarySessionResume> {
        if session.head_only() {
            return TransitionOutcome::Denied(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
                session.read().response_envelope().diagnostics_profile(),
                "HEAD-only binary egress does not admit a resumed byte continuation witness",
            ));
        }
        TransitionOutcome::Success(ForgeServerBinarySessionResume::from_session(session))
    }

    pub fn plan_binary_integrity(
        &self,
        session: &ForgeServerBinaryEgressSession,
    ) -> ForgeServerBinaryIntegrityDigest {
        ForgeServerBinaryIntegrityDigest::project(session)
    }

    pub fn execute_binary_egress(
        &self,
        session: ForgeServerBinaryEgressSession,
    ) -> ForgeServerBinaryDownloadOutcome<ForgeServerBinaryDownload> {
        let payload_bytes = if session.head_only() {
            Vec::new()
        } else {
            session.selected_body_bytes().to_vec()
        };
        let integrity_digest = ForgeServerBinaryIntegrityDigest::project(&session);
        let performance_receipt = match ForgeServerBinaryEgressPerformanceReceipt::build(
            ForgeServerBinaryEgressMetricSnapshot {
                requests: 1,
                bytes_emitted: payload_bytes.len() as u64,
                full_buffer_materializations: u64::from(!session.head_only()),
                range_requests: u64::from(session.range_honored()),
                full_requests: u64::from(!session.range_honored()),
                head_requests: u64::from(session.head_only()),
                resume_requests: u64::from(session.download_request().resume_request().is_some()),
                resumed_requests_admitted: u64::from(session.retry_posture().is_resume()),
                integrity_verifications: u64::from(
                    session.download_request().resume_request().is_some(),
                ),
                forbidden_fallbacks: 0,
            },
        ) {
            Ok(value) => value,
            Err(_) => {
                return TransitionOutcome::Failed(ForgeServerQueryHandoffFailure::new(
                    "compatibility_download_performance_receipt_failed",
                ));
            }
        };
        let file_envelope = crate::surfaces::compat_http::project_binary_egress_envelope(
            session.read(),
            Some(session.download_request().content_type().to_string()),
            payload_bytes.len() as u64,
            session.range_honored(),
            if session.head_only() {
                crate::ForgeServerFileTransferDisposition::HeadOnlyEgress
            } else {
                crate::ForgeServerFileTransferDisposition::SelectedEgress
            },
        );
        let certification_bundle =
            crate::surfaces::compat_http::build_download_certification_bundle(
                &self.operator_evidence,
                session.read().support_posture(),
                &file_envelope,
                session.read().response_envelope(),
                &performance_receipt,
            );
        TransitionOutcome::Success(ForgeServerBinaryDownload::new(
            session,
            payload_bytes,
            integrity_digest,
            performance_receipt,
            certification_bundle,
        ))
    }

    pub fn execute_binary_egress_with_resume(
        &self,
        session: ForgeServerBinaryEgressSession,
    ) -> ForgeServerBinaryDownloadOutcome<ForgeServerBinaryDownload> {
        self.execute_binary_egress(session)
    }

    pub fn download(
        &self,
        input: ForgeServerBinaryDownloadExecutionInput,
    ) -> ForgeServerBinaryDownloadOutcome<ForgeServerBinaryDownload> {
        match self.prepare_binary_egress(input) {
            TransitionOutcome::Success(value) => self.execute_binary_egress(value),
            TransitionOutcome::Denied(value) => TransitionOutcome::Denied(value),
            TransitionOutcome::Deferred(value) => TransitionOutcome::Deferred(value),
            TransitionOutcome::Stale(value) => TransitionOutcome::Stale(value),
            TransitionOutcome::RebindRequired(value) => TransitionOutcome::RebindRequired(value),
            TransitionOutcome::Failed(value) => TransitionOutcome::Failed(value),
        }
    }
}

fn validate_download_request(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    if prepared_request.request_contract().route_family()
        != crate::ForgeServerCompatHttpRouteFamily::Download
    {
        return Err(download_request_invalid(
            prepared_request,
            "compatibility binary egress requires the download route family",
        ));
    }
    if prepared_request.request_contract().body_present() {
        return Err(download_request_invalid(
            prepared_request,
            "compatibility binary egress does not admit request bodies",
        ));
    }
    Ok(())
}

fn download_request_invalid(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    detail: impl Into<String>,
) -> ForgeServerQueryHandoffDenial {
    ForgeServerQueryHandoffDenial::new(
        ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
        prepared_request
            .admission()
            .request_context()
            .diagnostics_profile(),
        detail,
    )
}

fn metadata_read_prepared_request(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
) -> ForgeServerCompatibilityPreparedRequest {
    let request_contract = prepared_request.request_contract();
    let headers = request_contract
        .canonical_headers()
        .iter()
        .filter(|(name, _)| *name != "range" && *name != "if-range")
        .map(|(name, values)| (name.to_string(), values.to_vec()))
        .collect::<BTreeMap<_, _>>();
    let metadata_contract = ForgeServerExternalRequestContract::new(
        request_contract.route_family(),
        request_contract.method().to_string(),
        request_contract.normalized_path().to_string(),
        request_contract.normalized_query_pairs().to_vec(),
        ForgeServerCanonicalHeaderSet::new(headers),
        request_contract.representation(),
        request_contract.version(),
        request_contract.diagnostics_profile(),
        request_contract.body_present(),
        request_contract.body_content_type().map(str::to_string),
    );
    ForgeServerCompatibilityPreparedRequest::new(
        prepared_request.admission().clone(),
        metadata_contract,
    )
}
