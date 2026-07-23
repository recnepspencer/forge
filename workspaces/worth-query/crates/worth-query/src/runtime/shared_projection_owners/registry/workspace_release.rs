use super::*;

impl super::super::super::WorthQueryWorkspace {
    pub(crate) fn register_shared_projection_owner(
        &mut self,
        bundle: crate::domain_installation::WorthQueryCheckedSharedOwnerRegistration,
    ) -> Result<
        WorthQuerySharedOwnerRegistration,
        crate::domain_installation::WorthQueryCheckedSharedOwnerRegistration,
    > {
        self.runtime.register_shared_projection_owner(bundle)
    }

    pub(crate) fn register_singleton_projection_owner(
        &mut self,
        bundle: crate::domain_installation::WorthQueryCheckedSharedOwnerRegistration,
    ) -> Result<
        WorthQuerySharedOwnerRegistration,
        crate::domain_installation::WorthQueryCheckedSharedOwnerRegistration,
    > {
        self.runtime.register_singleton_projection_owner(bundle)
    }

    pub(crate) fn release_shared_projection_lease(
        &mut self,
        capability: &std::sync::Arc<super::super::super::WorthQueryManagedLiveWorkspaceCapability>,
        token: WorthQuerySharedProjectionLeaseToken,
    ) -> Result<WorthQuerySharedLeaseRelease, WorthQuerySharedLeaseReleaseError> {
        let capability_counters = WorthQuerySharedLeaseReleaseCounters {
            capability_checks: 1,
            ..WorthQuerySharedLeaseReleaseCounters::default()
        };
        if let Err(error) = self
            .runtime
            .admit_managed_live_capability(capability, "shared-projection-owner")
        {
            return Err(WorthQuerySharedLeaseReleaseError {
                token,
                error,
                counters: capability_counters,
            });
        }
        match self.runtime.release_shared_projection_lease(token) {
            Ok(mut release) => {
                release.counters.capability_checks = 1;
                Ok(release)
            }
            Err(mut stopped) => {
                stopped.counters.capability_checks = 1;
                Err(stopped)
            }
        }
    }
}
