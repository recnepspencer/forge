use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use sha2::{Digest, Sha256};

mod authority;
mod denial;
mod rebuild_report;

pub use authority::WorthQueryInstalledPackageAuthority;
pub use denial::{
    WorthQueryInstalledPackageIndexDenial, WorthQueryInstalledPackageIndexDenialKind,
};
pub use rebuild_report::{
    WorthQueryInstalledPackageIndexCounters, WorthQueryInstalledPackageIndexRebuildReport,
};

use crate::admission::WorthQueryAdmittedPortableDomainPackage;
use crate::canonical_hash_encoding::hash_text_field;
use crate::generation::{WorthQueryInstallationGeneration, WorthQueryInstallationRuntimeIdentity};
use crate::installed_operation::WorthQueryInstalledOperationAuthority;
use crate::package::{
    WorthQueryPortableDefinition, WorthQueryPortableDefinitionKind,
    WorthQueryPortableDomainPackageIdentity,
};

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
        let mut counters = WorthQueryInstalledPackageIndexCounters::default();

        for package in packages {
            let owner = package.package().domain_identity().owner().to_string();
            counters.package_rows_examined += 1;
            if let Some(existing) = records.get(&owner) {
                if existing.package.package().identity() == package.package().identity() {
                    if existing.package.admission_identity() == package.admission_identity() {
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
        let identity = index_identity(&runtime, generation, &records, &definitions);
        Ok(Self {
            runtime,
            generation,
            packages: records,
            definitions,
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

    pub fn validate(
        &self,
        authority: &WorthQueryInstalledPackageAuthority,
    ) -> Result<(), WorthQueryInstalledPackageIndexDenial> {
        if authority.runtime_ordinal != self.runtime.ordinal() {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::ForeignRuntime,
                &authority.owner,
            ));
        }
        if authority.generation != self.generation {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::StaleGeneration,
                &authority.owner,
            ));
        }
        let record = self.packages.get(&authority.owner).ok_or_else(|| {
            WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::DomainNotInstalled,
                &authority.owner,
            )
        })?;
        if record.package.package().identity() != &authority.package_identity {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::PackageIdentityChanged,
                &authority.owner,
            ));
        }
        if record.package.admission_identity() != authority.admission_identity {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::AdmissionIdentityChanged,
                &authority.owner,
            ));
        }
        if record.authority_nonce != authority.authority_nonce {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::AuthorityMismatch,
                &authority.owner,
            ));
        }
        Ok(())
    }

    pub fn validate_operation(
        &self,
        authority: &WorthQueryInstalledOperationAuthority,
    ) -> Result<(), WorthQueryInstalledPackageIndexDenial> {
        let package = WorthQueryInstalledPackageAuthority {
            runtime_ordinal: authority.runtime_ordinal,
            generation: authority.generation,
            owner: authority.owner.clone(),
            package_identity: authority.package_identity.clone(),
            admission_identity: authority.admission_identity.clone(),
            authority_nonce: authority.package_authority_nonce,
        };
        self.validate(&package)?;
        let current = self.operation(&authority.owner, &authority.operation_slot)?;
        if current.operation_semantics != authority.operation_semantics {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::OperationSemanticsChanged,
                &authority.operation_slot,
            ));
        }
        Ok(())
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

fn authority_nonce(
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

fn index_identity(
    runtime: &WorthQueryInstallationRuntimeIdentity,
    generation: WorthQueryInstallationGeneration,
    records: &BTreeMap<String, WorthQueryInstalledPackageRecord>,
    definitions: &BTreeMap<
        (WorthQueryPortableDefinitionKind, String, String),
        WorthQueryPortableDefinition,
    >,
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
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests;
