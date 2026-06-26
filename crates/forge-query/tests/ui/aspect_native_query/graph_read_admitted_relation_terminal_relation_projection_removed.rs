use forge_query::facade::runtime::ForgeQueryAdmittedGraphReadRelation;

fn main() {
    let relation = relation_fixture();
    let _ = relation.terminal_relation_projection();
}

fn relation_fixture() -> ForgeQueryAdmittedGraphReadRelation {
    panic!("fixture only")
}
