use crate::{
    request_context::DiagnosticRichnessProfile, ForgeServerCanonicalFilename,
    ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityPreparedRequest,
    ForgeServerOperationFamily, ForgeServerOperationRequestInput,
};

use super::{
    ForgeServerOperationInputEnvelope, ForgeServerOperationRequestDenial,
    ForgeServerOperationRequestDenialCode,
};

pub(crate) fn lower_compat_http_request_input(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    operation_family: ForgeServerOperationFamily,
    operation_name: &str,
    payload_envelope: Option<ForgeServerOperationInputEnvelope>,
) -> Result<ForgeServerOperationRequestInput, ForgeServerOperationRequestDenial> {
    let diagnostics_profile = prepared_request
        .admission()
        .request_context()
        .diagnostics_profile();
    validate_compatibility_operation_binding(
        prepared_request,
        operation_name,
        diagnostics_profile,
    )?;
    let mut builder = ForgeServerOperationRequestInput::builder()
        .with_operation_family(operation_family)
        .with_operation_name(operation_name);
    if operation_family != ForgeServerOperationFamily::QueryDirectRead {
        if let Some(basis_digest) = lower_basis_digest(prepared_request, diagnostics_profile)? {
            builder = builder.with_basis_digest(basis_digest);
        }
    }
    if let Some(idempotency_key) = lower_idempotency_key(prepared_request, diagnostics_profile)? {
        builder = builder.with_idempotency_key(idempotency_key);
    }
    if let Some(payload_envelope) = payload_envelope {
        builder = builder.with_payload_envelope(payload_envelope);
    }
    Ok(builder.build())
}

fn lower_basis_digest(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> Result<Option<String>, ForgeServerOperationRequestDenial> {
    let values = prepared_request
        .request_contract()
        .normalized_query_pairs()
        .iter()
        .filter(|(name, _)| name == "basis")
        .map(|(_, value)| value.trim())
        .collect::<Vec<_>>();
    if values.len() > 1 {
        return Err(ForgeServerOperationRequestDenial::new(
            ForgeServerOperationRequestDenialCode::InvalidBasisDigest,
            diagnostics_profile,
            "operation request admits at most one canonical basis query parameter",
        ));
    }
    let Some(value) = values.first() else {
        return Ok(None);
    };
    if value.is_empty() {
        return Err(ForgeServerOperationRequestDenial::new(
            ForgeServerOperationRequestDenialCode::InvalidBasisDigest,
            diagnostics_profile,
            "operation request basis digest may not be blank",
        ));
    }
    Ok(Some((*value).to_string()))
}

fn lower_idempotency_key(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> Result<Option<String>, ForgeServerOperationRequestDenial> {
    let Some(values) = prepared_request
        .request_contract()
        .canonical_headers()
        .values("idempotency-key")
    else {
        return Ok(None);
    };
    if values.len() != 1 {
        return Err(ForgeServerOperationRequestDenial::new(
            ForgeServerOperationRequestDenialCode::InvalidIdempotencyKey,
            diagnostics_profile,
            "operation request requires a single canonical `idempotency-key` header value",
        ));
    }
    let value = values[0].trim();
    if value.is_empty() {
        return Err(ForgeServerOperationRequestDenial::new(
            ForgeServerOperationRequestDenialCode::InvalidIdempotencyKey,
            diagnostics_profile,
            "operation request `idempotency-key` may not be blank",
        ));
    }
    Ok(Some(value.to_string()))
}

fn validate_compatibility_operation_binding(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    operation_name: &str,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> Result<(), ForgeServerOperationRequestDenial> {
    let claimed = ForgeServerCanonicalFilename::admit(
        operation_name,
        diagnostics_profile,
        crate::ForgeServerQueryHandoffDenialCode::DirectDeclarationBindingInvalid,
    )
    .map_err(|denial| {
        ForgeServerOperationRequestDenial::new(
            ForgeServerOperationRequestDenialCode::InvalidOperationName,
            diagnostics_profile,
            denial.detail(),
        )
    })?;
    let route_stem = match prepared_request.request_contract().route_family() {
        ForgeServerCompatHttpRouteFamily::Read => "/compat/reads/",
        ForgeServerCompatHttpRouteFamily::Mutation => "/compat/mutations/",
        ForgeServerCompatHttpRouteFamily::Streaming => "/compat/streams/",
        ForgeServerCompatHttpRouteFamily::Upload => "/compat/uploads/",
        ForgeServerCompatHttpRouteFamily::Download => "/compat/downloads/",
        ForgeServerCompatHttpRouteFamily::Preflight => {
            return Err(ForgeServerOperationRequestDenial::new(
                ForgeServerOperationRequestDenialCode::CompatibilityBindingInvalid,
                diagnostics_profile,
                "preflight requests do not lower to canonical operation requests",
            ));
        }
    };
    let Some(remainder) = prepared_request
        .request_contract()
        .normalized_path()
        .strip_prefix(route_stem)
    else {
        return Err(ForgeServerOperationRequestDenial::new(
            ForgeServerOperationRequestDenialCode::CompatibilityBindingInvalid,
            diagnostics_profile,
            format!(
                "compatibility path `{}` did not match route stem `{route_stem}`",
                prepared_request.request_contract().normalized_path()
            ),
        ));
    };
    if remainder.is_empty() || remainder.contains('/') {
        return Err(ForgeServerOperationRequestDenial::new(
            ForgeServerOperationRequestDenialCode::CompatibilityBindingInvalid,
            diagnostics_profile,
            format!(
                "compatibility path `{}` must carry exactly one operation segment after `{route_stem}`",
                prepared_request.request_contract().normalized_path()
            ),
        ));
    }
    let expected = ForgeServerCanonicalFilename::admit(
        remainder,
        diagnostics_profile,
        crate::ForgeServerQueryHandoffDenialCode::DirectDeclarationBindingInvalid,
    )
    .map_err(|denial| {
        ForgeServerOperationRequestDenial::new(
            ForgeServerOperationRequestDenialCode::CompatibilityBindingInvalid,
            diagnostics_profile,
            denial.detail(),
        )
    })?;
    if expected.canonical() != claimed.canonical() {
        return Err(ForgeServerOperationRequestDenial::new(
            ForgeServerOperationRequestDenialCode::CompatibilityBindingInvalid,
            diagnostics_profile,
            format!(
                "compatibility operation name `{}` did not match the external request path identity `{}`",
                claimed.original(),
                expected.original(),
            ),
        ));
    }
    Ok(())
}
