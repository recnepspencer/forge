use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

mod authority;
mod authority_validation;
mod denial;
mod index_identity;
mod rebuild_report;

pub use authority::WorthQueryInstalledPackageAuthority;
pub use denial::{
    WorthQueryInstalledPackageIndexDenial, WorthQueryInstalledPackageIndexDenialKind,
};
pub use rebuild_report::{
    WorthQueryInstalledPackageIndexCounters, WorthQueryInstalledPackageIndexRebuildReport,
};

use crate::admission::WorthQueryAdmittedPortableDomainPackage;
use crate::domain_operation::WorthQueryValidatedDomainOperation;
use crate::generation::{WorthQueryInstallationGeneration, WorthQueryInstallationRuntimeIdentity};
use crate::installed_domain_operation::WorthQueryInstalledDomainOperationAuthority;
use crate::installed_operation::WorthQueryInstalledOperationAuthority;
use crate::package::{WorthQueryPortableDefinition, WorthQueryPortableDefinitionKind};

use index_identity::{authority_nonce, index_identity};

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
    identity: String,
    counters: WorthQueryInstalledPackageIndexCounters,
    indexed_operation_lookups: AtomicUsize,
}

impl WorthQueryInstalledPackageIndex {
    pub fn build(
        runtime: WorthQueryInstallationRuntimeIdentity,
        generation: WorthQueryInstallationGeneration,
        packages: impl IntoIterator<Item = WorthQueryAdmittedPortableDomainPackage>,
    ) -> Result<Self, WorthQueryInstalledPackageIndexDenial> {
        let mut records = BTreeMap::<String, WorthQueryInstalledPackageRecord>::new();
        let mut definitions = BTreeMap::new();
        let mut domain_operations = BTreeMap::new();
        let mut counters = WorthQueryInstalledPackageIndexCounters::default();

        for package in packages {
            let owner = package.package().domain_identity().owner().to_string();
            counters.package_rows_examined += 1;
            if let Some(existing) = records.get(&owner) {
                if existing
                    .package
                    .package()
                    .has_same_authoritative_meaning(package.package())
                {
                    if existing.package.has_same_admission_authority(&package) {
                        counters.equivalent_packages_converged += 1;
                        continue;
                    }
                    return Err(WorthQueryInstalledPackageIndexDenial::new(
                        WorthQueryInstalledPackageIndexDenialKind::ConflictingAdmissionProfile,
                        owner,
                    ));
                }
                return Err(WorthQueryInstalledPackageIndexDenial::new(
                    WorthQueryInstalledPackageIndexDenialKind::ConflictingPackage,
                    owner,
                ));
            }

            for definition in package.package().definitions() {
                counters.definition_rows_examined += 1;
                admit_definition(&mut definitions, &owner, definition)?;
            }
            for operation in package.package().validated_domain_operations() {
                counters.domain_operation_rows_examined += 1;
                domain_operations.insert(
                    (owner.clone(), operation.definition().identity().slot()),
                    operation.clone(),
                );
            }

            let authority_nonce = authority_nonce(
                &runtime,
                generation,
                package.package().identity(),
                package.admission_identity(),
            );
            records.insert(
                owner,
                WorthQueryInstalledPackageRecord {
                    package,
                    authority_nonce,
                },
            );
        }

        counters.installed_package_count = records.len();
        counters.installed_definition_count = definitions.len();
        counters.installed_domain_operation_count = domain_operations.len();
        let identity = index_identity(
            &runtime,
            generation,
            &records,
            &definitions,
            &domain_operations,
        );
        Ok(Self {
            runtime,
            generation,
            packages: records,
            definitions,
            domain_operations,
            identity,
            counters,
            indexed_operation_lookups: AtomicUsize::new(0),
        })
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

    pub fn rebuild_report(&self) -> WorthQueryInstalledPackageIndexRebuildReport {
        let rebuilt = self.rebuild();
        WorthQueryInstalledPackageIndexRebuildReport::new(
            self.identity.clone(),
            rebuilt.identity.clone(),
            rebuilt.counters,
        )
    }
}

fn admit_definition(
    definitions: &mut BTreeMap<
        (WorthQueryPortableDefinitionKind, String, String),
        WorthQueryPortableDefinition,
    >,
    owner: &str,
    definition: &WorthQueryPortableDefinition,
) -> Result<(), WorthQueryInstalledPackageIndexDenial> {
    let key = (
        definition.kind(),
        owner.to_string(),
        definition.slot().to_string(),
    );
    if let Some(existing) = definitions.get(&key) {
        if existing == definition {
            return Ok(());
        }
        return Err(WorthQueryInstalledPackageIndexDenial::new(
            WorthQueryInstalledPackageIndexDenialKind::ConflictingDefinition,
            definition.slot(),
        ));
    }
    definitions.insert(key, definition.clone());
    Ok(())
}

#[cfg(test)]
mod tests;
