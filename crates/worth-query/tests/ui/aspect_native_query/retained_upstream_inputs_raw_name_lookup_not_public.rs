use worth_query::facade::{WorthQueryDerivedView, WorthQueryRetainedUpstreamInputs};

fn main() {
    let upstreams = upstreams_fixture();
    let declaration = declaration_fixture();
    let _ = upstreams.live_rows("tasks.table");
    let _ = upstreams.retained_computed_rows("computed.titles");
    let _ = upstreams.single_retained_computed_row("computed.titles");
    let _ = upstreams.live_view_names();
    let _ = upstreams.computed_view_names();
    let _ = upstreams.declared_live_rows(&declaration, "tasks.table");
    let _ = upstreams.declared_retained_computed_rows(&declaration, "computed.titles");
    let _ = upstreams.single_declared_retained_computed_row(&declaration, "computed.titles");
    let _ = WorthQueryRetainedUpstreamInputs::new(
        Vec::new(),
        Vec::new(),
    );
}

fn upstreams_fixture() -> WorthQueryRetainedUpstreamInputs {
    panic!("fixture only")
}

fn declaration_fixture() -> WorthQueryDerivedView {
    panic!("fixture only")
}
