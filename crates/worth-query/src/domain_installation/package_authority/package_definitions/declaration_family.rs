use crate::application::{
    WorthQueryAsyncDeclarationSupport, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationFamilyMarker, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingRequirement, WorthQueryTemporalDeclarationSupport,
};

use super::super::{WorthQueryDomainIdentityComponentError, WorthQueryDomainIdentityNamespace};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainDeclarationFamilyDefinition {
    family_key: String,
    version: u32,
    required_capabilities: Vec<WorthQueryCapabilityFamily>,
    required_configuration: Vec<WorthQueryConfigSectionFamily>,
    operating_requirements: Vec<WorthQueryDomainOperatingRequirement>,
}

impl WorthQueryDomainDeclarationFamilyDefinition {
    fn new(
        name: impl Into<String>,
        version: u32,
    ) -> Result<Self, WorthQueryDomainIdentityComponentError> {
        let family_key = WorthQueryDomainIdentityNamespace::new(name)?
            .as_str()
            .to_string();
        Ok(Self {
            family_key,
            version,
            required_capabilities: Vec::new(),
            required_configuration: Vec::new(),
            operating_requirements: Vec::new(),
        })
    }

    pub fn from_marker<D, F>(version: u32) -> Result<Self, WorthQueryDomainIdentityComponentError>
    where
        D: WorthQueryDomainEntryMarker,
        F: WorthQueryDeclarationFamilyMarker<D>,
    {
        let mut definition = Self::new(F::semantic_family_key(), version)?;
        definition
            .required_capabilities
            .extend_from_slice(F::required_capability_families());
        definition
            .required_configuration
            .extend_from_slice(F::required_config_sections());
        if let Some(contract) = F::relational_truth_contract() {
            definition
                .required_capabilities
                .extend_from_slice(contract.required_capability_families());
            definition
                .required_configuration
                .extend_from_slice(contract.required_config_sections());
        }
        if let Some(contract) = F::bridge_continuation_contract() {
            definition
                .required_capabilities
                .extend_from_slice(contract.required_capability_families());
            definition
                .required_configuration
                .extend_from_slice(contract.required_config_sections());
        }
        if let Some(contract) = F::signal_compatibility_contract() {
            definition
                .required_capabilities
                .extend_from_slice(contract.required_capability_families());
            definition
                .required_configuration
                .extend_from_slice(contract.required_config_sections());
        }
        if F::temporal_declaration_support() != WorthQueryTemporalDeclarationSupport::Unsupported {
            definition
                .operating_requirements
                .push(WorthQueryDomainOperatingRequirement::TemporalQuery);
        }
        if F::async_declaration_support() != WorthQueryAsyncDeclarationSupport::Unsupported {
            definition
                .operating_requirements
                .push(WorthQueryDomainOperatingRequirement::AsyncResourceQuery);
        }
        definition.required_capabilities.sort();
        definition.required_capabilities.dedup();
        definition.required_configuration.sort();
        definition.required_configuration.dedup();
        definition.operating_requirements.sort();
        definition.operating_requirements.dedup();
        Ok(definition)
    }

    pub fn family_key(&self) -> &str {
        &self.family_key
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn required_capabilities(&self) -> &[WorthQueryCapabilityFamily] {
        &self.required_capabilities
    }

    pub fn required_configuration(&self) -> &[WorthQueryConfigSectionFamily] {
        &self.required_configuration
    }

    pub fn operating_requirements(&self) -> &[WorthQueryDomainOperatingRequirement] {
        &self.operating_requirements
    }

    pub(crate) fn slot_key(&self) -> &str {
        &self.family_key
    }

    pub(crate) fn canonical_part(&self) -> String {
        format!(
            "{}:{}:capabilities={}:configuration={}:operating={}",
            self.family_key,
            self.version,
            self.required_capabilities
                .iter()
                .map(|family| family.as_str())
                .collect::<Vec<_>>()
                .join(","),
            self.required_configuration
                .iter()
                .map(|section| section.as_str())
                .collect::<Vec<_>>()
                .join(","),
            self.operating_requirements
                .iter()
                .map(|requirement| requirement.as_str())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}
