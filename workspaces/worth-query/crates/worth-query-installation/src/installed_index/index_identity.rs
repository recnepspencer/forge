use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use super::WorthQueryInstalledPackageRecord;
use crate::canonical_hash_encoding::hash_text_field;
use crate::domain_computation::WorthQueryPortableArtifactContract;
use crate::domain_operation::WorthQueryValidatedDomainOperation;
use crate::generation::{WorthQueryInstallationGeneration, WorthQueryInstallationRuntimeIdentity};
use crate::package::{
    WorthQueryPortableDefinition, WorthQueryPortableDefinitionKind,
    WorthQueryPortableDomainPackageIdentity,
};
use worth_query_declaration::facade::application_schema::ErasedApplicationSchemaDeclaration;

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

pub(super) struct IndexIdentityInput<'a> {
    pub runtime: &'a WorthQueryInstallationRuntimeIdentity,
    pub generation: WorthQueryInstallationGeneration,
    pub records: &'a BTreeMap<String, WorthQueryInstalledPackageRecord>,
    pub definitions: &'a BTreeMap<
        (WorthQueryPortableDefinitionKind, String, String),
        WorthQueryPortableDefinition,
    >,
    pub domain_operations: &'a BTreeMap<(String, String), WorthQueryValidatedDomainOperation>,
    pub artifact_contracts:
        &'a BTreeMap<(String, String, u32, u32), WorthQueryPortableArtifactContract>,
    pub application_schemas: &'a BTreeMap<(String, String), ErasedApplicationSchemaDeclaration>,
}

pub(super) fn index_identity(input: IndexIdentityInput<'_>) -> String {
    let mut hasher = Sha256::new();
    hash_text_field(&mut hasher, "runtime", &input.runtime.ordinal().to_string());
    hash_text_field(
        &mut hasher,
        "generation",
        &input.generation.ordinal().to_string(),
    );
    hash_records(&mut hasher, input.records);
    hash_definitions(&mut hasher, input.definitions);
    hash_domain_operations(&mut hasher, input.domain_operations);
    hash_artifact_contracts(&mut hasher, input.artifact_contracts);
    hash_application_schemas(&mut hasher, input.application_schemas);
    format!("{:x}", hasher.finalize())
}

fn hash_records(hasher: &mut Sha256, records: &BTreeMap<String, WorthQueryInstalledPackageRecord>) {
    for (owner, record) in records {
        hash_text_field(hasher, "package-owner", owner);
        hash_text_field(
            hasher,
            "package-identity",
            record.package.package().identity().as_str(),
        );
        hash_text_field(
            hasher,
            "admission-identity",
            record.package.admission_identity(),
        );
    }
}

fn hash_definitions(
    hasher: &mut Sha256,
    definitions: &BTreeMap<
        (WorthQueryPortableDefinitionKind, String, String),
        WorthQueryPortableDefinition,
    >,
) {
    for ((kind, owner, slot), definition) in definitions {
        hash_text_field(hasher, "definition-kind", kind.as_str());
        hash_text_field(hasher, "definition-owner", owner);
        hash_text_field(hasher, "definition-slot", slot);
        hash_text_field(hasher, "definition-semantics", definition.semantics());
    }
}

fn hash_domain_operations(
    hasher: &mut Sha256,
    domain_operations: &BTreeMap<(String, String), WorthQueryValidatedDomainOperation>,
) {
    for ((owner, slot), operation) in domain_operations {
        let operation = operation.definition();
        hash_text_field(hasher, "domain-operation-owner", owner);
        hash_text_field(hasher, "domain-operation-slot", slot);
        hash_text_field(
            hasher,
            "domain-operation-identity",
            operation.canonical_identity(),
        );
    }
}

fn hash_artifact_contracts(
    hasher: &mut Sha256,
    artifact_contracts: &BTreeMap<(String, String, u32, u32), WorthQueryPortableArtifactContract>,
) {
    for ((owner, family, schema, protocol), contract) in artifact_contracts {
        hash_text_field(hasher, "artifact-contract-owner", owner);
        hash_text_field(hasher, "artifact-contract-family", family);
        hash_text_field(hasher, "artifact-contract-schema", &schema.to_string());
        hash_text_field(hasher, "artifact-contract-protocol", &protocol.to_string());
        hash_text_field(
            hasher,
            "artifact-contract-identity",
            contract.identity().as_str(),
        );
    }
}

fn hash_application_schemas(
    hasher: &mut Sha256,
    application_schemas: &BTreeMap<(String, String), ErasedApplicationSchemaDeclaration>,
) {
    for ((owner, name), schema) in application_schemas {
        hash_text_field(hasher, "application-schema-owner", owner);
        hash_text_field(hasher, "application-schema-name", name);
        hash_text_field(
            hasher,
            "application-schema-identity",
            schema.identity().as_str(),
        );
    }
}
