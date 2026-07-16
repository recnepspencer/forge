use worth_query::facade::runtime::{WorthQueryDerivedViewHandle, WorthQueryUnrefinedLiveShape, WorthQueryWorkspace};

fn main() {
    let workspace = workspace_fixture();
    let view = view_fixture();
    let _ = workspace.materialize(&view);
}

fn workspace_fixture() -> WorthQueryWorkspace {
    panic!("fixture only")
}

fn view_fixture() -> WorthQueryDerivedViewHandle<WorthQueryUnrefinedLiveShape> {
    panic!("fixture only")
}
