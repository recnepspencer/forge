use crate::{
    ForgeServerCompatibilityExecutionInput, ForgeServerCompatibilityExecutionOutcome,
    ForgeServerCompatibilityFacade, ForgeServerCompatibilityPreparedRequest,
    ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode,
    ForgeServerQueryHandoffFailure,
};
use forge_proof::TransitionOutcome;

use super::{
    performance::{
        ForgeServerBinaryEgressMetricSnapshot, ForgeServerBinaryEgressPerformanceReceipt,
    },
    ForgeServerBinaryDownload, ForgeServerBinaryDownloadRequest, ForgeServerBinaryEgressSession,
    ForgeServerBinaryIntegrityDigest, ForgeServerBinaryRetryPosture,
    ForgeServerBinarySessionResume, ForgeServerConditionalRangeRequest, ForgeServerRangeRequest,
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
        let head_only = input.prepared_request.request_contract().method() == "HEAD";
        let read = match self.read(ForgeServerCompatibilityExecutionInput::new(
            input.prepared_request.clone(),
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
                ))
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

fn derive_retry_posture(
    read: &crate::ForgeServerCompatibilityRead,
    download: &ForgeServerBinaryDownloadRequest,
    selected_start: usize,
    selected_end_exclusive: usize,
    head_only: bool,
    range_honored: bool,
    diagnostics_profile: forge_foundational::DiagnosticRichnessProfile,
) -> Result<ForgeServerBinaryRetryPosture, ForgeServerQueryHandoffDenial> {
    let Some(resume_request) = download.resume_request() else {
        return Ok(ForgeServerBinaryRetryPosture::ordinary(range_honored));
    };
    if head_only {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            "HEAD binary egress does not admit resumed byte delivery claims",
        ));
    }
    if resume_request.require_restart_stable_claim()
        && matches!(
            read.support_posture().durable_resume_support_posture(),
            forge_query::facade::ForgeQueryLowerRuntimeSupportPosture::Deferred
                | forge_query::facade::ForgeQueryLowerRuntimeSupportPosture::Forbidden
        )
    {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            format!(
                "restart-stable resume is not admitted for this binary delivery posture: `{}`",
                read.support_posture()
                    .durable_resume_support_posture()
                    .as_str()
            ),
        ));
    }
    let session_resume = resume_request.session_resume();
    if selected_start != session_resume.expected_next_start() {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            format!(
                "resume request expected byte {} but selected byte {}",
                session_resume.expected_next_start(),
                selected_start
            ),
        ));
    }
    if download.content_type() != session_resume.content_type() {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            "resume request changed the canonical binary download artifact or policy story",
        ));
    }
    if download.authorization().canonical_digest() != session_resume.authorization_digest() {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            "resume request widened the admitted authorization window",
        ));
    }
    if download.payload_digest() != session_resume.full_representation_digest() {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            format!(
                "resume integrity digest mismatch: expected full digest `{}` but observed `{}`",
                session_resume.full_representation_digest(),
                download.payload_digest()
            ),
        ));
    }
    if read.validator().entity_tag() != session_resume.validator_entity_tag() {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            format!(
                "resume validator mismatch: expected `{}` but observed `{}`",
                session_resume.validator_entity_tag(),
                read.validator().entity_tag()
            ),
        ));
    }
    if read.direct_context().workspace_digest() != session_resume.workspace_digest() {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            "resume request crossed into a different workspace delivery context",
        ));
    }
    if read.direct_context().branch_digest() != session_resume.branch_digest() {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            "resume request crossed into a different branch delivery context",
        ));
    }
    if let Some(expected_integrity) = resume_request.expected_integrity() {
        ForgeServerBinaryIntegrityDigest::project_for_validation(
            download,
            read.validator(),
            selected_start,
            selected_end_exclusive,
            head_only,
        )
        .verify_resume_expectation(expected_integrity, diagnostics_profile)?;
    }
    Ok(ForgeServerBinaryRetryPosture::resumed(
        session_resume.previous_session_digest(),
        session_resume.expected_next_start(),
        resume_request.require_restart_stable_claim(),
    ))
}
