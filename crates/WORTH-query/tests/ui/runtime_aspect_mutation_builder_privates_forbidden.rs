use worth_query::facade::WorthQueryAspectMutationBuilder;
use std::collections::BTreeSet;

fn main() {
    let _builder = WorthQueryAspectMutationBuilder {
        aspects: Vec::new(),
        seen_aspects: BTreeSet::new(),
        error: None,
    };
}
