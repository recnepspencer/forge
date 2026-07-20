use std::collections::BTreeMap;

use crate::request_context::DiagnosticRichnessProfile;

use super::WorthServerCompatHttpRouteFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerCompatibilityVersion {
    V1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerNegotiatedRepresentation {
    Json,
    Binary,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthServerCanonicalHeaderSet {
    rows: BTreeMap<String, Vec<String>>,
}

impl WorthServerCanonicalHeaderSet {
    pub(crate) fn new(rows: BTreeMap<String, Vec<String>>) -> Self {
        Self { rows }
    }

    pub fn values(&self, name: &str) -> Option<&[String]> {
        let normalized_name = name.trim().to_ascii_lowercase();
        self.rows.get(&normalized_name).map(Vec::as_slice)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &[String])> {
        self.rows
            .iter()
            .map(|(name, values)| (name.as_str(), values.as_slice()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerExternalRequestContract {
    route_family: WorthServerCompatHttpRouteFamily,
    method: String,
    normalized_path: String,
    normalized_query_pairs: Vec<(String, String)>,
    canonical_headers: WorthServerCanonicalHeaderSet,
    representation: WorthServerNegotiatedRepresentation,
    version: WorthServerCompatibilityVersion,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
    body_present: bool,
    body_content_type: Option<String>,
}

pub(crate) struct WorthServerExternalRequestContractParts {
    pub(crate) route_family: WorthServerCompatHttpRouteFamily,
    pub(crate) method: String,
    pub(crate) normalized_path: String,
    pub(crate) normalized_query_pairs: Vec<(String, String)>,
    pub(crate) canonical_headers: WorthServerCanonicalHeaderSet,
    pub(crate) representation: WorthServerNegotiatedRepresentation,
    pub(crate) version: WorthServerCompatibilityVersion,
    pub(crate) diagnostics_profile: Option<DiagnosticRichnessProfile>,
    pub(crate) body_present: bool,
    pub(crate) body_content_type: Option<String>,
}

impl WorthServerExternalRequestContract {
    pub(crate) fn new(parts: WorthServerExternalRequestContractParts) -> Self {
        let WorthServerExternalRequestContractParts {
            route_family,
            method,
            normalized_path,
            normalized_query_pairs,
            canonical_headers,
            representation,
            version,
            diagnostics_profile,
            body_present,
            body_content_type,
        } = parts;
        Self {
            route_family,
            method,
            normalized_path,
            normalized_query_pairs,
            canonical_headers,
            representation,
            version,
            diagnostics_profile,
            body_present,
            body_content_type,
        }
    }

    pub fn route_family(&self) -> WorthServerCompatHttpRouteFamily {
        self.route_family
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn normalized_path(&self) -> &str {
        &self.normalized_path
    }

    pub fn normalized_query_pairs(&self) -> &[(String, String)] {
        &self.normalized_query_pairs
    }

    pub fn canonical_headers(&self) -> &WorthServerCanonicalHeaderSet {
        &self.canonical_headers
    }

    pub fn representation(&self) -> WorthServerNegotiatedRepresentation {
        self.representation
    }

    pub fn version(&self) -> WorthServerCompatibilityVersion {
        self.version
    }

    pub fn diagnostics_profile(&self) -> Option<DiagnosticRichnessProfile> {
        self.diagnostics_profile
    }

    pub fn body_present(&self) -> bool {
        self.body_present
    }

    pub fn body_content_type(&self) -> Option<&str> {
        self.body_content_type.as_deref()
    }

    pub fn canonical_digest(&self) -> String {
        let query_digest = self
            .normalized_query_pairs
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join("&");
        let header_digest = self
            .canonical_headers
            .iter()
            .map(|(name, values)| format!("{name}={}", values.join("|")))
            .collect::<Vec<_>>()
            .join(";");
        format!(
            "family={};method={};path={};query={};headers={};representation={:?};version={:?};diagnostics={:?};body_present={};content_type={}",
            self.route_family.as_str(),
            self.method,
            self.normalized_path,
            query_digest,
            header_digest,
            self.representation,
            self.version,
            self.diagnostics_profile,
            self.body_present,
            self.body_content_type.as_deref().unwrap_or("none"),
        )
    }
}
