//! Worth UI's domain vocabulary over Query's ordinary capability facade.
//!
//! These functions declare UI intent. Query still owns canonicalization,
//! admission, planning, execution, lifecycle, and result/receipt assembly.

use worth_query::facade::{comparison, history, inspection, live, read};

const MEASUREMENT_ROOT: &str = "WorthUiMeasurement";

pub fn declare_measurement_read(
) -> Result<read::WorthQueryReadDeclaration, read::WorthQueryReadDeclarationStop> {
    read::declare(|query| {
        query.local_collection(
            MEASUREMENT_ROOT,
            measurement_schema(),
            |query| {
                query
                    .project(identity_selector())
                    .project(measurement_selector())
            },
            |shape| {
                shape
                    .field(identity_field())
                    .field(measurement_field())
            },
        )
    })
}

pub fn declare_measurement_live(
) -> Result<live::WorthQueryLiveDeclaration, live::WorthQueryLiveDeclarationStop> {
    live::declare("worth-ui.measurements", |query| {
        query.local_collection(
            MEASUREMENT_ROOT,
            measurement_schema(),
            |query| {
                query
                    .project(identity_selector())
                    .project(measurement_selector())
            },
            |shape| {
                shape
                    .field(identity_field())
                    .field(measurement_field())
            },
        )
    })
}

pub fn declare_measurement_history(
) -> Result<history::WorthQueryHistoricalPathDeclaration, history::WorthQueryHistoricalDeclarationStop>
{
    history::declare(|query| {
        query.local_collection(
            MEASUREMENT_ROOT,
            measurement_schema(),
            |query| {
                query
                    .project(identity_selector())
                    .project(measurement_selector())
            },
            |shape| {
                shape
                    .field(identity_field())
                    .field(measurement_field())
            },
        )
    })
    .map(history::WorthQueryHistoricalDeclaration::retained_snapshot)
}

pub fn declare_measurement_comparison(
) -> Result<comparison::WorthQueryComparisonRefinement, comparison::WorthQueryComparisonDeclarationStop>
{
    comparison::declare(|query| {
        query.local_collection(
            MEASUREMENT_ROOT,
            measurement_schema(),
            |query| {
                query
                    .project(identity_selector())
                    .project(measurement_selector())
            },
            |shape| {
                shape
                    .field(identity_field())
                    .field(measurement_field())
            },
        )
    })
    .map(comparison::WorthQueryComparisonDeclaration::diff)
}

pub fn inspect_measurement_read(
    completion: &read::WorthQueryReadCompletion,
) -> inspection::WorthQueryInspectionDeclaration {
    inspection::declare(completion).with_rich_inspection()
}

fn measurement_schema() -> read::QuerySchemaView {
    read::QuerySchemaView::new(
        "worth-ui-measurement-query",
        [
            read::SchemaFieldView::new(
                read::AspectName::new("identity").expect("static aspect must admit"),
                read::FieldName::new("id").expect("static field must admit"),
                read::SchemaFieldKind::String,
            ),
            read::SchemaFieldView::new(
                read::AspectName::new("measurement").expect("static aspect must admit"),
                read::FieldName::new("value").expect("static field must admit"),
                read::SchemaFieldKind::String,
            ),
        ],
        [],
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
