use std::sync::OnceLock;

use worth_foundational::facade::ScalarAspectType;
use worth_query::facade::{domain, read};

use crate::WorthUiDomainEntry;

use super::{
    WorthUiCollectionTextProjection, WorthUiCollectionTextProjectionFamily, LOWERING_FAMILY,
};

#[derive(Clone, Copy)]
pub(crate) struct WorthUiCollectionTextProjectionExecutor;

#[cfg(any(test, feature = "certification-construction"))]
#[derive(Clone, Copy)]
pub(crate) struct WorthUiPartialCollectionTextProjectionExecutor;

impl
    domain::WorthQueryDomainOperationExecutor<
        WorthUiDomainEntry,
        WorthUiCollectionTextProjection,
        WorthUiCollectionTextProjectionFamily,
    > for WorthUiCollectionTextProjectionExecutor
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

#[cfg(any(test, feature = "certification-construction"))]
impl
    domain::WorthQueryDomainOperationExecutor<
        WorthUiDomainEntry,
        WorthUiCollectionTextProjection,
        WorthUiCollectionTextProjectionFamily,
    > for WorthUiPartialCollectionTextProjectionExecutor
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
            domain::WorthQueryOperationResultState::Partial,
        )
        .with_warning(domain::WorthQueryOperationExecutionWarning::Partial(
            "certified partial collection".into(),
        )))
    }
}

fn installed_read_declaration() -> &'static read::WorthQueryReadDeclaration {
    static DECLARATION: OnceLock<read::WorthQueryReadDeclaration> = OnceLock::new();
    DECLARATION.get_or_init(|| {
        read::declare(|builder| {
            builder.local_collection(
                crate::installed_domain::query_text::QUERY_TEXT_ROOT,
                projection_schema(),
                |query| {
                    query
                        .project(selector("identity", "id"))
                        .project(selector("collection_item", "status"))
                        .project(selector("collection_item", "key"))
                        .order_by(
                            read::OrderingSelector::ascending("identity", "id")
                                .expect("static collection ordering must admit"),
                        )
                },
                |shape| {
                    shape
                        .field(result_field("identity", "id", "identity.id"))
                        .field(result_field(
                            "collection_item",
                            "status",
                            "collection_item.status",
                        ))
                        .field(result_field(
                            "collection_item",
                            "key",
                            "collection_item.key",
                        ))
                },
            )
        })
        .expect("installed collection text read must match portable meaning")
    })
}

fn projection_schema() -> read::QuerySchemaView {
    read::QuerySchemaView::new(
        "worth-ui-collection-text-projection",
        [
            read::SchemaFieldView::new(
                read::AspectName::new("identity").expect("static aspect must admit"),
                read::FieldName::new("id").expect("static field must admit"),
                ScalarAspectType::String,
            ),
            read::SchemaFieldView::new(
                read::AspectName::new("collection_item").expect("static aspect must admit"),
                read::FieldName::new("status").expect("static field must admit"),
                ScalarAspectType::String,
            ),
            read::SchemaFieldView::new(
                read::AspectName::new("collection_item").expect("static aspect must admit"),
                read::FieldName::new("key").expect("static field must admit"),
                ScalarAspectType::UInt64,
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
