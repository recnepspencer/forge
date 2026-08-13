//! R8.66: callers cannot forge performed re-dispatch evidence.

use worth_query_execution::facade::primary_graph::WorthQueryPerformedExternalRedispatch;

fn main() {
    let _ = WorthQueryPerformedExternalRedispatch {};
    let _ = <WorthQueryPerformedExternalRedispatch>::record;
}
