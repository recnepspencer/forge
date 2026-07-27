use std::sync::atomic::AtomicUsize;

use crate::admission::WorthQueryAdmittedPortableDomainPackage;

use super::artifact_contract_admission::admit_artifact_contract;
use super::index_identity::{authority_nonce, index_identity, IndexIdentityInput};
use super::*;

#[derive(Default)]
struct InstalledIndexConstruction {
    records: BTreeMap<String, WorthQueryInstalledPackageRecord>,
    definitions:
        BTreeMap<(WorthQueryPortableDefinitionKind, String, String), WorthQueryPortableDefinition>,
    domain_operations: BTreeMap<(String, String), WorthQueryValidatedDomainOperation>,
    artifact_contracts: BTreeMap<(String, String, u32, u32), WorthQueryPortableArtifactContract>,
    artifact_contract_slots: BTreeMap<(String, u32, u32), (String, String)>,
    application_schemas: BTreeMap<
        (String, String),
        worth_query_declaration::facade::application_schema::ErasedApplicationSchemaDeclaration,
    >,
    counters: WorthQueryInstalledPackageIndexCounters,
}

impl WorthQueryInstalledPackageIndex {
    pub fn build(
        runtime: WorthQueryInstallationRuntimeIdentity,
        generation: WorthQueryInstallationGeneration,
        packages: impl IntoIterator<Item = WorthQueryAdmittedPortableDomainPackage>,
    ) -> Result<Self, WorthQueryInstalledPackageIndexDenial> {
        let mut construction = InstalledIndexConstruction::default();
        for package in packages {
            let owner = package.package().domain_identity().owner().to_string();
            construction.counters.package_rows_examined += 1;
            if construction.admit_package_owner(&owner, &package)? {
                continue;
            }
            construction.admit_package_content(&owner, &package)?;
            construction.records.insert(
                owner,
                WorthQueryInstalledPackageRecord {
                    authority_nonce: authority_nonce(
                        &runtime,
                        generation,
                        package.package().identity(),
                        package.admission_identity(),
                    ),
                    package,
                },
            );
        }
        Ok(construction.finish(runtime, generation))
    }
}

impl InstalledIndexConstruction {
    fn finish(
        mut self,
        runtime: WorthQueryInstallationRuntimeIdentity,
        generation: WorthQueryInstallationGeneration,
    ) -> WorthQueryInstalledPackageIndex {
        self.counters.installed_package_count = self.records.len();
        self.counters.installed_definition_count = self.definitions.len();
        self.counters.installed_domain_operation_count = self.domain_operations.len();
        self.counters.installed_artifact_contract_count = self.artifact_contracts.len();
        self.counters.installed_application_schema_count = self.application_schemas.len();
        let identity = index_identity(IndexIdentityInput {
            runtime: &runtime,
            generation,
            records: &self.records,
            definitions: &self.definitions,
            domain_operations: &self.domain_operations,
            artifact_contracts: &self.artifact_contracts,
            application_schemas: &self.application_schemas,
        });
        WorthQueryInstalledPackageIndex {
            runtime,
            generation,
            packages: self.records,
            definitions: self.definitions,
            domain_operations: self.domain_operations,
            artifact_contracts: self.artifact_contracts,
            application_schemas: self.application_schemas,
            identity,
            counters: self.counters,
            indexed_operation_lookups: AtomicUsize::new(0),
        }
    }

    fn admit_package_owner(
        &mut self,
        owner: &str,
        package: &WorthQueryAdmittedPortableDomainPackage,
    ) -> Result<bool, WorthQueryInstalledPackageIndexDenial> {
        let Some(existing) = self.records.get(owner) else {
            return Ok(false);
        };
        if !existing
            .package
            .package()
            .has_same_authoritative_meaning(package.package())
        {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::ConflictingPackage,
                owner,
            ));
        }
        if !existing.package.has_same_admission_authority(package) {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::ConflictingAdmissionProfile,
                owner,
            ));
        }
        self.counters.equivalent_packages_converged += 1;
        Ok(true)
    }

    fn admit_package_content(
        &mut self,
        owner: &str,
        package: &WorthQueryAdmittedPortableDomainPackage,
    ) -> Result<(), WorthQueryInstalledPackageIndexDenial> {
        for definition in package.package().definitions() {
            self.counters.definition_rows_examined += 1;
            admit_definition(&mut self.definitions, owner, definition)?;
        }
        for operation in package.package().validated_domain_operations() {
            self.counters.domain_operation_rows_examined += 1;
            self.domain_operations.insert(
                (owner.to_string(), operation.definition().identity().slot()),
                operation.clone(),
            );
        }
        for contract in package.package().artifact_contracts() {
            self.counters.artifact_contract_rows_examined += 1;
            admit_artifact_contract(
                &mut self.artifact_contracts,
                &mut self.artifact_contract_slots,
                owner,
                contract,
            )?;
        }
        for schema in package.package().application_schemas() {
            self.counters.application_schema_rows_examined += 1;
            let key = (owner.to_string(), schema.name().to_string());
            if let Some(existing) = self.application_schemas.get(&key) {
                if existing != schema {
                    return Err(WorthQueryInstalledPackageIndexDenial::new(
                        WorthQueryInstalledPackageIndexDenialKind::ConflictingApplicationSchema,
                        schema.name(),
                    ));
                }
                continue;
            }
            self.application_schemas.insert(key, schema.clone());
        }
        Ok(())
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
