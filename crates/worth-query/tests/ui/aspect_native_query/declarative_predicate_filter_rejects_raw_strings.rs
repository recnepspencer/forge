use worth_query::facade::foundation::{DeclarativeEqualityFilter, DeclarativeNativeComparisonFilter, DeclarativePresenceFilter, DeclarativeSetMembershipFilter, DeclarativeStringContainsFilter, WorthQueryPredicateOperand};

fn main() {
    let _ =
        DeclarativeEqualityFilter::new("identity", "id", WorthQueryPredicateOperand::string("a".into()));
    let _ = DeclarativeNativeComparisonFilter::greater_than("metrics", "rank", 1);
    let _ = DeclarativeNativeComparisonFilter::less_than("metrics", "rank", 1);
    let _ = DeclarativeStringContainsFilter::new("title", "value", "needle");
    let _ = DeclarativeSetMembershipFilter::new(
        "status",
        "value",
        [WorthQueryPredicateOperand::string("open".into())],
    );
    let _ = DeclarativePresenceFilter::is_present("owner", "id");
}
