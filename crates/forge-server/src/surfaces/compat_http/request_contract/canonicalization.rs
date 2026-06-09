use std::collections::BTreeMap;

use super::input::ForgeServerCompatibilityRequestInput;
use crate::{
    ForgeServerCanonicalHeaderSet, ForgeServerCompatibilityDenial,
    ForgeServerCompatibilityDenialCode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ForgeServerCanonicalCompatibilityRequest {
    method: String,
    normalized_path: String,
    canonical_headers: ForgeServerCanonicalHeaderSet,
    normalized_query_pairs: Vec<(String, String)>,
}

impl ForgeServerCanonicalCompatibilityRequest {
    pub(crate) fn method(&self) -> &str {
        &self.method
    }

    pub(crate) fn normalized_path(&self) -> &str {
        &self.normalized_path
    }

    pub(crate) fn canonical_headers(&self) -> &ForgeServerCanonicalHeaderSet {
        &self.canonical_headers
    }

    pub(crate) fn normalized_query_pairs(&self) -> &[(String, String)] {
        &self.normalized_query_pairs
    }
}

pub(crate) fn canonicalize_request(
    input: &ForgeServerCompatibilityRequestInput,
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
) -> Result<ForgeServerCanonicalCompatibilityRequest, ForgeServerCompatibilityDenial> {
    let method = normalize_method(input.method(), diagnostics_profile)?;
    validate_body_contract(input, method.as_str(), diagnostics_profile)?;
    let path = normalize_path(input.path(), diagnostics_profile)?;
    let headers = normalize_headers(input.headers(), diagnostics_profile)?;
    let query_pairs = normalize_query_pairs(input.query_pairs(), diagnostics_profile)?;

    Ok(ForgeServerCanonicalCompatibilityRequest {
        method,
        normalized_path: path,
        canonical_headers: headers,
        normalized_query_pairs: query_pairs,
    })
}

fn validate_body_contract(
    input: &ForgeServerCompatibilityRequestInput,
    method: &str,
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
) -> Result<(), ForgeServerCompatibilityDenial> {
    if input
        .body_content_type()
        .is_some_and(|value| value.trim().is_empty())
    {
        return Err(ForgeServerCompatibilityDenial::new(
            ForgeServerCompatibilityDenialCode::InvalidBodyContentType,
            diagnostics_profile,
            "body content type must not be blank",
        ));
    }

    if input.body_content_type().is_some() && !input.body_present() {
        return Err(ForgeServerCompatibilityDenial::new(
            ForgeServerCompatibilityDenialCode::BodyMetadataWithoutBody,
            diagnostics_profile,
            "body content type requires a present request body",
        ));
    }

    if input.body_present() && matches!(method, "GET" | "HEAD" | "OPTIONS") {
        return Err(ForgeServerCompatibilityDenial::new(
            ForgeServerCompatibilityDenialCode::UnexpectedRequestBody,
            diagnostics_profile,
            format!("HTTP method `{method}` does not admit a request body at the compatibility boundary"),
        ));
    }

    Ok(())
}

fn normalize_method(
    method: &str,
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
) -> Result<String, ForgeServerCompatibilityDenial> {
    let normalized = method.trim().to_ascii_uppercase();
    let allowed = ["GET", "HEAD", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"];
    if allowed.contains(&normalized.as_str()) {
        Ok(normalized)
    } else {
        Err(ForgeServerCompatibilityDenial::new(
            ForgeServerCompatibilityDenialCode::UnsupportedHttpMethod,
            diagnostics_profile,
            format!("unsupported HTTP method `{}`", method.trim()),
        ))
    }
}

fn normalize_path(
    path: &str,
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
) -> Result<String, ForgeServerCompatibilityDenial> {
    let normalized = path.trim();
    if !normalized.starts_with('/') {
        return Err(ForgeServerCompatibilityDenial::new(
            ForgeServerCompatibilityDenialCode::InvalidPath,
            diagnostics_profile,
            "compatibility path must start with `/`",
        ));
    }
    if normalized.contains('?') {
        return Err(ForgeServerCompatibilityDenial::new(
            ForgeServerCompatibilityDenialCode::InvalidPath,
            diagnostics_profile,
            "compatibility path must not contain an inline query string",
        ));
    }
    Ok(normalized.to_string())
}

fn normalize_headers(
    headers: &[(String, String)],
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
) -> Result<ForgeServerCanonicalHeaderSet, ForgeServerCompatibilityDenial> {
    let mut normalized = BTreeMap::new();
    for (name, value) in headers {
        let header_name = name.trim().to_ascii_lowercase();
        if header_name.is_empty() {
            return Err(ForgeServerCompatibilityDenial::new(
                ForgeServerCompatibilityDenialCode::InvalidHeader,
                diagnostics_profile,
                "header name must not be empty",
            ));
        }
        normalized
            .entry(header_name)
            .or_insert_with(Vec::new)
            .push(value.trim().to_string());
    }

    reject_ambiguous_forwarded_headers(&normalized, diagnostics_profile)?;
    Ok(ForgeServerCanonicalHeaderSet::new(normalized))
}

fn reject_ambiguous_forwarded_headers(
    headers: &BTreeMap<String, Vec<String>>,
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
) -> Result<(), ForgeServerCompatibilityDenial> {
    for name in ["x-forwarded-proto", "x-forwarded-host"] {
        if let Some(values) = headers.get(name) {
            let distinct = values.iter().collect::<std::collections::BTreeSet<_>>();
            if distinct.len() > 1 {
                return Err(ForgeServerCompatibilityDenial::new(
                    ForgeServerCompatibilityDenialCode::AmbiguousForwardingHeaders,
                    diagnostics_profile,
                    format!("header `{name}` contains multiple conflicting values"),
                ));
            }
        }
    }
    Ok(())
}

fn normalize_query_pairs(
    query_pairs: &[(String, String)],
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
) -> Result<Vec<(String, String)>, ForgeServerCompatibilityDenial> {
    let mut normalized = Vec::with_capacity(query_pairs.len());
    for (name, value) in query_pairs {
        let normalized_name = name.trim().to_string();
        if normalized_name.is_empty() {
            return Err(ForgeServerCompatibilityDenial::new(
                ForgeServerCompatibilityDenialCode::InvalidQueryPair,
                diagnostics_profile,
                "query key must not be empty",
            ));
        }
        normalized.push((normalized_name, value.trim().to_string()));
    }
    normalized.sort();
    Ok(normalized)
}
