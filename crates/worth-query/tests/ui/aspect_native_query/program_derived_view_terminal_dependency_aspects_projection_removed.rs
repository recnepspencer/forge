use worth_query::facade::policy::WorthQueryDerivedView;

fn assert_no_terminal_dependency_projection(view: &WorthQueryDerivedView) {
    let _ = view.terminal_dependency_aspects_projection();
}

fn main() {}
