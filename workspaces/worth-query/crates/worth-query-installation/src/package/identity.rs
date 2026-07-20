use sha2::{Digest, Sha256};

use super::WorthQueryPortableDomainPackage;
use crate::canonical_hash_encoding::hash_text_field;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryPortableDomainIdentity {
    owner: String,
    major: u32,
    minor: u32,
}

impl WorthQueryPortableDomainIdentity {
    pub fn new(owner: impl Into<String>, major: u32, minor: u32) -> Self {
        Self {
            owner: owner.into(),
            major,
            minor,
        }
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryPortableDomainPackageIdentity(String);

impl WorthQueryPortableDomainPackageIdentity {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub(super) fn canonical_identity(
    package: &WorthQueryPortableDomainPackage,
) -> WorthQueryPortableDomainPackageIdentity {
    let mut hasher = Sha256::new();
    hash_text_field(&mut hasher, "domain-owner", package.identity.owner());
    hash_text_field(
        &mut hasher,
        "domain-major",
        &package.identity.major().to_string(),
    );
    hash_text_field(
        &mut hasher,
        "domain-minor",
        &package.identity.minor().to_string(),
    );
    for capability in &package.capabilities {
        hash_text_field(&mut hasher, "capability", capability.as_str());
    }
    for configuration in &package.configuration {
        hash_text_field(&mut hasher, "configuration", configuration.as_str());
    }
    for operating in &package.operating {
        hash_text_field(&mut hasher, "operating", operating.as_str());
    }
    for definition in &package.definitions {
        hash_text_field(&mut hasher, "definition-kind", definition.kind().as_str());
        hash_text_field(&mut hasher, "definition-slot", definition.slot());
        hash_text_field(&mut hasher, "definition-semantics", definition.semantics());
    }
    for operation in &package.domain_operations {
        hash_text_field(
            &mut hasher,
            "domain-operation-slot",
            &operation.identity().slot(),
        );
        hash_text_field(
            &mut hasher,
            "domain-operation-identity",
            operation.canonical_identity(),
        );
    }
    for contribution in &package.contributions {
        hash_text_field(&mut hasher, "contribution", contribution.as_str());
    }
    WorthQueryPortableDomainPackageIdentity(format!("{:x}", hasher.finalize()))
}
