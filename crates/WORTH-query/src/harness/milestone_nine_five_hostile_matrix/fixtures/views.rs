use crate::basis::{BasisAuthorityFamily, ExecutionBasisIntent, SnapshotLineageClass};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind, SchemaFieldView};
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeDescriptor, ViewShapePlanArtifact,
};
use worth_foundational::facade::AspectKey;

pub fn detail_view(
    canonical: &crate::canonicalization::CanonicalQueryBundle,
) -> ViewShapePlanArtifact {
    plan_view(
        canonical,
        detail_schema_view(),
        ViewShapeDescriptor::detail(),
    )
}

pub fn table_view(
    canonical: &crate::canonicalization::CanonicalQueryBundle,
) -> ViewShapePlanArtifact {
    plan_view(
        canonical,
        collection_schema_view(),
        ViewShapeDescriptor::table(),
    )
}

pub fn grouped_view(
    canonical: &crate::canonicalization::CanonicalQueryBundle,
) -> ViewShapePlanArtifact {
    plan_view(
        canonical,
        collection_schema_view(),
        ViewShapeDescriptor::kanban_grouped(aspect_key("status")),
    )
}

pub fn focused_inspector_view(
    canonical: &crate::canonicalization::CanonicalQueryBundle,
) -> ViewShapePlanArtifact {
    plan_view(
        canonical,
        detail_schema_view(),
        ViewShapeDescriptor::inspector_detail_focused(
            worth_foundational::facade::AspectKey::new("profile").unwrap(),
        ),
    )
}

fn plan_view(
    canonical: &crate::canonicalization::CanonicalQueryBundle,
    schema_view: QuerySchemaView,
    descriptor: ViewShapeDescriptor,
) -> ViewShapePlanArtifact {
    plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            canonical,
            schema_view,
            admit_view_shape(canonical, descriptor).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap()
}

fn basis_intent() -> ExecutionBasisIntent {
    ExecutionBasisIntent::new(
        BasisAuthorityFamily::Runtime,
        SnapshotLineageClass::CurrentHead,
        false,
    )
}

fn detail_schema_view() -> QuerySchemaView {
    QuerySchemaView::new(
        "milestone-nine-five-detail",
        [
            SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
        ],
        [],
    )
}

fn collection_schema_view() -> QuerySchemaView {
    QuerySchemaView::new(
        "milestone-nine-five-collection",
        [
            SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("status")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("lane")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
        ],
        [],
    )
}

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("milestone 9.5 grouped aspect should be foundational")
}
