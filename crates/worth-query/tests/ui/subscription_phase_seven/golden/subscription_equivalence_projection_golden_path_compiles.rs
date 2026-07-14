use worth_query::facade::runtime::QuerySubscriptionFamilySelection;

fn equivalence_projection_golden_path(selection: &QuerySubscriptionFamilySelection) {
    let _ = selection.equivalence_basis().equivalence_projection().label();
}

fn main() {}
