use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, GuidedAuthoringPath,
    IntegerComparisonPredicate, PresencePredicate, RootEntityKey, ScalarPredicateValue,
    SetMembershipPredicate, StringContainsPredicate,
};
use crate::harness::fixtures::schema_view::detail_schema_view;
use crate::validation::{validate_canonical_bundle, QueryValidationError, ValidationEvent};

fn canonical_predicate_bundle(
    configure: impl FnOnce(
        crate::authoring::DetailQueryBuilder,
    ) -> crate::authoring::DetailQueryBuilder,
) -> crate::facade::CanonicalQueryBundle {
    let root = RootEntityKey::new("user").expect("root should build");
    let query = configure(
        crate::authoring::DetailQueryBuilder::new(root)
            .project(AspectFieldSelector::new("identity", "id").expect("projection should build")),
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
    GuidedAuthoringPath::canonicalize_detail(query, shape).expect("bundle should canonicalize")
}

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
fn scalar_membership_predicate_validates_and_normalizes_by_intersection() {
    let validated = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query.where_in(
                SetMembershipPredicate::new(
                    "profile",
                    "age",
                    [
                        ScalarPredicateValue::Integer(18),
                        ScalarPredicateValue::Integer(21),
                        ScalarPredicateValue::Integer(34),
                    ],
                )
                .expect("predicate should build"),
            )
            .where_in(
                SetMembershipPredicate::new(
                    "profile",
                    "age",
                    [
                        ScalarPredicateValue::Integer(21),
                        ScalarPredicateValue::Integer(34),
                        ScalarPredicateValue::Integer(55),
                    ],
                )
                .expect("predicate should build"),
            )
        }),
        detail_schema_view(),
    )
    .expect("membership predicate should validate");

    assert_eq!(validated.counters().validated_predicate_count(), 1);
    assert_eq!(
        validated.query().predicates().entries()[0].predicate_family(),
        "scalar-membership"
    );
    assert_eq!(
        validated.query().predicates().entries()[0].value_basis(),
        "set:[integer:21,integer:34]"
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
fn redundant_integer_greater_than_predicates_normalize_to_strongest_bound() {
    let validated = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query.where_greater_than(
                IntegerComparisonPredicate::greater_than("profile", "age", 18)
                    .expect("predicate should build"),
            )
            .where_greater_than(
                IntegerComparisonPredicate::greater_than("profile", "age", 21)
                    .expect("predicate should build"),
            )
        }),
        detail_schema_view(),
    )
    .expect("redundant greater-than predicates should normalize");

    assert_eq!(validated.counters().validated_predicate_count(), 1);
    assert_eq!(validated.query().predicates().entries().len(), 1);
    assert_eq!(
        validated.query().predicates().entries()[0].predicate_family(),
        "integer-greater-than"
    );
    assert_eq!(
        validated.query().predicates().entries()[0].value_basis(),
        "integer:21"
    );
}

#[test]
fn bounded_integer_range_normalizes_to_two_validated_predicates() {
    let validated = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query.where_greater_than(
                IntegerComparisonPredicate::greater_than("profile", "age", 18)
                    .expect("predicate should build"),
            )
            .where_less_than(
                IntegerComparisonPredicate::less_than("profile", "age", 65)
                    .expect("predicate should build"),
            )
        }),
        detail_schema_view(),
    )
    .expect("bounded range should validate");

    assert_eq!(validated.counters().validated_predicate_count(), 2);
    assert_eq!(validated.query().predicates().entries().len(), 2);
}

#[test]
fn equality_inside_membership_normalizes_to_equality() {
    let validated = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query.where_in(
                SetMembershipPredicate::new(
                    "profile",
                    "age",
                    [
                        ScalarPredicateValue::Integer(18),
                        ScalarPredicateValue::Integer(21),
                        ScalarPredicateValue::Integer(34),
                    ],
                )
                .expect("predicate should build"),
            )
            .where_equal(
                EqualityPredicate::new("profile", "age", ScalarPredicateValue::Integer(21))
                    .expect("predicate should build"),
            )
        }),
        detail_schema_view(),
    )
    .expect("equality plus compatible membership should validate");

    assert_eq!(validated.counters().validated_predicate_count(), 1);
    assert_eq!(
        validated.query().predicates().entries()[0].predicate_family(),
        "equality"
    );
    assert_eq!(
        validated.query().predicates().entries()[0].value_basis(),
        "integer:21"
    );
}

#[test]
fn contradictory_predicates_reject_explicitly() {
    let error = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query.where_equal(
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
            query.where_in(
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
            query.where_greater_than(
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
