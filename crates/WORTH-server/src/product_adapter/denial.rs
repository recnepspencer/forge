#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerProductOperationDenialCode {
    PayloadSchemaMismatch,
    DeclaredPayloadValidator,
    ProductSemantic,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductOperationDenialFacts {
    code: WorthServerProductOperationDenialCode,
}

impl WorthServerProductOperationDenialFacts {
    pub fn code(&self) -> WorthServerProductOperationDenialCode {
        self.code
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductOperationDenial {
    reason_key: String,
    detail: String,
    facts: Option<WorthServerProductOperationDenialFacts>,
}

impl WorthServerProductOperationDenial {
    pub fn new(reason_key: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            reason_key: reason_key.into(),
            detail: detail.into(),
            facts: None,
        }
    }

    pub(crate) fn with_code(mut self, code: WorthServerProductOperationDenialCode) -> Self {
        self.facts = Some(WorthServerProductOperationDenialFacts { code });
        self
    }

    pub fn reason_key(&self) -> &str {
        &self.reason_key
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn facts(&self) -> Option<&WorthServerProductOperationDenialFacts> {
        self.facts.as_ref()
    }
}
