use worth_query::facade::domain::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryContributionCategoryFamily, WorthQueryDomainIdentityDeclaration,
    WorthQueryDomainIdentityName, WorthQueryDomainIdentityNamespace, WorthQueryDomainPackage,
    WorthQueryDomainSemanticVersion,
};

use super::HadwigerResearchDomainEntry;

pub fn hadwiger_research_domain_package() -> WorthQueryDomainPackage<HadwigerResearchDomainEntry> {
    WorthQueryDomainPackage::declare(
        HadwigerResearchDomainEntry,
        WorthQueryDomainIdentityDeclaration::new(
            WorthQueryDomainIdentityNamespace::new("WORTH.hadwiger")
                .expect("static namespace must admit"),
            WorthQueryDomainIdentityName::new("research").expect("static name must admit"),
            WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .requires_capability(WorthQueryCapabilityFamily::QueryRead)
    .requires_capability(WorthQueryCapabilityFamily::QueryComposition)
    .requires_capability(WorthQueryCapabilityFamily::WorkflowOrchestration)
    .requires_configuration(WorthQueryConfigSectionFamily::Query)
    .permits_contribution(WorthQueryDeclarationEntryContributionCategoryFamily::InvariantCapability)
    .permits_contribution(WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview)
}
