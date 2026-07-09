use worth_query::facade::WorthQueryRetainedScalarFactSet;

fn main() {
    let facts = fact_set_fixture();
    let _ = facts.field_value("identity.id");
}

fn fact_set_fixture() -> WorthQueryRetainedScalarFactSet {
    panic!("fixture only")
}
