use forge_query::facade::ForgeQueryLiveView;

fn cannot_read_boundary_state_through_cheap_value_getter(handle: ForgeQueryLiveView<()>) {
    let _ = handle.value();
}

fn main() {}
