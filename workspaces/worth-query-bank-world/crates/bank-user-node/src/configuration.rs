use std::num::NonZeroUsize;
use std::time::Duration;

use url::Url;

#[derive(Clone)]
pub struct BankUserNodeConfiguration {
    pub(super) issuer: String,
    pub(super) client_id: String,
    pub(super) client_secret: String,
    pub(super) introspection_url: String,
    pub(super) revocation_url: String,
    pub(super) bank_server_origin: Url,
    pub(super) maximum_deadline: Duration,
    pub(super) maximum_body_bytes: usize,
    pub(super) maximum_request_concurrency: NonZeroUsize,
    pub(super) maximum_live_streams: NonZeroUsize,
}

impl BankUserNodeConfiguration {
    pub fn builder() -> BankUserNodeConfigurationBuilder {
        BankUserNodeConfigurationBuilder::default()
    }
}

impl std::fmt::Debug for BankUserNodeConfiguration {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("BankUserNodeConfiguration")
            .field("issuer", &self.issuer)
            .field("client_id", &self.client_id)
            .field("client_secret", &"<redacted>")
            .field("introspection_url", &self.introspection_url)
            .field("revocation_url", &self.revocation_url)
            .field("bank_server_origin", &self.bank_server_origin)
            .field("maximum_deadline", &self.maximum_deadline)
            .field("maximum_body_bytes", &self.maximum_body_bytes)
            .field(
                "maximum_request_concurrency",
                &self.maximum_request_concurrency,
            )
            .field("maximum_live_streams", &self.maximum_live_streams)
            .finish()
    }
}

#[derive(Default)]
pub struct BankUserNodeConfigurationBuilder {
    issuer: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    introspection_url: Option<String>,
    revocation_url: Option<String>,
    bank_server_origin: Option<String>,
    maximum_request_concurrency: Option<NonZeroUsize>,
    maximum_live_streams: Option<NonZeroUsize>,
}

impl BankUserNodeConfigurationBuilder {
    pub fn issuer(mut self, value: impl Into<String>) -> Self {
        self.issuer = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    pub fn introspection_url(mut self, value: impl Into<String>) -> Self {
        self.introspection_url = Some(value.into());
        self
    }

    pub fn revocation_url(mut self, value: impl Into<String>) -> Self {
        self.revocation_url = Some(value.into());
        self
    }

    pub fn bank_server_origin(mut self, value: impl Into<String>) -> Self {
        self.bank_server_origin = Some(value.into());
        self
    }

    pub fn maximum_request_concurrency(mut self, value: NonZeroUsize) -> Self {
        self.maximum_request_concurrency = Some(value);
        self
    }

    pub fn maximum_live_streams(mut self, value: NonZeroUsize) -> Self {
        self.maximum_live_streams = Some(value);
        self
    }

    pub fn build(self) -> Result<BankUserNodeConfiguration, BankUserNodeConfigurationError> {
        let bank_server_origin = Url::parse(
            self.bank_server_origin
                .as_deref()
                .ok_or(BankUserNodeConfigurationError::InvalidBankServerOrigin)?,
        )
        .map_err(|_| BankUserNodeConfigurationError::InvalidBankServerOrigin)?;
        if !matches!(bank_server_origin.scheme(), "http" | "https")
            || bank_server_origin.cannot_be_a_base()
            || bank_server_origin.query().is_some()
            || bank_server_origin.fragment().is_some()
        {
            return Err(BankUserNodeConfigurationError::InvalidBankServerOrigin);
        }
        Ok(BankUserNodeConfiguration {
            issuer: required(self.issuer, BankUserNodeConfigurationError::InvalidIssuer)?,
            client_id: required(
                self.client_id,
                BankUserNodeConfigurationError::InvalidClientId,
            )?,
            client_secret: required(
                self.client_secret,
                BankUserNodeConfigurationError::InvalidClientSecret,
            )?,
            introspection_url: required(
                self.introspection_url,
                BankUserNodeConfigurationError::InvalidIntrospectionUrl,
            )?,
            revocation_url: required(
                self.revocation_url,
                BankUserNodeConfigurationError::InvalidRevocationUrl,
            )?,
            bank_server_origin,
            maximum_deadline: Duration::from_secs(30),
            maximum_body_bytes: 64 * 1_024,
            maximum_request_concurrency: self
                .maximum_request_concurrency
                .unwrap_or_else(|| NonZeroUsize::new(16).expect("constant is nonzero")),
            maximum_live_streams: self
                .maximum_live_streams
                .unwrap_or_else(|| NonZeroUsize::new(16).expect("constant is nonzero")),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankUserNodeConfigurationError {
    InvalidIssuer,
    InvalidClientId,
    InvalidClientSecret,
    InvalidIntrospectionUrl,
    InvalidRevocationUrl,
    InvalidBankServerOrigin,
}

impl std::fmt::Display for BankUserNodeConfigurationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "invalid Bank user-node configuration: {self:?}")
    }
}

impl std::error::Error for BankUserNodeConfigurationError {}

fn required(
    value: Option<String>,
    error: BankUserNodeConfigurationError,
) -> Result<String, BankUserNodeConfigurationError> {
    let value = value.ok_or(error)?;
    if value.is_empty()
        || value.trim() != value
        || value.len() > 2_048
        || value.chars().any(char::is_control)
    {
        return Err(error);
    }
    Ok(value)
}
