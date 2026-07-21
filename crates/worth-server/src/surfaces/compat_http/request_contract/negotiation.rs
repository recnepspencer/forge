use super::{
    canonicalization::WorthServerCanonicalCompatibilityRequest, WorthServerCompatibilityVersion,
    WorthServerNegotiatedRepresentation,
};
use crate::{WorthServerCompatibilityDenial, WorthServerCompatibilityDenialCode};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthServerCompatibilityNegotiation {
    representation: WorthServerNegotiatedRepresentation,
    version: WorthServerCompatibilityVersion,
}

impl WorthServerCompatibilityNegotiation {
    pub(crate) fn representation(self) -> WorthServerNegotiatedRepresentation {
        self.representation
    }

    pub(crate) fn version(self) -> WorthServerCompatibilityVersion {
        self.version
    }
}

pub(crate) fn negotiate_request(
    canonical_request: &WorthServerCanonicalCompatibilityRequest,
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
) -> Result<WorthServerCompatibilityNegotiation, WorthServerCompatibilityDenial> {
    let representation = negotiate_representation(canonical_request, diagnostics_profile)?;
    let version = negotiate_version(canonical_request, diagnostics_profile)?;
    Ok(WorthServerCompatibilityNegotiation {
        representation,
        version,
    })
}

fn negotiate_representation(
    canonical_request: &WorthServerCanonicalCompatibilityRequest,
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
) -> Result<WorthServerNegotiatedRepresentation, WorthServerCompatibilityDenial> {
    let accept_values = canonical_request
        .canonical_headers()
        .values("accept")
        .unwrap_or(&[]);
    let accepts_json = accept_values
        .iter()
        .flat_map(|value| value.split(','))
        .map(str::trim)
        .any(|token| matches!(token, "application/json" | "*/*"));
    let accepts_binary = accept_values
        .iter()
        .flat_map(|value| value.split(','))
        .map(str::trim)
        .any(|token| matches!(token, "application/octet-stream" | "*/*"));

    match canonical_request.route_family() {
        crate::WorthServerCompatHttpRouteFamily::Download => {
            if accept_values.is_empty() || accepts_binary {
                Ok(WorthServerNegotiatedRepresentation::Binary)
            } else {
                Err(WorthServerCompatibilityDenial::new(
                    WorthServerCompatibilityDenialCode::UnsupportedRepresentation,
                    diagnostics_profile,
                    format!("unsupported accept header `{}`", accept_values.join(", ")),
                ))
            }
        }
        _ => {
            if accept_values.is_empty() || accepts_json {
                Ok(WorthServerNegotiatedRepresentation::Json)
            } else {
                Err(WorthServerCompatibilityDenial::new(
                    WorthServerCompatibilityDenialCode::UnsupportedRepresentation,
                    diagnostics_profile,
                    format!("unsupported accept header `{}`", accept_values.join(", ")),
                ))
            }
        }
    }
}

fn negotiate_version(
    canonical_request: &WorthServerCanonicalCompatibilityRequest,
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
) -> Result<WorthServerCompatibilityVersion, WorthServerCompatibilityDenial> {
    let version_values = canonical_request
        .canonical_headers()
        .values("x-Worth-api-version")
        .unwrap_or(&[]);
    if version_values.is_empty() {
        return Ok(WorthServerCompatibilityVersion::V1);
    }

    let mut normalized_versions = version_values.iter().map(|value| value.trim());
    let Some(first_version) = normalized_versions.next() else {
        return Ok(WorthServerCompatibilityVersion::V1);
    };
    if first_version != "1" || normalized_versions.any(|value| value != first_version) {
        Err(WorthServerCompatibilityDenial::new(
            WorthServerCompatibilityDenialCode::UnsupportedApiVersion,
            diagnostics_profile,
            format!(
                "unsupported compatibility API version `{}`",
                version_values.join(", ")
            ),
        ))
    } else {
        Ok(WorthServerCompatibilityVersion::V1)
    }
}
