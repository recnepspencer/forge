use worth_foundational::facade::CanonicalizationRuleVersion;

pub trait WorthQueryArtifactFamily: 'static {
    const SEMANTIC_FAMILY: &'static str;
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryArtifactFamilyIdentity(String);

impl WorthQueryArtifactFamilyIdentity {
    pub(crate) fn declared<F: WorthQueryArtifactFamily>() -> Self {
        Self(F::SEMANTIC_FAMILY.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn is_portable(&self) -> bool {
        !self.0.is_empty()
            && self.0.trim() == self.0
            && !self.0.chars().any(char::is_whitespace)
            && self.0.contains('.')
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryArtifactSchemaVersion(u32);

impl WorthQueryArtifactSchemaVersion {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryArtifactProtocolVersion(u32);

impl WorthQueryArtifactProtocolVersion {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactContentIdentityContract {
    OwnerCanonicalProjection {
        projection_family: String,
        rule_version: CanonicalizationRuleVersion,
    },
    CallerDigestDefined,
}

impl WorthQueryArtifactContentIdentityContract {
    pub fn owner_canonical_projection(
        projection_family: impl Into<String>,
        rule_version: CanonicalizationRuleVersion,
    ) -> Self {
        Self::OwnerCanonicalProjection {
            projection_family: projection_family.into(),
            rule_version,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactContractIdentity(String);

impl WorthQueryArtifactContractIdentity {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn minted(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactContractReference {
    family: WorthQueryArtifactFamilyIdentity,
    schema_version: WorthQueryArtifactSchemaVersion,
    protocol_version: WorthQueryArtifactProtocolVersion,
}

impl WorthQueryArtifactContractReference {
    pub(crate) fn new(
        family: WorthQueryArtifactFamilyIdentity,
        schema_version: WorthQueryArtifactSchemaVersion,
        protocol_version: WorthQueryArtifactProtocolVersion,
    ) -> Self {
        Self {
            family,
            schema_version,
            protocol_version,
        }
    }

    pub fn family(&self) -> &WorthQueryArtifactFamilyIdentity {
        &self.family
    }

    pub const fn schema_version(&self) -> WorthQueryArtifactSchemaVersion {
        self.schema_version
    }

    pub const fn protocol_version(&self) -> WorthQueryArtifactProtocolVersion {
        self.protocol_version
    }
}
