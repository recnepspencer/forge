use worth_query_host::facade::{
    admission::resource_admission::WorthQueryAdmittedWorkflowResourcePlan,
    domain::WorthQueryInstalledDomainOperationAuthority,
    runtime::WorthQueryExecutionRuntime,
};

fn bypass(
    runtime: &WorthQueryExecutionRuntime,
    raw_operation: &WorthQueryInstalledDomainOperationAuthority,
    resources: WorthQueryAdmittedWorkflowResourcePlan,
) {
    let _ = runtime.start_workflow_resource_attempt(raw_operation, resources);
}

fn main() {}
