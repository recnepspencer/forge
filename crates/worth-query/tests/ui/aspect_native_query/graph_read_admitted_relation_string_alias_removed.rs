use worth_query::facade::runtime::WorthQueryAdmittedGraphReadRelation;

fn main() {
    let relation = relation_fixture();
    let _: &str = relation.relation();
}

fn relation_fixture() -> WorthQueryAdmittedGraphReadRelation {
    panic!("fixture only")
}
