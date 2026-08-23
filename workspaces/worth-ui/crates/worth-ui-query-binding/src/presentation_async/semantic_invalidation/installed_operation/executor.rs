use std::sync::OnceLock;

use worth_foundational::facade::ScalarAspectType;
use worth_query::facade::{domain, read};

use super::contracts::FIELDS;
use super::{
    WorthUiPresentationAsyncDomainEntry, WorthUiPresentationAsyncOperation,
    WorthUiPresentationAsyncOperationFamily,
};

#[derive(Clone, Copy)]
pub(crate) struct WorthUiPresentationAsyncOperationExecutor;

impl
    domain::WorthQueryDomainOperationExecutor<
        WorthUiPresentationAsyncDomainEntry,
        WorthUiPresentationAsyncOperation,
        WorthUiPresentationAsyncOperationFamily,
    > for WorthUiPresentationAsyncOperationExecutor
{
    const LOWERING_FAMILY: &'static str = "worth-ui-presentation-async-v1";
    const DETERMINISTIC: bool = true;
    const EXECUTION_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;
    const RESULT_WIDTH_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;

    fn installed_read_declaration(&self) -> Option<&read::WorthQueryReadDeclaration> {
        Some(installed_read_declaration())
    }

    fn execution_resource_support(&self) -> domain::WorthQueryExecutionResourceSupport {
        crate::installed_domain::execution_resources::operation_execution_resource_support()
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
            domain::WorthQueryOperationResultState::Ready,
        ))
    }
}

fn installed_read_declaration() -> &'static read::WorthQueryReadDeclaration {
    static DECLARATION: OnceLock<read::WorthQueryReadDeclaration> = OnceLock::new();
    DECLARATION.get_or_init(|| {
        read::declare(|builder| {
            builder.local_detail(
                "WorthUiPresentation",
                schema(),
                |mut query| {
                    for (aspect, field, _, _) in FIELDS {
                        query = query.project(selector(aspect, field));
                    }
                    query
                },
                |mut shape| {
                    for (aspect, field, _, _) in FIELDS {
                        shape = shape.field(result_field(aspect, field));
                    }
                    shape
                },
            )
        })
        .expect("installed presentation read must match portable meaning")
    })
}

fn schema() -> read::QuerySchemaView {
    read::QuerySchemaView::new(
        "worth-ui-presentation-async",
        FIELDS.map(|(aspect, field, _, _)| {
            read::SchemaFieldView::new(
                read::AspectName::new(aspect).expect("static presentation aspect must admit"),
                read::FieldName::new(field).expect("static presentation field must admit"),
                ScalarAspectType::UInt64,
            )
        }),
        [],
    )
}

fn selector(aspect: &str, field: &str) -> read::AspectFieldSelector {
    read::AspectFieldSelector::new(aspect, field).expect("static selector must admit")
}

fn result_field(aspect: &str, field: &str) -> read::AuthoredResultShapeField {
    read::AuthoredResultShapeField::new(aspect, field, format!("{aspect}.{field}"))
        .expect("static result field must admit")
}
