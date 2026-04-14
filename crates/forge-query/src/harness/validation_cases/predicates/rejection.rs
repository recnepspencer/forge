use super::canonical_predicate_bundle;
use crate::authoring::{
    EqualityPredicate, IntegerComparisonPredicate, PresencePredicate, ScalarPredicateValue,
    SetMembershipPredicate, StringContainsPredicate,
};
use crate::harness::fixtures::schema_view::detail_schema_view;
use crate::validation::{validate_canonical_bundle, QueryValidationError};

#[test]
fn contradictory_predicates_reject_explicitly() {
    let error = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query
                .where_equal(
                    EqualityPredicate::new("profile", "age", ScalarPredicateValue::Integer(18))
                        .expect("predicate should build"),
                )
                .where_greater_than(
                    IntegerComparisonPredicate::greater_than("profile", "age", 18)
                        .expect("predicate should build"),
                )
        }),
        detail_schema_view(),
    )
    .expect_err("contradictory predicates should reject");

    assert_eq!(
        error,
        QueryValidationError::ContradictoryPredicateSet {
            aspect: "profile".to_string(),
            field: "age".to_string(),
            reason: "equality-violates-greater-than",
        }
    );
}

#[test]
fn equality_outside_membership_rejects_explicitly() {
    let error = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query
                .where_in(
                    SetMembershipPredicate::new(
                        "profile",
                        "age",
                        [
                            ScalarPredicateValue::Integer(18),
                            ScalarPredicateValue::Integer(21),
                        ],
                    )
                    .expect("predicate should build"),
                )
                .where_equal(
                    EqualityPredicate::new("profile", "age", ScalarPredicateValue::Integer(34))
                        .expect("predicate should build"),
                )
        }),
        detail_schema_view(),
    )
    .expect_err("equality outside membership should reject");

    assert_eq!(
        error,
        QueryValidationError::ContradictoryPredicateSet {
            aspect: "profile".to_string(),
            field: "age".to_string(),
            reason: "equality-outside-membership",
        }
    );
}

#[test]
fn empty_integer_range_rejects_explicitly() {
    let error = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query
                .where_greater_than(
                    IntegerComparisonPredicate::greater_than("profile", "age", 65)
                        .expect("predicate should build"),
                )
                .where_less_than(
                    IntegerComparisonPredicate::less_than("profile", "age", 65)
                        .expect("predicate should build"),
                )
        }),
        detail_schema_view(),
    )
    .expect_err("empty range should reject");

    assert_eq!(
        error,
        QueryValidationError::ContradictoryPredicateSet {
            aspect: "profile".to_string(),
            field: "age".to_string(),
            reason: "empty-range",
        }
    );
}

#[test]
fn membership_predicate_without_schema_capability_rejects() {
    let error = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query.where_in(
                SetMembershipPredicate::new(
                    "profile",
                    "private_note",
                    [ScalarPredicateValue::String("secret".to_string())],
                )
                .expect("predicate should build"),
            )
        }),
        detail_schema_view(),
    )
    .expect_err("membership predicate without capability should reject");

    assert_eq!(
        error,
        QueryValidationError::IncompatiblePredicateFamily {
            aspect: "profile".to_string(),
            field: "private_note".to_string(),
            predicate_family: "scalar-membership",
            field_kind: "String",
        }
    );
}

#[test]
fn presence_predicate_without_schema_capability_rejects() {
    let error = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query.where_present(
                PresencePredicate::is_present("profile", "private_note")
                    .expect("predicate should build"),
            )
        }),
        detail_schema_view(),
    )
    .expect_err("presence predicate without capability should reject");

    assert_eq!(
        error,
        QueryValidationError::IncompatiblePredicateFamily {
            aspect: "profile".to_string(),
            field: "private_note".to_string(),
            predicate_family: "presence-is-present",
            field_kind: "String",
        }
    );
}

#[test]
fn incompatible_predicate_family_rejects() {
    let error = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query.where_equal(
                EqualityPredicate::new(
                    "profile",
                    "age",
                    ScalarPredicateValue::String("too-old".to_string()),
                )
                .expect("predicate should build"),
            )
        }),
        detail_schema_view(),
    )
    .expect_err("incompatible predicate should reject");

    assert_eq!(
        error,
        QueryValidationError::IncompatiblePredicateFamily {
            aspect: "profile".to_string(),
            field: "age".to_string(),
            predicate_family: "equality",
            field_kind: "Integer",
        }
    );
}

#[test]
fn text_predicate_without_schema_capability_rejects() {
    let error = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query.where_contains(
                StringContainsPredicate::new("profile", "private_note", "secret")
                    .expect("predicate should build"),
            )
        }),
        detail_schema_view(),
    )
    .expect_err("text predicate without capability should reject");

    assert_eq!(
        error,
        QueryValidationError::IncompatiblePredicateFamily {
            aspect: "profile".to_string(),
            field: "private_note".to_string(),
            predicate_family: "string-contains",
            field_kind: "String",
        }
    );
}

#[test]
fn workflow_predicate_context_rejects_explicitly() {
    let error = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query.where_equal(
                EqualityPredicate::new(
                    "workflow",
                    "status",
                    ScalarPredicateValue::String("done".to_string()),
                )
                .expect("predicate should build"),
            )
        }),
        detail_schema_view(),
    )
    .expect_err("workflow predicate should reject");

    assert_eq!(
        error,
        QueryValidationError::IllegalWorkflowPredicateCapabilityOrContextShape {
            aspect: "workflow".to_string(),
            field: "status".to_string(),
            predicate_family: "equality",
        }
    );
}
