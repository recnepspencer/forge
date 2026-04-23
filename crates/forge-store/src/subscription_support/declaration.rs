use super::{
    admission_error, stable_digest, SubscriptionSupportDeclarationDigest,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind, SubscriptionSupportRole,
    SUBSCRIPTION_SUPPORT_FAMILY_VERSION,
};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportAuthority {
    ForgeQuery,
    ForgeRuntimeBridge,
    Unadmitted(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportPayloadDigest(pub(crate) String);

impl SubscriptionSupportPayloadDigest {
    pub fn new(value: impl Into<String>) -> Result<Self, StoreError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(admission_error(
                "subscription-support payload digests must be non-empty",
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportScope {
    keys: Vec<String>,
}

impl SubscriptionSupportScope {
    pub fn from_canonical(keys: Vec<String>) -> Result<Self, StoreError> {
        if keys.is_empty() {
            return Err(admission_error(
                "subscription-support scopes must name at least one stable key",
            ));
        }
        if keys.iter().any(|key| key.trim().is_empty()) {
            return Err(admission_error(
                "subscription-support scope keys must be non-empty",
            ));
        }
        if keys.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(admission_error(
                "subscription-support scope keys must be canonical sorted unique order",
            ));
        }
        Ok(Self { keys })
    }

    pub fn canonicalize(mut keys: Vec<String>) -> Result<Self, StoreError> {
        keys.sort();
        keys.dedup();
        Self::from_canonical(keys)
    }

    pub fn keys(&self) -> &[String] {
        &self.keys
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RawSubscriptionSupportDeclaration {
    pub(crate) family_id: SubscriptionSupportFamilyId,
    pub(crate) family_kind: SubscriptionSupportFamilyKind,
    pub(crate) role: SubscriptionSupportRole,
    pub(crate) authority: SubscriptionSupportAuthority,
    pub(crate) family_version: u16,
    pub(crate) compatibility_binding: String,
    pub(crate) scope: SubscriptionSupportScope,
    pub(crate) payload_digest: SubscriptionSupportPayloadDigest,
}

impl RawSubscriptionSupportDeclaration {
    pub fn new(
        family_id: SubscriptionSupportFamilyId,
        family_kind: SubscriptionSupportFamilyKind,
        role: SubscriptionSupportRole,
        authority: SubscriptionSupportAuthority,
        compatibility_binding: impl Into<String>,
        scope: SubscriptionSupportScope,
        payload_digest: SubscriptionSupportPayloadDigest,
    ) -> Self {
        Self {
            family_id,
            family_kind,
            role,
            authority,
            family_version: SUBSCRIPTION_SUPPORT_FAMILY_VERSION,
            compatibility_binding: compatibility_binding.into(),
            scope,
            payload_digest,
        }
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn role(&self) -> SubscriptionSupportRole {
        self.role
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdmittedSubscriptionSupportDeclaration {
    pub(crate) declaration: RawSubscriptionSupportDeclaration,
    pub(crate) declaration_digest: SubscriptionSupportDeclarationDigest,
}

impl AdmittedSubscriptionSupportDeclaration {
    pub(crate) fn new(declaration: RawSubscriptionSupportDeclaration) -> Result<Self, StoreError> {
        let declaration_digest = SubscriptionSupportDeclarationDigest(stable_digest(&declaration)?);
        Ok(Self {
            declaration,
            declaration_digest,
        })
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.declaration.family_kind
    }

    pub fn role(&self) -> SubscriptionSupportRole {
        self.declaration.role
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.declaration.family_id
    }

    pub fn declaration_digest(&self) -> &SubscriptionSupportDeclarationDigest {
        &self.declaration_digest
    }

    pub fn payload_digest(&self) -> &SubscriptionSupportPayloadDigest {
        &self.declaration.payload_digest
    }
}
