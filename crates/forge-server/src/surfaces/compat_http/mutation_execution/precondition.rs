use crate::{
    ForgeServerCompatibilityPreparedRequest, ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDenialCode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerMutationPrecondition {
    requested_basis_digest: Option<String>,
    if_match: Option<String>,
    observed_basis_digest: String,
    validator: String,
    request_identity_digest: String,
    canonical_digest: String,
}

impl ForgeServerMutationPrecondition {
    pub(crate) fn from_prepared_request(
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        operation_name: &str,
        mutation_request_digest: &str,
        observed_basis_digest: &str,
    ) -> Result<Self, ForgeServerQueryHandoffDenial> {
        let requested_basis_digest = read_single_query_pair(prepared_request, "basis")?;
        let if_match = read_single_header(prepared_request, "if-match")?;
        let validator = format!(
            "\"compat-http-mutation-validator-v1|basis:{}|operation:{}|request:{}\"",
            observed_basis_digest,
            operation_name.trim(),
            mutation_request_digest,
        );
        let request_identity_digest = format!(
            "compat-http-mutation-request-precondition-v1|basis:{}|if-match:{}",
            requested_basis_digest.as_deref().unwrap_or("none"),
            if_match.as_deref().unwrap_or("none"),
        );
        let canonical_digest = format!(
            "compat-http-mutation-precondition-v1|requested_basis:{}|if-match:{}|observed_basis:{}|validator:{}",
            requested_basis_digest.as_deref().unwrap_or("none"),
            if_match.as_deref().unwrap_or("none"),
            observed_basis_digest,
            validator,
        );
        Ok(Self {
            requested_basis_digest,
            if_match,
            observed_basis_digest: observed_basis_digest.to_string(),
            validator,
            request_identity_digest,
            canonical_digest,
        })
    }

    pub(crate) fn enforce(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
    ) -> Result<(), ForgeServerQueryHandoffDenial> {
        let request_context = prepared_request.admission().request_context();
        if let Some(expected_basis) = self.requested_basis_digest.as_deref() {
            if expected_basis != self.observed_basis_digest {
                return Err(ForgeServerQueryHandoffDenial::new(
                    ForgeServerQueryHandoffDenialCode::CompatibilityMutationPreconditionFailed,
                    request_context.diagnostics_profile(),
                    format!(
                        "compatibility mutation basis precondition `{expected_basis}` did not match the admitted mutation basis `{}`",
                        self.observed_basis_digest,
                    ),
                ));
            }
        }
        if let Some(if_match) = self.if_match.as_deref() {
            if if_match != self.validator {
                return Err(ForgeServerQueryHandoffDenial::new(
                    ForgeServerQueryHandoffDenialCode::CompatibilityMutationPreconditionFailed,
                    request_context.diagnostics_profile(),
                    format!(
                        "compatibility mutation validator precondition `{if_match}` did not match the canonical mutation validator `{}`",
                        self.validator
                    ),
                ));
            }
        }
        Ok(())
    }

    pub fn validator(&self) -> &str {
        &self.validator
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn request_identity_digest(&self) -> &str {
        &self.request_identity_digest
    }
}

fn read_single_query_pair(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    query_name: &str,
) -> Result<Option<String>, ForgeServerQueryHandoffDenial> {
    let request_context = prepared_request.admission().request_context();
    let values = prepared_request
        .request_contract()
        .normalized_query_pairs()
        .iter()
        .filter_map(|(name, value)| (name == query_name).then_some(value.as_str()))
        .collect::<Vec<_>>();
    if values.len() > 1 {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityMutationRequestInvalid,
            request_context.diagnostics_profile(),
            format!("compatibility mutation admits at most one `{query_name}` query value"),
        ));
    }
    Ok(values.first().map(|value| (*value).to_string()))
}

fn read_single_header(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    header_name: &str,
) -> Result<Option<String>, ForgeServerQueryHandoffDenial> {
    let request_context = prepared_request.admission().request_context();
    let Some(values) = prepared_request
        .request_contract()
        .canonical_headers()
        .values(header_name)
    else {
        return Ok(None);
    };
    if values.len() != 1 {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityMutationRequestInvalid,
            request_context.diagnostics_profile(),
            format!(
                "compatibility mutation requires a single canonical `{header_name}` header value"
            ),
        ));
    }
    Ok(Some(values[0].trim().to_string()))
}
