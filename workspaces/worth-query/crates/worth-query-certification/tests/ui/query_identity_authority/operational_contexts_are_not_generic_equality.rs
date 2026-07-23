use worth_query::facade::{
    comparison::WorthQueryComparisonContext, history::WorthQueryHistoricalContext,
};

fn require_generic_equality<T: PartialEq>() {}

fn main() {
    require_generic_equality::<WorthQueryHistoricalContext>();
    require_generic_equality::<WorthQueryComparisonContext>();
}
