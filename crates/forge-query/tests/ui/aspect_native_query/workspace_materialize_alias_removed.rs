use forge_query::facade::{ForgeQueryDerivedViewHandle, ForgeQueryNativeRow, ForgeQueryWorkspace};

fn main() {
    let workspace = workspace_fixture();
    let view = view_fixture();
    let _ = workspace.materialize(&view);
}

fn workspace_fixture() -> ForgeQueryWorkspace {
    panic!("fixture only")
}

fn view_fixture() -> ForgeQueryDerivedViewHandle<ForgeQueryNativeRow> {
    panic!("fixture only")
}
