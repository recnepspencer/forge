#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerProductOperationDenialCode {
    PayloadSchemaMismatch,
    DeclaredPayloadValidator,
    ProductSemantic,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductOperationDenialFacts {
    code: ForgeServerProductOperationDenialCode,
}

impl ForgeServerProductOperationDenialFacts {
    pub fn code(&self) -> ForgeServerProductOperationDenialCode {
        self.code
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductOperationDenial {
    reason_key: String,
    detail: String,
    facts: Option<ForgeServerProductOperationDenialFacts>,
}

impl ForgeServerProductOperationDenial {
    pub fn new(reason_key: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            reason_key: reason_key.into(),
            detail: detail.into(),
            facts: None,
        }
    }

    pub(crate) fn with_code(mut self, code: ForgeServerProductOperationDenialCode) -> Self {
        self.facts = Some(ForgeServerProductOperationDenialFacts { code });
        self
    }

    pub fn reason_key(&self) -> &str {
        &self.reason_key
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn facts(&self) -> Option<&ForgeServerProductOperationDenialFacts> {
        self.facts.as_ref()
    }
}
