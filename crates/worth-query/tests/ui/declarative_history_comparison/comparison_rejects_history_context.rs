use worth_query::facade::comparison::WorthQueryComparisonRefinement;
use worth_query::facade::history::WorthQueryHistoricalContext;

fn mix(declaration: WorthQueryComparisonRefinement, context: WorthQueryHistoricalContext) {
    let _request = declaration.using(context);
}

fn main() {}
