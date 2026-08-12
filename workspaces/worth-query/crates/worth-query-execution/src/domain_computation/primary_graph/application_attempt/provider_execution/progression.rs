use super::phase::{progress_application_commit, WorthQueryProgressedApplicationCommit};

pub(super) fn execute_provider_progression<Schema, Operation, Input, Scope>(
    application: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
        Schema,
    >,
    running: super::phase::WorthQueryRunningApplicationCommit<Schema, Operation, Input, Scope>,
) -> WorthQueryProgressedApplicationCommit
where
    Schema: worth_query_installation::facade::ApplicationSchema,
    Input: Clone + Send + Sync + 'static,
{
    progress_application_commit(application, running)
}
