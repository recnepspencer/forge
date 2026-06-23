use forge_query::facade::{
    ForgeQueryDerivedArtifactBinding, ForgeQueryDerivedViewHandle, ForgeQueryNativeRow,
};

fn main() {
    let binding = binding_fixture();
    let left = view_fixture();
    let right = view_fixture();

    let _ = binding.consume_scalar_fields(&left, ["title.value"]);
    let _ = binding.verify_scalar_alignment(
        &left,
        &right,
        [("left.title", "right.title")],
    );
}

fn binding_fixture() -> ForgeQueryDerivedArtifactBinding {
    panic!("fixture only")
}

fn view_fixture() -> ForgeQueryDerivedViewHandle<ForgeQueryNativeRow> {
    panic!("fixture only")
}
