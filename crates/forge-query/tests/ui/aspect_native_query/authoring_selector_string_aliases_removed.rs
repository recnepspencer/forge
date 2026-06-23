use forge_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, IntegerComparisonPredicate,
    OrderingSelector, PredicateSelector, PresencePredicate, ScalarPredicateValue,
    SetMembershipPredicate, StringContainsPredicate,
};

fn main() {
    let projection = AspectFieldSelector::new("identity", "id").unwrap();
    let _ = projection.aspect();
    let _ = projection.field();

    let result_field = AuthoredResultShapeField::new("identity", "id", "id").unwrap();
    let _ = result_field.source_aspect();
    let _ = result_field.source_field();

    let ordering = OrderingSelector::ascending("identity", "id").unwrap();
    let _ = ordering.aspect();
    let _ = ordering.field();

    let equality =
        EqualityPredicate::new("identity", "id", ScalarPredicateValue::String("one".into()))
            .unwrap();
    let _ = equality.aspect();
    let _ = equality.field();

    let integer = IntegerComparisonPredicate::greater_than("metrics", "rank", 1).unwrap();
    let _ = integer.aspect();
    let _ = integer.field();

    let contains = StringContainsPredicate::new("profile", "bio", "forge").unwrap();
    let _ = contains.aspect();
    let _ = contains.field();

    let membership = SetMembershipPredicate::new(
        "status",
        "value",
        [ScalarPredicateValue::String("open".into())],
    )
    .unwrap();
    let _ = membership.aspect();
    let _ = membership.field();

    let presence = PresencePredicate::is_present("identity", "id").unwrap();
    let _ = presence.aspect();
    let _ = presence.field();

    let predicate = PredicateSelector::Presence(presence);
    let _ = predicate.aspect();
    let _ = predicate.field();
}
