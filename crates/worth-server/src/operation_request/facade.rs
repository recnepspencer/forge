use crate::{
    request_context::DiagnosticRichnessProfile, WorthServerAdmission, WorthServerCanonicalFilename,
    WorthServerCompatibilityPreparedRequest, WorthServerOperationIdentity,
    WorthServerOperationInputEnvelope, WorthServerOperationRegistry, WorthServerOperationRequest,
    WorthServerOperationRequestDenial, WorthServerOperationRequestDenialCode,
    WorthServerOperationRequestInput, WorthServerOperationRequestReceipt,
    WorthServerResolvedRequestContext,
};

use super::surface_lowering::lower_compat_http_request_input;

#[derive(Clone, Debug)]
pub struct WorthServerOperationRequestFacade {
    operation_registry: WorthServerOperationRegistry,
}

impl WorthServerOperationRequestFacade {
    pub(crate) fn new(operation_registry: WorthServerOperationRegistry) -> Self {
        Self { operation_registry }
    }

    pub fn admit(
        &self,
        resolved_request_context: WorthServerResolvedRequestContext,
        input: WorthServerOperationRequestInput,
    ) -> Result<WorthServerOperationRequest, WorthServerOperationRequestDenial> {
        let diagnostics_profile = resolved_request_context
            .request_context()
            .diagnostics_profile();
        let operation_family = input
            .operation_family()
            .ok_or_else(|| missing_family(diagnostics_profile))?;
        let operation_name = admit_identifier(
            input
                .operation_name()
                .ok_or_else(|| missing_name(diagnostics_profile))?,
            diagnostics_profile,
            WorthServerOperationRequestDenialCode::InvalidOperationName,
            "operation name",
        )?;
        self.operation_registry
            .admit_operation_name(operation_family, &operation_name)
            .map_err(|denial| match denial {
                crate::WorthServerOperationDenial::UnknownOperationName { .. } => {
                    WorthServerOperationRequestDenial::new(
                        WorthServerOperationRequestDenialCode::UnknownOperationName,
                        diagnostics_profile,
                        denial.detail(),
                    )
                }
                _ => WorthServerOperationRequestDenial::new(
                    WorthServerOperationRequestDenialCode::InvalidOperationName,
                    diagnostics_profile,
                    denial.detail(),
                ),
            })?;
        let basis_digest = admit_optional_token(
            input.basis_digest(),
            diagnostics_profile,
            WorthServerOperationRequestDenialCode::InvalidBasisDigest,
            "basis digest",
        )?;
        let idempotency_key = admit_optional_token(
            input.idempotency_key(),
            diagnostics_profile,
            WorthServerOperationRequestDenialCode::InvalidIdempotencyKey,
            "idempotency key",
        )?;
        let product_session_identity = admit_optional_token(
            input.product_session_identity(),
            diagnostics_profile,
            WorthServerOperationRequestDenialCode::InvalidProductSessionIdentity,
            "product session identity",
        )?;
        let payload_envelope = input.payload_envelope().cloned();
        validate_payload_envelope(payload_envelope.as_ref(), diagnostics_profile)?;
        let identity = WorthServerOperationIdentity::new(
            operation_family,
            resolved_request_context
                .request_context()
                .workspace_target()
                .tenant_id()
                .to_string(),
            resolved_request_context
                .request_context()
                .workspace_target()
                .workspace_id()
                .to_string(),
            resolved_request_context
                .request_context()
                .branch_target()
                .canonical_label()
                .to_string(),
            operation_name,
            basis_digest,
            idempotency_key,
            product_session_identity,
            payload_envelope
                .as_ref()
                .map(|payload| payload.payload_identity().to_string()),
        );
        let receipt = WorthServerOperationRequestReceipt::new(
            resolved_request_context.surface_family(),
            resolved_request_context.transport_class(),
            diagnostics_profile,
            request_context_digest(&resolved_request_context),
            None,
        );
        Ok(WorthServerOperationRequest::new(
            resolved_request_context,
            identity,
            payload_envelope,
            receipt,
        ))
    }

