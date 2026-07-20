use super::{
    WorthQueryDomainDeclarationFamilyDefinition, WorthQueryDomainGraphObligationDefinition,
    WorthQueryDomainGraphReadOperationDefinition, WorthQueryDomainIdentityDeclaration,
    WorthQueryDomainInvariantDefinition, WorthQueryDomainOperationDefinition,
    WorthQueryDomainOperationDefinitionRecord, WorthQueryDomainOperationGraphParticipationRecord,
    WorthQueryDomainOperationRequiredDomainRecord, WorthQueryDomainPackageValidationDenial,
    WorthQueryValidatedDomainPackage,
};
use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryContributionCategoryFamily, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingRequirement,
};

pub struct WorthQueryDomainPackage<D: WorthQueryDomainEntryMarker> {
    pub(crate) marker: D,
    pub(crate) identity: WorthQueryDomainIdentityDeclaration<D>,
    pub(crate) required_capabilities: Vec<WorthQueryCapabilityFamily>,
    pub(crate) required_configuration: Vec<WorthQueryConfigSectionFamily>,
    pub(crate) operating_requirements: Vec<WorthQueryDomainOperatingRequirement>,
    pub(crate) invariant_definitions: Vec<WorthQueryDomainInvariantDefinition>,
    pub(crate) graph_obligations: Vec<WorthQueryDomainGraphObligationDefinition>,
    pub(crate) graph_read_operations: Vec<WorthQueryDomainGraphReadOperationDefinition>,
    pub(crate) declaration_families: Vec<WorthQueryDomainDeclarationFamilyDefinition>,
    pub(crate) domain_operations: Vec<WorthQueryDomainOperationDefinitionRecord>,
    pub(crate) operation_graph_participations:
        Vec<WorthQueryDomainOperationGraphParticipationRecord>,
    pub(crate) operation_required_domains: Vec<WorthQueryDomainOperationRequiredDomainRecord>,
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
            domain_operations: Vec::new(),
            operation_graph_participations: Vec::new(),
            operation_required_domains: Vec::new(),
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
    pub fn graph_obligation(
        mut self,
        definition: WorthQueryDomainGraphObligationDefinition,
    ) -> Self {
        self.graph_obligations.push(definition);
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
        self.required_capabilities
            .extend_from_slice(definition.required_capabilities());
        self.required_configuration
            .extend_from_slice(definition.required_configuration());
        self.operating_requirements
            .extend_from_slice(definition.operating_requirements());
        self.declaration_families.push(definition);
        self
    }

    #[must_use]
    pub fn declaration_families(
        mut self,
        definitions: impl IntoIterator<Item = WorthQueryDomainDeclarationFamilyDefinition>,
    ) -> Self {
        for definition in definitions {
            self.required_capabilities
                .extend_from_slice(definition.required_capabilities());
            self.required_configuration
                .extend_from_slice(definition.required_configuration());
            self.operating_requirements
                .extend_from_slice(definition.operating_requirements());
            self.declaration_families.push(definition);
        }
        self
    }

    #[must_use]
    pub fn operation<O, F>(
        mut self,
        definition: WorthQueryDomainOperationDefinition<D, O, F>,
    ) -> Self
    where
        O: 'static,
        F: 'static,
    {
        self.domain_operations
            .push(WorthQueryDomainOperationDefinitionRecord::from_typed(
                definition,
            ));
        self
    }

    #[must_use]
    pub fn operation_graph_participation<O: 'static, F: 'static, G: 'static>(
        mut self,
        role: impl Into<String>,
    ) -> Self {
        self.operation_graph_participations.push(
            WorthQueryDomainOperationGraphParticipationRecord::typed::<O, F, G>(role),
        );
        self
    }

    #[must_use]
    pub fn operation_required_domain<O: 'static, F: 'static, R: 'static>(
        mut self,
        role: impl Into<String>,
    ) -> Self {
        self.operation_required_domains.push(
            WorthQueryDomainOperationRequiredDomainRecord::typed::<O, F, R>(role),
        );
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
