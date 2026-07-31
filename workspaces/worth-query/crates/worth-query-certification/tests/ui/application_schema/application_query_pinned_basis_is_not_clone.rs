use worth_query_execution::facade::primary_graph::WorthQueryApplicationPinnedBasis;

struct Schema;

fn assert_clone<T: Clone>() {}

fn main() {
    assert_clone::<WorthQueryApplicationPinnedBasis<Schema>>();
}