    pub fn admit_from_compat_http(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        operation_family: crate::WorthServerOperationFamily,
        operation_name: &str,
        payload_envelope: Option<WorthServerOperationInputEnvelope>,
    ) -> Result<WorthServerOperationRequest, WorthServerOperationRequestDenial> {
        let lowered = lower_compat_http_request_input(
            prepared_request,
            operation_family,
            operation_name,
            payload_envelope,
        )?;
        let mut request = self.admit(
            prepared_request
                .admission()
                .resolved_request_context()
                .clone(),
            lowered,
        )?;
        request = WorthServerOperationRequest::new(
            request.resolved_request_context().clone(),
            request.identity().clone(),
            request.payload_envelope().cloned(),
            WorthServerOperationRequestReceipt::new(
                request.resolved_request_context().surface_family(),
                request.resolved_request_context().transport_class(),
                request.receipt().diagnostics_profile(),
                request_context_digest(request.resolved_request_context()),
                Some(prepared_request.request_contract().canonical_digest()),
            ),
        );
        Ok(request)
    }

    pub fn admit_from_compat_http_with_request_input(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        input: WorthServerOperationRequestInput,
    ) -> Result<WorthServerOperationRequest, WorthServerOperationRequestDenial> {
        let diagnostics_profile = prepared_request
            .admission()
            .request_context()
            .diagnostics_profile();
        let operation_family = input
            .operation_family()
            .ok_or_else(|| missing_family(diagnostics_profile))?;
        let operation_name = input
            .operation_name()
            .ok_or_else(|| missing_name(diagnostics_profile))?;
        let mut merged = lower_compat_http_request_input(
            prepared_request,
            operation_family,
            operation_name,
            input.payload_envelope().cloned(),
        )?;
        let mut builder = WorthServerOperationRequestInput::builder()
            .with_operation_family(
                merged
                    .operation_family()
                    .expect("compat lowering should preserve operation family"),
            )
            .with_operation_name(
                merged
                    .operation_name()
                    .expect("compat lowering should preserve operation name"),
            );
        let merged_basis_digest = merged
            .basis_digest()
            .map(canonicalize_product_basis_digest_if_needed);
        if let (Some(input_basis_digest), Some(request_basis_digest)) =
            (input.basis_digest(), merged_basis_digest.as_deref())
        {
            if input_basis_digest != request_basis_digest {
                return Err(WorthServerOperationRequestDenial::new(
                    WorthServerOperationRequestDenialCode::InvalidBasisDigest,
                    diagnostics_profile,
                    format!(
                        "compatibility request admitted basis digest `{request_basis_digest}` did not match product operation basis digest `{input_basis_digest}`"
                    ),
                ));
            }
        }
        if let Some(basis_digest) = input
            .basis_digest()
            .map(str::to_string)
            .or(merged_basis_digest)
        {
            builder = builder.with_basis_digest(basis_digest);
        }
        if let (Some(input_idempotency_key), Some(request_idempotency_key)) =
            (input.idempotency_key(), merged.idempotency_key())
        {
            if input_idempotency_key != request_idempotency_key {
                return Err(WorthServerOperationRequestDenial::new(
                    WorthServerOperationRequestDenialCode::InvalidIdempotencyKey,
                    diagnostics_profile,
                    format!(
                        "compatibility request admitted idempotency key `{request_idempotency_key}` did not match product operation idempotency key `{input_idempotency_key}`"
                    ),
                ));
            }
        }
        if let Some(idempotency_key) = input
            .idempotency_key()
            .map(|key| key.to_string())
            .or_else(|| merged.idempotency_key().map(str::to_string))
        {
            builder = builder.with_idempotency_key(idempotency_key);
        }
        if let Some(product_session_identity) = input.product_session_identity() {
            builder = builder.with_product_session_identity(product_session_identity);
        }
        if let Some(payload_envelope) = merged.payload_envelope().cloned() {
            builder = builder.with_payload_envelope(payload_envelope);
        }
        merged = builder.build();
        let request = self.admit(
            prepared_request
                .admission()
                .resolved_request_context()
                .clone(),
            merged,
        )?;
        Ok(WorthServerOperationRequest::new(
            request.resolved_request_context().clone(),
            request.identity().clone(),
            request.payload_envelope().cloned(),
            WorthServerOperationRequestReceipt::new(
                request.resolved_request_context().surface_family(),
                request.resolved_request_context().transport_class(),
                request.receipt().diagnostics_profile(),
                request_context_digest(request.resolved_request_context()),
                Some(prepared_request.request_contract().canonical_digest()),
            ),
        ))
    }

