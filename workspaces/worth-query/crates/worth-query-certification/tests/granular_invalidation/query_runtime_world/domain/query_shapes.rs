use std::sync::OnceLock;

use worth_query::facade::read;
use worth_query_host::facade::declaration::{
    authoring::{
        AspectFieldSelector, AuthoredQueryBundleRequest, AuthoredResultShapeField,
        CollectionQueryBuilder, CollectionResultShapeBuilder, DetailQueryBuilder,
        DetailResultShapeBuilder, OrderingSelector, RootEntityKey,
    },
    binding::QueryBindingDescriptor,
    canonicalization::{canonicalize_request, CanonicalQueryBundle},
};

pub(super) fn detail_read_declaration() -> &'static read::WorthQueryReadDeclaration {
    static DECLARATION: OnceLock<read::WorthQueryReadDeclaration> = OnceLock::new();
    DECLARATION.get_or_init(|| {
        read::declare(|builder| {
            builder.local_detail(
                "TemporalIntent",
                schema_view(),
                |query| {
                    query
                        .project(field("IntentIdentityField"))
                        .project(field("IntentGateField"))
                },
                |shape| {
                    shape
                        .field(result_field("IntentIdentityField", "identity"))
                        .field(result_field("IntentGateField", "gate"))
                },
            )
        })
        .expect("the temporal certification read must remain canonical")
    })
}

pub(super) fn ordered_read_declaration() -> &'static read::WorthQueryReadDeclaration {
    static DECLARATION: OnceLock<read::WorthQueryReadDeclaration> = OnceLock::new();
    DECLARATION.get_or_init(|| {
        read::declare(|builder| {
            builder.local_collection(
                "TemporalIntent",
                schema_view(),
                |query| {
                    query
                        .project(field("IntentIdentityField"))
                        .project(field("IntentGateField"))
                        .order_by(
                            OrderingSelector::ascending("IntentFacts", "IntentGateField").unwrap(),
                        )
                },
                |shape| shape.field(result_field("IntentIdentityField", "identity")),
            )
        })
        .expect("the ordered certification read must remain canonical")
    })
}

pub(super) fn detail_patch_query() -> CanonicalQueryBundle {
    let query = DetailQueryBuilder::new(root())
        .project(field("IntentIdentityField"))
        .project(field("IntentGateField"))
        .build()
        .unwrap()
        .into_raw();
    let shape = DetailResultShapeBuilder::new()
        .field(result_field("IntentIdentityField", "identity"))
        .field(result_field("IntentGateField", "gate"))
        .build()
        .unwrap()
        .into_raw();
    canonicalize_request(
        AuthoredQueryBundleRequest::for_ordinary_read(query, shape, QueryBindingDescriptor::new())
            .unwrap(),
    )
    .unwrap()
}

pub(super) fn ordered_portfolio_query() -> CanonicalQueryBundle {
    let query = CollectionQueryBuilder::new(root())
        .project(field("IntentIdentityField"))
        .project(field("IntentGateField"))
        .order_by(OrderingSelector::ascending("IntentFacts", "IntentGateField").unwrap())
        .build()
        .unwrap()
        .into_raw();
    let shape = CollectionResultShapeBuilder::new()
        .field(result_field("IntentIdentityField", "identity"))
        .build()
        .unwrap()
        .into_raw();
    canonicalize_request(
        AuthoredQueryBundleRequest::for_ordinary_read(query, shape, QueryBindingDescriptor::new())
            .unwrap(),
    )
    .unwrap()
}

fn schema_view() -> read::QuerySchemaView {
    read::QuerySchemaView::new(
        "temporal-primary",
        [
            read::SchemaFieldView::new(
                read::AspectName::new("IntentFacts").unwrap(),
                read::FieldName::new("IntentIdentityField").unwrap(),
                read::ScalarAspectType::String,
            ),
            read::SchemaFieldView::new(
                read::AspectName::new("IntentFacts").unwrap(),
                read::FieldName::new("IntentGateField").unwrap(),
                read::ScalarAspectType::String,
            ),
        ],
        [],
    )
}

fn root() -> RootEntityKey {
    RootEntityKey::new("TemporalIntent").unwrap()
}

fn field(name: &str) -> AspectFieldSelector {
    AspectFieldSelector::new("IntentFacts", name).unwrap()
}

fn result_field(name: &str, alias: &str) -> AuthoredResultShapeField {
    AuthoredResultShapeField::new("IntentFacts", name, alias).unwrap()
}
