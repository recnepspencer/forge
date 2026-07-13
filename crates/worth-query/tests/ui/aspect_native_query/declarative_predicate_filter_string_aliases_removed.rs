use worth_query::facade::foundation::{AspectFieldKey, DeclarativeEqualityFilter, DeclarativeIntegerComparisonFilter, DeclarativePredicateFilter, DeclarativePresenceFilter, DeclarativeSetMembershipFilter, DeclarativeStringContainsFilter, ScalarPredicateValue};

fn main() {
    let equality = DeclarativeEqualityFilter::new(
        AspectFieldKey::from_authoring_parts("identity", "id").unwrap(),
        ScalarPredicateValue::String("a".into()),
    );
    let _ = equality.aspect();
    let _ = equality.field();

    let integer = DeclarativeIntegerComparisonFilter::greater_than(
        AspectFieldKey::from_authoring_parts("metrics", "rank").unwrap(),
        1,
    );
    let _ = integer.aspect();
    let _ = integer.field();

    let contains = DeclarativeStringContainsFilter::new(
        AspectFieldKey::from_authoring_parts("title", "value").unwrap(),
        "needle",
    );
    let _ = contains.aspect();
    let _ = contains.field();

    let membership = DeclarativeSetMembershipFilter::new(
        AspectFieldKey::from_authoring_parts("status", "value").unwrap(),
        [ScalarPredicateValue::String("open".into())],
    );
    let _ = membership.aspect();
    let _ = membership.field();

    let presence =
        DeclarativePresenceFilter::is_present(AspectFieldKey::from_authoring_parts("owner", "id").unwrap());
    let _ = presence.aspect();
    let _ = presence.field();

    let predicate = DeclarativePredicateFilter::Presence(presence);
    let _ = predicate.aspect();
    let _ = predicate.field();
}
