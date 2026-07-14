use crate::{
    WorthServerCompatibilityPreparedRequest, WorthServerQueryHandoffDenial,
    WorthServerQueryHandoffDenialCode, WorthServerReadValidator,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerConditionalRangeRequest {
    if_range: Option<String>,
    canonical_digest: String,
}

impl WorthServerConditionalRangeRequest {
    pub(crate) fn from_prepared_request(
        prepared_request: &WorthServerCompatibilityPreparedRequest,
    ) -> Result<Self, WorthServerQueryHandoffDenial> {
        let values = prepared_request
            .request_contract()
            .canonical_headers()
            .values("if-range");
        let if_range = match values {
            None => None,
            Some([]) => None,
            Some([value]) => Some(value.trim().to_string()),
            Some(_) => {
                return Err(WorthServerQueryHandoffDenial::new(
                    WorthServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
                    prepared_request
                        .admission()
                        .request_context()
                        .diagnostics_profile(),
                    "binary download accepts at most one If-Range header value",
                ));
            }
        };
        let canonical_digest = format!(
            "compat-http-conditional-range-v1|if_range={}",
            if_range.as_deref().unwrap_or("none"),
        );
        Ok(Self {
            if_range,
            canonical_digest,
        })
    }

    pub fn if_range(&self) -> Option<&str> {
        self.if_range.as_deref()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub(crate) fn admits_range(&self, validator: &WorthServerReadValidator) -> bool {
        self.if_range
            .as_deref()
            .is_none_or(|value| value == validator.entity_tag())
    }
}
