use worth_query::facade::foundation::{DeclarativeEqualityFilter, DeclarativeIntegerComparisonFilter, DeclarativePresenceFilter, DeclarativeSetMembershipFilter, DeclarativeStringContainsFilter, ScalarPredicateValue};

fn main() {
    let _ =
        DeclarativeEqualityFilter::new("identity", "id", ScalarPredicateValue::String("a".into()));
    let _ = DeclarativeIntegerComparisonFilter::greater_than("metrics", "rank", 1);
    let _ = DeclarativeIntegerComparisonFilter::less_than("metrics", "rank", 1);
    let _ = DeclarativeStringContainsFilter::new("title", "value", "needle");
    let _ = DeclarativeSetMembershipFilter::new(
        "status",
        "value",
        [ScalarPredicateValue::String("open".into())],
    );
    let _ = DeclarativePresenceFilter::is_present("owner", "id");
}
