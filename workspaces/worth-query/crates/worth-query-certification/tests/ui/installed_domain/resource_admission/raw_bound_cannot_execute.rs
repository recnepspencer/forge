use worth_query_host::facade::{
    admission::resource_admission::WorthQueryAdmittedExecutionResourcePlan,
    domain::WorthQueryInstalledDomainOperationAuthority,
    runtime::WorthQueryExecutionRuntime,
};

fn bypass(
    runtime: &WorthQueryExecutionRuntime,
    raw_operation: &WorthQueryInstalledDomainOperationAuthority,
    resources: WorthQueryAdmittedExecutionResourcePlan,
) {
    let _ = runtime.start_direct_resource_attempt(raw_operation, resources);
}

fn main() {}
