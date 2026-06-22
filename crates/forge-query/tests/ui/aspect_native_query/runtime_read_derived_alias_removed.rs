use forge_query::facade::{ForgeQueryDerivedViewHandle, ForgeQueryNativeRow, ForgeQueryRuntime};

fn main() {
    let runtime = runtime_fixture();
    let view = view_fixture();
    let _ = runtime.read_derived(&view);
}

fn runtime_fixture() -> ForgeQueryRuntime {
    panic!("fixture only")
}

fn view_fixture() -> ForgeQueryDerivedViewHandle<ForgeQueryNativeRow> {
    panic!("fixture only")
}
