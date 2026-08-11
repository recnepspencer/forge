#![allow(unreachable_code)]

use worth_query_execution::facade::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use worth_query_host::facade::domain::ApplicationSchema;

fn bypass_runtime_admission<Schema: ApplicationSchema>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
) {
    let _ = application.admit_external_dispatch_attempt(todo!());
}

fn main() {}
