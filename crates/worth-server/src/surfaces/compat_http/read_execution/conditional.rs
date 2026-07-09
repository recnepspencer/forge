use crate::{
    WorthServerCompatibilityPreparedRequest, WorthServerQueryHandoffDenial,
    WorthServerQueryHandoffDenialCode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerConditionalRead {
    if_match: Option<String>,
    if_none_match: Option<String>,
    canonical_digest: String,
}

impl WorthServerConditionalRead {
    pub(crate) fn from_prepared_request(
        prepared_request: &WorthServerCompatibilityPreparedRequest,
    ) -> Result<Self, WorthServerQueryHandoffDenial> {
        let request_context = prepared_request.admission().request_context();
        let if_match = read_single_header(prepared_request, "if-match")?;
        let if_none_match = read_single_header(prepared_request, "if-none-match")?;
        if if_match.is_some() && if_none_match.is_some() {
            return Err(WorthServerQueryHandoffDenial::new(
                WorthServerQueryHandoffDenialCode::CompatibilityConditionalRequestInvalid,
                request_context.diagnostics_profile(),
                "compatibility read does not admit simultaneous if-match and if-none-match validators",
            ));
        }

        let canonical_digest = format!(
            "compat-http-conditional-v1|if-match:{}|if-none-match:{}",
            if_match.as_deref().unwrap_or("none"),
            if_none_match.as_deref().unwrap_or("none"),
        );
        Ok(Self {
            if_match,
            if_none_match,
            canonical_digest,
        })
    }

    pub fn if_match(&self) -> Option<&str> {
        self.if_match.as_deref()
    }

    pub fn if_none_match(&self) -> Option<&str> {
        self.if_none_match.as_deref()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

fn read_single_header(
    prepared_request: &WorthServerCompatibilityPreparedRequest,
    header_name: &str,
) -> Result<Option<String>, WorthServerQueryHandoffDenial> {
    let request_context = prepared_request.admission().request_context();
    let values: &[String] = match prepared_request
        .request_contract()
        .canonical_headers()
        .values(header_name)
    {
        Some(values) => values,
        None => return Ok(None),
    };

    if values.len() != 1 {
        return Err(WorthServerQueryHandoffDenial::new(
            WorthServerQueryHandoffDenialCode::CompatibilityConditionalRequestInvalid,
            request_context.diagnostics_profile(),
            format!("compatibility read requires a single canonical `{header_name}` header value"),
        ));
    }

    let value = values[0].trim();
    if value.is_empty() {
        return Err(WorthServerQueryHandoffDenial::new(
            WorthServerQueryHandoffDenialCode::CompatibilityConditionalRequestInvalid,
            request_context.diagnostics_profile(),
            format!("compatibility read `{header_name}` header may not be blank"),
        ));
    }

    Ok(Some(value.to_string()))
}
