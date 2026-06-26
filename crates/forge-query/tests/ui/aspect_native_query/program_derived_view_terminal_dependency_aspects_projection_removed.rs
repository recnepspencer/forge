use forge_query::facade::ForgeQueryDerivedView;

fn assert_no_terminal_dependency_projection(view: &ForgeQueryDerivedView) {
    let _ = view.terminal_dependency_aspects_projection();
}

fn main() {}
