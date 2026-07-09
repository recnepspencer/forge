use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::WorthQueryWorkspace;
use worth_server::{
    WorthServerQueryHandoffOperation, WorthServerQueryWorkspaceBindingError,
    WorthServerQueryWorkspaceBindingRequest, WorthServerQueryWorkspaceBindingTarget,
    WorthServerQueryWorkspaceProvider,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct RealMutationWorkspaceProvider;

impl WorthServerQueryWorkspaceProvider for RealMutationWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "real-mutation-workspace-provider"
    }

    fn bind_workspace(
        &self,
        request: &WorthServerQueryWorkspaceBindingRequest,
    ) -> Result<WorthQueryWorkspace, WorthServerQueryWorkspaceBindingError> {
        match request.target() {
            WorthServerQueryWorkspaceBindingTarget::QueryHandoff {
                operation:
                    WorthServerQueryHandoffOperation::DirectMutation { .. }
                    | WorthServerQueryHandoffOperation::QueryMutation { .. },
            } => {}
            target => {
                return Err(WorthServerQueryWorkspaceBindingError::new(
                    "workspace_target",
                    format!(
                        "real mutation workspace provider only supports mutation handoff targets, got {target:?}"
                    ),
                ));
            }
        }

        let workspace_id = request
            .resolved_request_context()
            .request_context()
            .workspace_target()
            .workspace_id();
        in_memory_test_runtime()
            .with_schema(task_schema())
            .workspace(workspace_id)
            .map_err(|error| {
                WorthServerQueryWorkspaceBindingError::new("workspace_bind", format!("{error:?}"))
            })
    }
}

fn task_schema() -> WorthQueryTestBackendSchema {
    WorthQueryTestBackendSchema::single_collection("Task")
        .aspect("identity.id", "identity.id")
        .expect("task schema identity aspect should be valid")
        .aspect("title.value", "title.value")
        .expect("task schema title aspect should be valid")
}
