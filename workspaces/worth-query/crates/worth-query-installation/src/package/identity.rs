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
    hash_domain_identity(&mut hasher, package);
    hash_requirements(&mut hasher, package);
    hash_definitions(&mut hasher, package);
    hash_domain_operations(&mut hasher, package);
    hash_contracts_and_schemas(&mut hasher, package);
    for contribution in &package.contributions {
        hash_text_field(&mut hasher, "contribution", contribution.as_str());
    }
    WorthQueryPortableDomainPackageIdentity(format!("{:x}", hasher.finalize()))
}

fn hash_domain_identity(hasher: &mut Sha256, package: &WorthQueryPortableDomainPackage) {
    hash_text_field(hasher, "domain-owner", package.identity.owner());
    hash_text_field(
        hasher,
        "domain-major",
        &package.identity.major().to_string(),
    );
    hash_text_field(
        hasher,
        "domain-minor",
        &package.identity.minor().to_string(),
    );
}

fn hash_requirements(hasher: &mut Sha256, package: &WorthQueryPortableDomainPackage) {
    for capability in &package.capabilities {
        hash_text_field(hasher, "capability", capability.as_str());
    }
    for configuration in &package.configuration {
        hash_text_field(hasher, "configuration", configuration.as_str());
    }
    for operating in &package.operating {
        hash_text_field(hasher, "operating", operating.as_str());
    }
}

fn hash_definitions(hasher: &mut Sha256, package: &WorthQueryPortableDomainPackage) {
    for definition in &package.definitions {
        hash_text_field(hasher, "definition-kind", definition.kind().as_str());
        hash_text_field(hasher, "definition-slot", definition.slot());
        hash_text_field(hasher, "definition-semantics", definition.semantics());
    }
}

fn hash_domain_operations(hasher: &mut Sha256, package: &WorthQueryPortableDomainPackage) {
    for operation in &package.domain_operations {
        hash_text_field(
            hasher,
            "domain-operation-slot",
            &operation.identity().slot(),
        );
        hash_text_field(
            hasher,
            "domain-operation-identity",
            operation.canonical_identity(),
        );
    }
}

fn hash_contracts_and_schemas(hasher: &mut Sha256, package: &WorthQueryPortableDomainPackage) {
    for contract in &package.artifact_contracts {
        hash_text_field(
            hasher,
            "artifact-contract-identity",
            contract.identity().as_str(),
        );
    }
    for schema in &package.application_schemas {
        hash_text_field(hasher, "application-schema-owner", schema.owner());
        hash_text_field(hasher, "application-schema-name", schema.name());
        hash_text_field(
            hasher,
            "application-schema-identity",
            schema.identity().as_str(),
        );
    }
}
