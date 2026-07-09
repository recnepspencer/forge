use worth_query::facade::{WorthQueryDerivedViewHandle, WorthQueryNativeRow, WorthQueryRuntime};

fn main() {
    let runtime = runtime_fixture();
    let view = view_fixture();
    let _ = runtime.read_derived(&view);
}

fn runtime_fixture() -> WorthQueryRuntime {
    panic!("fixture only")
}

fn view_fixture() -> WorthQueryDerivedViewHandle<WorthQueryNativeRow> {
    panic!("fixture only")
}