    pub fn admit_from_compat_http_with_basis_digest(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        operation_family: crate::WorthServerOperationFamily,
        operation_name: &str,
        basis_digest: Option<&str>,
        payload_envelope: Option<WorthServerOperationInputEnvelope>,
    ) -> Result<WorthServerOperationRequest, WorthServerOperationRequestDenial> {
        let request = self.admit_from_compat_http(
            prepared_request,
            operation_family,
            operation_name,
            payload_envelope,
        )?;
        let identity = request.identity().with_basis_digest(basis_digest);
        Ok(WorthServerOperationRequest::new(
            request.resolved_request_context().clone(),
            identity,
            request.payload_envelope().cloned(),
            request.receipt().clone(),
        ))
    }

    pub fn admit_from_compat_http_with_product_session_identity(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        operation_family: crate::WorthServerOperationFamily,
        operation_name: &str,
        product_session_identity: Option<&str>,
        payload_envelope: Option<WorthServerOperationInputEnvelope>,
    ) -> Result<WorthServerOperationRequest, WorthServerOperationRequestDenial> {
        let request = self.admit_from_compat_http(
            prepared_request,
            operation_family,
            operation_name,
            payload_envelope,
        )?;
        let identity = WorthServerOperationIdentity::new(
            request.identity().operation_family(),
            request
                .resolved_request_context()
                .request_context()
                .workspace_target()
                .tenant_id()
                .to_string(),
            request
                .resolved_request_context()
                .request_context()
                .workspace_target()
                .workspace_id()
                .to_string(),
            request
                .resolved_request_context()
                .request_context()
                .branch_target()
                .canonical_label()
                .to_string(),
            request.identity().operation_name().to_string(),
            request.identity().basis_digest().map(str::to_string),
            request.identity().idempotency_key().map(str::to_string),
            product_session_identity.map(str::to_string),
            request.identity().payload_identity().map(str::to_string),
        );
        Ok(WorthServerOperationRequest::new(
            request.resolved_request_context().clone(),
            identity,
            request.payload_envelope().cloned(),
            request.receipt().clone(),
        ))
    }

    pub fn admit_from_worth_native_admission(
        &self,
        admission: &WorthServerAdmission,
        input: WorthServerOperationRequestInput,
    ) -> Result<WorthServerOperationRequest, WorthServerOperationRequestDenial> {
        self.admit(admission.resolved_request_context().clone(), input)
    }
}

fn canonicalize_product_basis_digest_if_needed(value: &str) -> String {
    crate::WorthServerProductOperationBaseDigest::canonicalize_text(value)
        .unwrap_or_else(|_| value.trim().to_string())
}

fn missing_family(
    diagnostics_profile: DiagnosticRichnessProfile,
) -> WorthServerOperationRequestDenial {
    WorthServerOperationRequestDenial::new(
        WorthServerOperationRequestDenialCode::MissingOperationFamily,
        diagnostics_profile,
        "operation request requires an operation family before planning",
    )
}

