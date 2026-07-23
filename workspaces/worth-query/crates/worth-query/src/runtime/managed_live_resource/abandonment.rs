use super::super::*;

impl WorthQueryRuntime {
    pub(crate) fn managed_live_capability(
        &self,
    ) -> std::sync::Arc<WorthQueryManagedLiveWorkspaceCapability> {
        std::sync::Arc::clone(&self.managed_live_resource_capability)
    }

    pub(crate) fn admit_managed_live_capability(
        &self,
        capability: &std::sync::Arc<WorthQueryManagedLiveWorkspaceCapability>,
        resource_name: &str,
    ) -> Result<(), WorthQueryRuntimeError> {
        if WorthQueryManagedLiveWorkspaceCapability::same_instance(
            &self.managed_live_resource_capability,
            capability,
        ) {
            return Ok(());
        }
        Err(WorthQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: resource_name.to_string(),
            stage: "managed-workspace-capability-admission",
            message: "managed live handle belongs to a different workspace instance".to_string(),
        })
    }

    pub(crate) fn reap_abandoned_managed_live_resources(
        &mut self,
    ) -> Result<(), WorthQueryRuntimeError> {
        self.reap_abandoned_shared_projection_leases()?;
        let abandoned = self.managed_live_resource_capability.take_abandoned();
        let mut pending = abandoned.into_iter();
        while let Some(resource) = pending.next() {
            if let Err(error) = self.close_managed_live_view(
                resource.view(),
                WorthQueryManagedLiveResourceCloseCause::Abandonment,
            ) {
                let mut retry = vec![resource];
                retry.extend(pending);
                self.managed_live_resource_capability
                    .restore_abandoned(retry);
                return Err(error);
            }
        }
        Ok(())
    }

    pub(crate) fn managed_live_resource_is_abandoned(&self, name: &str) -> bool {
        self.managed_live_resource_capability.contains_name(name)
    }
}
