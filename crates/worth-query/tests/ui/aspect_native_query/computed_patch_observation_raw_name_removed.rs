use worth_query::facade::runtime::{WorthQueryRuntime, WorthQueryWorkspace};

fn runtime_raw_name(mut runtime: WorthQueryRuntime) {
    let _ = runtime.drain_derived_patches("computed.title_list");
}

fn workspace_raw_name(mut workspace: WorthQueryWorkspace) {
    let _ = workspace.observe_computed("computed.title_list");
}

fn main() {}
