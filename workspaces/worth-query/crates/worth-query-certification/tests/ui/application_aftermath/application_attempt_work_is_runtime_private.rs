use worth_query_host::facade::domain::ApplicationSchema;
use worth_query_host::facade::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

fn cannot_read_runtime_work<Schema: ApplicationSchema>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
) {
    let _ = runtime.application_attempt_work();
}

fn main() {}
