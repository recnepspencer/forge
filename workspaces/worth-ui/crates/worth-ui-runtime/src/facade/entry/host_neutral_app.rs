/// Frozen application meaning before any host or platform mechanics are
/// selected. This value is move-only and has no public host-binding operation.
#[must_use]
pub struct WorthUiHostNeutralApp {
    pub(crate) prepared:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    pub(crate) mounted_frame_retention_budget: crate::mounting::UiMountedFrameRetentionBudget,
    pub(crate) host_observation_capacity:
        crate::facade::observation_report::UiHostObservationCapacity,
}

impl WorthUiHostNeutralApp {
    pub(crate) fn new(
        prepared: crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
        mounted_frame_retention_budget: crate::mounting::UiMountedFrameRetentionBudget,
        host_observation_capacity: crate::facade::observation_report::UiHostObservationCapacity,
    ) -> Self {
        Self {
            prepared,
            mounted_frame_retention_budget,
            host_observation_capacity,
        }
    }

    pub fn generation_identity(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        self.prepared.generation_identity()
    }

    /// Inspect the immutable capability snapshot without selecting or
    /// acquiring a host.
    pub fn capabilities(&self) -> &crate::facade::registry::snapshot::CapabilitySnapshot {
        self.prepared.capabilities()
    }

    pub fn mounted_frame_retention_budget(&self) -> crate::mounting::UiMountedFrameRetentionBudget {
        self.mounted_frame_retention_budget
    }

    pub fn host_observation_capacity(
        &self,
    ) -> crate::facade::observation_report::UiHostObservationCapacity {
        self.host_observation_capacity
    }
}
