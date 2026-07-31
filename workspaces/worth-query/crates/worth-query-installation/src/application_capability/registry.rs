use std::{collections::BTreeMap, sync::Arc};

use worth_query_declaration::facade::{
    application_capability::ErasedApplicationCapabilityContract,
    application_schema::{ApplicationSchemaBindingIdentity, ApplicationSchemaMember},
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
    operation: String,
    input_type: String,
}

impl ApplicationCapabilityRegistryKey {
    pub(crate) fn new(name: &str, operation: &str, input_type: &str) -> Self {
        Self {
            name: name.to_string(),
            operation: operation.to_string(),
            input_type: input_type.to_string(),
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
            contract.operation(),
            contract.input_type(),
        );
        let key = ApplicationCapabilityRegistryKey::new(
            contract.name(),
            contract.operation(),
            contract.input_type(),
        );
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
