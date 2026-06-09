use super::{
    canonicalization::ForgeServerCanonicalCompatibilityRequest, ForgeServerCompatibilityVersion,
    ForgeServerNegotiatedRepresentation,
};
use crate::{ForgeServerCompatibilityDenial, ForgeServerCompatibilityDenialCode};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ForgeServerCompatibilityNegotiation {
    representation: ForgeServerNegotiatedRepresentation,
    version: ForgeServerCompatibilityVersion,
}

impl ForgeServerCompatibilityNegotiation {
    pub(crate) fn representation(self) -> ForgeServerNegotiatedRepresentation {
        self.representation
    }

    pub(crate) fn version(self) -> ForgeServerCompatibilityVersion {
        self.version
    }
}

pub(crate) fn negotiate_request(
    canonical_request: &ForgeServerCanonicalCompatibilityRequest,
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
) -> Result<ForgeServerCompatibilityNegotiation, ForgeServerCompatibilityDenial> {
    let representation = negotiate_representation(canonical_request, diagnostics_profile)?;
    let version = negotiate_version(canonical_request, diagnostics_profile)?;
    Ok(ForgeServerCompatibilityNegotiation {
        representation,
        version,
    })
}

fn negotiate_representation(
    canonical_request: &ForgeServerCanonicalCompatibilityRequest,
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
) -> Result<ForgeServerNegotiatedRepresentation, ForgeServerCompatibilityDenial> {
    let accept_values = canonical_request
        .canonical_headers()
        .values("accept")
        .unwrap_or(&[]);
    if accept_values.is_empty() {
        Ok(ForgeServerNegotiatedRepresentation::Json)
    } else if accept_values
        .iter()
        .flat_map(|value| value.split(','))
        .map(str::trim)
        .any(|token| matches!(token, "application/json" | "*/*"))
    {
        Ok(ForgeServerNegotiatedRepresentation::Json)
    } else {
        Err(ForgeServerCompatibilityDenial::new(
            ForgeServerCompatibilityDenialCode::UnsupportedRepresentation,
            diagnostics_profile,
            format!("unsupported accept header `{}`", accept_values.join(", ")),
        ))
    }
}

fn negotiate_version(
    canonical_request: &ForgeServerCanonicalCompatibilityRequest,
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
) -> Result<ForgeServerCompatibilityVersion, ForgeServerCompatibilityDenial> {
    let version_values = canonical_request
        .canonical_headers()
        .values("x-forge-api-version")
        .unwrap_or(&[]);
    if version_values.is_empty() {
        return Ok(ForgeServerCompatibilityVersion::V1);
    }

    let mut normalized_versions = version_values.iter().map(|value| value.trim());
    let Some(first_version) = normalized_versions.next() else {
        return Ok(ForgeServerCompatibilityVersion::V1);
    };
    if first_version != "1" || normalized_versions.any(|value| value != first_version) {
        Err(ForgeServerCompatibilityDenial::new(
            ForgeServerCompatibilityDenialCode::UnsupportedApiVersion,
            diagnostics_profile,
            format!(
                "unsupported compatibility API version `{}`",
                version_values.join(", ")
            ),
        ))
    } else {
        Ok(ForgeServerCompatibilityVersion::V1)
    }
}
