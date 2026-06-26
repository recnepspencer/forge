use forge_query::facade::ForgeQueryRetainedScalarFactSet;

fn main() {
    let facts = fact_set_fixture();
    let _ = facts.field_value("identity.id");
}

fn fact_set_fixture() -> ForgeQueryRetainedScalarFactSet {
    panic!("fixture only")
}
