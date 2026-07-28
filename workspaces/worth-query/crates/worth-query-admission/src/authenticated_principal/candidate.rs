use std::time::SystemTime;

use worth_query_declaration::facade::authentication::WorthQueryExternalPrincipalIdentity;

use super::{WorthQueryAuthenticationAudience, WorthQueryAuthenticationMethod};

const MAX_ATTRIBUTE_COUNT: usize = 32;
const MAX_ATTRIBUTE_KEY_BYTES: usize = 64;
const MAX_ATTRIBUTE_VALUE_BYTES: usize = 1_024;
const MAX_ATTRIBUTE_TOTAL_BYTES: usize = 8_192;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPrincipalAttribute {
    key: String,
    value: String,
}

impl WorthQueryPrincipalAttribute {
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, WorthQueryValidatedExternalPrincipalDenial> {
        let key = key.into();
        let value = value.into();
        if !valid_text(&key, MAX_ATTRIBUTE_KEY_BYTES)
            || !valid_text(&value, MAX_ATTRIBUTE_VALUE_BYTES)
        {
            return Err(WorthQueryValidatedExternalPrincipalDenial::InvalidAttribute);
        }
        Ok(Self { key, value })
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Non-authoritative evidence returned by one authentication adapter.
///
/// Possessing or constructing this value grants no Query authority. Only an
/// admitted adapter wrapper can validate it and mint a sealed proof.
pub struct WorthQueryValidatedExternalPrincipal {
    identity: WorthQueryExternalPrincipalIdentity,
    audience: WorthQueryAuthenticationAudience,
    method: WorthQueryAuthenticationMethod,
    validated_at: SystemTime,
    expires_at: SystemTime,
    attributes: Vec<WorthQueryPrincipalAttribute>,
}

impl WorthQueryValidatedExternalPrincipal {
    pub fn new(
        identity: WorthQueryExternalPrincipalIdentity,
        audience: WorthQueryAuthenticationAudience,
        method: WorthQueryAuthenticationMethod,
        validated_at: SystemTime,
        expires_at: SystemTime,
        attributes: Vec<WorthQueryPrincipalAttribute>,
    ) -> Result<Self, WorthQueryValidatedExternalPrincipalDenial> {
        if expires_at <= validated_at {
            return Err(WorthQueryValidatedExternalPrincipalDenial::InvalidTimeRange);
        }
        if attributes.len() > MAX_ATTRIBUTE_COUNT {
            return Err(WorthQueryValidatedExternalPrincipalDenial::TooManyAttributes);
        }
        let total_bytes = attributes.iter().fold(0_usize, |total, attribute| {
            total
                .saturating_add(attribute.key.len())
                .saturating_add(attribute.value.len())
        });
        if total_bytes > MAX_ATTRIBUTE_TOTAL_BYTES {
            return Err(WorthQueryValidatedExternalPrincipalDenial::AttributesTooLarge);
        }
        Ok(Self {
            identity,
            audience,
            method,
            validated_at,
            expires_at,
            attributes,
        })
    }

    pub fn identity(&self) -> &WorthQueryExternalPrincipalIdentity {
        &self.identity
    }

    pub fn audience(&self) -> &WorthQueryAuthenticationAudience {
        &self.audience
    }

    pub fn method(&self) -> &WorthQueryAuthenticationMethod {
        &self.method
    }

    pub fn validated_at(&self) -> SystemTime {
        self.validated_at
    }

    pub fn expires_at(&self) -> SystemTime {
        self.expires_at
    }

    pub fn attributes(&self) -> &[WorthQueryPrincipalAttribute] {
        &self.attributes
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        WorthQueryExternalPrincipalIdentity,
        WorthQueryAuthenticationAudience,
        WorthQueryAuthenticationMethod,
        SystemTime,
        SystemTime,
        Vec<WorthQueryPrincipalAttribute>,
    ) {
        (
            self.identity,
            self.audience,
            self.method,
            self.validated_at,
            self.expires_at,
            self.attributes,
        )
    }
}

impl std::fmt::Debug for WorthQueryValidatedExternalPrincipal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryValidatedExternalPrincipal")
            .field("identity", &self.identity)
            .field("audience", &self.audience)
            .field("method", &self.method)
            .field("validated_at", &self.validated_at)
            .field("expires_at", &self.expires_at)
            .field("attribute_count", &self.attributes.len())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryValidatedExternalPrincipalDenial {
    InvalidTimeRange,
    InvalidAttribute,
    TooManyAttributes,
    AttributesTooLarge,
}

fn valid_text(value: &str, maximum_bytes: usize) -> bool {
    !value.is_empty()
        && value.trim() == value
        && value.len() <= maximum_bytes
        && !value.chars().any(char::is_control)
}
