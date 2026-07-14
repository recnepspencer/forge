use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryContributionCategoryFamily, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingRequirement,
};
use crate::runtime::WorthQueryGraphObligationRegistration;

use super::{
    WorthQueryDomainDeclarationFamilyDefinition, WorthQueryDomainGraphReadOperationDefinition,
    WorthQueryDomainIdentityDeclaration, WorthQueryDomainInvariantDefinition,
    WorthQueryDomainPackageValidationDenial, WorthQueryValidatedDomainPackage,
};

pub struct WorthQueryDomainPackage<D: WorthQueryDomainEntryMarker> {
    pub(crate) marker: D,
    pub(crate) identity: WorthQueryDomainIdentityDeclaration<D>,
    pub(crate) required_capabilities: Vec<WorthQueryCapabilityFamily>,
    pub(crate) required_configuration: Vec<WorthQueryConfigSectionFamily>,
    pub(crate) operating_requirements: Vec<WorthQueryDomainOperatingRequirement>,
    pub(crate) invariant_definitions: Vec<WorthQueryDomainInvariantDefinition>,
    pub(crate) graph_obligations: Vec<WorthQueryGraphObligationRegistration>,
    pub(crate) graph_read_operations: Vec<WorthQueryDomainGraphReadOperationDefinition>,
    pub(crate) declaration_families: Vec<WorthQueryDomainDeclarationFamilyDefinition>,
    pub(crate) contribution_policy: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
}

impl<D: WorthQueryDomainEntryMarker> WorthQueryDomainPackage<D> {
    pub fn declare(marker: D, identity: WorthQueryDomainIdentityDeclaration<D>) -> Self {
        Self {
            marker,
            identity,
            required_capabilities: Vec::new(),
            required_configuration: Vec::new(),
            operating_requirements: Vec::new(),
            invariant_definitions: Vec::new(),
            graph_obligations: Vec::new(),
            graph_read_operations: Vec::new(),
            declaration_families: Vec::new(),
            contribution_policy: Vec::new(),
        }
    }

    #[must_use]
    pub fn requires_capability(mut self, family: WorthQueryCapabilityFamily) -> Self {
        self.required_capabilities.push(family);
        self
    }

    #[must_use]
    pub fn requires_configuration(mut self, section: WorthQueryConfigSectionFamily) -> Self {
        self.required_configuration.push(section);
        self
    }

    #[must_use]
    pub fn requires_operating_posture(
        mut self,
        requirement: WorthQueryDomainOperatingRequirement,
    ) -> Self {
        self.operating_requirements.push(requirement);
        self
    }

    #[must_use]
    pub fn invariant(mut self, definition: WorthQueryDomainInvariantDefinition) -> Self {
        self.invariant_definitions.push(definition);
        self
    }

    #[must_use]
    pub fn graph_obligation(mut self, registration: WorthQueryGraphObligationRegistration) -> Self {
        self.graph_obligations.push(registration);
        self
    }

    #[must_use]
    pub fn graph_read_operation(
        mut self,
        definition: WorthQueryDomainGraphReadOperationDefinition,
    ) -> Self {
        self.graph_read_operations.push(definition);
        self
    }

    #[must_use]
    pub fn declaration_family(
        mut self,
        definition: WorthQueryDomainDeclarationFamilyDefinition,
    ) -> Self {
        self.declaration_families.push(definition);
        self
    }

    #[must_use]
    pub fn declaration_families(
        mut self,
        definitions: impl IntoIterator<Item = WorthQueryDomainDeclarationFamilyDefinition>,
    ) -> Self {
        self.declaration_families.extend(definitions);
        self
    }

    #[must_use]
    pub fn permits_contribution(
        mut self,
        category: WorthQueryDeclarationEntryContributionCategoryFamily,
    ) -> Self {
        self.contribution_policy.push(category);
        self
    }

    pub(crate) fn validate(
        self,
    ) -> Result<WorthQueryValidatedDomainPackage<D>, WorthQueryDomainPackageValidationDenial> {
        super::validation::validate_domain_package(self)
    }
}
