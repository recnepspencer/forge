use worth_query::facade::foundation::WorthQueryAsyncResourceRequestIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthServerExternalResourceBudget {
    max_request_bytes: usize,
    max_response_bytes: usize,
    deadline_millis: u64,
}

impl WorthServerExternalResourceBudget {
    pub fn bounded(
        max_request_bytes: usize,
        max_response_bytes: usize,
        deadline_millis: u64,
    ) -> Result<Self, WorthServerExternalResourceIntentError> {
        if max_request_bytes == 0 || max_response_bytes == 0 || deadline_millis == 0 {
            return Err(WorthServerExternalResourceIntentError::InvalidBudget);
        }
        Ok(Self {
            max_request_bytes,
            max_response_bytes,
            deadline_millis,
        })
    }

    pub fn max_request_bytes(&self) -> usize {
        self.max_request_bytes
    }

    pub fn max_response_bytes(&self) -> usize {
        self.max_response_bytes
    }

    pub fn deadline_millis(&self) -> u64 {
        self.deadline_millis
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerExternalResourceIntent {
    request_identity: WorthQueryAsyncResourceRequestIdentity,
    provider_identity: String,
    contract_identity: String,
    basis_identity: String,
    request_body: Vec<u8>,
    budget: WorthServerExternalResourceBudget,
}

impl WorthServerExternalResourceIntent {
    pub fn builder() -> WorthServerExternalResourceIntentBuilder {
        WorthServerExternalResourceIntentBuilder::default()
    }

    pub fn request_identity(&self) -> &WorthQueryAsyncResourceRequestIdentity {
        &self.request_identity
    }

    pub fn provider_identity(&self) -> &str {
        &self.provider_identity
    }

    pub fn contract_identity(&self) -> &str {
        &self.contract_identity
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn request_body(&self) -> &[u8] {
        &self.request_body
    }

    pub fn budget(&self) -> WorthServerExternalResourceBudget {
        self.budget
    }
}

#[derive(Clone, Debug, Default)]
pub struct WorthServerExternalResourceIntentBuilder {
    request_identity: Option<WorthQueryAsyncResourceRequestIdentity>,
    provider_identity: Option<String>,
    contract_identity: Option<String>,
    basis_identity: Option<String>,
    request_body: Option<Vec<u8>>,
    budget: Option<WorthServerExternalResourceBudget>,
}

impl WorthServerExternalResourceIntentBuilder {
    pub fn with_request_identity(
        mut self,
        request_identity: WorthQueryAsyncResourceRequestIdentity,
    ) -> Self {
        self.request_identity = Some(request_identity);
        self
    }

    pub fn with_provider_identity(mut self, provider_identity: impl Into<String>) -> Self {
        self.provider_identity = Some(provider_identity.into());
        self
    }

    pub fn with_contract_identity(mut self, contract_identity: impl Into<String>) -> Self {
        self.contract_identity = Some(contract_identity.into());
        self
    }

    pub fn with_basis_identity(mut self, basis_identity: impl Into<String>) -> Self {
        self.basis_identity = Some(basis_identity.into());
        self
    }

    pub fn with_request_body(mut self, request_body: Vec<u8>) -> Self {
        self.request_body = Some(request_body);
        self
    }

    pub fn with_budget(mut self, budget: WorthServerExternalResourceBudget) -> Self {
        self.budget = Some(budget);
        self
    }

    pub fn build(
        self,
    ) -> Result<WorthServerExternalResourceIntent, WorthServerExternalResourceIntentError> {
        Ok(WorthServerExternalResourceIntent {
            request_identity: self
                .request_identity
                .ok_or(WorthServerExternalResourceIntentError::MissingRequestIdentity)?,
            provider_identity: required_text(
                self.provider_identity,
                WorthServerExternalResourceIntentError::MissingProviderIdentity,
            )?,
            contract_identity: required_text(
                self.contract_identity,
                WorthServerExternalResourceIntentError::MissingContractIdentity,
            )?,
            basis_identity: required_text(
                self.basis_identity,
                WorthServerExternalResourceIntentError::MissingBasisIdentity,
            )?,
            request_body: self
                .request_body
                .ok_or(WorthServerExternalResourceIntentError::MissingRequestBody)?,
            budget: self
                .budget
                .ok_or(WorthServerExternalResourceIntentError::MissingBudget)?,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerExternalResourceIntentError {
    MissingRequestIdentity,
    MissingProviderIdentity,
    MissingContractIdentity,
    MissingBasisIdentity,
    MissingRequestBody,
    MissingBudget,
    InvalidBudget,
}

fn required_text(
    value: Option<String>,
    missing: WorthServerExternalResourceIntentError,
) -> Result<String, WorthServerExternalResourceIntentError> {
    value
        .filter(|value| !value.trim().is_empty())
        .ok_or(missing)
}
