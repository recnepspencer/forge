use std::collections::BTreeMap;

use crate::request_context::DiagnosticRichnessProfile;

use super::ForgeServerCompatHttpRouteFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerCompatibilityVersion {
    V1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerNegotiatedRepresentation {
    Json,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ForgeServerCanonicalHeaderSet {
    rows: BTreeMap<String, Vec<String>>,
}

impl ForgeServerCanonicalHeaderSet {
    pub(crate) fn new(rows: BTreeMap<String, Vec<String>>) -> Self {
        Self { rows }
    }

    pub fn values(&self, name: &str) -> Option<&[String]> {
        self.rows.get(name).map(Vec::as_slice)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &[String])> {
        self.rows
            .iter()
            .map(|(name, values)| (name.as_str(), values.as_slice()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerExternalRequestContract {
    route_family: ForgeServerCompatHttpRouteFamily,
    method: String,
    normalized_path: String,
    normalized_query_pairs: Vec<(String, String)>,
    canonical_headers: ForgeServerCanonicalHeaderSet,
    representation: ForgeServerNegotiatedRepresentation,
    version: ForgeServerCompatibilityVersion,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
    body_present: bool,
    body_content_type: Option<String>,
}

impl ForgeServerExternalRequestContract {
    pub(crate) fn new(
        route_family: ForgeServerCompatHttpRouteFamily,
        method: String,
        normalized_path: String,
        normalized_query_pairs: Vec<(String, String)>,
        canonical_headers: ForgeServerCanonicalHeaderSet,
        representation: ForgeServerNegotiatedRepresentation,
        version: ForgeServerCompatibilityVersion,
        diagnostics_profile: Option<DiagnosticRichnessProfile>,
        body_present: bool,
        body_content_type: Option<String>,
    ) -> Self {
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

    pub fn route_family(&self) -> ForgeServerCompatHttpRouteFamily {
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

    pub fn canonical_headers(&self) -> &ForgeServerCanonicalHeaderSet {
        &self.canonical_headers
    }

    pub fn representation(&self) -> ForgeServerNegotiatedRepresentation {
        self.representation
    }

    pub fn version(&self) -> ForgeServerCompatibilityVersion {
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
