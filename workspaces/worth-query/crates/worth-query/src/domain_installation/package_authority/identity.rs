use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainIdentityComponentError {
    Empty,
    InvalidCharacter,
    InvalidBoundary,
}
use crate::evidence_identity::WorthQueryEvidenceIdentity;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryDomainIdentityNamespace(String);

impl WorthQueryDomainIdentityNamespace {
    pub fn new(value: impl Into<String>) -> Result<Self, WorthQueryDomainIdentityComponentError> {
        validate_identity_component(value.into(), true).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryDomainIdentityName(String);

impl WorthQueryDomainIdentityName {
    pub fn new(value: impl Into<String>) -> Result<Self, WorthQueryDomainIdentityComponentError> {
        validate_identity_component(value.into(), false).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryDomainSemanticVersion {
    major: u32,
    minor: u32,
}

impl WorthQueryDomainSemanticVersion {
    pub const fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }
    pub const fn major(self) -> u32 {
        self.major
    }
    pub const fn minor(self) -> u32 {
        self.minor
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainIdentityDeclaration<D> {
    namespace: WorthQueryDomainIdentityNamespace,
    name: WorthQueryDomainIdentityName,
    semantic_version: WorthQueryDomainSemanticVersion,
    marker: PhantomData<fn() -> D>,
}

impl<D> WorthQueryDomainIdentityDeclaration<D> {
    pub fn new(
        namespace: WorthQueryDomainIdentityNamespace,
        name: WorthQueryDomainIdentityName,
        semantic_version: WorthQueryDomainSemanticVersion,
    ) -> Self {
        Self {
            namespace,
            name,
            semantic_version,
            marker: PhantomData,
        }
    }

    pub fn namespace(&self) -> &WorthQueryDomainIdentityNamespace {
        &self.namespace
    }
    pub fn name(&self) -> &WorthQueryDomainIdentityName {
        &self.name
    }
    pub fn semantic_version(&self) -> WorthQueryDomainSemanticVersion {
        self.semantic_version
    }

    pub(crate) fn canonical_owner(&self) -> String {
        format!("{}.{}", self.namespace.as_str(), self.name.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainPackageIdentity(WorthQueryEvidenceIdentity);

impl WorthQueryDomainPackageIdentity {
    pub(crate) fn new(identity: WorthQueryEvidenceIdentity) -> Self {
        Self(identity)
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    pub(crate) fn evidence_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.0
    }
}

fn validate_identity_component(
    value: String,
    allow_period: bool,
) -> Result<String, WorthQueryDomainIdentityComponentError> {
    if value.is_empty() {
        return Err(WorthQueryDomainIdentityComponentError::Empty);
    }
    if value.starts_with(['.', '-', '_']) || value.ends_with(['.', '-', '_']) {
        return Err(WorthQueryDomainIdentityComponentError::InvalidBoundary);
    }
    if !value.chars().all(|character| {
        character.is_ascii_alphanumeric()
            || character == '-'
            || character == '_'
            || (allow_period && character == '.')
    }) {
        return Err(WorthQueryDomainIdentityComponentError::InvalidCharacter);
    }
    Ok(value)
}
