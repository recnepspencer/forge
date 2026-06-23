#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerOperationReadinessDenialCode {
    MissingQuerySupport,
    UnsupportedQuerySupport,
    DownstreamDeliveryRequiresReadIntent,
    RuntimeBackedResumeUnsupported,
    DurableResumeDeferred,
    UnsupportedProductSupport,
    UnknownProductSupport,
    FixtureOnlyProductSupport,
    IncompatibleSupportBasis,
    InvalidPreconditionInput,
    PreconditionFailed,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ForgeServerOperationReadinessDenialFacts {
    expected_basis_digest: Option<String>,
    observed_basis_digest: Option<String>,
    expected_validator: Option<String>,
    observed_validator: Option<String>,
}

impl ForgeServerOperationReadinessDenialFacts {
    pub fn expected_basis_digest(&self) -> Option<&str> {
        self.expected_basis_digest.as_deref()
    }

    pub fn observed_basis_digest(&self) -> Option<&str> {
        self.observed_basis_digest.as_deref()
    }

    pub fn expected_validator(&self) -> Option<&str> {
        self.expected_validator.as_deref()
    }

    pub fn observed_validator(&self) -> Option<&str> {
        self.observed_validator.as_deref()
    }

    pub(crate) fn with_basis_mismatch(
        mut self,
        expected_basis_digest: impl Into<String>,
        observed_basis_digest: impl Into<String>,
    ) -> Self {
        self.expected_basis_digest = Some(expected_basis_digest.into());
        self.observed_basis_digest = Some(observed_basis_digest.into());
        self
    }

    pub(crate) fn with_validator_mismatch(
        mut self,
        expected_validator: impl Into<String>,
        observed_validator: impl Into<String>,
    ) -> Self {
        self.expected_validator = Some(expected_validator.into());
        self.observed_validator = Some(observed_validator.into());
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationReadinessDenial {
    code: ForgeServerOperationReadinessDenialCode,
    detail: String,
    facts: Option<ForgeServerOperationReadinessDenialFacts>,
}

impl ForgeServerOperationReadinessDenial {
    pub(crate) fn new(
        code: ForgeServerOperationReadinessDenialCode,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            detail: detail.into(),
            facts: None,
        }
    }

    pub(crate) fn with_facts(mut self, facts: ForgeServerOperationReadinessDenialFacts) -> Self {
        self.facts = Some(facts);
        self
    }

    pub fn code(&self) -> ForgeServerOperationReadinessDenialCode {
        self.code.clone()
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn facts(&self) -> Option<&ForgeServerOperationReadinessDenialFacts> {
        self.facts.as_ref()
    }
}
