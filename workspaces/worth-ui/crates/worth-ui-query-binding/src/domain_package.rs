use worth_query::facade::domain::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryContributionCategoryFamily,
    WorthQueryDomainGraphObligationDefinition, WorthQueryDomainGraphReadOperationDefinition,
    WorthQueryDomainIdentityDeclaration, WorthQueryDomainIdentityName,
    WorthQueryDomainIdentityNamespace, WorthQueryDomainPackage, WorthQueryDomainSemanticVersion,
};
use worth_query::facade::read::RelationName;
use worth_query::facade::runtime::{
    WorthQueryGraphObligationKind, WorthQueryGraphObligationOperatingWorldSelector,
    WorthQueryGraphTouchSelector,
};

use crate::WorthUiDomainEntry;

pub fn worth_ui_domain_package() -> WorthQueryDomainPackage<WorthUiDomainEntry> {
    WorthQueryDomainPackage::declare(
        WorthUiDomainEntry,
        WorthQueryDomainIdentityDeclaration::new(
            WorthQueryDomainIdentityNamespace::new("WORTH.ui")
                .expect("static Worth UI namespace must admit"),
            WorthQueryDomainIdentityName::new("runtime")
                .expect("static Worth UI domain name must admit"),
            WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .requires_capability(WorthQueryCapabilityFamily::QueryRead)
    .requires_capability(WorthQueryCapabilityFamily::QueryComposition)
    .requires_capability(WorthQueryCapabilityFamily::WorkflowOrchestration)
    .requires_configuration(WorthQueryConfigSectionFamily::Query)
    .requires_configuration(WorthQueryConfigSectionFamily::Relational)
    .graph_read_operation(measurement_allocation_operation())
    .graph_obligation(WorthQueryDomainGraphObligationDefinition::new(
        WorthQueryDomainIdentityName::new("measurement-allocation-integrity")
            .expect("static Worth UI invariant name must admit"),
        WorthQueryDomainSemanticVersion::new(1, 0),
        WorthQueryGraphObligationKind::BlockingInvariant,
        WorthQueryGraphTouchSelector::relation_kind("measurement.allocation")
            .expect("static Worth UI relation must admit"),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    ))
    .permits_contribution(WorthQueryDeclarationEntryContributionCategoryFamily::Admission)
    .permits_contribution(WorthQueryDeclarationEntryContributionCategoryFamily::InvariantCapability)
    .permits_contribution(WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview)
}

pub(crate) fn measurement_allocation_operation() -> WorthQueryDomainGraphReadOperationDefinition {
    WorthQueryDomainGraphReadOperationDefinition::new(
        WorthQueryDomainIdentityName::new("measurement-allocation")
            .expect("static Worth UI operation name must admit"),
        1,
    )
    .accepts_relation(
        RelationName::new("measurement.allocation").expect("static Worth UI relation must admit"),
    )
}
