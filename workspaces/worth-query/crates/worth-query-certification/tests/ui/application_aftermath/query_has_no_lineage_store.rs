use worth_query_host::facade::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use worth_query_host::facade::domain::ApplicationSchema;

fn cannot_read_parallel_lineage<Schema: ApplicationSchema>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
) {
    let _ = runtime.linear_lineage_head();
}

fn main() {}
