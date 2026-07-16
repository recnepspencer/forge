use worth_query::facade::runtime::{WorthQueryDerivedArtifactBinding, WorthQueryDerivedViewHandle, WorthQueryUnrefinedLiveShape};

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

fn binding_fixture() -> WorthQueryDerivedArtifactBinding {
    panic!("fixture only")
}

fn view_fixture() -> WorthQueryDerivedViewHandle<WorthQueryUnrefinedLiveShape> {
    panic!("fixture only")
}
