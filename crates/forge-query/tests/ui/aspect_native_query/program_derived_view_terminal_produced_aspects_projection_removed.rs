use forge_query::facade::ForgeQueryDerivedView;

fn assert_no_terminal_produced_projection(view: &ForgeQueryDerivedView) {
    let _ = view.terminal_produced_aspects_projection();
}

fn main() {}
