use forge_query::facade::ForgeQueryDerivedView;

fn assert_no_neutral_derived_view_aspect_aliases(view: &ForgeQueryDerivedView) {
    let _ = view.dependency_aspects();
    let _ = view.produced_aspects();
}

fn main() {}
