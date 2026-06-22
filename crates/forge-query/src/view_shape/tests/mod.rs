use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RootEntityKey,
};

mod grouped;
mod inspector;
mod support;
mod table_detail;
mod temporal_async_posture;

fn detail_schema_view() -> crate::schema_view::QuerySchemaView {
    crate::schema_view::QuerySchemaView::new(
        "view-shape-detail",
        [
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                crate::schema_view::SchemaFieldKind::String,
            ),
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
        ],
        [],
    )
}

fn collection_schema_view() -> crate::schema_view::QuerySchemaView {
    crate::schema_view::QuerySchemaView::new(
        "view-shape-collection",
        [
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                crate::schema_view::SchemaFieldKind::String,
            ),
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("status")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("lane")
                    .expect("schema field literal must be valid"),
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
        ],
        [],
    )
}

fn wide_collection_schema_view() -> crate::schema_view::QuerySchemaView {
    crate::schema_view::QuerySchemaView::new(
        "view-shape-wide-collection",
        [
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                crate::schema_view::SchemaFieldKind::String,
            ),
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("status")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("lane")
                    .expect("schema field literal must be valid"),
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("meta")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("priority")
                    .expect("schema field literal must be valid"),
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
        ],
        [],
    )
}

fn direct_detail() -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_detail(
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        crate::authoring::RawAuthoredResultShape::detail_builder()
            .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .field(
                AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap(),
            )
            .build()
            .unwrap(),
    )
    .unwrap()
}

fn direct_collection() -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_collection(
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .project(AspectFieldSelector::new("status", "lane").unwrap())
            .order_by(
                crate::authoring::OrderingSelector::ascending("profile", "display_name").unwrap(),
            )
            .build()
            .unwrap(),
        crate::authoring::RawAuthoredResultShape::collection_builder()
            .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .field(
                AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap(),
            )
            .field(AuthoredResultShapeField::new("status", "lane", "lane").unwrap())
            .build()
            .unwrap(),
    )
    .unwrap()
}

fn wide_collection() -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_collection(
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .project(AspectFieldSelector::new("status", "lane").unwrap())
            .project(AspectFieldSelector::new("meta", "priority").unwrap())
            .order_by(
                crate::authoring::OrderingSelector::ascending("profile", "display_name").unwrap(),
            )
            .build()
            .unwrap(),
        crate::authoring::RawAuthoredResultShape::collection_builder()
            .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .field(
                AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap(),
            )
            .field(AuthoredResultShapeField::new("status", "lane", "lane").unwrap())
            .field(AuthoredResultShapeField::new("meta", "priority", "priority").unwrap())
            .build()
            .unwrap(),
    )
    .unwrap()
}

fn basis_intent() -> crate::basis::ExecutionBasisIntent {
    crate::basis::ExecutionBasisIntent::new(
        crate::basis::BasisAuthorityFamily::Runtime,
        crate::basis::SnapshotLineageClass::CurrentHead,
        false,
    )
}
