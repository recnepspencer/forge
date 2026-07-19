use super::canonical_predicate_bundle;
use crate::authoring::{
    EqualityPredicate, NativeComparisonPredicate, SetMembershipPredicate, StringContainsPredicate,
    WorthQueryPredicateOperand,
};
use crate::harness::fixtures::schema_view::detail_schema_view;
use crate::validation::validate_canonical_bundle;
use worth_foundational::facade::{AspectValue, InternedString};

fn native_basis(value: AspectValue) -> String {
    worth_foundational::facade::prepare_aspect_value_identity_basis(&value)
        .as_str()
        .to_owned()
}

#[test]
fn string_contains_predicates_normalize_by_subsumption() {
    let validated = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query
                .where_contains(
                    StringContainsPredicate::new("profile", "display_name", "est")
                        .expect("predicate should build"),
                )
                .where_contains(
                    StringContainsPredicate::new("profile", "display_name", "tester")
                        .expect("predicate should build"),
                )
        }),
        detail_schema_view(),
    )
    .expect("subsumed contains predicates should validate");

    assert_eq!(validated.counters().validated_predicate_count(), 1);
    assert_eq!(validated.query().predicates().entries().len(), 1);
    assert_eq!(
        validated.query().predicates().entries()[0].predicate_family(),
        "string-contains"
    );
    assert_eq!(
        validated.query().predicates().entries()[0].value_basis(),
        native_basis(AspectValue::String(InternedString::Raw("tester".into())))
    );
}

#[test]
fn scalar_membership_predicate_validates_and_normalizes_by_intersection() {
    let validated = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query
                .where_in(
                    SetMembershipPredicate::new(
                        "profile",
                        "age",
                        [
                            WorthQueryPredicateOperand::int64(18),
                            WorthQueryPredicateOperand::int64(21),
                            WorthQueryPredicateOperand::int64(34),
                        ],
                    )
                    .expect("predicate should build"),
                )
                .where_in(
                    SetMembershipPredicate::new(
                        "profile",
                        "age",
                        [
                            WorthQueryPredicateOperand::int64(21),
                            WorthQueryPredicateOperand::int64(34),
                            WorthQueryPredicateOperand::int64(55),
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
        format!(
            "set:[{},{}]",
            native_basis(AspectValue::Int64(21)),
            native_basis(AspectValue::Int64(34))
        )
    );
}

#[test]
fn redundant_native_greater_than_predicates_normalize_to_strongest_bound() {
    let validated = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query
                .where_greater_than(
                    NativeComparisonPredicate::greater_than("profile", "age", 18)
                        .expect("predicate should build"),
                )
                .where_greater_than(
                    NativeComparisonPredicate::greater_than("profile", "age", 21)
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
        "native-greater-than"
    );
    assert_eq!(
        validated.query().predicates().entries()[0].value_basis(),
        native_basis(AspectValue::Int64(21))
    );
}

#[test]
fn bounded_integer_range_normalizes_to_two_validated_predicates() {
    let validated = validate_canonical_bundle(
        canonical_predicate_bundle(|query| {
            query
                .where_greater_than(
                    NativeComparisonPredicate::greater_than("profile", "age", 18)
                        .expect("predicate should build"),
                )
                .where_less_than(
                    NativeComparisonPredicate::less_than("profile", "age", 65)
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
            query
                .where_in(
                    SetMembershipPredicate::new(
                        "profile",
                        "age",
                        [
                            WorthQueryPredicateOperand::int64(18),
                            WorthQueryPredicateOperand::int64(21),
                            WorthQueryPredicateOperand::int64(34),
                        ],
                    )
                    .expect("predicate should build"),
                )
                .where_equal(
                    EqualityPredicate::new("profile", "age", WorthQueryPredicateOperand::int64(21))
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
        native_basis(AspectValue::Int64(21))
    );
}
