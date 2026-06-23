use forge_query::facade::{ForgeQueryRuntime, ForgeQueryWorkspace};

fn runtime_raw_name(mut runtime: ForgeQueryRuntime) {
    let _ = runtime.drain_derived_patches("computed.title_list");
}

fn workspace_raw_name(mut workspace: ForgeQueryWorkspace) {
    let _ = workspace.observe_computed("computed.title_list");
}

fn main() {}
