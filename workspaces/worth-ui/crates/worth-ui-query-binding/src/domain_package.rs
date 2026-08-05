use worth_query::facade::domain::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryContributionCategoryFamily, WorthQueryDomainIdentityDeclaration,
    WorthQueryDomainIdentityName, WorthQueryDomainIdentityNamespace, WorthQueryDomainPackage,
    WorthQueryDomainSemanticVersion,
};

use crate::{
    installed_domain::{
        collection_text_projection, measurement_recording, scalar_text_projection,
        snapshot_measurement,
    },
    WorthUiDomainEntry,
};

pub fn worth_ui_domain_package() -> WorthQueryDomainPackage<WorthUiDomainEntry> {
    finish_domain_package(
        domain_package_base()
            .operation(snapshot_measurement::snapshot_measurement_definition())
            .operation(scalar_text_projection::scalar_text_projection_definition())
            .operation(collection_text_projection::collection_text_projection_definition())
            .operation(measurement_recording::measurement_recording_definition()),
    )
}

#[cfg(test)]
pub(crate) fn worth_ui_domain_package_with_snapshot_definition(
    snapshot_definition: worth_query::facade::domain::WorthQueryDomainOperationDefinition<
        WorthUiDomainEntry,
        snapshot_measurement::WorthUiSnapshotMeasurement,
        snapshot_measurement::WorthUiSnapshotMeasurementFamily,
    >,
) -> WorthQueryDomainPackage<WorthUiDomainEntry> {
    finish_domain_package(
        domain_package_base()
            .operation(snapshot_definition)
            .operation(scalar_text_projection::scalar_text_projection_definition())
            .operation(collection_text_projection::collection_text_projection_definition())
            .operation(measurement_recording::measurement_recording_definition()),
    )
}

fn domain_package_base() -> WorthQueryDomainPackage<WorthUiDomainEntry> {
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
    .graph_read_operation(snapshot_measurement::measurement_allocation_operation())
}

fn finish_domain_package(
    package: WorthQueryDomainPackage<WorthUiDomainEntry>,
) -> WorthQueryDomainPackage<WorthUiDomainEntry> {
    package
        .permits_contribution(WorthQueryDeclarationEntryContributionCategoryFamily::Admission)
        .permits_contribution(
            WorthQueryDeclarationEntryContributionCategoryFamily::InvariantCapability,
        )
        .permits_contribution(WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview)
}
