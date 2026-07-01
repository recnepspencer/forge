use worth_ui_runtime::graph::topology::parent_child_topology::UiGraphTopology;

fn main() {
    let _ = std::mem::MaybeUninit::<UiGraphTopology>::uninit();
}
