use crate::authoring::{AspectName, FieldName};
use crate::schema_view::{QuerySchemaView, SchemaFieldView};
use crate::typed::{
    TypedCollectionQueryBuilder, TypedCollectionResultShapeBuilder, TypedDetailQueryBuilder,
    TypedDetailResultShapeBuilder, TypedGuidedAuthoringPath, TypedSchemaField,
};
use crate::validation::{validate_canonical_bundle, QueryValidationError};
use worth_foundational::facade::ScalarAspectType;

worth_query_schema! {
    schema UserSchema("user") {
        fields {
            field IdentityId("identity", "id", String) => [projectable, equality(String)];
            field DisplayName("profile", "display_name", String) => [projectable, equality(String), contains, membership, presence];
            field Age("profile", "age", Int64) => [projectable, equality(i64), native_comparable, membership, presence, orderable];
            field Rank("profile", "rank", Int64) => [ordering_only];
            field PrivateNote("profile", "private_note", String) => [non_queryable];
            field Bio("content", "bio", ContentRef) => [non_queryable];
            field WorkflowStatus("workflow", "status", String) => [workflow];
        }
        relations {
            relation ManagerRelation("manager", 1);
        }
    }
}

#[test]
fn typed_detail_query_api_canonicalizes_and_validates() {
    let query = TypedDetailQueryBuilder::<UserSchema>::new()
        .project::<IdentityId>()
        .project::<DisplayName>()
        .where_contains::<DisplayName>("est")
        .where_greater_than::<Age>(18)
        .order_by_descending::<Rank>()
        .traverse::<ManagerRelation>(1)
        .expect("typed traversal should build")
        .build()
        .expect("typed query should build");

    let shape = TypedDetailResultShapeBuilder::<UserSchema>::new()
        .field::<IdentityId>()
        .field_as::<DisplayName>("name")
        .build()
        .expect("typed result shape should build");

    let bundle = TypedGuidedAuthoringPath::canonicalize_detail(query, shape)
        .expect("typed guided path should canonicalize");
    let validated = validate_canonical_bundle(bundle, UserSchema::schema_view())
        .expect("typed query should validate");

    assert_eq!(validated.query().projection().len(), 2);
    assert_eq!(validated.query().predicates().entries().len(), 2);
    assert_eq!(validated.query().ordering().entries().len(), 1);
    assert_eq!(validated.query().traversal().len(), 1);
    assert_eq!(validated.result_shape().bindings().len(), 2);
}

#[test]
fn generated_schema_view_matches_typed_surface_expectations() {
    let schema = UserSchema::schema_view();

    assert!(!schema.basis().as_str().is_empty());
    assert_eq!(
        schema_field(&schema, DisplayName::ASPECT, DisplayName::FIELD)
            .expect("display name field should exist")
            .kind(),
        &ScalarAspectType::String
    );
    assert!(
        schema_field(&schema, DisplayName::ASPECT, DisplayName::FIELD)
            .expect("display name field should exist")
            .is_text_predicate_queryable()
    );
    assert!(!schema_field(&schema, Rank::ASPECT, Rank::FIELD)
        .expect("rank field should exist")
        .is_queryable());
    assert!(schema_field(&schema, Rank::ASPECT, Rank::FIELD)
        .expect("rank field should exist")
        .is_orderable());
    assert!(
        !schema_field(&schema, PrivateNote::ASPECT, PrivateNote::FIELD)
            .expect("private note field should exist")
            .is_queryable()
    );
    assert_eq!(
        schema_field(&schema, Bio::ASPECT, Bio::FIELD)
            .expect("bio field should exist")
            .kind(),
        &ScalarAspectType::ContentRef
    );
    assert_eq!(
        schema_field(&schema, WorkflowStatus::ASPECT, WorkflowStatus::FIELD)
            .expect("workflow field should exist")
            .kind(),
        &ScalarAspectType::String
    );
}

fn schema_field<'a>(
    schema: &'a QuerySchemaView,
    aspect: &str,
    field: &str,
) -> Option<&'a SchemaFieldView> {
    let aspect = AspectName::new(aspect).expect("typed schema aspect constant is valid");
    let field = FieldName::new(field).expect("typed schema field constant is valid");
    schema.field(&aspect, &field)
}

#[test]
fn typed_traversal_still_rejects_runtime_depth_violation() {
    let query = TypedDetailQueryBuilder::<UserSchema>::new()
        .project::<IdentityId>()
        .traverse::<ManagerRelation>(3)
        .expect("typed traversal should build")
        .build()
        .expect("typed query should build");
    let shape = TypedDetailResultShapeBuilder::<UserSchema>::new()
        .field::<IdentityId>()
        .build()
        .expect("typed result shape should build");

    let bundle = TypedGuidedAuthoringPath::canonicalize_detail(query, shape)
        .expect("typed guided path should canonicalize");
    let error = validate_canonical_bundle(bundle, UserSchema::schema_view())
        .expect_err("schema depth violation must still reject at runtime");

    assert_eq!(
        error,
        QueryValidationError::IllegalTraversalDepth {
            relation: "manager".to_string(),
            requested_depth: 3,
            max_depth: 1,
        }
    );
}

#[test]
fn typed_collection_query_api_canonicalizes_and_validates() {
    let query = TypedCollectionQueryBuilder::<UserSchema>::new()
        .project::<IdentityId>()
        .project::<DisplayName>()
        .where_in::<DisplayName, _>(["Esther".to_string(), "Ada".to_string()])
        .order_by_ascending::<Age>()
        .build()
        .expect("typed collection query should build");

    let shape = TypedCollectionResultShapeBuilder::<UserSchema>::new()
        .field::<IdentityId>()
        .field::<DisplayName>()
        .build()
        .expect("typed collection result shape should build");

    let bundle = TypedGuidedAuthoringPath::canonicalize_collection(query, shape)
        .expect("typed collection guided path should canonicalize");
    let validated = validate_canonical_bundle(bundle, UserSchema::schema_view())
        .expect("typed collection query should validate");

    assert_eq!(validated.query().projection().len(), 2);
    assert_eq!(validated.query().predicates().entries().len(), 1);
    assert_eq!(validated.query().ordering().entries().len(), 1);
    assert_eq!(validated.result_shape().bindings().len(), 2);
}
