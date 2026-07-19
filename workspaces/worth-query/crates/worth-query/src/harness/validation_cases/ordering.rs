use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, OrderingSelector,
    RootEntityKey,
};
use crate::harness::fixtures::schema_view::{detail_schema_view, legal_ordering_only_bundle};
use crate::validation::{validate_canonical_bundle, QueryValidationError, ValidationEvent};

#[test]
fn ordering_only_field_validates_without_projection_widening() {
    let validated = validate_canonical_bundle(legal_ordering_only_bundle(), detail_schema_view())
        .expect("ordering-only bundle should validate");

    assert_eq!(validated.counters().validated_ordering_field_count(), 1);
    assert_eq!(validated.query().ordering().entries().len(), 1);
    assert!(!validated.query().ordering().entries()[0].projected());
    assert!(validated.report().events().iter().any(|event| matches!(
        event,
        ValidationEvent::OrderingValidated {
            aspect,
            field,
            direction: "descending",
            projected: false,
            ..
        } if aspect == "profile" && field == "rank"
    )));
}

#[test]
fn non_orderable_field_rejects() {
    let root = RootEntityKey::new("user").expect("root should build");
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .project(AspectFieldSelector::new("identity", "id").expect("projection should build"))
        .order_by(
            OrderingSelector::ascending("profile", "private_note").expect("ordering should build"),
        )
        .build()
        .expect("query should build");
    let shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(
            AuthoredResultShapeField::new("identity", "id", "id")
                .expect("shape field should build"),
        )
        .build()
        .expect("shape should build");
    let bundle =
        GuidedAuthoringPath::canonicalize_detail(query, shape).expect("bundle should canonicalize");

    let error = validate_canonical_bundle(bundle, detail_schema_view())
        .expect_err("non-orderable field should reject");

    assert_eq!(
        error,
        QueryValidationError::NonOrderableField {
            aspect: "profile".to_string(),
            field: "private_note".to_string(),
        }
    );
}

#[test]
fn duplicate_ordering_entries_collapse_in_validated_ordering_set() {
    let root = RootEntityKey::new("user").expect("root should build");
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .project(AspectFieldSelector::new("identity", "id").expect("projection should build"))
        .order_by(OrderingSelector::ascending("profile", "rank").expect("ordering should build"))
        .order_by(OrderingSelector::ascending("profile", "rank").expect("ordering should build"))
        .build()
        .expect("query should build");
    let shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(
            AuthoredResultShapeField::new("identity", "id", "id")
                .expect("shape field should build"),
        )
        .build()
        .expect("shape should build");
    let bundle =
        GuidedAuthoringPath::canonicalize_detail(query, shape).expect("bundle should canonicalize");

    let validated =
        validate_canonical_bundle(bundle, detail_schema_view()).expect("ordering should validate");

    assert_eq!(validated.query().ordering().entries().len(), 1);
    assert_eq!(validated.counters().validated_ordering_field_count(), 1);
}
