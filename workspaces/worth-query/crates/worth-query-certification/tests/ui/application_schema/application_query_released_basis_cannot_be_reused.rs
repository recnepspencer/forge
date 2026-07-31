use worth_query_execution::facade::primary_graph::WorthQueryApplicationPinnedBasis;

struct Schema;

fn release_then_reuse(basis: WorthQueryApplicationPinnedBasis<Schema>) {
    let _receipt = basis.release();
    let _ = basis.identity();
}

fn main() {}
