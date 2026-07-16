use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, CollectionQueryBuilder,
    CollectionResultShapeBuilder, DetailQueryBuilder, DetailResultShapeBuilder,
    GuidedAuthoringPath, OrderingSelector, RootEntityKey, TraversalSelector,
};
use crate::canonicalization::CanonicalQueryBundle;
use crate::schema_view::{QuerySchemaView, ScalarAspectType, SchemaFieldView, SchemaRelationView};

pub fn detail_schema_view() -> QuerySchemaView {
    QuerySchemaView::new(
        "detail-v1",
        [
            SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                ScalarAspectType::String,
            ),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                ScalarAspectType::String,
            )
            .text_predicate_queryable()
            .membership_predicate_queryable()
            .presence_predicate_queryable(),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("age")
                    .expect("schema field literal must be valid"),
                ScalarAspectType::Int64,
            )
            .membership_predicate_queryable()
            .presence_predicate_queryable(),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("rank")
                    .expect("schema field literal must be valid"),
                ScalarAspectType::Int64,
            )
            .ordering_only(),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("private_note")
                    .expect("schema field literal must be valid"),
                ScalarAspectType::String,
            )
            .non_queryable(),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("content")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("bio")
                    .expect("schema field literal must be valid"),
                ScalarAspectType::ContentRef,
            )
            .non_queryable(),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("workflow")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("status")
                    .expect("schema field literal must be valid"),
                ScalarAspectType::String,
            )
            .workflow_semantic(),
        ],
        [SchemaRelationView::new(
            crate::authoring::RelationName::new("manager")
                .expect("schema relation literal must be valid"),
            1,
        )],
    )
}

pub fn structured_content_queryable_schema_view() -> QuerySchemaView {
    QuerySchemaView::new(
        "structured-content-v1",
        [
            SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                ScalarAspectType::String,
            ),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("content")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("bio")
                    .expect("schema field literal must be valid"),
                ScalarAspectType::ContentRef,
            )
            .non_orderable(),
        ],
        [],
    )
}

pub fn workflow_queryable_schema_view() -> QuerySchemaView {
    QuerySchemaView::new(
        "workflow-v1",
        [
            SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                ScalarAspectType::String,
            ),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("workflow")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("status")
                    .expect("schema field literal must be valid"),
                ScalarAspectType::String,
            )
            .workflow_predicate_queryable()
            .non_orderable(),
        ],
        [],
    )
}

pub fn alternate_detail_schema_view() -> QuerySchemaView {
    QuerySchemaView::new(
        "detail-v2",
        [
            SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                ScalarAspectType::String,
            ),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                ScalarAspectType::String,
            )
            .text_predicate_queryable()
            .membership_predicate_queryable()
            .presence_predicate_queryable(),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("age")
                    .expect("schema field literal must be valid"),
                ScalarAspectType::Int64,
            )
            .membership_predicate_queryable()
            .presence_predicate_queryable(),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("rank")
                    .expect("schema field literal must be valid"),
                ScalarAspectType::Int64,
            )
            .ordering_only(),
        ],
        [SchemaRelationView::new(
            crate::authoring::RelationName::new("manager")
                .expect("schema relation literal must be valid"),
            1,
        )],
    )
}

pub fn legal_structured_content_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").expect("root should build");
    let query = DetailQueryBuilder::new(root)
        .project(AspectFieldSelector::new("identity", "id").expect("id projection should build"))
        .project(
            AspectFieldSelector::new("content", "bio").expect("content projection should build"),
        )
        .build()
        .expect("structured content detail query should build");

    let result_shape = DetailResultShapeBuilder::new()
        .field(
            AuthoredResultShapeField::new("identity", "id", "id")
                .expect("result-shape field should build"),
        )
        .field(
            AuthoredResultShapeField::new("content", "bio", "bio")
                .expect("structured content result-shape field should build"),
        )
        .build()
        .expect("structured content result shape should build");

    GuidedAuthoringPath::canonicalize_detail(query, result_shape)
        .expect("legal structured content query should canonicalize")
}

pub fn legal_workflow_predicate_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").expect("root should build");
    let query = DetailQueryBuilder::new(root)
        .project(AspectFieldSelector::new("identity", "id").expect("id projection should build"))
        .where_equal(
            crate::authoring::EqualityPredicate::new(
                "workflow",
                "status",
                crate::authoring::WorthQueryPredicateOperand::string("done".to_string()),
            )
            .expect("workflow equality predicate should build"),
        )
        .build()
        .expect("workflow predicate query should build");

    let result_shape = DetailResultShapeBuilder::new()
        .field(
            AuthoredResultShapeField::new("identity", "id", "id")
                .expect("result-shape field should build"),
        )
        .build()
        .expect("workflow predicate result shape should build");

    GuidedAuthoringPath::canonicalize_detail(query, result_shape)
        .expect("legal workflow predicate query should canonicalize")
}

pub fn legal_detail_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").expect("root should build");
    let query = DetailQueryBuilder::new(root)
        .project(AspectFieldSelector::new("identity", "id").expect("id projection should build"))
        .project(
            AspectFieldSelector::new("profile", "display_name")
                .expect("profile projection should build"),
        )
        .traverse(TraversalSelector::bounded("manager", 1).expect("traversal should build"))
        .build()
        .expect("detail query should build");

    let result_shape = DetailResultShapeBuilder::new()
        .field(
            AuthoredResultShapeField::new("identity", "id", "id")
                .expect("result-shape field should build"),
        )
        .field(
            AuthoredResultShapeField::new("profile", "display_name", "name")
                .expect("result-shape field should build"),
        )
        .build()
        .expect("detail result shape should build");

    GuidedAuthoringPath::canonicalize_detail(query, result_shape)
        .expect("legal detail query should canonicalize")
}

pub fn legal_collection_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").expect("root should build");
    let query = CollectionQueryBuilder::new(root)
        .project(AspectFieldSelector::new("identity", "id").expect("id projection should build"))
        .project(
            AspectFieldSelector::new("profile", "display_name")
                .expect("profile projection should build"),
        )
        .build()
        .expect("collection query should build");

    let result_shape = CollectionResultShapeBuilder::new()
        .field(
            AuthoredResultShapeField::new("identity", "id", "id")
                .expect("result-shape field should build"),
        )
        .field(
            AuthoredResultShapeField::new("profile", "display_name", "name")
                .expect("result-shape field should build"),
        )
        .build()
        .expect("collection result shape should build");

    GuidedAuthoringPath::canonicalize_collection(query, result_shape)
        .expect("legal collection query should canonicalize")
}

pub fn legal_ordering_only_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").expect("root should build");
    let query = DetailQueryBuilder::new(root)
        .project(AspectFieldSelector::new("identity", "id").expect("id projection should build"))
        .project(
            AspectFieldSelector::new("profile", "display_name")
                .expect("profile projection should build"),
        )
        .order_by(OrderingSelector::descending("profile", "rank").expect("ordering should build"))
        .build()
        .expect("detail query should build");

    let result_shape = DetailResultShapeBuilder::new()
        .field(
            AuthoredResultShapeField::new("identity", "id", "id")
                .expect("result-shape field should build"),
        )
        .field(
            AuthoredResultShapeField::new("profile", "display_name", "name")
                .expect("result-shape field should build"),
        )
        .build()
        .expect("detail result shape should build");

    GuidedAuthoringPath::canonicalize_detail(query, result_shape)
        .expect("legal ordering-only query should canonicalize")
}
