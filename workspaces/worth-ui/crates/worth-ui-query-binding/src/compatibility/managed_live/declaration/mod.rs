//! Pre-operation live declaration mechanics retained only for managed-live
//! compatibility until Query 9.14 phases 17, 19, 23, and 24 land.

#[cfg(test)]
mod tests;

use worth_foundational::facade::ScalarAspectType;
use worth_query::facade::{domain, read};

use crate::WorthUiDomainEntry;

const MEASUREMENT_ROOT: &str = "WorthUiMeasurement";

pub(crate) trait WorthUiManagedLiveDeclarationExt {
    fn live_measurements(
        &self,
        resource_name: &str,
    ) -> Result<
        domain::WorthQueryInstalledDomainLiveDeclaration<WorthUiDomainEntry>,
        Box<worth_query::facade::live::WorthQueryLiveDeclarationStop>,
    >;
}

impl WorthUiManagedLiveDeclarationExt
    for domain::WorthQueryInstalledDomainHandle<WorthUiDomainEntry>
{
    fn live_measurements(
        &self,
        resource_name: &str,
    ) -> Result<
        domain::WorthQueryInstalledDomainLiveDeclaration<WorthUiDomainEntry>,
        Box<worth_query::facade::live::WorthQueryLiveDeclarationStop>,
    > {
        let operation = self.graph_read_operation(&measurement_allocation_operation());
        self.live(resource_name, |query| {
            query.local_collection_with_installed_operation(
                operation,
                MEASUREMENT_ROOT,
                measurement_schema(),
                |query| {
                    query
                        .project(identity_selector())
                        .project(measurement_selector())
                },
                |shape| shape.field(identity_field()).field(measurement_field()),
            )
        })
        .map_err(Box::new)
    }
}

pub(crate) fn measurement_allocation_operation(
) -> domain::WorthQueryDomainGraphReadOperationDefinition {
    domain::WorthQueryDomainGraphReadOperationDefinition::new(
        domain::WorthQueryDomainIdentityName::new("measurement-allocation")
            .expect("static Worth UI operation name must admit"),
        1,
    )
    .accepts_relation(
        read::RelationName::new("measurement.allocation")
            .expect("static Worth UI relation must admit"),
    )
}

fn measurement_schema() -> read::QuerySchemaView {
    read::QuerySchemaView::new(
        "worth-ui-measurement-query",
        [
            read::SchemaFieldView::new(
                read::AspectName::new("identity").expect("static aspect must admit"),
                read::FieldName::new("id").expect("static field must admit"),
                ScalarAspectType::String,
            ),
            read::SchemaFieldView::new(
                read::AspectName::new("measurement").expect("static aspect must admit"),
                read::FieldName::new("value").expect("static field must admit"),
                ScalarAspectType::Float32,
            ),
        ],
        [read::SchemaRelationView::new(
            read::RelationName::new("measurement.allocation").expect("static relation must admit"),
            1,
        )],
    )
}

fn identity_selector() -> read::AspectFieldSelector {
    read::AspectFieldSelector::new("identity", "id").expect("static selector must admit")
}

fn measurement_selector() -> read::AspectFieldSelector {
    read::AspectFieldSelector::new("measurement", "value").expect("static selector must admit")
}

fn identity_field() -> read::AuthoredResultShapeField {
    read::AuthoredResultShapeField::new("identity", "id", "identity.id")
        .expect("static result field must admit")
}

fn measurement_field() -> read::AuthoredResultShapeField {
    read::AuthoredResultShapeField::new("measurement", "value", "measurement.value")
        .expect("static result field must admit")
}
