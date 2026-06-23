use forge_query::facade::ForgeQueryDerivedArtifactBinding;

fn main() {
    let binding = binding_fixture();

    let _ = binding.consume_scalar_fields_by_name("derived.example", ["field.example"]);
    let _ = binding.verify_scalar_alignment_by_name(
        "derived.left",
        "derived.right",
        [("left.field", "right.field")],
    );
}

fn binding_fixture() -> ForgeQueryDerivedArtifactBinding {
    panic!("fixture only")
}
