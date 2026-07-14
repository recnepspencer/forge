use worth_query::facade::policy::WorthQueryDerivedView;

fn assert_no_terminal_produced_projection(view: &WorthQueryDerivedView) {
    let _ = view.terminal_produced_aspects_projection();
}

fn main() {}
