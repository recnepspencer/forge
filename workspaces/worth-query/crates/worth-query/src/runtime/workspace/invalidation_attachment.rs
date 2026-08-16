use super::WorthQueryWorkspace;
use worth_query_execution::facade::primary_graph::WorthQueryGranularInvalidationInstallation;

impl WorthQueryWorkspace {
    pub(crate) fn is_attached_to_primary_runtime(
        &self,
        primary: &WorthQueryGranularInvalidationInstallation,
    ) -> bool {
        self.runtime
            .primary_runtime_invalidation_installation
            .as_ref()
            .is_some_and(|installed| installed.is_same_current_runtime_as(primary))
    }
}
