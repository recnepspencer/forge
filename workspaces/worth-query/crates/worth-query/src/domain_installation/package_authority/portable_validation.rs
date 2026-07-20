use worth_query_installation::facade::{
    WorthQueryPortableDefinition, WorthQueryPortableDomainIdentity,
    WorthQueryPortableDomainPackage, WorthQueryPortablePackageValidationDenial,
    WorthQueryValidatedPortableDomainPackage,
};

use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryContributionCategoryFamily, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingRequirement,
};

use super::{
    WorthQueryDomainDeclarationFamilyDefinition, WorthQueryDomainGraphObligationDefinition,
    WorthQueryDomainGraphReadOperationDefinition, WorthQueryDomainIdentityDeclaration,
    WorthQueryDomainInvariantDefinition, WorthQueryDomainOperationDefinitionRecord,
};

pub(super) struct WorthQueryPortablePackageDeclaration<'a, D> {
    pub(super) identity: &'a WorthQueryDomainIdentityDeclaration<D>,
    pub(super) required_capabilities: &'a [WorthQueryCapabilityFamily],
    pub(super) required_configuration: &'a [WorthQueryConfigSectionFamily],
    pub(super) operating_requirements: &'a [WorthQueryDomainOperatingRequirement],
    pub(super) invariant_definitions: &'a [WorthQueryDomainInvariantDefinition],
    pub(super) graph_obligations: &'a [WorthQueryDomainGraphObligationDefinition],
    pub(super) graph_read_operations: &'a [WorthQueryDomainGraphReadOperationDefinition],
    pub(super) declaration_families: &'a [WorthQueryDomainDeclarationFamilyDefinition],
    pub(super) domain_operations: &'a [WorthQueryDomainOperationDefinitionRecord],
    pub(super) contribution_policy: &'a [WorthQueryDeclarationEntryContributionCategoryFamily],
}

pub(super) fn validate_portable_package<D>(
    package: WorthQueryPortablePackageDeclaration<'_, D>,
) -> Result<WorthQueryValidatedPortableDomainPackage, WorthQueryPortablePackageValidationDenial>
where
    D: WorthQueryDomainEntryMarker,
{
    let mut portable = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        package.identity.canonical_owner(),
        package.identity.semantic_version().major(),
        package.identity.semantic_version().minor(),
    ));
    for capability in package.required_capabilities {
        portable = portable.requires_capability(capability.as_str());
    }
    for section in package.required_configuration {
        portable = portable.requires_configuration(section.as_str());
    }
    for requirement in package.operating_requirements {
        portable = portable.requires_operating_posture(requirement.as_str());
    }
    for definition in package.invariant_definitions {
        portable = portable.definition(WorthQueryPortableDefinition::invariant(
            definition.slot_key(),
            definition.canonical_part(),
        ));
    }
    for definition in package.graph_obligations {
        portable = portable.definition(WorthQueryPortableDefinition::graph_obligation(
            definition.slot_key(),
            definition.canonical_part(),
        ));
    }
    for definition in package.graph_read_operations {
        portable = portable.definition(WorthQueryPortableDefinition::graph_read_operation(
            definition.slot_key(),
            definition.canonical_part(),
        ));
    }
    for definition in package.declaration_families {
        portable = portable.definition(WorthQueryPortableDefinition::declaration_family(
            definition.slot_key(),
            definition.canonical_part(),
        ));
    }
    for operation in package.domain_operations {
        portable = portable.domain_operation(operation.definition().clone());
    }
    for category in package.contribution_policy {
        portable = portable.permits_contribution(category.as_str());
    }
    portable.validate()
}
