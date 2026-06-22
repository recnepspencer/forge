use forge_query::facade::runtime::ForgeQueryAdmittedGraphReadRelation;

fn main() {
    let relation = relation_fixture();
    let _: &str = relation.relation();
}

fn relation_fixture() -> ForgeQueryAdmittedGraphReadRelation {
    panic!("fixture only")
}
