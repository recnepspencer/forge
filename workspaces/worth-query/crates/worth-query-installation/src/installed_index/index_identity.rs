use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use super::WorthQueryInstalledPackageRecord;
use crate::canonical_hash_encoding::hash_text_field;
use crate::domain_operation::WorthQueryValidatedDomainOperation;
use crate::generation::{WorthQueryInstallationGeneration, WorthQueryInstallationRuntimeIdentity};
use crate::package::{
    WorthQueryPortableDefinition, WorthQueryPortableDefinitionKind,
    WorthQueryPortableDomainPackageIdentity,
};

pub(super) fn authority_nonce(
    runtime: &WorthQueryInstallationRuntimeIdentity,
    generation: WorthQueryInstallationGeneration,
    package: &WorthQueryPortableDomainPackageIdentity,
    admission_identity: &str,
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(runtime.ordinal().to_le_bytes());
    hasher.update(generation.ordinal().to_le_bytes());
    hasher.update(package.as_str().as_bytes());
    hasher.update(admission_identity.as_bytes());
    hasher.finalize().into()
}

pub(super) fn index_identity(
    runtime: &WorthQueryInstallationRuntimeIdentity,
    generation: WorthQueryInstallationGeneration,
    records: &BTreeMap<String, WorthQueryInstalledPackageRecord>,
    definitions: &BTreeMap<
        (WorthQueryPortableDefinitionKind, String, String),
        WorthQueryPortableDefinition,
    >,
    domain_operations: &BTreeMap<(String, String), WorthQueryValidatedDomainOperation>,
) -> String {
    let mut hasher = Sha256::new();
    hash_text_field(&mut hasher, "runtime", &runtime.ordinal().to_string());
    hash_text_field(&mut hasher, "generation", &generation.ordinal().to_string());
    for (owner, record) in records {
        hash_text_field(&mut hasher, "package-owner", owner);
        hash_text_field(
            &mut hasher,
            "package-identity",
            record.package.package().identity().as_str(),
        );
        hash_text_field(
            &mut hasher,
            "admission-identity",
            record.package.admission_identity(),
        );
    }
    for ((kind, owner, slot), definition) in definitions {
        hash_text_field(&mut hasher, "definition-kind", kind.as_str());
        hash_text_field(&mut hasher, "definition-owner", owner);
        hash_text_field(&mut hasher, "definition-slot", slot);
        hash_text_field(&mut hasher, "definition-semantics", definition.semantics());
    }
    for ((owner, slot), operation) in domain_operations {
        let operation = operation.definition();
        hash_text_field(&mut hasher, "domain-operation-owner", owner);
        hash_text_field(&mut hasher, "domain-operation-slot", slot);
        hash_text_field(
            &mut hasher,
            "domain-operation-identity",
            operation.canonical_identity(),
        );
    }
    format!("{:x}", hasher.finalize())
}
