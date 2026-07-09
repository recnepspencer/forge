use worth_query::facade::runtime::WorthQueryAdmittedGraphReadRelation;

fn main() {
    let relation = relation_fixture();
    let _ = relation.terminal_relation_projection();
}

fn relation_fixture() -> WorthQueryAdmittedGraphReadRelation {
    panic!("fixture only")
}
