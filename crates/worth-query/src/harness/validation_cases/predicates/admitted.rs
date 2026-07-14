use super::canonical_predicate_bundle;
use crate::authoring::{IntegerComparisonPredicate, PresencePredicate, StringContainsPredicate};
use crate::harness::fixtures::schema_view::{detail_schema_view, workflow_queryable_schema_view};
use crate::validation::{validate_canonical_bundle, ValidationEvent};

#[test]
fn integer_greater_than_predicate_validates() {
    let validated = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query.where_greater_than(
                IntegerComparisonPredicate::greater_than("profile", "age", 18)
                    .expect("predicate should build"),
            )
        }),
        detail_schema_view(),
    )
    .expect("greater-than predicate should validate");

    assert_eq!(validated.counters().validated_predicate_count(), 1);
    assert!(validated.report().events().iter().any(|event| matches!(
        event,
        ValidationEvent::PredicateValidated {
            predicate_family: "integer-greater-than",
            ..
        }
    )));
}

#[test]
fn integer_less_than_predicate_validates() {
    let validated = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query.where_less_than(
                IntegerComparisonPredicate::less_than("profile", "age", 65)
                    .expect("predicate should build"),
            )
        }),
        detail_schema_view(),
    )
    .expect("less-than predicate should validate");

    assert_eq!(validated.counters().validated_predicate_count(), 1);
    assert_eq!(
        validated.query().predicates().entries()[0].predicate_family(),
        "integer-less-than"
    );
}

#[test]
fn string_contains_predicate_validates_when_schema_admits_it() {
    let validated = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query.where_contains(
                StringContainsPredicate::new("profile", "display_name", "est")
                    .expect("predicate should build"),
            )
        }),
        detail_schema_view(),
    )
    .expect("contains predicate should validate");

    assert_eq!(validated.counters().validated_predicate_count(), 1);
    assert_eq!(
        validated.query().predicates().entries()[0].predicate_family(),
        "string-contains"
    );
}

#[test]
fn presence_predicate_validates_when_schema_admits_it() {
    let validated = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query.where_present(
                PresencePredicate::is_present("profile", "display_name")
                    .expect("predicate should build"),
            )
        }),
        detail_schema_view(),
    )
    .expect("presence predicate should validate");

    assert_eq!(validated.counters().validated_predicate_count(), 1);
    assert_eq!(
        validated.query().predicates().entries()[0].predicate_family(),
        "presence-is-present"
    );
}

#[test]
fn workflow_predicate_validates_when_schema_admits_it() {
    let validated = validate_canonical_bundle(
        crate::harness::fixtures::schema_view::legal_workflow_predicate_bundle(),
        workflow_queryable_schema_view(),
    )
    .expect("workflow predicate should validate when schema admits it");

    assert_eq!(validated.counters().validated_predicate_count(), 1);
    assert_eq!(
        validated.query().predicates().entries()[0].predicate_family(),
        "equality"
    );
    assert_eq!(
        validated.query().predicates().entries()[0].value_basis(),
        "string:done"
    );
}
