/// Frozen application meaning before any host or platform mechanics are
/// selected. This value is move-only and has no public host-binding operation.
#[must_use]
pub struct WorthUiHostNeutralApp {
    prepared: crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    mounted_frame_retention_budget: crate::mounting::UiMountedFrameRetentionBudget,
    host_observation_capacity: crate::facade::observation_report::UiHostObservationCapacity,
    font_collection: std::sync::Arc<worth_ui_text::UiGlobalFontCollection>,
}

impl WorthUiHostNeutralApp {
    pub(crate) fn new(
        prepared: crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
        mounted_frame_retention_budget: crate::mounting::UiMountedFrameRetentionBudget,
        host_observation_capacity: crate::facade::observation_report::UiHostObservationCapacity,
        font_collection: std::sync::Arc<worth_ui_text::UiGlobalFontCollection>,
    ) -> Self {
        Self {
            prepared,
            mounted_frame_retention_budget,
            host_observation_capacity,
            font_collection,
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

    /// Bind the one qualified native mechanics bundle inside the runtime-owned
    /// platform gate. No public application surface can call this transition.
    pub(crate) fn bind_qualified_native(
        self,
        host: worth_ui_host_native::WorthUiNativeMechanicsAdapter,
    ) -> crate::facade::WorthUiApp {
        self.bind_exact_host(host)
    }

    pub(in crate::facade::entry) fn bind_exact_host<Host>(
        self,
        host: Host,
    ) -> crate::facade::WorthUiApp
    where
        Host: crate::facade::measurement_exchange::WorthUiOperationalHostAdapter + 'static,
    {
        let mut plan =
            crate::facade::prepared_application_authority::WorthUiHostSessionPlan::prepare(host);
        plan.set_mounted_frame_retention_budget(self.mounted_frame_retention_budget);
        plan.set_host_observation_capacity(self.host_observation_capacity);
        crate::facade::WorthUiApp::from_prepared_authority(
            self.prepared,
            plan,
            self.font_collection,
        )
    }
}
