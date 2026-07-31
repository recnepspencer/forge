use worth_query_execution::facade::primary_graph::WorthQueryApplicationProjectionRow;

struct Schema;
struct Query;

fn main() {
    let _ = WorthQueryApplicationProjectionRow::<Schema, Query>::new;
}
