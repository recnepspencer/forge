use std::sync::OnceLock;

use worth_foundational::facade::ScalarAspectType;
use worth_query::facade::{domain, read};

use crate::WorthUiDomainEntry;

use super::{
    WorthUiSnapshotMeasurement, WorthUiSnapshotMeasurementFamily, LOWERING_FAMILY, MEASUREMENT_ROOT,
};

#[derive(Clone, Copy)]
pub(crate) struct WorthUiSnapshotMeasurementExecutor;

#[cfg(any(test, feature = "certification-construction"))]
#[derive(Clone, Copy)]
pub(crate) struct WorthUiPartialSnapshotMeasurementExecutor;

#[cfg(test)]
#[derive(Clone, Copy)]
pub(crate) struct WorthUiSnapshotMeasurementValueAliasExecutor;

impl
    domain::WorthQueryDomainOperationExecutor<
        WorthUiDomainEntry,
        WorthUiSnapshotMeasurement,
        WorthUiSnapshotMeasurementFamily,
    > for WorthUiSnapshotMeasurementExecutor
{
    const LOWERING_FAMILY: &'static str = LOWERING_FAMILY;
    const DETERMINISTIC: bool = true;
    const EXECUTION_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;
    const RESULT_WIDTH_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;

    fn installed_read_declaration(&self) -> Option<&read::WorthQueryReadDeclaration> {
        Some(installed_read_declaration())
    }

    fn execute(
        &self,
        (): (),
        context: &domain::WorthQueryOperationExecutionContext<'_>,
        workspace: &mut domain::WorthQueryOperationWorkspace<'_>,
    ) -> Result<
        domain::WorthQueryOperationExecutionMaterial<read::WorthQueryReadCompletion>,
        domain::WorthQueryOperationExecutorFailure,
    > {
        execute_snapshot(context, workspace)
    }
}

#[cfg(any(test, feature = "certification-construction"))]
impl
    domain::WorthQueryDomainOperationExecutor<
        WorthUiDomainEntry,
        WorthUiSnapshotMeasurement,
        WorthUiSnapshotMeasurementFamily,
    > for WorthUiPartialSnapshotMeasurementExecutor
{
    const LOWERING_FAMILY: &'static str = LOWERING_FAMILY;
    const DETERMINISTIC: bool = true;
    const EXECUTION_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;
    const RESULT_WIDTH_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;

    fn installed_read_declaration(&self) -> Option<&read::WorthQueryReadDeclaration> {
        Some(installed_read_declaration())
    }

    fn execute(
        &self,
        (): (),
        context: &domain::WorthQueryOperationExecutionContext<'_>,
        workspace: &mut domain::WorthQueryOperationWorkspace<'_>,
    ) -> Result<
        domain::WorthQueryOperationExecutionMaterial<read::WorthQueryReadCompletion>,
        domain::WorthQueryOperationExecutorFailure,
    > {
        Ok(domain::WorthQueryOperationExecutionMaterial::new(
            context.execute_installed_read(workspace)?,
            domain::WorthQueryOperationResultState::Partial,
        )
        .with_warning(domain::WorthQueryOperationExecutionWarning::Partial(
            "certified partial snapshot".into(),
        )))
    }
}

#[cfg(test)]
impl
    domain::WorthQueryDomainOperationExecutor<
        WorthUiDomainEntry,
        WorthUiSnapshotMeasurement,
        WorthUiSnapshotMeasurementFamily,
    > for WorthUiSnapshotMeasurementValueAliasExecutor
{
    const LOWERING_FAMILY: &'static str = LOWERING_FAMILY;
    const DETERMINISTIC: bool = true;
    const EXECUTION_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;
    const RESULT_WIDTH_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;

    fn installed_read_declaration(&self) -> Option<&read::WorthQueryReadDeclaration> {
        Some(installed_value_alias_read_declaration())
    }

    fn execute(
        &self,
        (): (),
        context: &domain::WorthQueryOperationExecutionContext<'_>,
        workspace: &mut domain::WorthQueryOperationWorkspace<'_>,
    ) -> Result<
        domain::WorthQueryOperationExecutionMaterial<read::WorthQueryReadCompletion>,
        domain::WorthQueryOperationExecutorFailure,
    > {
        execute_snapshot(context, workspace)
    }
}

fn execute_snapshot(
    context: &domain::WorthQueryOperationExecutionContext<'_>,
    workspace: &mut domain::WorthQueryOperationWorkspace<'_>,
) -> Result<
    domain::WorthQueryOperationExecutionMaterial<read::WorthQueryReadCompletion>,
    domain::WorthQueryOperationExecutorFailure,
> {
    Ok(domain::WorthQueryOperationExecutionMaterial::new(
        context.execute_installed_read(workspace)?,
        domain::WorthQueryOperationResultState::Ready,
    ))
}

fn installed_read_declaration() -> &'static read::WorthQueryReadDeclaration {
    static DECLARATION: OnceLock<read::WorthQueryReadDeclaration> = OnceLock::new();
    DECLARATION.get_or_init(|| {
        read::declare(|builder| {
            builder.local_collection(
                MEASUREMENT_ROOT,
                measurement_schema(),
                |query| {
                    query
                        .project(selector("identity", "id"))
                        .project(selector("measurement", "value"))
                },
                |shape| {
                    shape
                        .field(result_field("identity", "id", "identity.id"))
                        .field(result_field("measurement", "value", "measurement.value"))
                },
            )
        })
        .expect("installed measurement read must match its portable definition")
    })
}

#[cfg(test)]
fn installed_value_alias_read_declaration() -> &'static read::WorthQueryReadDeclaration {
    static DECLARATION: OnceLock<read::WorthQueryReadDeclaration> = OnceLock::new();
    DECLARATION.get_or_init(|| {
        read::declare(|builder| {
            builder.local_collection(
                MEASUREMENT_ROOT,
                measurement_schema(),
                |query| {
                    query
                        .project(selector("identity", "id"))
                        .project(selector("measurement", "value"))
                },
                |shape| {
                    shape
                        .field(result_field("identity", "id", "identity.id"))
                        .field(result_field("measurement", "value", "value"))
                },
            )
        })
        .expect("drifted installed read must match its portable test definition")
    })
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
        [],
    )
}

fn selector(aspect: &str, field: &str) -> read::AspectFieldSelector {
    read::AspectFieldSelector::new(aspect, field).expect("static selector must admit")
}

fn result_field(aspect: &str, field: &str, alias: &str) -> read::AuthoredResultShapeField {
    read::AuthoredResultShapeField::new(aspect, field, alias)
        .expect("static result field must admit")
}
