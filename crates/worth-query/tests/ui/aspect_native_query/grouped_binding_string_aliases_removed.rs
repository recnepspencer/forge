use worth_query::facade::runtime::QueryResultBindingProof;

fn main() {
    let binding = binding_fixture();
    let _ = binding.source_aspect();
    let _ = binding.source_field();
    let _ = binding.field_key();
}

fn binding_fixture() -> QueryResultBindingProof {
    panic!("fixture only")
}
