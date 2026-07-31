use std::{collections::BTreeMap, sync::Arc};

use worth_query_declaration::facade::{
    application_capability::{ApplicationCapabilityRef, ErasedApplicationCapabilityContract},
    application_schema::{
        ApplicationOperationRef, ApplicationSchemaBindingIdentity, ApplicationSchemaMember,
    },
};

use crate::{
    authority_cryptography::AuthoritySeal, installed_index::WorthQueryInstalledPackageAuthority,
};

use super::{
    authority_seal::derive_capability_authority_seal, canonical_basis::prepare_capability_basis,
    WorthQueryApplicationCapabilityInstallationDenial,
    WorthQueryApplicationCapabilityInstallationDenialKind, WorthQueryCapabilityCanonicalArtifact,
    WorthQueryInstalledApplicationCapabilityIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct ApplicationCapabilityRegistryKey {
    name: String,
    capability_type: String,
    operation: String,
    operation_type: String,
    input_type: String,
}

impl ApplicationCapabilityRegistryKey {
    pub(crate) fn from_contract(contract: &ErasedApplicationCapabilityContract) -> Self {
        Self {
            name: contract.name().to_string(),
            capability_type: contract.capability_type().to_string(),
            operation: contract.operation().to_string(),
            operation_type: contract.operation_type().to_string(),
            input_type: contract.input_type().to_string(),
        }
    }

    pub(crate) fn from_references<Schema, Capability, Operation, Input>(
        capability: &ApplicationCapabilityRef<Schema, Capability>,
        operation: &ApplicationOperationRef<Schema, Operation, Input>,
    ) -> Self {
        Self {
            name: capability.name().to_string(),
            capability_type: std::any::type_name::<Capability>().to_string(),
            operation: operation.name().to_string(),
            operation_type: std::any::type_name::<Operation>().to_string(),
            input_type: std::any::type_name::<Input>().to_string(),
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

pub(crate) struct CompiledApplicationCapability {
    canonical: WorthQueryCapabilityCanonicalArtifact,
    identity: WorthQueryInstalledApplicationCapabilityIdentity,
    authority_identity: AuthoritySeal,
    contract: ErasedApplicationCapabilityContract,
}

impl CompiledApplicationCapability {
    pub(crate) fn canonical(&self) -> &WorthQueryCapabilityCanonicalArtifact {
        &self.canonical
    }

    pub(crate) fn identity(&self) -> &WorthQueryInstalledApplicationCapabilityIdentity {
        &self.identity
    }

    pub(crate) fn authority_identity(&self) -> &AuthoritySeal {
        &self.authority_identity
    }

    pub(crate) fn contract(&self) -> &ErasedApplicationCapabilityContract {
        &self.contract
    }
}

pub(crate) fn compile_capability_registry(
    package: &WorthQueryInstalledPackageAuthority,
    binding: &ApplicationSchemaBindingIdentity,
    members: &[ApplicationSchemaMember],
) -> Result<
    BTreeMap<ApplicationCapabilityRegistryKey, Arc<CompiledApplicationCapability>>,
    WorthQueryApplicationCapabilityInstallationDenial,
> {
    let mut registry = BTreeMap::new();
    for contract in members.iter().filter_map(|member| match member {
        ApplicationSchemaMember::ApplicationCapability { contract } => Some(contract),
        _ => None,
    }) {
        let canonical = prepare_capability_basis(
            binding.package_identity(),
            binding.schema_identity(),
            contract,
        )?;
        let identity =
            WorthQueryInstalledApplicationCapabilityIdentity::from_canonical(*canonical.digest());
        let authority_identity = derive_capability_authority_seal(
            &package.authority_key,
            binding,
            identity.bytes(),
            contract,
        );
        let key = ApplicationCapabilityRegistryKey::from_contract(contract);
        let compiled = Arc::new(CompiledApplicationCapability {
            canonical,
            identity,
            authority_identity,
            contract: contract.clone(),
        });
        if registry.insert(key, compiled).is_some() {
            return Err(WorthQueryApplicationCapabilityInstallationDenial::new(
                WorthQueryApplicationCapabilityInstallationDenialKind::CapabilityMeaningChanged,
                contract.name(),
            ));
        }
    }
    Ok(registry)
}
