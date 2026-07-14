use worth_query::facade::runtime::WorthQueryVerifiedAssumptionSet;

fn assert_no_neutral_asserted_path_alias(assumptions: &WorthQueryVerifiedAssumptionSet) {
    let _ = assumptions.asserted_aspect_paths();
}

fn main() {}
