use worth_query_execution::facade::primary_graph::{
    WorthQueryApplicationHistoricalBasis, WorthQueryApplicationPreviewBasis,
};

struct Schema;

fn assert_clone<T: Clone>() {}

fn main() {
    assert_clone::<WorthQueryApplicationHistoricalBasis<Schema>>();
    assert_clone::<WorthQueryApplicationPreviewBasis<Schema>>();

    let _ = WorthQueryApplicationHistoricalBasis::<Schema> {};
    let _ = WorthQueryApplicationPreviewBasis::<Schema> {};
}
