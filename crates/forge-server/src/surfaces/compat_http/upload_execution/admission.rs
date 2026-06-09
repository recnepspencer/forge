use std::collections::BTreeSet;

use crate::{
    ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityPreparedRequest,
    ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode,
};

use super::request::{ForgeServerMultipartUpload, ForgeServerUploadExpectation};

const MAX_DECLARED_UPLOAD_PART_BYTES: u64 = 8 * 1024 * 1024;

pub(crate) fn validate_upload_admission(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    upload: &ForgeServerMultipartUpload,
    operation_name: &str,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    validate_upload_route_family(prepared_request)?;
    validate_upload_request_contract(prepared_request, upload.expectation())?;
    validate_upload_shape(prepared_request, upload, operation_name)?;
    Ok(())
}

fn validate_upload_route_family(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    if prepared_request.request_contract().route_family()
        != ForgeServerCompatHttpRouteFamily::Upload
    {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            "compatibility upload execution requires the upload route family",
        ));
    }
    Ok(())
}

fn validate_upload_request_contract(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    expectation: ForgeServerUploadExpectation,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    let diagnostics_profile = prepared_request
        .admission()
        .request_context()
        .diagnostics_profile();
    let request_contract = prepared_request.request_contract();
    if !request_contract.body_present() {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            "compatibility multipart upload requires a request body before admission can continue",
        ));
    }
    let Some(content_type) = request_contract.body_content_type() else {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            "compatibility multipart upload requires an explicit multipart/form-data content type",
        ));
    };
    if !content_type.starts_with("multipart/form-data") {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            format!(
                "compatibility upload content type `{content_type}` is not admitted; expected multipart/form-data"
            ),
        ));
    }
    let expect_values = request_contract.canonical_headers().values("expect");
    if let Some(values) = expect_values {
        if values.len() != 1 || values[0] != "100-continue" {
            return Err(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
                diagnostics_profile,
                "compatibility upload admits only `Expect: 100-continue` as an expectation header",
            ));
        }
    }
    if expectation.requires_early_admission()
        && !matches!(expect_values, Some(values) if values == ["100-continue"])
    {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            "compatibility upload expectation requires `Expect: 100-continue` so the server can deny before bulk body transfer begins",
        ));
    }
    Ok(())
}

fn validate_upload_shape(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    upload: &ForgeServerMultipartUpload,
    operation_name: &str,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    let diagnostics_profile = prepared_request
        .admission()
        .request_context()
        .diagnostics_profile();
    if operation_name.trim().is_empty() {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            "compatibility upload operation name may not be blank",
        ));
    }
    let metadata_body = upload.manifest().metadata_body();
    if !metadata_body.is_object() {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            "compatibility upload manifest metadata must be a JSON object carrying the canonical mutation body",
        ));
    }
    let declared_parts = normalized_part_names(
        upload.manifest().declared_file_parts(),
        diagnostics_profile,
        "manifest file part",
    )?;
    if declared_parts.is_empty() {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            "compatibility upload manifest must declare at least one file part",
        ));
    }
    let observed_parts = normalized_upload_parts(upload, diagnostics_profile)?;
    if observed_parts.is_empty() {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            "compatibility upload must supply at least one declared file part",
        ));
    }
    let observed_names = observed_parts
        .iter()
        .map(|(name, _, _)| name.clone())
        .collect::<BTreeSet<_>>();
    let declared_names = declared_parts.into_iter().collect::<BTreeSet<_>>();
    if declared_names != observed_names {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            "compatibility upload part graph did not match the declared manifest file-part set",
        ));
    }
    Ok(())
}

fn normalized_part_names(
    part_names: &[String],
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
    label: &str,
) -> Result<Vec<String>, ForgeServerQueryHandoffDenial> {
    let mut normalized = Vec::with_capacity(part_names.len());
    let mut seen = BTreeSet::new();
    for value in part_names {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
                diagnostics_profile,
                format!("compatibility upload {label} names may not be blank"),
            ));
        }
        if !seen.insert(trimmed.to_string()) {
            return Err(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
                diagnostics_profile,
                format!("compatibility upload {label} names may not repeat"),
            ));
        }
        normalized.push(trimmed.to_string());
    }
    Ok(normalized)
}

fn normalized_upload_parts(
    upload: &ForgeServerMultipartUpload,
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
) -> Result<Vec<(String, String, u64)>, ForgeServerQueryHandoffDenial> {
    let mut normalized = Vec::with_capacity(upload.parts().len());
    let mut seen = BTreeSet::new();
    for part in upload.parts() {
        let name = part.name().trim();
        if name.is_empty() {
            return Err(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
                diagnostics_profile,
                "compatibility upload part names may not be blank",
            ));
        }
        if !seen.insert(name.to_string()) {
            return Err(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
                diagnostics_profile,
                format!("compatibility upload part `{name}` was supplied more than once"),
            ));
        }
        let content_type = part.content_type().trim().to_ascii_lowercase();
        if content_type.is_empty() {
            return Err(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
                diagnostics_profile,
                format!("compatibility upload part `{name}` requires an explicit content type"),
            ));
        }
        if part.declared_length() > MAX_DECLARED_UPLOAD_PART_BYTES {
            return Err(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
                diagnostics_profile,
                format!(
                    "compatibility upload part `{name}` declared `{}` bytes, which exceeds the phase-five early-admission cap `{MAX_DECLARED_UPLOAD_PART_BYTES}`",
                    part.declared_length()
                ),
            ));
        }
        normalized.push((name.to_string(), content_type, part.declared_length()));
    }
    Ok(normalized)
}
