use worth_query::facade::read::declare;

fn main() {
    let declaration = declare(|read| unreachable!()).unwrap();
    declaration.plan_graph_read_access_in_authority();
}