fn missing_name(
    diagnostics_profile: DiagnosticRichnessProfile,
) -> WorthServerOperationRequestDenial {
    WorthServerOperationRequestDenial::new(
        WorthServerOperationRequestDenialCode::MissingOperationName,
        diagnostics_profile,
        "operation request requires a declared operation name before planning",
    )
}

fn admit_identifier(
    value: &str,
    diagnostics_profile: DiagnosticRichnessProfile,
    denial_code: WorthServerOperationRequestDenialCode,
    label: &str,
) -> Result<String, WorthServerOperationRequestDenial> {
    let admitted = WorthServerCanonicalFilename::admit(
        value,
        diagnostics_profile,
        crate::WorthServerQueryHandoffDenialCode::DirectDeclarationBindingInvalid,
    )
    .map_err(|denial| {
        WorthServerOperationRequestDenial::new(
            denial_code.clone(),
            diagnostics_profile,
            denial.detail(),
        )
    })?;
    if admitted.canonical().contains(' ') {
        return Err(WorthServerOperationRequestDenial::new(
            denial_code,
            diagnostics_profile,
            format!("{label} may not contain whitespace"),
        ));
    }
    Ok(admitted.canonical().to_string())
}

fn admit_optional_token(
    value: Option<&str>,
    diagnostics_profile: DiagnosticRichnessProfile,
    denial_code: WorthServerOperationRequestDenialCode,
    label: &str,
) -> Result<Option<String>, WorthServerOperationRequestDenial> {
    value
        .map(|value| admit_token(value, diagnostics_profile, denial_code, label))
        .transpose()
}

fn admit_token(
    value: &str,
    diagnostics_profile: DiagnosticRichnessProfile,
    denial_code: WorthServerOperationRequestDenialCode,
    label: &str,
) -> Result<String, WorthServerOperationRequestDenial> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(WorthServerOperationRequestDenial::new(
            denial_code,
            diagnostics_profile,
            format!("{label} may not be blank"),
        ));
    }
    if trimmed.chars().any(|ch| ch.is_control() || !ch.is_ascii()) {
        return Err(WorthServerOperationRequestDenial::new(
            denial_code,
            diagnostics_profile,
            format!("{label} must stay ASCII-printable"),
        ));
    }
    Ok(trimmed.to_string())
}

fn validate_payload_envelope(
    payload_envelope: Option<&WorthServerOperationInputEnvelope>,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> Result<(), WorthServerOperationRequestDenial> {
    let Some(payload_envelope) = payload_envelope else {
        return Ok(());
    };
    if payload_envelope.payload_identity().trim().is_empty() {
        return Err(WorthServerOperationRequestDenial::new(
            WorthServerOperationRequestDenialCode::InvalidPayloadEnvelope,
            diagnostics_profile,
            "operation payload envelope must carry a non-blank payload identity",
        ));
    }
    if let Some(schema_identity) = payload_envelope.declared_schema_identity() {
        if schema_identity.trim().is_empty()
            || schema_identity
                .chars()
                .any(|ch| ch.is_control() || !ch.is_ascii())
        {
            return Err(WorthServerOperationRequestDenial::new(
                WorthServerOperationRequestDenialCode::InvalidDeclaredSchemaIdentity,
                diagnostics_profile,
                "operation payload declared schema identity must stay ASCII-printable and non-blank",
            ));
        }
    }
    Ok(())
}

fn request_context_digest(resolved_request_context: &WorthServerResolvedRequestContext) -> String {
    let request_context = resolved_request_context.request_context();
    format!(
        "surface={:?};transport={:?};principal={};tenant={};workspace={};branch={};diagnostics={:?}",
        resolved_request_context.surface_family(),
        resolved_request_context.transport_class(),
        request_context.authenticated_principal().principal_id(),
        request_context.workspace_target().tenant_id(),
        request_context.workspace_target().workspace_id(),
        request_context.branch_target().branch_digest(),
        request_context.diagnostics_profile(),
    )
}
