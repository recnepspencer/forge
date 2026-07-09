use worth_query::facade::WorthQueryLiveView;

fn cannot_read_boundary_state_through_cheap_value_getter(handle: WorthQueryLiveView<()>) {
    let _ = handle.value();
}

fn main() {}
