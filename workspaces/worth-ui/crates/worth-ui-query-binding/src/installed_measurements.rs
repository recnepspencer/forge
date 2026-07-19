//! Worth UI measurement vocabulary over Query's runtime-installed domain handle.

use worth_foundational::facade::{AspectValue, CanonicalF32, ScalarAspectType};
use worth_query::facade::{domain, read};

use crate::{domain_package::measurement_allocation_operation, WorthUiDomainEntry};

const MEASUREMENT_ROOT: &str = "WorthUiMeasurement";

pub trait WorthUiQueryExt {
    fn measurements(
        &self,
    ) -> Result<
        domain::WorthQueryInstalledDomainReadDeclaration<WorthUiDomainEntry>,
        read::WorthQueryReadDeclarationStop,
    >;

    fn live_measurements(
        &self,
    ) -> Result<
        domain::WorthQueryInstalledDomainLiveDeclaration<WorthUiDomainEntry>,
        worth_query::facade::live::WorthQueryLiveDeclarationStop,
    >;

    fn record_measurement(
        &self,
        label: domain::WorthQuerySessionLabel,
        contribution: WorthUiMeasurementContribution,
    ) -> Result<
        domain::WorthQueryInstalledDomainWorkflowDeclaration<WorthUiDomainEntry>,
        domain::WorthQueryMutationDeclarationStop,
    >;
}

impl WorthUiQueryExt for domain::WorthQueryInstalledDomainHandle<WorthUiDomainEntry> {
    fn measurements(
        &self,
    ) -> Result<
        domain::WorthQueryInstalledDomainReadDeclaration<WorthUiDomainEntry>,
        read::WorthQueryReadDeclarationStop,
    > {
        let operation = self.graph_read_operation(&measurement_allocation_operation());
        self.read(|query| {
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
    }

    fn live_measurements(
        &self,
    ) -> Result<
        domain::WorthQueryInstalledDomainLiveDeclaration<WorthUiDomainEntry>,
        worth_query::facade::live::WorthQueryLiveDeclarationStop,
    > {
        let operation = self.graph_read_operation(&measurement_allocation_operation());
        self.live("worth-ui.measurements", |query| {
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
    }

    fn record_measurement(
        &self,
        label: domain::WorthQuerySessionLabel,
        contribution: WorthUiMeasurementContribution,
    ) -> Result<
        domain::WorthQueryInstalledDomainWorkflowDeclaration<WorthUiDomainEntry>,
        domain::WorthQueryMutationDeclarationStop,
    > {
        self.mutation(|mutation| {
            mutation
                .set_aspect(
                    domain::WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")?,
                    domain::WorthQueryAuthoredAspectValue::string(contribution.identity),
                )
                .set_aspect(
                    domain::WorthQueryAspectTouch::from_authoring_ingress_text(
                        "measurement.value",
                    )?,
                    domain::WorthQueryAuthoredAspectValue::native(AspectValue::Float32(
                        contribution.value,
                    )),
                )
                .build_insert(MEASUREMENT_ROOT)
        })
        .map(|mutation| mutation.workflow(label))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiMeasurementContribution {
    identity: String,
    value: CanonicalF32,
}

impl WorthUiMeasurementContribution {
    pub fn new(identity: impl Into<String>, value: f32) -> Self {
        Self {
            identity: identity.into(),
            value: CanonicalF32::from_f32(value),
        }
    }
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
