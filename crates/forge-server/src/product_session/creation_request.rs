#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductSessionCreationRequest {
    operation_name: String,
    basis_digest: Option<String>,
    expiry_seconds: u64,
}

impl ForgeServerProductSessionCreationRequest {
    pub fn for_operation(operation_name: impl Into<String>) -> Self {
        Self {
            operation_name: operation_name.into(),
            basis_digest: None,
            expiry_seconds: 300,
        }
    }

    pub fn with_basis_digest(mut self, basis_digest: impl Into<String>) -> Self {
        self.basis_digest = Some(basis_digest.into());
        self
    }

    pub fn with_expiry_seconds(mut self, expiry_seconds: u64) -> Self {
        self.expiry_seconds = expiry_seconds;
        self
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn basis_digest(&self) -> Option<&str> {
        self.basis_digest.as_deref()
    }

    pub fn expiry_seconds(&self) -> u64 {
        self.expiry_seconds
    }
}
