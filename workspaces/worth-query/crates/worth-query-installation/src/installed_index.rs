use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

mod application_schema;
mod artifact_contract_admission;
mod artifact_contract_authority;
mod authority;
mod authority_validation;
mod construction;
mod denial;
mod index_identity;
mod rebuild_report;
mod relation;

pub use authority::WorthQueryInstalledPackageAuthority;
pub use denial::{
    WorthQueryInstalledPackageIndexDenial, WorthQueryInstalledPackageIndexDenialKind,
};
pub use rebuild_report::{
    WorthQueryInstalledPackageIndexCounters, WorthQueryInstalledPackageIndexRebuildReport,
};
pub use relation::WorthQueryInstalledPackageIndexRelation;

use crate::admission::WorthQueryAdmittedPortableDomainPackage;
use crate::domain_computation::WorthQueryPortableArtifactContract;
use crate::domain_operation::WorthQueryValidatedDomainOperation;
use crate::generation::{WorthQueryInstallationGeneration, WorthQueryInstallationRuntimeIdentity};
use crate::installed_domain_operation::WorthQueryInstalledDomainOperationAuthority;
use crate::installed_operation::WorthQueryInstalledOperationAuthority;
use crate::package::{WorthQueryPortableDefinition, WorthQueryPortableDefinitionKind};
use worth_query_declaration::facade::application_schema::ErasedApplicationSchemaDeclaration;

#[derive(Debug)]
struct WorthQueryInstalledPackageRecord {
    package: WorthQueryAdmittedPortableDomainPackage,
    authority_nonce: [u8; 32],
}

#[derive(Debug)]
pub struct WorthQueryInstalledPackageIndex {
    runtime: WorthQueryInstallationRuntimeIdentity,
    generation: WorthQueryInstallationGeneration,
    packages: BTreeMap<String, WorthQueryInstalledPackageRecord>,
    definitions:
        BTreeMap<(WorthQueryPortableDefinitionKind, String, String), WorthQueryPortableDefinition>,
    domain_operations: BTreeMap<(String, String), WorthQueryValidatedDomainOperation>,
    artifact_contracts: BTreeMap<(String, String, u32, u32), WorthQueryPortableArtifactContract>,
    application_schemas: BTreeMap<(String, String), ErasedApplicationSchemaDeclaration>,
    identity: String,
    counters: WorthQueryInstalledPackageIndexCounters,
    indexed_operation_lookups: AtomicUsize,
}

impl WorthQueryInstalledPackageIndex {
    pub fn runtime_ordinal(&self) -> u64 {
        self.runtime.ordinal()
    }

    pub fn generation(&self) -> WorthQueryInstallationGeneration {
        self.generation
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn counters(&self) -> WorthQueryInstalledPackageIndexCounters {
        self.counters
    }

    pub fn installed_definition_count(&self) -> usize {
        self.definitions.len()
    }

    pub fn installed_domain_operation_count(&self) -> usize {
        self.domain_operations.len()
    }

    pub fn installed_artifact_contract_count(&self) -> usize {
        self.artifact_contracts.len()
    }

    pub fn installed_application_schema_count(&self) -> usize {
        self.application_schemas.len()
    }

    pub fn indexed_operation_lookups(&self) -> usize {
        self.indexed_operation_lookups.load(Ordering::Relaxed)
    }

    pub fn domain(
        &self,
        owner: &str,
    ) -> Result<WorthQueryInstalledPackageAuthority, WorthQueryInstalledPackageIndexDenial> {
        let record = self.packages.get(owner).ok_or_else(|| {
            WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::DomainNotInstalled,
                owner,
            )
        })?;
        Ok(WorthQueryInstalledPackageAuthority {
            runtime_ordinal: self.runtime.ordinal(),
            generation: self.generation,
            owner: owner.to_string(),
            package_identity: record.package.package().identity().clone(),
            admission_identity: record.package.admission_identity().to_string(),
            authority_nonce: record.authority_nonce,
        })
    }

    pub fn operation(
        &self,
        owner: &str,
        operation_slot: &str,
    ) -> Result<WorthQueryInstalledOperationAuthority, WorthQueryInstalledPackageIndexDenial> {
        let record = self.packages.get(owner).ok_or_else(|| {
            WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::DomainNotInstalled,
                owner,
            )
        })?;
        self.indexed_operation_lookups
            .fetch_add(1, Ordering::Relaxed);
        let semantics = self
            .definitions
            .get(&(
                WorthQueryPortableDefinitionKind::GraphReadOperation,
                owner.to_string(),
                operation_slot.to_string(),
            ))
            .map(WorthQueryPortableDefinition::semantics)
            .ok_or_else(|| {
                WorthQueryInstalledPackageIndexDenial::new(
                    WorthQueryInstalledPackageIndexDenialKind::OperationNotInstalled,
                    operation_slot,
                )
            })?;
        Ok(WorthQueryInstalledOperationAuthority {
            runtime_ordinal: self.runtime.ordinal(),
            generation: self.generation,
            owner: owner.to_string(),
            package_identity: record.package.package().identity().clone(),
            admission_identity: record.package.admission_identity().to_string(),
            package_authority_nonce: record.authority_nonce,
            operation_slot: operation_slot.to_string(),
            operation_semantics: semantics.to_string(),
        })
    }

    pub fn domain_operation(
        &self,
        owner: &str,
        operation_slot: &str,
    ) -> Result<WorthQueryInstalledDomainOperationAuthority, WorthQueryInstalledPackageIndexDenial>
    {
        let record = self.packages.get(owner).ok_or_else(|| {
            WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::DomainNotInstalled,
                owner,
            )
        })?;
        self.indexed_operation_lookups
            .fetch_add(1, Ordering::Relaxed);
        let validated = self
            .domain_operations
            .get(&(owner.to_string(), operation_slot.to_string()))
            .cloned()
            .ok_or_else(|| {
                WorthQueryInstalledPackageIndexDenial::new(
                    WorthQueryInstalledPackageIndexDenialKind::OperationNotInstalled,
                    operation_slot,
                )
            })?;
        Ok(WorthQueryInstalledDomainOperationAuthority {
            runtime_ordinal: self.runtime.ordinal(),
            generation: self.generation,
            owner: owner.to_string(),
            package_identity: record.package.package().identity().clone(),
            admission_identity: record.package.admission_identity().to_string(),
            package_authority_nonce: record.authority_nonce,
            validated,
        })
    }

    pub fn rebuild(&self) -> Self {
        Self::build(
            self.runtime.retained(),
            self.generation,
            self.packages.values().map(|record| record.package.clone()),
        )
        .expect("an admitted installed package set must rebuild without conflict")
    }

    pub fn successor_generation(&self) -> Self {
        Self::build(
            self.runtime.retained(),
            self.generation.successor(),
            self.packages.values().map(|record| record.package.clone()),
        )
        .expect("an admitted installed package set must advance without conflict")
    }

    pub fn rebuild_report(&self) -> WorthQueryInstalledPackageIndexRebuildReport {
        let rebuilt = self.rebuild();
        WorthQueryInstalledPackageIndexRebuildReport::new(
            self.identity.clone(),
            rebuilt.identity.clone(),
            rebuilt.counters,
        )
    }
}

#[cfg(test)]
mod relation_tests;
#[cfg(test)]
mod tests;
