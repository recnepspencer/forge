use worth_query::facade::policy::WorthQueryDerivedView;

fn assert_no_neutral_derived_view_aspect_aliases(view: &WorthQueryDerivedView) {
    let _ = view.dependency_aspects();
    let _ = view.produced_aspects();
}

fn main() {}
