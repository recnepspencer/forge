mod definition;
mod identity;
mod validation_denial;

use std::collections::BTreeMap;

pub use definition::{WorthQueryPortableDefinition, WorthQueryPortableDefinitionKind};
pub use identity::{WorthQueryPortableDomainIdentity, WorthQueryPortableDomainPackageIdentity};
pub use validation_denial::{
    WorthQueryPortablePackageValidationDenial, WorthQueryPortablePackageValidationDenialKind,
};

use crate::package::identity::canonical_identity;
use crate::package_requirements::{
    WorthQueryInstallationCapabilityFamily, WorthQueryInstallationConfigSectionFamily,
    WorthQueryInstallationContributionCategory, WorthQueryInstallationOperatingRequirement,
};

#[derive(Clone, Debug)]
pub struct WorthQueryPortableDomainPackage {
    identity: WorthQueryPortableDomainIdentity,
    capabilities: Vec<WorthQueryInstallationCapabilityFamily>,
    configuration: Vec<WorthQueryInstallationConfigSectionFamily>,
    operating: Vec<WorthQueryInstallationOperatingRequirement>,
    definitions: Vec<WorthQueryPortableDefinition>,
    contributions: Vec<WorthQueryInstallationContributionCategory>,
}

impl WorthQueryPortableDomainPackage {
    pub fn new(identity: WorthQueryPortableDomainIdentity) -> Self {
        Self {
            identity,
            capabilities: Vec::new(),
            configuration: Vec::new(),
            operating: Vec::new(),
            definitions: Vec::new(),
            contributions: Vec::new(),
        }
    }

    pub fn requires_capability(mut self, value: impl Into<String>) -> Self {
        self.capabilities
            .push(WorthQueryInstallationCapabilityFamily::new(value));
        self
    }

    pub fn requires_configuration(mut self, value: impl Into<String>) -> Self {
        self.configuration
            .push(WorthQueryInstallationConfigSectionFamily::new(value));
        self
    }

    pub fn requires_operating_posture(mut self, value: impl Into<String>) -> Self {
        self.operating
            .push(WorthQueryInstallationOperatingRequirement::new(value));
        self
    }

    pub fn definition(mut self, definition: WorthQueryPortableDefinition) -> Self {
        self.definitions.push(definition);
        self
    }

    pub fn permits_contribution(mut self, value: impl Into<String>) -> Self {
        self.contributions
            .push(WorthQueryInstallationContributionCategory::new(value));
        self
    }

    pub fn validate(
        mut self,
    ) -> Result<WorthQueryValidatedPortableDomainPackage, WorthQueryPortablePackageValidationDenial>
    {
        validate_required_meaning(&self)?;
        canonicalize(&mut self.capabilities);
        canonicalize(&mut self.configuration);
        canonicalize(&mut self.operating);
        self.contributions.sort();
        reject_duplicate_contributions(&self.contributions)?;
        self.definitions.sort();
        reject_definition_conflicts(&self.definitions)?;

        let identity = canonical_identity(&self);
        Ok(WorthQueryValidatedPortableDomainPackage {
            package: self,
            identity,
        })
    }
}

#[derive(Clone, Debug)]
pub struct WorthQueryValidatedPortableDomainPackage {
    package: WorthQueryPortableDomainPackage,
    identity: WorthQueryPortableDomainPackageIdentity,
}

impl WorthQueryValidatedPortableDomainPackage {
    pub fn domain_identity(&self) -> &WorthQueryPortableDomainIdentity {
        &self.package.identity
    }

    pub fn identity(&self) -> &WorthQueryPortableDomainPackageIdentity {
        &self.identity
    }

    pub fn capabilities(&self) -> &[WorthQueryInstallationCapabilityFamily] {
        &self.package.capabilities
    }

    pub fn configuration(&self) -> &[WorthQueryInstallationConfigSectionFamily] {
        &self.package.configuration
    }

    pub fn operating_requirements(&self) -> &[WorthQueryInstallationOperatingRequirement] {
        &self.package.operating
    }

    pub fn definitions(&self) -> &[WorthQueryPortableDefinition] {
        &self.package.definitions
    }

    pub fn contribution_policy(&self) -> &[WorthQueryInstallationContributionCategory] {
        &self.package.contributions
    }
}

fn validate_required_meaning(
    package: &WorthQueryPortableDomainPackage,
) -> Result<(), WorthQueryPortablePackageValidationDenial> {
    if package.identity.owner().trim().is_empty() {
        return Err(WorthQueryPortablePackageValidationDenial::empty_domain_owner());
    }
    if let Some(definition) = package
        .definitions
        .iter()
        .find(|definition| definition.slot().trim().is_empty())
    {
        return Err(
            WorthQueryPortablePackageValidationDenial::empty_definition_slot(definition.kind()),
        );
    }
    if let Some(definition) = package
        .definitions
        .iter()
        .find(|definition| definition.semantics().trim().is_empty())
    {
        return Err(
            WorthQueryPortablePackageValidationDenial::empty_definition_semantics(
                definition.kind(),
                definition.slot(),
            ),
        );
    }
    if let Some(requirement) = empty_requirement(package) {
        return Err(WorthQueryPortablePackageValidationDenial::empty_requirement(requirement));
    }
    Ok(())
}

fn reject_duplicate_contributions(
    contributions: &[WorthQueryInstallationContributionCategory],
) -> Result<(), WorthQueryPortablePackageValidationDenial> {
    if contributions.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(WorthQueryPortablePackageValidationDenial::duplicate_contribution_category());
    }
    Ok(())
}

fn reject_definition_conflicts(
    definitions: &[WorthQueryPortableDefinition],
) -> Result<(), WorthQueryPortablePackageValidationDenial> {
    let mut slots = BTreeMap::new();
    for definition in definitions {
        let key = (definition.kind(), definition.slot().to_string());
        if let Some(existing) = slots.insert(key, definition.semantics().to_string()) {
            return Err(if existing == definition.semantics() {
                WorthQueryPortablePackageValidationDenial::duplicate_definition(
                    definition.kind(),
                    definition.slot(),
                )
            } else {
                WorthQueryPortablePackageValidationDenial::conflicting_definition(
                    definition.kind(),
                    definition.slot(),
                )
            });
        }
    }
    Ok(())
}

fn empty_requirement(package: &WorthQueryPortableDomainPackage) -> Option<&'static str> {
    if package
        .capabilities
        .iter()
        .any(|value| value.as_str().trim().is_empty())
    {
        return Some("capability");
    }
    if package
        .configuration
        .iter()
        .any(|value| value.as_str().trim().is_empty())
    {
        return Some("configuration");
    }
    if package
        .operating
        .iter()
        .any(|value| value.as_str().trim().is_empty())
    {
        return Some("operating");
    }
    if package
        .contributions
        .iter()
        .any(|value| value.as_str().trim().is_empty())
    {
        return Some("contribution");
    }
    None
}

fn canonicalize<T: Ord>(values: &mut Vec<T>) {
    values.sort();
    values.dedup();
}
