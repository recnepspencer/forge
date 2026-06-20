use forge_query::facade::runtime::{
    ForgeQueryReadFamily, ForgeQueryReadGraph, ForgeQueryReadResult, ForgeQueryRuntimeError,
    ForgeQueryWorkspace, QuerySchemaView, SchemaFieldKind, SchemaFieldView, SchemaRelationView,
};
use forge_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate,
    ForgeQueryGraphReadDomainOperationDeclaration, OrderingSelector, ScalarPredicateValue,
    TraversalSelector,
};

pub fn graph_access_family(
    workspace: &mut ForgeQueryWorkspace,
    family_name: &str,
) -> ForgeQueryReadFamily {
    workspace
        .define_read_family(family_name, graph_access_collection_read_graph)
        .expect("graph-access family should admit")
}

pub fn execute_graph_access_compose_read(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
    workspace.compose_read(graph_access_collection_read_graph)
}

pub fn dense_over_budget_family(
    workspace: &mut ForgeQueryWorkspace,
    family_name: &str,
) -> ForgeQueryReadFamily {
    workspace
        .define_read_family(family_name, dense_over_budget_read_graph)
        .expect("dense over-budget family should define before access admission")
}

pub fn execute_dense_over_budget_compose_read(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
    workspace.compose_read(dense_over_budget_read_graph)
}

pub fn unregistered_domain_operation_family(
    workspace: &mut ForgeQueryWorkspace,
    family_name: &str,
) -> ForgeQueryReadFamily {
    workspace
        .define_read_family(family_name, unregistered_domain_operation_read_graph)
        .expect("domain operation family should define before access admission")
}

pub fn execute_unregistered_domain_operation_compose_read(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
    workspace.compose_read(unregistered_domain_operation_read_graph)
}

fn graph_access_collection_read_graph(
    read: forge_query::facade::runtime::ForgeQueryReadBuilder,
) -> Result<ForgeQueryReadGraph, forge_query::facade::runtime::ForgeQueryReadDenial> {
    read.anchored_collection(
        "user",
        relation_schema(),
        |query| {
            query
                .traverse(traversal("manager", 2))
                .project(field("identity", "id"))
        },
        |shape| shape.field(result_field("identity", "id", "id")),
    )
}

fn dense_over_budget_read_graph(
    read: forge_query::facade::runtime::ForgeQueryReadBuilder,
) -> Result<ForgeQueryReadGraph, forge_query::facade::runtime::ForgeQueryReadDenial> {
    read.explicit_broad_search_collection(
        "user",
        dense_schema(),
        |query| {
            query
                .traverse(traversal("manager", 8))
                .where_equal(equality("status", "value", "active"))
                .project(field("identity", "id"))
                .order_by(
                    OrderingSelector::ascending("profile", "display_name")
                        .expect("ordering should build"),
                )
        },
        |shape| shape.field(result_field("identity", "id", "id")),
    )
}

fn unregistered_domain_operation_read_graph(
    read: forge_query::facade::runtime::ForgeQueryReadBuilder,
) -> Result<ForgeQueryReadGraph, forge_query::facade::runtime::ForgeQueryReadDenial> {
    read.anchored_collection(
        "user",
        relation_schema(),
        |query| {
            query
                .domain_graph_operation(visible_face_neighborhood_operation())
                .traverse(traversal("manager", 2))
                .project(field("identity", "id"))
        },
        |shape| shape.field(result_field("identity", "id", "id")),
    )
}

fn visible_face_neighborhood_operation() -> ForgeQueryGraphReadDomainOperationDeclaration {
    ForgeQueryGraphReadDomainOperationDeclaration::new(
        "worth.geometry.visible_face_neighborhood",
        1,
        "worth.geometry",
    )
    .expect("operation key should admit")
    .admit_relation_reference("manager")
    .expect("operation reference should admit")
    .requires_support_family("worth.geometry.visible_face_neighborhood.access")
    .expect("support family should admit")
}

fn relation_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-read-access-read-surface-manager",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("profile", "display_name", SchemaFieldKind::String),
            SchemaFieldView::new("status", "value", SchemaFieldKind::String),
        ],
        [SchemaRelationView::new("manager", 2)],
    )
}

fn dense_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-read-access-read-surface-dense-manager",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("profile", "display_name", SchemaFieldKind::String),
            SchemaFieldView::new("status", "value", SchemaFieldKind::String),
        ],
        [SchemaRelationView::new("manager", 8)],
    )
}

fn field(aspect: &str, field: &str) -> AspectFieldSelector {
    AspectFieldSelector::new(aspect, field).expect("field selector should build")
}

fn result_field(aspect: &str, field: &str, delivered: &str) -> AuthoredResultShapeField {
    AuthoredResultShapeField::new(aspect, field, delivered)
        .expect("result-shape field should build")
}

fn traversal(name: &str, depth: u8) -> TraversalSelector {
    TraversalSelector::bounded(name, depth).expect("traversal selector should build")
}

fn equality(aspect: &str, field: &str, value: &str) -> EqualityPredicate {
    EqualityPredicate::new(
        aspect,
        field,
        ScalarPredicateValue::String(value.to_string()),
    )
    .expect("equality predicate should build")
}
